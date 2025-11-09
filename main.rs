#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui_extras::RetainedImage;
use image::{self, DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    time::SystemTime,
};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;

static TEMP_RESULT_PATH: &str = "temp.png";

// YEET v2 Format Specification
// Header (variable size):
// - Magic bytes: "YEET" (4 bytes)
// - Version: 2 (1 byte)
// - Flags: (1 byte)
//   - Bit 0: Compression (0=none, 1=zlib)
//   - Bit 1: Alpha channel (0=RGB, 1=RGBA)
//   - Bit 2: Binary mode (0=hex text, 1=binary)
//   - Bits 3-7: Reserved
// - Width: u32 (4 bytes)
// - Height: u32 (4 bytes)
// - Metadata length: u16 (2 bytes)
// - Metadata: JSON string (variable)
// - Data length: u32 (4 bytes)
// - Pixel data: (variable)

#[derive(Debug)]
struct YeetMetadata {
    author: Option<String>,
    created: Option<String>,
    software: String,
}

impl YeetMetadata {
    fn new() -> Self {
        Self {
            author: None,
            created: Some(format!("{:?}", SystemTime::now())),
            software: "YEET v2.0".to_string(),
        }
    }

    fn to_json(&self) -> String {
        let mut json = String::from("{");
        if let Some(ref author) = self.author {
            json.push_str(&format!("\"author\":\"{}\",", author));
        }
        if let Some(ref created) = self.created {
            json.push_str(&format!("\"created\":\"{}\",", created));
        }
        json.push_str(&format!("\"software\":\"{}\"", self.software));
        json.push_str("}");
        json
    }

    fn from_json(json: &str) -> Self {
        // Simple JSON parsing (you could use serde_json for production)
        let mut metadata = Self::new();
        
        if let Some(author_start) = json.find("\"author\":\"") {
            if let Some(author_end) = json[author_start + 10..].find("\"") {
                metadata.author = Some(json[author_start + 10..author_start + 10 + author_end].to_string());
            }
        }
        
        metadata
    }
}

fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn decompress_data(data: &[u8]) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();
    decompressed
}

fn png_to_yeet_v2(path: PathBuf, compress: bool, use_binary: bool) -> Result<(), std::io::Error> {
    let img = image::open(&path).expect("File not found!");
    let width = img.width();
    let height = img.height();
    
    // Check if image has alpha channel
    let has_alpha = matches!(img.color(), image::ColorType::Rgba8 | image::ColorType::Rgba16);
    
    // Prepare metadata
    let metadata = YeetMetadata::new();
    let metadata_json = metadata.to_json();
    let metadata_bytes = metadata_json.as_bytes();
    
    // Prepare pixel data
    let pixel_data = if use_binary {
        // Binary mode: 3 or 4 bytes per pixel
        let mut data = Vec::with_capacity((width * height * if has_alpha { 4 } else { 3 }) as usize);
        for pixel in img.pixels() {
            data.push(pixel.2[0]); // R
            data.push(pixel.2[1]); // G
            data.push(pixel.2[2]); // B
            if has_alpha {
                data.push(pixel.2[3]); // A
            }
        }
        data
    } else {
        // Hex text mode: 6 or 8 characters per pixel
        let mut str = String::with_capacity((width * height * if has_alpha { 8 } else { 6 }) as usize);
        for pixel in img.pixels() {
            str.push_str(&format!("{:02X}{:02X}{:02X}", pixel.2[0], pixel.2[1], pixel.2[2]));
            if has_alpha {
                str.push_str(&format!("{:02X}", pixel.2[3]));
            }
        }
        str.into_bytes()
    };
    
    // Apply compression if requested
    let (final_data, is_compressed) = if compress {
        (compress_data(&pixel_data), true)
    } else {
        (pixel_data, false)
    };
    
    // Build flags byte
    let mut flags: u8 = 0;
    if is_compressed { flags |= 0b00000001; }
    if has_alpha { flags |= 0b00000010; }
    if use_binary { flags |= 0b00000100; }
    
    // Write file
    if let Some(path_str) = path.to_str() {
        let path_to_yeet = path_str.replace(".png", ".yeet");
        let mut file = File::create(path_to_yeet)?;
        
        // Write header
        file.write_all(b"YEET")?;                                    // Magic bytes
        file.write_all(&[2])?;                                       // Version
        file.write_all(&[flags])?;                                   // Flags
        file.write_all(&width.to_le_bytes())?;                       // Width
        file.write_all(&height.to_le_bytes())?;                      // Height
        file.write_all(&(metadata_bytes.len() as u16).to_le_bytes())?; // Metadata length
        file.write_all(metadata_bytes)?;                             // Metadata
        file.write_all(&(final_data.len() as u32).to_le_bytes())?;  // Data length
        file.write_all(&final_data)?;                                // Pixel data
        
        file.flush()?;
        
        let original_size = width * height * if has_alpha { 4 } else { 3 };
        let compression_ratio = 100.0 * (1.0 - (final_data.len() as f64 / original_size as f64));
        println!("✓ Converted to YEET v2");
        println!("  Dimensions: {}×{}", width, height);
        println!("  Alpha: {}", has_alpha);
        println!("  Binary mode: {}", use_binary);
        println!("  Compressed: {} ({:.1}% reduction)", is_compressed, compression_ratio);
        println!("  Final size: {} bytes", final_data.len() + 20 + metadata_bytes.len());
    }
    
    Ok(())
}

