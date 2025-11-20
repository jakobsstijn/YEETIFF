//! YEET v3 - Next-Generation Image Format
//!
//! **Status:** Alpha / Experimental
//!
//! New Features:
//! - ICC color profile support
//! - Multi-frame animation
//! - Enhanced compression (Brotli/Zstd)
//! - HDR support (16-bit per channel)
//! - Extended EXIF-like metadata
//!
//! This is a work-in-progress implementation. For production use, see yeet-core (v2).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)] // Remove when implementation is complete

use eframe::egui;
use egui_extras::RetainedImage;
use image::{ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

// ============================================================================
// YEET v3 Format Specification (DRAFT)
// ============================================================================
//
// Header:
// - Magic bytes: "YEET" (4 bytes)
// - Version: 3 (1 byte)
// - Flags: (1 byte)
//   - Bit 0-1: Compression (00=none, 01=zlib, 10=brotli, 11=zstd)
//   - Bit 2: Alpha channel (0=RGB, 1=RGBA)
//   - Bit 3: Binary mode (0=hex, 1=binary)
//   - Bit 4: Animation (0=single, 1=multi-frame)
//   - Bit 5: ICC profile embedded (0=no, 1=yes)
//   - Bit 6: HDR mode (0=8-bit, 1=16-bit)
//   - Bit 7: Reserved
// - Width: u32 (4 bytes)
// - Height: u32 (4 bytes)
// - Frame count: u32 (4 bytes, 1 for static images)
// - Loop count: u32 (4 bytes, 0=infinite)
// - Metadata length: u32 (4 bytes)
// - Metadata: JSON (variable)
// - ICC profile length: u32 (4 bytes, 0 if none)
// - ICC profile data: (variable)
// - Frame data: (variable, repeated for animations)
//
// Frame structure (for animations):
// - Frame delay: u32 (4 bytes, milliseconds)
// - Frame data length: u32 (4 bytes)
// - Frame pixel data: (variable)
//
// ============================================================================

static TEMP_RESULT_PATH: &str = "temp_v3.png";

// ============================================================================
// Data Structures
// ============================================================================

/// Extended metadata for v3 format
#[derive(Debug, Clone, Serialize, Deserialize)]
struct YeetMetadataV3 {
    // Basic info
    author: Option<String>,
    created: Option<String>,
    software: String,
    
    // Color management
    color_profile: Option<String>,
    color_space: Option<String>,
    
    // Animation
    frame_count: u32,
    frame_delay: Option<u32>,
    loop_count: u32,
    
    // Technical
    bit_depth: u8,
    hdr: bool,
    
    // Extended EXIF-like data
    camera: Option<CameraMetadata>,
    dpi: Option<(u32, u32)>,
    orientation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CameraMetadata {
    make: Option<String>,
    model: Option<String>,
    iso: Option<u32>,
    exposure: Option<String>,
    aperture: Option<String>,
    focal_length: Option<String>,
}

impl Default for YeetMetadataV3 {
    fn default() -> Self {
        Self {
            author: None,
            created: Some(chrono::Utc::now().to_rfc3339()),
            software: "YEET v3.0-alpha".to_string(),
            color_profile: None,
            color_space: Some("sRGB".to_string()),
            frame_count: 1,
            frame_delay: None,
            loop_count: 0,
            bit_depth: 8,
            hdr: false,
            camera: None,
            dpi: None,
            orientation: Some("normal".to_string()),
        }
    }
}

/// Frame data for animations
#[derive(Debug, Clone)]
struct YeetFrame {
    delay: u32,          // Milliseconds
    data: Vec<u8>,       // Pixel data (possibly compressed)
}

/// Complete v3 image structure
#[derive(Debug, Clone)]
struct YeetImageV3 {
    width: u32,
    height: u32,
    has_alpha: bool,
    is_hdr: bool,
    metadata: YeetMetadataV3,
    icc_profile: Option<Vec<u8>>,
    frames: Vec<YeetFrame>,
}

// ============================================================================
// Compression (TODO: Add Brotli/Zstd)
// ============================================================================

use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use brotli::enc::BrotliEncoderParams;
use std::io::Cursor;

fn compress_data(data: &[u8], algorithm: CompressionAlgorithm) -> Vec<u8> {
    match algorithm {
        CompressionAlgorithm::None => data.to_vec(),
        CompressionAlgorithm::Zlib => {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
            encoder.write_all(data).unwrap();
            encoder.finish().unwrap()
        }
        CompressionAlgorithm::Brotli => {
            let mut output = Vec::new();
            let params = BrotliEncoderParams::default();
            let mut reader = Cursor::new(data);
            brotli::BrotliCompress(&mut reader, &mut output, &params).unwrap();
            println!("[INFO] Brotli compression: {} -> {} bytes ({:.1}% reduction)", 
                     data.len(), output.len(), 
                     100.0 * (1.0 - output.len() as f64 / data.len() as f64));
            output
        }
        CompressionAlgorithm::Zstd => {
            let compressed = zstd::encode_all(data, 19).unwrap(); // Level 19 = best compression
            println!("[INFO] Zstd compression: {} -> {} bytes ({:.1}% reduction)", 
                     data.len(), compressed.len(), 
                     100.0 * (1.0 - compressed.len() as f64 / data.len() as f64));
            compressed
        }
    }
}

fn decompress_data(data: &[u8], algorithm: CompressionAlgorithm) -> Vec<u8> {
    match algorithm {
        CompressionAlgorithm::None => data.to_vec(),
        CompressionAlgorithm::Zlib => {
            let mut decoder = ZlibDecoder::new(data);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed).unwrap();
            decompressed
        }
        CompressionAlgorithm::Brotli => {
            let mut output = Vec::new();
            let mut reader = Cursor::new(data);
            brotli::BrotliDecompress(&mut reader, &mut output).unwrap();
            output
        }
        CompressionAlgorithm::Zstd => {
            zstd::decode_all(data).unwrap()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum CompressionAlgorithm {
    None = 0,
    Zlib = 1,
    Brotli = 2,
    Zstd = 3,
}

impl From<u8> for CompressionAlgorithm {
    fn from(value: u8) -> Self {
        match value & 0b00000011 {
            0 => CompressionAlgorithm::None,
            1 => CompressionAlgorithm::Zlib,
            2 => CompressionAlgorithm::Brotli,
            3 => CompressionAlgorithm::Zstd,
            _ => CompressionAlgorithm::None,
        }
    }
}

// ============================================================================
// ICC Profile Support
// ============================================================================

use lcms2::*;

fn extract_icc_profile(path: &PathBuf) -> Option<Vec<u8>> {
    // Try to extract ICC profile from PNG
    if let Ok(file) = std::fs::File::open(path) {
        let decoder = png::Decoder::new(file);
        if let Ok(reader) = decoder.read_info() {
            if let Some(icc_profile) = reader.info().icc_profile.clone() {
                println!("[INFO] Extracted ICC profile: {} bytes", icc_profile.len());
                return Some(icc_profile.to_vec());
            }
        }
    }
    None
}

fn apply_icc_profile(data: &mut Vec<u8>, profile_data: &[u8], width: u32, height: u32, has_alpha: bool) {
    // Create ICC profile from data
    let profile = match Profile::new_icc(profile_data) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[WARN] Failed to load ICC profile: {:?}", e);
            return;
        }
    };
    
    // Get sRGB profile for display
    let srgb_profile = Profile::new_srgb();
    
    // Create transform
    let pixel_format = if has_alpha { 
        PixelFormat::RGBA_8 
    } else { 
        PixelFormat::RGB_8 
    };
    
    let transform = match Transform::new(
        &profile,
        pixel_format,
        &srgb_profile,
        pixel_format,
        Intent::Perceptual,
    ) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[WARN] Failed to create color transform: {:?}", e);
            return;
        }
    };
    
    // Apply transform to image data
    let pixels = (width * height) as usize;
    let bytes_per_pixel = if has_alpha { 4 } else { 3 };
    
    if data.len() != pixels * bytes_per_pixel {
        eprintln!("[WARN] Data size mismatch for ICC transform");
        return;
    }
    
    // Process in chunks for better performance
    let chunk_size = 1024; // Process 1024 pixels at a time
    for chunk_start in (0..pixels).step_by(chunk_size) {
        let chunk_end = (chunk_start + chunk_size).min(pixels);
        let byte_start = chunk_start * bytes_per_pixel;
        let byte_end = chunk_end * bytes_per_pixel;
        
        let chunk = &mut data[byte_start..byte_end];
        transform.transform_pixels(chunk, chunk);
    }
    
    println!("[INFO] Applied ICC color correction");
}

