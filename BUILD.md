# YEET Project - Quick Build Guide

## Build All Components

### Prerequisites
- Rust 1.70+ installed
- Python 3.7+ (for installer)

### Build Everything

**Windows:**
```powershell
# Build all Rust components
cargo build --release --workspace

# Build installer
cd yeet-installer
python build_installer.py
```

**Linux/macOS:**
```bash
# Build all Rust components
cargo build --release --workspace

# Installer is Windows-only
```

## Build Individual Components

### yeet-core (Stable Viewer)

```bash
cd yeet-core
cargo build --release
```

Output: `target/release/yeet` or `target/release/yeet.exe`

### yeet-v3 (Experimental)

```bash
cd yeet-v3
cargo build --release
```

Output: `target/release/yeet-v3` or `target/release/yeet-v3.exe`

### yeet-legacy (v1 Support)

```bash
cd yeet-legacy
cargo build --release
```

Output: `target/release/yeet-legacy` or `target/release/yeet-legacy.exe`

### Windows Installer

```bash
cd yeet-installer

# Install Python dependencies
pip install -r requirements.txt

# Build installer
python build_installer.py
```

Output: `dist/YeetInstaller.exe`

## Quick Test

```bash
# Test yeet-core
cd yeet-core
cargo run -- compile test.png --compress --binary
cargo run -- test.yeet
```

## Development Build (Fast)

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Run without building
cargo run --bin yeet -- image.yeet
```

## Release Build (Optimized)

```bash
# Optimized build (slower compilation, fast runtime)
cargo build --release

# With maximum optimizations
cargo build --release --locked
```

## Clean Build

```bash
# Remove build artifacts
cargo clean

# Rebuild everything
cargo build --release --workspace
```

## Common Issues

**Issue:** "command not found: cargo"
- Install Rust: https://rustup.rs/

**Issue:** "linker not found"
- **Windows:** Install Visual Studio Build Tools
- **Linux:** `sudo apt install build-essential`
- **macOS:** `xcode-select --install`

**Issue:** Python module not found
```bash
pip install -r yeet-installer/requirements.txt
```

## Distribution

**Create release:**
1. Build all components: `cargo build --release --workspace`
2. Build installer: `cd yeet-installer && python build_installer.py`
3. Collect binaries:
   - `target/release/yeet` (or `.exe`)
   - `yeet-installer/dist/YeetInstaller.exe`

## Next Steps

- Read [ARCHITECTURE.md](docs/ARCHITECTURE.md) for code structure
- See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for development guide
- Check [README.md](README.md) for usage instructions