fn yeet_to_png_v2(path: PathBuf) -> (u32, u32) {
    let mut file = File::open(&path).expect("Couldn't open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Couldn't read file");
    
    // Check if this is a v2 file (starts with "YEET" magic bytes)
    if contents.len() < 4 || &contents[0..4] != b"YEET" {
        // Fallback to v1 format
        eprintln!("Detected v1 format, using legacy parser");
        drop(file); // Close the file before calling v1
        return yeet_to_png_v1(path);
    }
    
    eprintln!("Detected v2 format");
    let mut offset = 0;
    
    // Read magic bytes
    let magic = &contents[offset..offset + 4];
    offset += 4;
    
    if magic != b"YEET" {
        panic!("Invalid YEET file: missing magic bytes");
    }
    
    // Read version
    let version = contents[offset];
    offset += 1;
    
    if version == 1 {
        // Fallback to v1 format (legacy)
        return yeet_to_png_v1(path);
    } else if version != 2 {
        panic!("Unsupported YEET version: {}", version);
    }
    
    // Read flags
    let flags = contents[offset];
    offset += 1;
    
    let is_compressed = (flags & 0b00000001) != 0;
    let has_alpha = (flags & 0b00000010) != 0;
    let is_binary = (flags & 0b00000100) != 0;
    
    // Read dimensions
    let width = u32::from_le_bytes([
        contents[offset],
        contents[offset + 1],
        contents[offset + 2],
        contents[offset + 3],
    ]);
    offset += 4;
    
    let height = u32::from_le_bytes([
        contents[offset],
        contents[offset + 1],
        contents[offset + 2],
        contents[offset + 3],
    ]);
    offset += 4;
    
    // Read metadata
    let metadata_len = u16::from_le_bytes([contents[offset], contents[offset + 1]]) as usize;
    offset += 2;
    
    let _metadata_json = String::from_utf8_lossy(&contents[offset..offset + metadata_len]);
    offset += metadata_len;
    
    // Read data length
    let data_len = u32::from_le_bytes([
        contents[offset],
        contents[offset + 1],
        contents[offset + 2],
        contents[offset + 3],
    ]) as usize;
    offset += 4;
    
    // Read pixel data
    let mut pixel_data = contents[offset..offset + data_len].to_vec();
    
    // Decompress if needed
    if is_compressed {
        pixel_data = decompress_data(&pixel_data);
    }
    
    // Parse pixel data based on mode
    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = if is_binary {
        // Binary mode
        let bytes_per_pixel = if has_alpha { 4 } else { 3 };
        ImageBuffer::from_fn(width, height, |x, y| {
            let idx = ((y * width + x) as usize) * bytes_per_pixel;
            let r = pixel_data[idx];
            let g = pixel_data[idx + 1];
            let b = pixel_data[idx + 2];
            let a = if has_alpha { pixel_data[idx + 3] } else { 255 };
            Rgba([r, g, b, a])
        })
    } else {
        // Hex text mode
        let hex_str = String::from_utf8_lossy(&pixel_data);
        let chars_per_pixel = if has_alpha { 8 } else { 6 };
        ImageBuffer::from_fn(width, height, |x, y| {
            let idx = ((y * width + x) as usize) * chars_per_pixel;
            let hex = &hex_str[idx..idx + chars_per_pixel];
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = if has_alpha {
                u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
            } else {
                255
            };
            Rgba([r, g, b, a])
        })
    };
    
    // Save as PNG
    img_buffer.save(TEMP_RESULT_PATH).expect("Failed to save temp PNG");
    
    (width, height)
}

