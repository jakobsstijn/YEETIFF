# YEETIFF
**Y**et Even **E**xtremely **E**xpressier  **T**ranscoded **I**mage **F**ile **F**ormat.

A playful, uncompressed image format that represents pixels as readable hexadecimal color codes.

## What is .yeet?

YEET is an experimental image file format created for fun and learning. Unlike traditional compressed formats like PNG or JPEG, YEET stores images in a human-readable text format where each pixel is represented by its hex color code (e.g., `FF0000` for red).

### File Structure Explained

YEET supports two format versions:

#### **YEET v2 (Current)** - Feature-Rich Format

```
┌──────────┬─────────┬───────┬────────┬────────┬──────────┬──────────┬─────────────┐
│  Magic   │ Version │ Flags │ Width  │ Height │ Metadata │ DataLen  │ Pixel Data  │
│  (4B)    │  (1B)   │ (1B)  │  (4B)  │  (4B)  │  (var)   │  (4B)    │   (var)     │
└──────────┴─────────┴───────┴────────┴────────┴──────────┴──────────┴─────────────┘
```

**Features:**
- ✅ **Magic bytes**: "YEET" identifier
- ✅ **Alpha channel support**: RGBA (8 hex chars) or RGB (6 hex chars)
- ✅ **Compression**: Optional zlib compression
- ✅ **Binary mode**: Store as raw bytes instead of hex text
- ✅ **Metadata**: JSON metadata (author, creation date, software)

**Flags byte:**
- Bit 0: Compression (0=none, 1=zlib)
- Bit 1: Alpha channel (0=RGB, 1=RGBA)
- Bit 2: Binary mode (0=hex text, 1=binary)
- Bits 3-7: Reserved for future use

#### **YEET v1 (Legacy)** - Simple Format

```
┌─────────────┬──────────────┬────────────────────────────┐
│  Width (4B) │ Height (4B)  │  Pixel Data (hex colors)   │
└─────────────┴──────────────┴────────────────────────────┘
```

**Pixel Data Examples:**

**v2 with Alpha (RGBA):**
```
YEET                    (magic bytes)
02                      (version 2)
02                      (flags: alpha enabled)
02 00 00 00            (width = 2)
02 00 00 00            (height = 2)
00 00                  (metadata length = 0)
00 00 00 20            (data length = 32 bytes)
FF0000FF 00FF00FF      (Red opaque, Green opaque)
0000FF80 FFFFFF00      (Blue 50% transparent, White transparent)
```

**v1 RGB only:**
```
02 00 00 00            (width = 2)
02 00 00 00            (height = 2)
FF0000 00FF00          (Red, Green)
0000FF FFFF00          (Blue, Yellow)
```

### Technical Details

**YEET v2 Format:**

**Header Components:**
- **Magic (4 bytes)**: "YEET" identifier
- **Version (1 byte)**: Format version (currently 2)
- **Flags (1 byte)**: Feature flags
  - Bit 0: Compression enabled
  - Bit 1: Alpha channel present (RGBA vs RGB)
  - Bit 2: Binary mode (raw bytes vs hex text)
- **Width (4 bytes)**: Image width as u32 (little-endian)
- **Height (4 bytes)**: Image height as u32 (little-endian)
- **Metadata Length (2 bytes)**: Size of metadata JSON
- **Metadata (variable)**: JSON string with creation info
- **Data Length (4 bytes)**: Compressed/uncompressed pixel data size
- **Pixel Data (variable)**: Image data

**Pixel Encoding:**

**Text mode (hex):**
- RGB: 6 characters per pixel (RRGGBB)
  - RR: Red (00-FF)
  - GG: Green (00-FF)
  - BB: Blue (00-FF)
- RGBA: 8 characters per pixel (RRGGBBAA)
  - RR: Red (00-FF)
  - GG: Green (00-FF)
  - BB: Blue (00-FF)
  - AA: Alpha/transparency (00=transparent, FF=opaque)

**Binary mode:**
- RGB: 3 bytes per pixel
- RGBA: 4 bytes per pixel

**Compression:**
- Optional zlib compression for smaller file sizes
- Typically achieves 30-70% size reduction on photographic images

### Why Use YEET?