// ============================================================================
// v3 Conversion (Placeholder Implementation)
// ============================================================================

fn png_to_yeet_v3(
    path: PathBuf,
    compress: CompressionAlgorithm,
    use_binary: bool,
) -> Result<(), std::io::Error> {
    println!("[INFO] v3 format is experimental");
    
    let img = image::open(&path).expect("File not found!");
    let width = img.width();
    let height = img.height();
    
    let has_alpha = matches!(
        img.color(),
        image::ColorType::Rgba8 | image::ColorType::Rgba16
    );
    
    // Extract ICC profile (TODO)
    let icc_profile = extract_icc_profile(&path);
    
    // Prepare metadata
    let mut metadata = YeetMetadataV3::default();
    metadata.frame_count = 1;
    if icc_profile.is_some() {
        metadata.color_profile = Some("embedded".to_string());
    }
    
    let metadata_json = serde_json::to_string(&metadata).unwrap();
    let metadata_bytes = metadata_json.as_bytes();
    
    // Encode pixel data
    let pixel_data = if use_binary {
        let bytes_per_pixel = if has_alpha { 4 } else { 3 };
        let mut data = Vec::with_capacity((width * height * bytes_per_pixel) as usize);
        
        for pixel in img.pixels() {
            data.push(pixel.2[0]);
            data.push(pixel.2[1]);
            data.push(pixel.2[2]);
            if has_alpha {
                data.push(pixel.2[3]);
            }
        }
        data
    } else {
        let chars_per_pixel = if has_alpha { 8 } else { 6 };
        let mut str = String::with_capacity((width * height * chars_per_pixel) as usize);
        
        for pixel in img.pixels() {
            str.push_str(&format!("{:02X}{:02X}{:02X}", pixel.2[0], pixel.2[1], pixel.2[2]));
            if has_alpha {
                str.push_str(&format!("{:02X}", pixel.2[3]));
            }
        }
        str.into_bytes()
    };
    
    // Compress
    let compressed_data = compress_data(&pixel_data, compress);
    
    // Build flags
    let mut flags: u8 = (compress as u8) & 0b00000011;
    if has_alpha { flags |= 0b00000100; }
    if use_binary { flags |= 0b00001000; }
    if icc_profile.is_some() { flags |= 0b00100000; }
    
    // Write file
    if let Some(path_str) = path.to_str() {
        let output_path = path_str.replace(".png", ".yeet");
        let mut file = File::create(&output_path)?;
        
        // Header
        file.write_all(b"YEET")?;
        file.write_all(&[3])?; // Version 3
        file.write_all(&[flags])?;
        file.write_all(&width.to_le_bytes())?;
        file.write_all(&height.to_le_bytes())?;
        file.write_all(&1u32.to_le_bytes())?; // Frame count
        file.write_all(&0u32.to_le_bytes())?; // Loop count
        
        // Metadata
        file.write_all(&(metadata_bytes.len() as u32).to_le_bytes())?;
        file.write_all(metadata_bytes)?;
        
        // ICC profile
        if let Some(ref profile) = icc_profile {
            file.write_all(&(profile.len() as u32).to_le_bytes())?;
            file.write_all(profile)?;
        } else {
            file.write_all(&0u32.to_le_bytes())?;
        }
        
        // Frame data
        file.write_all(&0u32.to_le_bytes())?; // Frame delay
        file.write_all(&(compressed_data.len() as u32).to_le_bytes())?;
        file.write_all(&compressed_data)?;
        
        file.flush()?;
        
        println!("[OK] Converted to YEET v3: {}", output_path);
        println!("  Dimensions: {}x{}", width, height);
        println!("  Format: v3 (experimental)");
        println!("  ICC Profile: {}", icc_profile.is_some());
    }
    
    Ok(())
}