fn yeet_to_png_v1(path: PathBuf) -> (u32, u32) {
    // Legacy v1 format support
    let mut contents: Vec<u8> = fs::read(&path).expect("Couldn't read file.");
    let binding: Vec<_> = contents.drain(0..8).collect();
    
    let width = u32::from_ne_bytes([binding[0], binding[1], binding[2], binding[3]]);
    let height = u32::from_ne_bytes([binding[4], binding[5], binding[6], binding[7]]);
    
    let sanitized_content = String::from_utf8_lossy(&contents).replace("\n", "");
    
    let result: Vec<&str> = sanitized_content
        .as_bytes()
        .chunks(6)
        .map(std::str::from_utf8)
        .collect::<Result<_, _>>()
        .expect("Invalid UTF-8 sequence");
    
    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        let idx = (y * width + x) as usize;
        if idx < result.len() {
            let hex = format!("#{}", result[idx]);
            if let Ok(color) = hex.parse::<css_color_parser::Color>() {
                return Rgba([color.r, color.g, color.b, 255]);
            }
        }
        Rgba([0, 0, 0, 255])
    });
    
    img_buffer.save(TEMP_RESULT_PATH).expect("Failed to save temp PNG");
    
    (width, height)
}

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  View:    {} <file.yeet>", args[0]);
        eprintln!("  Convert: {} compile <file.png> [--compress] [--binary]", args[0]);
        eprintln!("  Batch:   {} batch <directory>", args[0]);
        std::process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "compile" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                eprintln!("Usage: {} compile <file.png> [--compress] [--binary]", args[0]);
                std::process::exit(1);
            }
            
            let path: PathBuf = (&args[2]).into();
            let compress = args.contains(&"--compress".to_string());
            let binary = args.contains(&"--binary".to_string());
            
            match png_to_yeet_v2(path, compress, binary) {
                Ok(()) => {
                    println!("✓ Successfully converted PNG to YEET v2");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("✗ Failed to convert: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "batch" => {
            if args.len() < 3 {
                eprintln!("Error: No directory specified");
                eprintln!("Usage: {} batch <directory>", args[0]);
                std::process::exit(1);
            }
            
            let dir_path = PathBuf::from(&args[2]);
            let compress = args.contains(&"--compress".to_string());
            let binary = args.contains(&"--binary".to_string());
            
            if !dir_path.is_dir() {
                eprintln!("Error: Not a valid directory");
                std::process::exit(1);
            }
            
            let entries = fs::read_dir(&dir_path).expect("Failed to read directory");
            let mut count = 0;
            
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "png" {
                            println!("\nConverting: {:?}", path.file_name().unwrap());
                            match png_to_yeet_v2(path, compress, binary) {
                                Ok(()) => count += 1,
                                Err(e) => eprintln!("  Error: {}", e),
                            }
                        }
                    }
                }
            }
            
            println!("\n✓ Batch conversion complete: {} files", count);
            Ok(())
        }
        _ => {
            // Assume it's a file to view
            let file_path: PathBuf = command.into();
            
            if !file_path.exists() {
                eprintln!("Error: File not found: {:?}", file_path);
                std::process::exit(1);
            }
            
            let (width, height) = yeet_to_png_v2(file_path);
            
            let options = eframe::NativeOptions {
                resizable: true,
                initial_window_size: Some(egui::vec2(width as f32, height as f32)),
                ..Default::default()
            };
            
            eframe::run_native(
                "YEET Image Viewer",
                options,
                Box::new(|_cc| Box::<ImagePreview>::default()),
            )
        }
    }
}

struct ImagePreview {
    image: RetainedImage,
}

impl Default for ImagePreview {
    fn default() -> Self {
        let image_data = std::fs::read(TEMP_RESULT_PATH).expect("Failed to read image file");
        fs::remove_file(TEMP_RESULT_PATH).ok();
        
        Self {
            image: RetainedImage::from_image_bytes(TEMP_RESULT_PATH, &image_data).unwrap(),
        }
    }
}

impl eframe::App for ImagePreview {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                self.image.show(ui);
            });
        });
    }
}
