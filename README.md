# YEETIFF
**Y**et Even **E**xtremely **E**xpressier  **T**ranscoded **I**mage **F**ile **F**ormat.

A playful, uncompressed image format that represents pixels as readable hexadecimal color codes.

![Example](https://cdn.discordapp.com/attachments/1074408238939906220/1130764354661384192/image.png)

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
- ❌ **Windows only** - Current implementation is Windows-specific

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

**Option 1: Command Line**
```bash
cargo run path/to/image.yeet
```

**Option 2: File Association (Windows)**

1. Right-click any `.yeet` file in Windows Explorer
2. Select **"Open with"** → **"Choose another app"**
3. Click **"More apps"** → **"Look for another app on this PC"**
4. Navigate to the compiled `yeet.exe` in your project's target folder
5. ✅ Check **"Always use this app to open .yeet files"**

Now you can double-click `.yeet` files to open them directly!

**Option 3: Python Viewer (Recommended)**

For a more user-friendly experience, check out the Python-based viewer in the `yeet-viewer-python` folder:
- Modern GUI with menu bar
- Resizable window with scrollbars
- Save images as PNG
- Easy Windows installer with file association

## Project Structure

```
yeet-image/
├── Cargo.toml          # Rust project configuration
├── main.rs             # Source code for YEET converter/viewer
├── image.yeet          # Sample YEET image
└── README.md           # This file

yeet-viewer-python/     # Alternative Python-based viewer
├── yeet_viewer.py      # Main viewer application
├── installer_gui.py    # Graphical installer for Windows
├── build_exe.py        # Script to build standalone .exe
└── requirements.txt    # Python dependencies
```

## Current Limitations

⚠️ **Known Issues:**

1. **Pre-creation required**: You must create an empty `.yeet` file before converting PNG to YEET
2. **Window sizing**: Preview window dimensions may not match the image exactly
3. **Large file sizes**: Images consume significantly more disk space than compressed formats
4. **Performance**: Loading large images can be slow due to text parsing
5. **Hex parsing bug**: Some images may contain `#0` sequences that cause crashes
6. **No transparency**: Alpha channel is not supported
7. **Platform limitation**: Currently only works on Windows

## Future Improvements

Potential enhancements for this format:
- [ ] Add alpha channel support for transparency
- [ ] Implement basic compression (e.g., run-length encoding)
- [ ] Cross-platform support (Linux, macOS)
- [ ] Batch conversion tools
- [ ] Optional binary mode for smaller file sizes
- [ ] Image metadata support (author, creation date, etc.)

## Contributing

This is an experimental/educational project! If you have ideas for improvements or want to fix bugs, contributions are welcome.

## License

See LICENSE file for details.

## Acknowledgments

Created as a fun exploration of image file formats. YEET demonstrates the trade-offs between human readability and storage efficiency in digital media.

---

**Remember:** YEET is not meant to replace PNG, JPEG, or other established formats. It's a learning tool and experiment in alternative image storage methods! 🎨