**Advantages:**
- ✅ **Human-readable** - Open in any text editor and see/edit pixel values directly (v1 and v2 text mode)
- ✅ **Educational** - Perfect for learning how images are stored digitally
- ✅ **Flexible** - Multiple modes: text/binary, compressed/uncompressed, RGB/RGBA
- ✅ **Alpha channel** - Full transparency support in v2
- ✅ **Compression** - Optional zlib compression reduces file sizes significantly
- ✅ **Metadata** - Store author, creation date, and software info
- ✅ **Fun** - Create images by typing hex codes!

**Trade-offs:**
- 📏 **File size** - Text mode is larger, but use `--compress --binary` for competitive sizes
- ⚡ **Performance** - Text parsing is slower, but binary mode is fast

**File Size Comparison (1920×1080 image):**
- PNG: ~200 KB (compressed, industry standard)
- YEET v2 (binary + compressed): ~500 KB (2.5x PNG, still practical!)
- YEET v2 (text mode, compressed): ~4 MB (human-readable)
- YEET v2 (text, uncompressed): ~12 MB (fully editable)
- YEET v1 (text): ~12 MB (legacy format)

**💡 Pro Tip:** For production use, always use `--compress --binary` for the best balance between file size and performance. For education or experimentation, text mode lets you see and edit every pixel!

YEET v2 is a fully-featured image format with modern capabilities - choose the mode that fits your needs!

## Getting Started

### Prerequisites
- Rust programming language installed
- Windows operating system
- A PNG image you want to convert

### Building the Project

1. **Clone this repository:**
   ```bash
   git clone <repository-url>
   cd yeet-image
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

### Converting PNG to YEET

YEET v2 supports multiple conversion modes:

**Basic conversion (uncompressed, text mode):**
```bash
cargo run compile path/to/image.png
```

**With compression (recommended for smaller files):**
```bash
cargo run compile path/to/image.png --compress
```

**Binary mode (even smaller, but not human-readable):**
```bash
cargo run compile path/to/image.png --binary
```

**All options combined:**
```bash
cargo run compile path/to/image.png --compress --binary
```

**Examples:**
```bash
# Standard text format with transparency
cargo run compile C:\Users\YourName\Pictures\logo.png

# Compressed binary for smallest file size
cargo run compile C:\Users\YourName\Pictures\photo.png --compress --binary
```

This will create `image.yeet` in the same directory as the original PNG.

**Features automatically detected:**
- ✅ **Alpha channel**: Automatically detected from PNG
- ✅ **Dimensions**: Preserved exactly
- ✅ **Metadata**: Creation date and software version included

### Viewing YEET Images

**Option 1: Standalone Installer (Recommended - Easiest!)**

Use the complete Windows installer that bundles everything:

1. **Build the YEET viewer:**
   ```bash
   cargo build --release
   ```

2. **Build the installer:**
   ```bash
   cd yeet-installer
   BUILD.bat
   # Or: python build_installer.py
   ```

3. **Run the installer:**
   ```bash
   dist\YeetInstaller.exe
   ```

The installer will:
- ✅ Install YEET Viewer to Program Files
- ✅ Register .yeet file extension
- ✅ Set up double-click file associations
- ✅ Add context menu integration
- ✅ Create uninstaller entry

**After installation, just double-click any `.yeet` file to view it!**

**Option 2: Command Line (Development)**
```bash
cargo run path/to/image.yeet
```

**Option 3: Direct Executable**
```bash
# After building
target\release\yeet.exe path/to/image.yeet
```

## Project Structure

```
yeet-format/
├── Cargo.toml              # Rust project configuration
├── src/
│   └── main.rs             # YEET viewer/converter implementation
├── target/
│   ├── debug/
│   │   └── yeet.exe        # Debug build
│   └── release/
│       └── yeet.exe        # Optimized release build
├── image.yeet              # Sample YEET image
├── README.md               # This file
└── yeet-installer/         # Complete Windows installer
    ├── installer_gui.py    # Multi-step wizard GUI
    ├── yeet_viewer.py      # Python viewer (fallback)
    ├── build_installer.py  # Builds standalone installer EXE
    ├── BUILD.bat           # Quick build script
    ├── requirements.txt    # Python dependencies
    ├── logo.png            # Installer logo
    └── dist/
        └── YeetInstaller.exe  # (~21MB, bundles everything!)