// ============================================================================
// v3 Reading & Viewing
// ============================================================================

fn read_yeet_v3(path: PathBuf) -> Result<YeetImageV3, std::io::Error> {
    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let mut pos = 0;
    
    // Verify magic bytes
    if &buffer[pos..pos+4] != b"YEET" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid YEET file: wrong magic bytes"
        ));
    }
    pos += 4;
    
    // Read version
    let version = buffer[pos];
    pos += 1;
    if version != 3 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Expected v3, got v{}", version)
        ));
    }
    
    // Read flags
    let flags = buffer[pos];
    pos += 1;
    
    let compression = CompressionAlgorithm::from(flags & 0b00000011);
    let has_alpha = (flags & 0b00000100) != 0;
    let is_binary = (flags & 0b00001000) != 0;
    let is_animated = (flags & 0b00010000) != 0;
    let has_icc = (flags & 0b00100000) != 0;
    let is_hdr = (flags & 0b01000000) != 0;
    
    // Read dimensions
    let width = u32::from_le_bytes([buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]]);
    pos += 4;
    let height = u32::from_le_bytes([buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]]);
    pos += 4;
    
    // Read frame count
    let frame_count = u32::from_le_bytes([buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]]);
    pos += 4;
    
    // Read loop count
    let loop_count = u32::from_le_bytes([buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]]);
    pos += 4;
    
    // Read metadata
    let metadata_len = u32::from_le_bytes([buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]]) as usize;
    pos += 4;
    
    let metadata_str = String::from_utf8_lossy(&buffer[pos..pos+metadata_len]);
    let metadata: YeetMetadataV3 = serde_json::from_str(&metadata_str)
        .unwrap_or_else(|_| YeetMetadataV3::default());
    pos += metadata_len;
    
    // Read ICC profile if present
    let icc_profile = if has_icc {
        let icc_len = u32::from_le_bytes([buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]]) as usize;
        pos += 4;
        let profile = buffer[pos..pos+icc_len].to_vec();
        pos += icc_len;
        Some(profile)
    } else {
        let icc_len = u32::from_le_bytes([buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]]) as usize;
        pos += 4;
        if icc_len > 0 {
            eprintln!("[WARN] ICC data present but flag not set");
        }
        None
    };
    
    // Read frames
    let mut frames = Vec::new();
    for _ in 0..frame_count {
        // Frame delay
        let delay = u32::from_le_bytes([buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]]);
        pos += 4;
        
        // Frame data length
        let data_len = u32::from_le_bytes([buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]]) as usize;
        pos += 4;
        
        // Frame data
        let compressed_data = &buffer[pos..pos+data_len];
        let data = decompress_data(compressed_data, compression);
        pos += data_len;
        
        frames.push(YeetFrame { delay, data });
    }
    
    println!("[INFO] Loaded YEET v3: {}x{}, {} frames", width, height, frame_count);
    if has_icc {
        println!("[INFO] ICC profile present");
    }
    
    Ok(YeetImageV3 {
        width,
        height,
        has_alpha,
        is_hdr,
        metadata,
        icc_profile,
        frames,
    })
}

