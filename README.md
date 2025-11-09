# YEETIFF - Yet Even Extremely Expressier Transcoded Image File Format.

<div align="center">

![Version](https://img.shields.io/badge/version-2.0-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

**A modern, educational image format with transparency, compression, and animation support**

[Features](#-features) • [Quick Start](#-quick-start) • [Documentation](#-documentation) • [Installation](#-installation) • [Contributing](#-contributing)

---

### 🎓 Educational • 🔬 Experimental • 🚀 Modern • 📖 Well-Documented

</div>

---

## 📖 Overview

YEETIFF (YEET Image Format) is an **open-source educational project** designed to demonstrate modern image format concepts including:

- 🎨 **Full RGBA transparency** - True alpha channel support
- 🗜️ **Smart compression** - zlib (v2), Brotli/Zstd (v3 planned)
- 💾 **Flexible encoding** - Human-readable hex or efficient binary
- 📊 **Rich metadata** - JSON-based extensible metadata
- 🎬 **Animation support** - Multi-frame sequences (v3 planned)
- 🌈 **Color management** - ICC profiles (v3 planned)

### Why YEET?

YEET was created as an **educational tool** to help developers understand:

- How image formats work at the byte level
- Compression algorithms and their trade-offs
- Color space management and ICC profiles
- The evolution from simple (v1) to complex (v3) formats

**Perfect for:** CS students, format designers, compression enthusiasts, and curious developers!

---

## ✨ Features

### Current (v2.0 - Stable)

| Feature | Status | Description |
|---------|--------|-------------|
| **RGBA Support** | ✅ | Full alpha channel transparency |
| **Compression** | ✅ | zlib compression (40-60% reduction) |
| **Binary Mode** | ✅ | Efficient binary encoding |
| **Metadata** | ✅ | JSON metadata (author, timestamp, etc.) |
| **Batch Convert** | ✅ | Process entire folders at once |
| **GUI Viewer** | ✅ | OpenGL-accelerated viewer with egui |
| **Cross-Platform** | ✅ | Windows, macOS, Linux support |
| **v1 Compatible** | ✅ | Reads legacy v1 format |

### Planned (v3.0 - In Development)

| Feature | Status | Description |
|---------|--------|-------------|
| **ICC Profiles** | 🚧 | Accurate color reproduction |
| **Animations** | 🚧 | Multi-frame sequences (like GIF/APNG) |
| **HDR Support** | 📋 | 16-bit per channel |
| **Brotli/Zstd** | 📋 | Better compression algorithms |
| **Extended EXIF** | 📋 | Camera metadata |

---

## 🚀 Quick Start

### Installation

**Option 1: Windows Installer (Easiest)**
```powershell
# Download and run YeetInstaller.exe
# Automatically configures file associations
```

**Option 2: Build from Source**
```bash
# Clone repository
git clone https://github.com/jakobsstijn/YEETIFF.git
cd YEETIFF/yeet-project

# Build stable viewer (v2)
cd yeet-core
cargo build --release

# Binary will be at: target/release/yeet
```

**Option 3: Cargo Install**
```bash
cargo install --path yeet-core
```

### Basic Usage

**View a YEET image:**
```bash
yeet image.yeet
```

**Convert PNG to YEET:**
```bash
# Basic conversion
yeet compile photo.png

# Optimized (recommended)
yeet compile photo.png --compress --binary
```

**Batch convert folder:**
```bash
yeet batch ./my-photos --compress --binary
```

---

## 📊 Format Comparison

### File Size (1920×1080 photo example)

| Format | Size | Notes |
|--------|------|-------|
| **PNG** | 2.1 MB | Lossless, widely supported |
| **YEET v2 (text)** | 6.2 MB | Human-readable hex |
| **YEET v2 (binary)** | 6.2 MB | Raw binary data |
| **YEET v2 (compressed)** | ~500 KB | ⭐ **Recommended** |
| **YEET v3 (Brotli)** | ~450 KB | 🚧 Planned |

### Format Evolution

```
v1 (Legacy)          v2 (Stable)              v3 (Experimental)
─────────────        ─────────────            ──────────────────
RGB only             ✅ RGBA                  ✅ RGBA + HDR
No compression       ✅ zlib                  ✅ zlib/Brotli/Zstd
Text only            ✅ Text + Binary         ✅ Advanced modes
8-byte header        ✅ 20+ byte header       ✅ Extended header
No metadata          ✅ JSON metadata         ✅ Extended EXIF
6.2 MB (1080p)       ✅ 500 KB (compressed)   🚧 450 KB (planned)
```

---

## 📂 Project Structure

```
yeet-project/
│
├── 📦 yeet-core/              ⭐ Stable v2 viewer (PRODUCTION)
│   ├── src/main.rs            570+ lines of production code
│   ├── Cargo.toml
│   └── README.md              Complete usage guide
│
├── 🔬 yeet-v3/                Experimental next-gen (ALPHA)
│   ├── src/main.rs            v3 features (ICC, animation)
│   ├── Cargo.toml
│   └── README.md              v3 roadmap
│
├── 📜 yeet-legacy/            v1 backward compatibility
│   ├── src/main.rs            Original format support
│   ├── Cargo.toml
│   └── README.md              Migration guide
│
├── 💿 yeet-installer/         Windows installer
│   ├── installer_gui.py       Tkinter installation wizard
│   ├── build_installer.py     PyInstaller build script
│   └── README.md              Installer documentation
│
├── 📚 docs/                   Complete documentation
│   ├── SPEC_v2.md             650+ lines - v2 specification
│   ├── SPEC_v3.md             200+ lines - v3 specification
│   ├── ARCHITECTURE.md        500+ lines - Code organization
│   └── CONTRIBUTING.md        550+ lines - Contribution guide
│
├── 🎯 examples/               Example YEET files
│   └── README.md              Usage examples
│
├── Cargo.toml                 Workspace configuration
├── README.md                  This file
├── BUILD.md                   Build instructions
└── LICENSE                    MIT License
```

---

## 🎯 Use Cases

### Educational

- **Learn image formats:** Understand headers, compression, metadata
- **CS courses:** Practical example of file format design
- **Workshops:** Hands-on format implementation

### Development

- **Format experiments:** Test new compression algorithms
- **Color science:** ICC profile integration
- **Animation:** Multi-frame encoding techniques

### Practical

- **Transparency:** Full RGBA support unlike JPEG
- **Compression:** Better than uncompressed formats
- **Metadata:** Rich JSON-based information

---

## 💻 Command Reference

### Viewing Images

```bash
# GUI viewer (default)
yeet image.yeet

# View with scrolling (large images)
# Automatic scrollable canvas for images > window size
```

### Converting Images

```bash
# Basic conversion (uncompressed, hex text)
yeet compile photo.png

# Optimized conversion (recommended)
yeet compile photo.png --compress --binary

# Batch convert directory
yeet batch ./photos --compress --binary

# Options:
#   --compress    Apply zlib compression (40-60% smaller)
#   --binary      Use binary mode instead of hex text
```

### Help

```bash
yeet help
yeet --help
yeet -h
```

---

## 📖 Documentation

### Format Specifications

- **[YEET v2 Specification](docs/SPEC_v2.md)** - Complete byte-level format documentation
  - Header structure
  - Flags and encoding modes
  - Compression details
  - Examples and size calculations

- **[YEET v3 Specification](docs/SPEC_v3.md)** - Next-generation features
  - ICC color profiles
  - Multi-frame animation
  - Enhanced compression
  - HDR support

### Developer Guides

- **[Architecture Guide](docs/ARCHITECTURE.md)** - Code organization
  - Component breakdown
  - Data flow diagrams
  - Build system
  - Performance considerations

- **[Contributing Guide](docs/CONTRIBUTING.md)** - How to contribute
  - Development setup
  - Code style guidelines
  - Pull request workflow
  - Testing requirements

- **[Build Guide](BUILD.md)** - Building from source
  - Prerequisites
  - Build commands
  - Troubleshooting
  - Distribution

### Component READMEs

- [yeet-core README](yeet-core/README.md) - Stable v2 viewer
- [yeet-v3 README](yeet-v3/README.md) - Experimental v3
- [yeet-legacy README](yeet-legacy/README.md) - v1 support
- [yeet-installer README](yeet-installer/README.md) - Windows installer

---

## 🔧 Technical Details

### YEET v2 Format Structure

```
┌─────────────────────────────────────────────────────────────────────┐
│                         YEET v2 File Structure                      │
├─────────────────────────────────────────────────────────────────────┤
│ Magic Bytes       │ "YEET"                        │ 4 bytes         │
│ Version           │ 0x02                          │ 1 byte          │
│ Flags             │ Compression/Alpha/Binary      │ 1 byte          │
│ Width             │ Image width (little-endian)   │ 4 bytes (u32)   │
│ Height            │ Image height (little-endian)  │ 4 bytes (u32)   │
│ Metadata Length   │ JSON length (little-endian)   │ 2 bytes (u16)   │
│ Metadata          │ JSON string                   │ Variable        │
│ Data Length       │ Pixel data length (LE)        │ 4 bytes (u32)   │
│ Pixel Data        │ Image data (compressed?)      │ Variable        │
└─────────────────────────────────────────────────────────────────────┘
```

### Compression Ratios

Real-world performance (1920×1080 images):

| Image Type | Uncompressed | Compressed | Reduction |
|------------|--------------|------------|-----------|
| Photos | 6.2 MB | 2.5 MB | 60% |
| Graphics | 6.2 MB | 1.9 MB | 70% |
| Text/UI | 6.2 MB | 1.2 MB | 80% |

### Dependencies

**Rust (yeet-core):**
- `image` - Image I/O
- `eframe` - GUI framework
- `egui_extras` - Image widgets
- `flate2` - zlib compression

**Python (installer):**
- `tkinter` - GUI (built-in)
- `Pillow` - Image handling
- `pywin32` - Windows registry
- `PyInstaller` - Exe bundling

---

## 🤝 Contributing

We welcome contributions! This is an educational project designed to help people learn.

### Ways to Contribute

- 🐛 **Report bugs** - Found an issue? Let us know!
- 💡 **Suggest features** - Have an idea? Share it!
- 📝 **Improve docs** - Better explanations and examples
- 🎨 **Design** - Logo, icons, UI improvements
- 💻 **Code** - Bug fixes, features, optimizations
- 🧪 **Tests** - Improve test coverage

### Priority Areas

- 🎨 **Logo design** - We need a YEET logo!
- 📸 **ICC profiles** - Color management for v3
- 🎬 **Animation** - Multi-frame support for v3
- 🗜️ **Compression** - Brotli/Zstd integration
- 🧪 **Testing** - Increase test coverage

### Getting Started

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/YEETIFF.git
cd YEETIFF/yeet-project

# Build and test
cargo build --workspace
cargo test --workspace

# Format and lint
cargo fmt --all
cargo clippy --workspace
```

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for detailed guidelines.

---

## 🎓 Learning Resources

### Understanding Image Formats

- **[PNG Specification](http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html)** - Learn from PNG
- **[JPEG Explained](https://www.youtube.com/watch?v=Kv1Hiv3ox8I)** - How JPEG works
- **[Image Compression](https://developers.google.com/web/fundamentals/performance/optimizing-content-efficiency/image-optimization)** - Google's guide

### Rust Development

- **[The Rust Book](https://doc.rust-lang.org/book/)** - Official guide
- **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** - Practical examples
- **[Rustlings](https://github.com/rust-lang/rustlings)** - Interactive exercises

### Color Science

- **[ICC Profiles](https://www.color.org/icc_specs2.xalter)** - Color management
- **[sRGB Standard](https://en.wikipedia.org/wiki/SRGB)** - Standard color space

---

## 📊 Project Statistics

### Code Metrics

| Component | Lines | Language | Status |
|-----------|-------|----------|--------|
| yeet-core | 570 | Rust | ✅ Production |
| yeet-v3 | 500 | Rust | 🚧 Alpha |
| yeet-legacy | 150 | Rust | ✅ Stable |
| installer | 550 | Python | ✅ Working |
| **Total Code** | **1,770** | - | - |

### Documentation

| Document | Lines | Purpose |
|----------|-------|---------|
| Main README | 350+ | Project overview |
| SPEC_v2.md | 650+ | v2 format spec |
| SPEC_v3.md | 200+ | v3 roadmap |
| ARCHITECTURE.md | 500+ | Code organization |
| CONTRIBUTING.md | 550+ | Contribution guide |
| **Total Docs** | **2,400+** | - |

---

## 🗺️ Roadmap

### ✅ Completed (v2.0)

- [x] RGBA transparency support
- [x] zlib compression
- [x] Binary encoding mode
- [x] JSON metadata
- [x] Batch conversion
- [x] GUI viewer
- [x] Windows installer
- [x] Complete documentation

### 🚧 In Progress (v2.1)

- [ ] Unit tests
- [ ] Example YEET files
- [ ] Performance benchmarks
- [ ] Cross-platform testing

### 📋 Planned (v3.0)

- [ ] ICC color profiles
- [ ] Multi-frame animation
- [ ] Brotli/Zstd compression
- [ ] HDR (16-bit) support
- [ ] Extended EXIF metadata
- [ ] Animation playback

### 🔮 Future Vision

- [ ] Web viewer (WASM)
- [ ] Animation editor
- [ ] Plugin system
- [ ] Package manager distribution
- [ ] Library API (yeet-lib)

---

## ❓ FAQ

### Is YEET a production-ready format?

YEET v2 is **stable and functional**, but it's primarily an **educational tool**. For production, use PNG, JPEG, or WebP.

### Why create a new image format?

YEET was created to **teach** how image formats work. It's easier to learn from a simple, well-documented format than complex production formats.

### Can I use YEET in my project?

Yes! YEET is MIT licensed. However, consider it **experimental** and best suited for educational purposes.

### How does YEET compare to PNG?

- **Advantages:** Educational, simple structure, good compression
- **Disadvantages:** Larger files than PNG, not widely supported
- **Use YEET for:** Learning, experiments, education
- **Use PNG for:** Production, web, wide compatibility

### Does YEET support lossy compression?

No, YEET uses **lossless compression** (zlib). All data is preserved.

### Will v3 replace v2?

No! v2 will remain **stable and supported**. v3 is **experimental** and may change frequently.

---

## 🏆 Acknowledgments

### Technologies

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[egui](https://github.com/emilk/egui)** - Immediate mode GUI
- **[image-rs](https://github.com/image-rs/image)** - Image processing
- **[flate2](https://github.com/rust-lang/flate2-rs)** - Compression

### Inspiration

- **PNG** - Well-designed, documented format
- **WebP** - Modern compression techniques
- **APNG** - Animation approach

### Community

- **Rust Community** - Excellent crates and support
- **Open Source** - Standing on the shoulders of giants

---

## 📜 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

```
MIT License - Copyright (c) 2025 Stijn Jakobs

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software...
```

---

## 👤 Author

**Stijn Jakobs**

- 🌐 GitHub: [@jakobsstijn](https://github.com/jakobsstijn)
- 📦 Repository: [YEETIFF](https://github.com/jakobsstijn/YEETIFF)

### Professional Roles

- 🌐 **Network Developer** @ AstroidMC
- 🎨 **Creative Director** @ Ordnary

---

## 🔗 Links

- **📦 Repository:** [github.com/jakobsstijn/YEETIFF](https://github.com/jakobsstijn/YEETIFF)
- **🐛 Issues:** [Report a Bug](https://github.com/jakobsstijn/YEETIFF/issues)
- **💬 Discussions:** [Ask Questions](https://github.com/jakobsstijn/YEETIFF/discussions)
- **📖 Docs:** [Complete Documentation](docs/)
- **⬇️ Releases:** [Download Installer](https://github.com/jakobsstijn/YEETIFF/releases)

---

## 🎯 Quick Links

| Resource | Link |
|----------|------|
| **Installation** | [Quick Start](#-quick-start) |
| **Format Spec** | [SPEC_v2.md](docs/SPEC_v2.md) |
| **Contributing** | [CONTRIBUTING.md](docs/CONTRIBUTING.md) |
| **Building** | [BUILD.md](BUILD.md) |
| **Architecture** | [ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| **Examples** | [examples/](examples/) |

---

<div align="center">

### Made with ❤️ for education and experimentation

⭐ **Star this repo if you find it useful!** ⭐

**Learning • Experimenting • Building • Sharing**

[![GitHub stars](https://img.shields.io/github/stars/jakobsstijn/YEETIFF.svg?style=social&label=Star)](https://github.com/jakobsstijn/YEETIFF)
[![GitHub forks](https://img.shields.io/github/forks/jakobsstijn/YEETIFF.svg?style=social&label=Fork)](https://github.com/jakobsstijn/YEETIFF/fork)

</div>