```

## Installation Methods

### For End Users (Easy):

1. **Download `YeetInstaller.exe`** from releases
2. **Run as Administrator**
3. **Follow the wizard** - Click through Welcome → License → Install
4. **Done!** `.yeet` files now open automatically

### For Developers:

```bash
# Build Rust viewer
cargo build --release

# Build complete installer
cd yeet-installer
python build_installer.py

# Test installer
dist\YeetInstaller.exe
```

See `yeet-installer/README.md` for detailed build instructions.

## Current Limitations

⚠️ **Known Issues:**

1. **File size**: Uncompressed text mode creates very large files
   - **Solution**: Use `--compress --binary` for ~60-70% size reduction
2. **Parsing speed**: Text mode is slower than binary formats
   - **Solution**: Use binary mode for better performance

✅ **Fixed Issues:**
- ~~No alpha channel~~ - **FIXED in v2!** Full RGBA support
- ~~No compression~~ - **FIXED in v2!** Optional zlib compression
- ~~No metadata~~ - **FIXED in v2!** JSON metadata support
- ~~Pre-creation required~~ - **FIXED!** No longer needed
- ~~Windows only~~ - Cross-platform with Windows installer
- ~~Window sizing~~ - Improved with proper viewer implementation
- ~~Hex parsing bug~~ - Resolved in latest version

## Format Versions

### YEET v2 (Current - Recommended)
- ✅ Alpha channel (RGBA)
- ✅ Compression (zlib)
- ✅ Binary mode option
- ✅ Metadata support
- ✅ Backward compatible with v1

### YEET v1 (Legacy)
- ⚠️ RGB only (no alpha)
- ⚠️ No compression
- ⚠️ Text mode only
- ℹ️ Still supported for reading

## Future Improvements

Potential enhancements for this format:
- [x] ~~Add alpha channel support~~ - **DONE in v2!**
- [x] ~~Implement compression~~ - **DONE in v2!**
- [x] ~~Binary mode for smaller files~~ - **DONE in v2!**
- [x] ~~Metadata support~~ - **DONE in v2!**
- [x] ~~Cross-platform support~~ - Rust is cross-platform, installer for Windows
- [x] ~~Easy installation~~ - Complete installer wizard available
- [ ] Batch conversion tools
- [ ] Progressive loading for large images
- [ ] Color profile support (ICC)
- [ ] Animated YEET support (multi-frame)
- [ ] Drag-and-drop conversion GUI
- [ ] YEET v3 with better compression algorithms

## Key Features

✨ **What makes YEET v2 special:**

- 🎨 **Rust-based viewer** - Fast, native performance with OpenGL rendering
- 🖼️ **Complete installer** - Professional multi-step wizard for Windows
- 📦 **Standalone distribution** - Single 21MB EXE includes everything
- 🔧 **Windows integration** - Registry-based file associations
- 🗑️ **Clean uninstall** - Removes all traces properly
- 📝 **Human-readable format** - Edit images in text editor (text mode)
- 💾 **Efficient storage** - Binary + compression for practical file sizes
- 🎭 **Full transparency** - RGBA with alpha channel support
- 📊 **Metadata support** - Author, creation date, software info
- 🔄 **Format flexibility** - Text/binary, compressed/uncompressed modes
- 🎓 **Educational** - Learn about image formats and Windows installers
- 🔀 **Backward compatible** - Reads both v1 and v2 formats

## Contributing

This is an experimental/educational project! If you have ideas for improvements or want to fix bugs, contributions are welcome.

## Technical Stack

- **Viewer/Converter**: Rust (glutin, gl for OpenGL rendering)
- **Installer GUI**: Python (tkinter)
- **Packaging**: PyInstaller (creates standalone EXE)
- **File Associations**: Windows Registry API (via pywin32)

## License

MIT License - See LICENSE file for details.

## Developer

**Stijn Jakobs** - 2025

## Acknowledgments

Created as a fun exploration of image file formats and Windows installer development. YEET demonstrates:
- The trade-offs between human readability and storage efficiency
- How file format registration works in Windows
- Building cross-language projects (Rust + Python)
- Creating professional installers with embedded resources

---

**Remember:** YEET is not meant to replace PNG, JPEG, or other established formats. It's a learning tool and experiment in alternative image storage methods! 🎨

**Ready to try it?** Download the installer from releases or build it yourself! 🚀
