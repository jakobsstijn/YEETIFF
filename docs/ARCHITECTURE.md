# YEETIFF Project Architecture

This document describes the code organization and architecture of the YEET image format project.

## Project Structure

```
yeet-project/
├── yeet-core/              # Stable v2 implementation (PRODUCTION)
│   ├── src/
│   │   └── main.rs         # All-in-one viewer and converter
│   ├── Cargo.toml
│   └── README.md
│
├── yeet-v3/                # Experimental v3 (DEVELOPMENT)
│   ├── src/
│   │   └── main.rs         # v3 features (ICC, animation, HDR)
│   ├── Cargo.toml
│   └── README.md
│
├── yeet-legacy/            # v1 format support (LEGACY)
│   ├── src/
│   │   └── main.rs         # Original format parser
│   ├── Cargo.toml
│   └── README.md
│
├── yeet-installer/         # Windows installer (DISTRIBUTION)
│   ├── installer_gui.py    # Tkinter installation wizard
│   ├── build_installer.py  # PyInstaller build script
│   ├── requirements.txt
│   └── README.md
│
├── docs/                   # Documentation
│   ├── SPEC_v2.md          # v2 format specification
│   ├── SPEC_v3.md          # v3 format specification
│   ├── ARCHITECTURE.md     # This file
│   └── CONTRIBUTING.md     # Contribution guide
│
├── examples/               # Example YEET files
│
├── Cargo.toml              # Workspace configuration
└── README.md               # Main project README
```

## Component Overview

### yeet-core (v2 - Stable)

**Purpose:** Production-ready viewer and converter

**Features:**
- RGBA transparency support
- zlib compression
- Binary encoding
- JSON metadata
- Batch conversion
- GUI viewer (egui)
- Backward compatible with v1

**Technology Stack:**
- **Language:** Rust 2021
- **GUI:** eframe + egui
- **Image I/O:** image crate
- **Compression:** flate2 (zlib)

**Main Components:**

```rust
main.rs (500+ lines)
├── YeetMetadata          // Metadata structure
├── compress_data()       // zlib compression
├── decompress_data()     // zlib decompression
├── png_to_yeet_v2()      // PNG → YEET converter
├── yeet_to_png_v2()      // YEET → PNG converter
├── yeet_to_png_v1()      // Legacy v1 parser
├── ImagePreview          // egui viewer widget
└── main()                // CLI argument handling
```

**Build Output:**
- Binary: `yeet` or `yeet.exe`
- Size: ~5-8 MB (release build, stripped)

### yeet-v3 (Experimental)

**Purpose:** Next-generation features (in development)

**Planned Features:**
- ICC color profiles
- Multi-frame animation
- Brotli/Zstd compression
- HDR (16-bit per channel)
- Extended EXIF-like metadata

**Technology Stack:**
- **Language:** Rust 2021
- **Serialization:** serde + serde_json
- **Compression:** flate2, (brotli, zstd - planned)
- **Color:** lcms2 (planned)

**Current Status:**
- ✅ Basic v3 file structure
- ✅ Extended metadata schema
- 🚧 ICC profile support (in progress)
- 🚧 Animation encoding (in progress)
- ❌ Viewer (not implemented)

### yeet-legacy (v1 Support)

**Purpose:** Backward compatibility with original format

**v1 Format:**
- 8-byte header (width + height)
- Hex color codes (RRGGBB)
- No compression, no alpha, no metadata

**Usage:** Rare - most v1 files should be converted to v2

### yeet-installer (Windows Distribution)

**Purpose:** User-friendly Windows installation

**Components:**

1. **installer_gui.py**
   - Tkinter wizard UI
   - File association registration
   - Program Files installation
   - Uninstaller creation

2. **build_installer.py**
   - PyInstaller configuration
   - Bundles yeet.exe
   - Creates standalone installer

**Build Process:**
```
1. Build yeet-core → yeet.exe
2. Run build_installer.py
3. PyInstaller packages everything
4. Output: YeetInstaller.exe (~20 MB)
```

## Data Flow

### PNG to YEET Conversion

```
PNG File
  │
  ├─→ Load with image crate
  │
  ├─→ Detect alpha channel
  │
  ├─→ Encode pixels (hex or binary)
  │
  ├─→ Compress? (zlib)
  │
  ├─→ Build header
  │
  ├─→ Write metadata
  │
  └─→ Write pixel data
       │
       └─→ YEET File
```

### YEET to Display

```
YEET File
  │
  ├─→ Read magic bytes
  │
  ├─→ Parse header (version, flags, dimensions)
  │
  ├─→ Read metadata
  │
  ├─→ Read pixel data
  │
  ├─→ Decompress? (zlib)
  │
  ├─→ Parse pixels (hex or binary)
  │
  ├─→ Build image buffer
  │
  ├─→ Save temp PNG
  │
  └─→ Display in egui viewer
```

## Format Evolution

### v1 → v2 Migration

**Changes:**
- Added magic bytes "YEET"
- Added version byte
- Added flags (compression, alpha, binary)
- Added metadata (JSON)
- Added compression support

**Backward Compatibility:**
- v2 parser detects v1 files (no magic bytes)
- Falls back to v1 parser
- User should re-convert to v2

