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

fn compress_data(data: &[u8], algorithm: CompressionAlgorithm) -> Vec<u8> {
    match algorithm {
        CompressionAlgorithm::None => data.to_vec(),
        CompressionAlgorithm::Zlib => {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
            encoder.write_all(data).unwrap();
            encoder.finish().unwrap()
        }
        CompressionAlgorithm::Brotli => {
            // TODO: Implement Brotli compression
            eprintln!("[WARN] Brotli not yet implemented, using zlib");
            compress_data(data, CompressionAlgorithm::Zlib)
        }
        CompressionAlgorithm::Zstd => {
            // TODO: Implement Zstd compression
            eprintln!("[WARN] Zstd not yet implemented, using zlib");
            compress_data(data, CompressionAlgorithm::Zlib)
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
            eprintln!("[WARN] Brotli not yet implemented");
            decompress_data(data, CompressionAlgorithm::Zlib)
        }
        CompressionAlgorithm::Zstd => {
            eprintln!("[WARN] Zstd not yet implemented");
            decompress_data(data, CompressionAlgorithm::Zlib)
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
// ICC Profile Support (TODO: Implement)
// ============================================================================

fn extract_icc_profile(_path: &PathBuf) -> Option<Vec<u8>> {
    // TODO: Extract ICC profile from PNG using lcms2 or similar
    eprintln!("[TODO] ICC profile extraction not yet implemented");
    None
}

fn apply_icc_profile(_data: &mut [u8], _profile: &[u8]) {
    // TODO: Apply ICC color correction
    eprintln!("[TODO] ICC profile application not yet implemented");
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
// Main Entry Point
// ============================================================================

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }
    
    println!("[WARN] YEET v3 is experimental. Use yeet-core for production.");
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
        _ => {
            eprintln!("[ERROR] Viewing v3 files not yet implemented");
            eprintln!("Use yeet-core to view v2 files");
            std::process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    println!("YEET v3 - Experimental Format");
    println!();
    println!("USAGE:");
    println!("  {} compile <file.png> [options]", program);
    println!();
    println!("OPTIONS:");
    println!("  --compress    Use zlib compression");
    println!("  --brotli      Use Brotli compression (TODO)");
    println!("  --zstd        Use Zstd compression (TODO)");
    println!("  --binary      Binary encoding");
    println!();
    println!("EXPERIMENTAL FEATURES:");
    println!("  - ICC color profiles (TODO)");
    println!("  - Multi-frame animation (TODO)");
    println!("  - HDR support (TODO)");
    println!("  - Enhanced compression (TODO)");
    println!();
    println!("For production use, see yeet-core (v2)");
}