fn yeet_v3_to_image(yeet_img: &YeetImageV3, frame_index: usize) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let frame = &yeet_img.frames[frame_index];
    let mut data = frame.data.clone();
    
    // Apply ICC profile if present
    if let Some(ref profile) = yeet_img.icc_profile {
        apply_icc_profile(&mut data, profile, yeet_img.width, yeet_img.height, yeet_img.has_alpha);
    }
    
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = 
        ImageBuffer::new(yeet_img.width, yeet_img.height);
    
    let bytes_per_pixel = if yeet_img.has_alpha { 4 } else { 3 };
    
    for y in 0..yeet_img.height {
        for x in 0..yeet_img.width {
            let idx = ((y * yeet_img.width + x) * bytes_per_pixel) as usize;
            
            let r = data[idx];
            let g = data[idx + 1];
            let b = data[idx + 2];
            let a = if yeet_img.has_alpha { data[idx + 3] } else { 255 };
            
            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }
    
    img
}

// ============================================================================
// GUI Application with Animation Support
// ============================================================================

struct YeetV3ViewerApp {
    image: Option<YeetImageV3>,
    current_frame: usize,
    last_frame_time: std::time::Instant,
    playing: bool,
    loaded_image: Option<RetainedImage>,
}

impl YeetV3ViewerApp {
    fn new(image: YeetImageV3) -> Self {
        let is_animated = image.frames.len() > 1;
        Self {
            image: Some(image),
            current_frame: 0,
            last_frame_time: std::time::Instant::now(),
            playing: is_animated,
            loaded_image: None,
        }
    }
    