### v2 → v3 Migration

**Planned Changes:**
- Extended metadata schema
- ICC profile embedding
- Multi-frame support
- Multiple compression algorithms
- HDR pixel encoding

**Forward Compatibility:**
- v2 files include v3-compatible metadata fields
- v3 viewer will read v2 files
- v2 viewer ignores unknown v3 metadata

## Code Conventions

### Rust Style

- **Edition:** 2021
- **Formatting:** `cargo fmt` (rustfmt)
- **Linting:** `cargo clippy`
- **Documentation:** Doc comments (`///`)

### File Organization

**Single-file binaries:**
- yeet-core: `src/main.rs` (all-in-one)
- yeet-v3: `src/main.rs` (monolithic for now)
- yeet-legacy: `src/main.rs` (simple)

**Future modularization (v3):**
```
src/
├── main.rs            // Entry point
├── format/
│   ├── v2.rs          // v2 parser
│   ├── v3.rs          // v3 parser
│   └── common.rs      // Shared types
├── compression/
│   ├── zlib.rs
│   ├── brotli.rs
│   └── zstd.rs
├── color/
│   └── icc.rs         // ICC profiles
├── animation/
│   └── frames.rs      // Multi-frame handling
└── gui/
    └── viewer.rs      // egui viewer
```

### Python Style (Installer)

- **Version:** Python 3.7+
- **GUI:** tkinter (built-in)
- **Packaging:** PyInstaller
- **Encoding:** UTF-8

## Build System

### Workspace Configuration

All Rust projects use cargo workspace:

```toml
[workspace]
members = ["yeet-core", "yeet-v3", "yeet-legacy"]
```

**Benefits:**
- Shared dependency resolution
- Unified `target/` directory
- Parallel builds
- Consistent versioning

### Build Commands

```bash
# Build everything
cargo build --release

# Build specific component
cargo build --release -p yeet-core

# Build installer (requires Python)
cd yeet-installer
python build_installer.py

# Run tests
cargo test --workspace

# Format code
cargo fmt --all

# Lint
cargo clippy --workspace
```

## Dependencies

### Core Dependencies (Shared)

```toml
image = "0.24"         # Image loading/saving
eframe = "0.22"        # GUI framework
egui_extras = "0.22"   # Image widgets
flate2 = "1.0"         # zlib compression
```

### v3-Specific

```toml
serde = "1.0"          # Serialization
serde_json = "1.0"     # JSON
chrono = "0.4"         # Timestamps
# (planned: brotli, zstd, lcms2)
```

### Installer

```txt
Pillow          # Image handling
pywin32         # Windows registry
PyInstaller     # Exe bundling
```

## Performance Considerations

### Memory Usage

**yeet-core:**
- Loads entire image into memory
- Peak usage: ~4× uncompressed image size
- Example: 1920×1080 RGBA → ~32 MB peak

**Future optimizations:**
- Streaming decompression
- Tiled loading for large images
- Memory-mapped files

### Speed

**Conversion (PNG → YEET):**
- 1920×1080: ~100-300ms (uncompressed)
- 1920×1080: ~500-800ms (compressed)

**Viewing (YEET → Display):**
- Load + decompress: ~50-200ms
- GUI rendering: 60 FPS (egui)

### File Size

**Compression ratios (1920×1080 photo):**
- Uncompressed: 6.2 MB
- zlib compressed: ~500 KB (92% reduction)
- v3 brotli (planned): ~450 KB (93% reduction)

## Testing Strategy

### Current Tests

- ✅ Basic PNG → YEET conversion
- ✅ YEET → PNG round-trip
- ✅ Compression/decompression
- ✅ v1 backward compatibility

### Planned Tests

- Unit tests for each module
- Integration tests for full pipeline
- Benchmark suite for performance
- Fuzzing for robustness
- Format validation tests

## Distribution

### Release Artifacts

1. **Source code** (GitHub)
2. **yeet-core binary** (Linux, macOS, Windows)
3. **YeetInstaller.exe** (Windows only)
4. **Documentation** (Markdown)

### Installation Methods

**Method 1: Pre-built Installer**
- Download YeetInstaller.exe
- Run installation wizard
- Auto-configures file associations

**Method 2: Cargo Install**
```bash
cargo install --path yeet-core
```

**Method 3: Build from Source**
```bash
git clone https://github.com/jakobsstijn/YEETIFF
cd yeet-project/yeet-core
cargo build --release
```

## Future Architecture Plans

### Modularization (v3)

Split `yeet-v3/src/main.rs` into:
- Format modules (v2, v3 parsers)
- Compression modules
- Color management module
- Animation module
- GUI module

### Plugin System

Allow third-party compression algorithms:

```rust
trait CompressionPlugin {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
    fn decompress(&self, data: &[u8]) -> Vec<u8>;
}
```

### Library vs Binary

Create `yeet-lib` crate for embedding in other projects:

```rust
use yeet_lib::{YeetImage, YeetError};

let img = YeetImage::from_file("image.yeet")?;
img.save_png("output.png")?;
```

## Contributing Guidelines

See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Code style guide
- Pull request process
- Issue templates
- Development setup

---

**Last Updated:** November 2025  
**Maintainer:** Stijn Jakobs  
**License:** MIT
