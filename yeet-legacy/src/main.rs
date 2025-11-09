//! YEET v1 Legacy Format Support
//!
//! This module provides backward compatibility with the original YEET v1 format.
//! 
//! v1 Format:
//! - 8-byte header (width + height as u32 native-endian)
//! - Hex color codes (RRGGBB, 6 chars per pixel)
//! - No compression, no alpha channel, no metadata

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui_extras::RetainedImage;
use image::{ImageBuffer, Rgba};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

static TEMP_RESULT_PATH: &str = "temp_legacy.png";

/// Convert PNG to YEET v1 legacy format
fn png_to_yeet_v1(path: PathBuf) -> Result<(), std::io::Error> {
    let img = image::open(&path).expect("File not found!");
    let width = img.width();
    let height = img.height();
    
    // Build hex string
    let mut hex_data = String::new();
    for pixel in img.pixels() {
        hex_data.push_str(&format!("{:02X}{:02X}{:02X}", pixel.2[0], pixel.2[1], pixel.2[2]));
    }
    
    // Write v1 file
    if let Some(path_str) = path.to_str() {
        let output_path = path_str.replace(".png", ".yeet");
        let mut file = File::create(&output_path)?;
        
        // Write header (width and height)
        file.write_all(&width.to_ne_bytes())?;
        file.write_all(&height.to_ne_bytes())?;
        
        // Write hex color data
        file.write_all(hex_data.as_bytes())?;
        file.flush()?;
        
        println!("[OK] Converted to YEET v1: {}", output_path);
        println!("  Dimensions: {}x{}", width, height);
        println!("  Format: RGB hex text (no alpha, no compression)");
        println!("  Size: {} bytes", 8 + hex_data.len());
    }
    
    Ok(())
}

/// Convert YEET v1 to PNG for viewing
fn yeet_to_png_v1(path: PathBuf) -> (u32, u32) {
    let mut contents: Vec<u8> = fs::read(&path).expect("Couldn't read file");
    let header: Vec<_> = contents.drain(0..8).collect();
    
    // Read dimensions
    let width = u32::from_ne_bytes([header[0], header[1], header[2], header[3]]);
    let height = u32::from_ne_bytes([header[4], header[5], header[6], header[7]]);
    
    // Parse hex colors
    let sanitized = String::from_utf8_lossy(&contents).replace("\n", "");
    let colors: Vec<&str> = sanitized
        .as_bytes()
        .chunks(6)
        .map(std::str::from_utf8)
        .collect::<Result<_, _>>()
        .expect("Invalid UTF-8 sequence");
    
    // Build image
    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = 
        ImageBuffer::from_fn(width, height, |x, y| {
            let idx = (y * width + x) as usize;
            if idx < colors.len() {
                let hex = format!("#{}", colors[idx]);
                if let Ok(color) = hex.parse::<css_color_parser::Color>() {
                    return Rgba([color.r, color.g, color.b, 255]);
                }
            }
            Rgba([0, 0, 0, 255])
        });
    
    img_buffer.save(TEMP_RESULT_PATH).expect("Failed to save PNG");
    (width, height)
}

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "compile" => {
            if args.len() < 3 {
                eprintln!("[ERROR] No input file specified");
                std::process::exit(1);
            }
            
            let path: PathBuf = args[2].clone().into();
            match png_to_yeet_v1(path) {
                Ok(()) => println!("[OK] Conversion complete"),
                Err(e) => eprintln!("[ERROR] {}", e),
            }
            Ok(())
        }
        _ => {
            // View file
            let path: PathBuf = command.into();
            if !path.exists() {
                eprintln!("[ERROR] File not found: {:?}", path);
                std::process::exit(1);
            }
            
            let (width, height) = yeet_to_png_v1(path);
            
            let options = eframe::NativeOptions {
                resizable: true,
                initial_window_size: Some(egui::vec2(width as f32, height as f32)),
                ..Default::default()
            };
            
            eframe::run_native(
                "YEET v1 Legacy Viewer",
                options,
                Box::new(|_cc| Box::<ImagePreview>::default()),
            )
        }
    }
}

fn print_usage(program: &str) {
    println!("YEET v1 Legacy Format Viewer");
    println!();
    println!("USAGE:");
    println!("  View:    {} <file.yeet>", program);
    println!("  Convert: {} compile <file.png>", program);
    println!();
    println!("NOTE: v1 format has no compression, no alpha, no metadata");
    println!("      Consider using yeet-core (v2) for modern features");
}

struct ImagePreview {
    image: RetainedImage,
}

impl Default for ImagePreview {
    fn default() -> Self {
        let image_data = std::fs::read(TEMP_RESULT_PATH)
            .expect("Failed to read image");
        fs::remove_file(TEMP_RESULT_PATH).ok();
        
        Self {
            image: RetainedImage::from_image_bytes(TEMP_RESULT_PATH, &image_data)
                .expect("Failed to load image"),
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