    fn update_current_frame(&mut self) {
        if let Some(ref img) = self.image {
            let frame_img = yeet_v3_to_image(img, self.current_frame);
            
            // Save to temp file for RetainedImage
            frame_img.save(TEMP_RESULT_PATH).ok();
            
            if let Ok(retained) = RetainedImage::from_image_bytes(
                "temp_frame",
                &std::fs::read(TEMP_RESULT_PATH).unwrap()
            ) {
                self.loaded_image = Some(retained);
            }
        }
    }
}

impl eframe::App for YeetV3ViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle animation
        if let Some(ref img) = self.image {
            if self.playing && img.frames.len() > 1 {
                let current_delay = img.frames[self.current_frame].delay;
                let delay_ms = if current_delay > 0 { current_delay } else { 100 }; // Default 100ms
                
                if self.last_frame_time.elapsed().as_millis() >= delay_ms as u128 {
                    self.current_frame = (self.current_frame + 1) % img.frames.len();
                    self.update_current_frame();
                    self.last_frame_time = std::time::Instant::now();
                }
                
                // Request repaint for smooth animation
                ctx.request_repaint();
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref img) = self.image {
                // Info panel
                ui.horizontal(|ui| {
                    ui.heading("YEET v3 Viewer");
                    ui.separator();
                    ui.label(format!("{}x{}", img.width, img.height));
                    
                    if img.frames.len() > 1 {
                        ui.separator();
                        ui.label(format!("Frame {}/{}", self.current_frame + 1, img.frames.len()));
                        
                        if ui.button(if self.playing { "⏸ Pause" } else { "▶ Play" }).clicked() {
                            self.playing = !self.playing;
                        }
                        
                        if ui.button("⏮ Prev").clicked() {
                            self.current_frame = if self.current_frame == 0 {
                                img.frames.len() - 1
                            } else {
                                self.current_frame - 1
                            };
                            self.update_current_frame();
                        }
                        
                        if ui.button("⏭ Next").clicked() {
                            self.current_frame = (self.current_frame + 1) % img.frames.len();
                            self.update_current_frame();
                        }
                    }
                    
                    if img.icc_profile.is_some() {
                        ui.separator();
                        ui.label("🎨 ICC Profile");
                    }
                });
                
                ui.separator();
                
                // Metadata panel (collapsible)
                ui.collapsing("📊 Metadata", |ui| {
                    egui::Grid::new("metadata_grid")
                        .num_columns(2)
                        .spacing([10.0, 5.0])
                        .show(ui, |ui| {
                            if let Some(ref author) = img.metadata.author {
                                ui.label("Author:");
                                ui.label(author);
                                ui.end_row();
                            }
                            if let Some(ref created) = img.metadata.created {
                                ui.label("Created:");
                                ui.label(created);
                                ui.end_row();
                            }
                            ui.label("Software:");
                            ui.label(&img.metadata.software);
                            ui.end_row();
                            
                            if let Some(ref color_space) = img.metadata.color_space {
                                ui.label("Color Space:");
                                ui.label(color_space);
                                ui.end_row();
                            }
                            
                            ui.label("Bit Depth:");
                            ui.label(format!("{}-bit", img.metadata.bit_depth));
                            ui.end_row();
                            
                            if img.frames.len() > 1 {
                                ui.label("Total Frames:");
                                ui.label(format!("{}", img.frames.len()));
                                ui.end_row();
                                
                                if let Some(delay) = img.metadata.frame_delay {
                                    ui.label("Frame Delay:");
                                    ui.label(format!("{}ms", delay));
                                    ui.end_row();
                                }
                            }
                        });
                });
                
                ui.separator();
                
                // Image display
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.loaded_image.is_none() {
                            self.update_current_frame();
                        }
                        
                        if let Some(ref retained_img) = self.loaded_image {
                            retained_img.show(ui);
                        }
                    });
            }
        });
    }
}

