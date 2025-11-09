# YEETIFF
**Y**et Even **E**xtremely **E**xpressier  **T**ranscoded **I**mage **F**ile **F**ormat.

A playful, uncompressed image format that represents pixels as readable hexadecimal color codes.

## What is .yeet?

YEET is an experimental image file format created for fun and learning. Unlike traditional compressed formats like PNG or JPEG, YEET stores images in a human-readable text format where each pixel is represented by its hex color code (e.g., `FF0000` for red).

### File Structure Explained

Every .yeet file follows this simple structure:

```
┌─────────────┬──────────────┬────────────────────────────┐
│  Width (4B) │ Height (4B)  │  Pixel Data (hex colors)   │
└─────────────┴──────────────┴────────────────────────────┘
```

**Practical Example - 2x2 Image:**
```
Binary Header:
  02 00 00 00  (width = 2)
  02 00 00 00  (height = 2)

Hex Color Data:
  FF0000 00FF00
  0000FF FFFF00
```

This creates a simple 2x2 grid:
- **Red** (FF0000) | **Green** (00FF00)
- **Blue** (0000FF) | **Yellow** (FFFF00)

### Technical Details

**Header (8 bytes total):**
- **Bytes 0-3**: Image width as 32-bit unsigned integer (little-endian on most systems)
- **Bytes 4-7**: Image height as 32-bit unsigned integer (little-endian on most systems)

**Pixel Data (6 characters per pixel):**
Each pixel is encoded as a 6-character hexadecimal string in RRGGBB format:
- **RR**: Red intensity (00 = none, FF = maximum)
- **GG**: Green intensity (00 = none, FF = maximum)  
- **BB**: Blue intensity (00 = none, FF = maximum)

Newline characters are permitted between pixels for readability but are ignored during parsing.

### Why Use YEET?

**Advantages:**
- ✅ **Human-readable** - Open in any text editor and see/edit pixel values directly
- ✅ **Educational** - Perfect for learning how images are stored digitally
- ✅ **Simple** - No complex compression algorithms, just raw pixel data
- ✅ **Fun** - Create images by typing hex codes!

**Disadvantages (why it's "experimental"):**
- ❌ **Massive files** - A 1920×1080 HD image becomes ~12MB (compare to ~200KB PNG)
- ❌ **No compression** - Each pixel requires 6 bytes of text storage
- ❌ **Slow parsing** - Text-to-pixel conversion takes time
- ❌ **No alpha channel** - Transparency is not supported

Despite these limitations, YEET is an interesting exploration of image storage formats!

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

To convert a PNG image to YEET format:

```bash
cargo run compile path/to/your/image.png
```

**Example:**
```bash
cargo run compile C:\Users\YourName\Pictures\photo.png
```

This will create `photo.yeet` in the same directory as the original PNG.

> **Note:** You need to create an empty `.yeet` file with the same name before conversion (this is a known limitation).

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

1. **Pre-creation required**: You must create an empty `.yeet` file before converting PNG to YEET
2. **Large file sizes**: Images consume significantly more disk space than compressed formats
3. **Performance**: Loading large images can be slow due to text parsing
4. **No transparency**: Alpha channel is not supported

✅ **Fixed Issues:**
- ~~Windows only~~ - Now cross-platform with installer for easy Windows setup
- ~~Window sizing~~ - Improved with proper viewer implementation
- ~~Hex parsing bug~~ - Resolved in latest version

## Future Improvements

Potential enhancements for this format:
- [ ] Add alpha channel support for transparency
- [ ] Implement basic compression (e.g., run-length encoding)
- [x] ~~Cross-platform support~~ - Rust is cross-platform, installer for Windows
- [x] ~~Easy installation~~ - Complete installer wizard available
- [ ] Batch conversion tools
- [ ] Optional binary mode for smaller file sizes
- [ ] Image metadata support (author, creation date, etc.)
- [ ] Drag-and-drop conversion GUI

## Key Features

✨ **What makes this project special:**

- 🎨 **Rust-based viewer** - Fast, native performance
- 🖼️ **Complete installer** - Professional multi-step wizard
- 📦 **Standalone distribution** - Single 21MB EXE includes everything
- 🔧 **Windows integration** - Registry-based file associations
- 🗑️ **Clean uninstall** - Removes all traces properly
- 📝 **Human-readable format** - Edit images in any text editor
- 🎓 **Educational** - Learn about image formats and Windows installers

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
