# YEET Image Format - Complete Installer

Complete Windows installer for the YEET image format (.yeet files). This creates a standalone installer executable that bundles everything needed to view .yeet files on Windows.

## 🎯 What is .yeet format?

**YEETIFF** (Yet Even Expressier Transcoded Image File Format) is a flexible image format with two versions:

**YEET v2 (Current):**
- **Header**: Magic bytes "YEET" + version + flags
- **Alpha channel**: Full RGBA support with transparency
- **Compression**: Optional zlib compression for smaller files
- **Binary mode**: Raw bytes or human-readable hex
- **Metadata**: JSON metadata (author, date, software)
- **Structure**: Variable size based on options

**YEET v1 (Legacy):**
- **First 4 bytes**: Width (unsigned 32-bit integer)
- **Next 4 bytes**: Height (unsigned 32-bit integer)
- **Remaining**: RGB hex color codes (6 chars per pixel)

The viewer supports both formats automatically!

## 📦 What does this installer do?

The installer provides a **complete solution** for YEET file support on Windows:
- ✅ Installs YEET Viewer to `C:\Program Files\YeetViewer\`
- ✅ Registers `.yeet` file extension in Windows Registry
- ✅ Associates .yeet files with the viewer (double-click to open)
- ✅ Adds Windows context menu integration
- ✅ Creates uninstaller entry in Windows Programs & Features
- ✅ Multi-step wizard GUI with progress tracking
- ✅ Full install/uninstall support

## 🚀 Quick Start - Build the Installer

### Prerequisites
- Python 3.7 or higher
- Rust toolchain (to build the YEET viewer)
- Windows OS

### Step 1: Build the YEET Viewer (Rust)

First, build the Rust-based viewer executable:

```bash
cd ../yeet-format
cargo build --release
```

This creates: `yeet-format/target/release/yeet.exe`

### Step 2: Build the Installer

```bash
# Option A: Use the batch script (easiest)
BUILD.bat

# Option B: Manual build
pip install -r requirements.txt
python build_installer.py
```

The installer will be created at: `dist/YeetInstaller.exe` (~21 MB)

### What gets bundled?

The installer executable contains:
- 🎨 YEET Viewer (yeet.exe) - embedded
- 🖼️ Logo image (if logo.png exists)
- 🐍 Python runtime + all dependencies
- 📝 Viewer Python script
- 🔧 Registry configuration tools

## 📁 Project Structure

```
yeet-installer/
├── installer_gui.py      # Main installer wizard GUI
├── yeet_viewer.py         # Python-based YEET viewer (fallback)
├── build_installer.py     # Build script for standalone installer
├── build_exe.py           # Build script for viewer only
├── requirements.txt       # Python dependencies
├── logo.png              # Optional: Logo for installer GUI
├── BUILD.bat             # Quick build script for Windows
└── dist/
    └── YeetInstaller.exe  # Generated standalone installer
```

## 🎨 How to Use the Installer

### For End Users:

1. **Run the installer** (as Administrator)
   ```bash
   dist\YeetInstaller.exe
   ```

2. **Follow the wizard:**
   - Welcome screen
   - License agreement (MIT)
   - Choose Install or Uninstall
   - Installation progress with detailed logging
   - Completion confirmation

3. **Done!** Now you can double-click any `.yeet` file to view it

### For Developers/Distributors:

**Option 1: Test without building**
```bash
python installer_gui.py
```

**Option 2: Build and distribute**
```bash
BUILD.bat
# Share dist/YeetInstaller.exe with users
```

## 🔧 Advanced Configuration

### How the installer finds the viewer

The installer searches for the YEET viewer in this priority order:

1. **Bundled in installer** (when running as compiled EXE) - `sys._MEIPASS/yeet.exe`
2. **Rust release build** - `../yeet-format/target/release/yeet.exe`
3. **Rust debug build** - `../yeet-format/target/debug/yeet.exe`
4. **Python viewer build** - `./dist/YeetViewer.exe`

### Customizing the build

Edit `build_installer.py` to customize:

```python
# Add custom icon
"--icon", "custom_icon.ico",

# Change installer name
"--name", "MyCustomInstaller",

# Add more data files
"--add-data", "extra_file.txt;.",
```

### Adding a logo

Place a `logo.png` (128x128 or larger) in the installer directory. It will be automatically included and displayed in the installer GUI.

## 📝 What the Installer Does Internally

### Installation Process:

1. **Check admin privileges** - Required for Program Files and Registry
2. **Create installation directory** - `C:\Program Files\YeetViewer\`
3. **Copy viewer executable** - Extracts bundled `yeet.exe`
4. **Register file extension** - Creates registry keys:
   ```
   HKEY_CLASSES_ROOT\.yeet → "YeetImageFile"
   HKEY_CLASSES_ROOT\YeetImageFile → "YEET Image File"
   HKEY_CLASSES_ROOT\YeetImageFile\DefaultIcon
   HKEY_CLASSES_ROOT\YeetImageFile\shell\open\command
   ```
5. **Set file associations** - Configure Content Type and PerceivedType
6. **Create uninstaller entry** - Add to Windows Programs & Features
7. **Notify Windows** - SHChangeNotify to refresh file associations

### Uninstallation Process:

1. **Remove registry entries** - Delete all .yeet associations
2. **Remove files** - Delete `C:\Program Files\YeetViewer\`
3. **Remove uninstaller entry** - Clean up Programs & Features
4. **Notify Windows** - Refresh Explorer

## 🛠️ Dependencies

### Build-time (for creating installer):
```txt
pyinstaller>=6.0.0    # Bundles Python app into EXE
Pillow>=10.0.0        # Image processing (for viewer)
pywin32>=306          # Windows API access (registry, etc.)
```

### Runtime (for end users):
**None!** The standalone installer includes everything.

## ⚠️ Troubleshooting

### "Build failed" error

1. **Uninstall old pathlib:**
   ```bash
   py -m pip uninstall pathlib -y
   ```

2. **Upgrade PyInstaller:**
   ```bash
   py -m pip install --upgrade pyinstaller
   ```

3. **Check dependencies:**
   ```bash
   pip install -r requirements.txt
   ```

### "yeet.exe not found" during installation

Build the Rust viewer first:
```bash
cd ../yeet-format
cargo build --release
```

### Antivirus blocks the installer

This is a false positive common with PyInstaller executables:
- Upload to VirusTotal to verify safety
- Add exception in your antivirus
- Consider code signing certificate for distribution

### "Administrator rights required" error

Right-click `YeetInstaller.exe` → "Run as administrator"

Or the installer will prompt to restart with admin rights.

## 📜 License

MIT License - See installer wizard for full text

## 🤝 Credits

- **Developed by:** Stijn Jakobs
- **YEET Format:** Original specification
- **Installer Framework:** PyInstaller + Tkinter

## 🔗 Related Files

- `../yeet-format/` - Main Rust implementation
- `yeet_viewer.py` - Python viewer (fallback)
- `installer_gui.py` - Installer GUI code
- `build_installer.py` - Build automation

---

**Need help?** Check the installation log in the installer GUI or run `python installer_gui.py` for development mode.

## License

This project is designed to work with the .yeet image format. See the original project for format specifications.

## References

- [Windows File Type Registration Documentation](https://learn.microsoft.com/en-us/windows/win32/shell/how-to-register-a-file-type-for-a-new-application)
- Original YEET/BRUH format implementation