// ============================================================================
// Main Entry Point
// ============================================================================

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }
    
    println!("[INFO] YEET v3 - Advanced Features Enabled");
    println!("  ✅ Brotli/Zstd compression");
    println!("  ✅ ICC color profiles");
    println!("  ✅ Multi-frame animation");
    println!();
    
    let command = &args[1];
    
    match command.as_str() {
        "compile" => {
            if args.len() < 3 {
                eprintln!("[ERROR] No input file");
                std::process::exit(1);
            }
            
            let path: PathBuf = args[2].clone().into();
            let compress = if args.contains(&"--brotli".to_string()) {
                CompressionAlgorithm::Brotli
            } else if args.contains(&"--zstd".to_string()) {
                CompressionAlgorithm::Zstd
            } else if args.contains(&"--compress".to_string()) {
                CompressionAlgorithm::Zlib
            } else {
                CompressionAlgorithm::None
            };
            
            let binary = args.contains(&"--binary".to_string());
            
            match png_to_yeet_v3(path, compress, binary) {
                Ok(()) => println!("[OK] Conversion complete"),
                Err(e) => eprintln!("[ERROR] {}", e),
            }
            Ok(())
        }
        "help" | "--help" | "-h" => {
            print_usage(&args[0]);
            Ok(())
        }
        _ => {
            // Try to view the file
            let path: PathBuf = args[1].clone().into();
            
            match read_yeet_v3(path.clone()) {
                Ok(img) => {
                    let options = eframe::NativeOptions {
                        viewport: egui::ViewportBuilder::default()
                            .with_inner_size([1024.0, 768.0])
                            .with_title(format!("YEET v3 Viewer - {}", 
                                path.file_name().unwrap_or_default().to_string_lossy())),
                        ..Default::default()
                    };
                    
                    eframe::run_native(
                        "YEET v3 Viewer",
                        options,
                        Box::new(|_cc| Box::new(YeetV3ViewerApp::new(img))),
                    )
                }
                Err(e) => {
                    eprintln!("[ERROR] Failed to load file: {}", e);
                    eprintln!("Tip: Use 'compile' command to convert PNG to YEET v3");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn print_usage(program: &str) {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║          YEET v3 - Next Generation Image Format          ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("USAGE:");
    println!("  {} <file.yeet>                    View YEET v3 file", program);
    println!("  {} compile <file.png> [options]  Convert PNG to YEET v3", program);
    println!();
    println!("COMPRESSION OPTIONS:");
    println!("  --compress    Use zlib compression (v2 compatible)");
    println!("  --brotli      Use Brotli compression ✨ NEW!");
    println!("  --zstd        Use Zstd compression ✨ NEW!");
    println!("  --binary      Binary encoding (recommended)");
    println!();
    println!("FEATURES:");
    println!("  ✅ ICC color profiles       Accurate color reproduction");
    println!("  ✅ Multi-frame animation    GIF/APNG alternative");
    println!("  ✅ Brotli/Zstd compression  Better than zlib");
    println!("  ✅ Rich metadata            Extended EXIF-like data");
    println!("  🚧 HDR support              16-bit per channel (planned)");
    println!();
    println!("EXAMPLES:");
    println!("  # Convert with Brotli (best compression)");
    println!("  {} compile photo.png --brotli --binary", program);
    println!();
    println!("  # Convert with Zstd (fast)");
    println!("  {} compile photo.png --zstd --binary", program);
    println!();
    println!("  # View YEET v3 file with animation");
    println!("  {} animation.yeet", program);
    println!();
    println!("VIEWER CONTROLS:");
    println!("  - Automatic animation playback");
    println!("  - Play/Pause button");
    println!("  - Frame navigation (Prev/Next)");
    println!("  - ICC color correction applied");
    println!("  - Metadata viewer");
    println!();
    println!("For stable/production use, see yeet-core (v2)");
}
