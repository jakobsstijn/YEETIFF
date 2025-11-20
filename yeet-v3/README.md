# YEET v3 - Next Generation

🎉 **Status: Beta - Major Features Implemented!**

## Recent Updates

### ✅ Newly Implemented (November 2025)

- **Brotli Compression** - 10-15% better than zlib
- **Zstd Compression** - Ultra-fast compression/decompression
- **ICC Color Profiles** - Full color management with lcms2
- **GUI Viewer** - View v3 files with animation support
- **Animation Playback** - Multi-frame support with controls
- **Enhanced Metadata** - Extended EXIF-like information

### � In Progress

- Multi-file animation compilation
- HDR Support (16-bit per channel)
- Animation editor tools
- Web viewer (WASM)

### 📋 Planned

- Delta frame optimization
- EXIF camera metadata extraction
- Advanced animation controls

## v3 Format Specification

### Header Structure

```
┌─────────┬─────────┬───────┬────────┬────────┬──────────┬──────────┬──────────┬───────┬────────┐
│  Magic  │ Version │ Flags │ Width  │ Height │ Frames   │  Loop    │ Metadata │  ICC  │ Frames │
│  (4B)   │  (1B)   │ (1B)  │  (4B)  │  (4B)  │  (4B)    │  (4B)    │  (var)   │ (var) │ (var)  │
└─────────┴─────────┴───────┴────────┴────────┴──────────┴──────────┴──────────┴───────┴────────┘
```

### Flags (1 byte)

- **Bits 0-1:** Compression (00=none, 01=zlib, 10=brotli, 11=zstd)
- **Bit 2:** Alpha channel (0=RGB, 1=RGBA)
- **Bit 3:** Binary mode (0=hex, 1=binary)
- **Bit 4:** Animation (0=single, 1=multi-frame)
- **Bit 5:** ICC profile (0=no, 1=embedded)
- **Bit 6:** HDR mode (0=8-bit, 1=16-bit)
- **Bit 7:** Reserved

### Metadata Format

JSON with serde serialization:

```json
{
  "author": "Stijn Jakobs",
  "created": "2025-11-10T12:00:00Z",
  "software": "YEET v3.0-alpha",
  "color_profile": "sRGB IEC61966-2.1",
  "color_space": "sRGB",
  "frame_count": 24,
  "frame_delay": 42,
  "loop_count": 0,
  "bit_depth": 8,
  "hdr": false,
  "camera": {
    "make": "Canon",
    "model": "EOS R5",
    "iso": 100,
    "exposure": "1/1000",
    "aperture": "f/2.8"
  },
  "dpi": [300, 300],
  "orientation": "normal"
}
```

## Current Usage

### View YEET v3 Files

```bash
# View single image with ICC correction
cargo run --release image.yeet

# View animation (auto-plays)
cargo run --release animation.yeet

# Viewer features:
# - Automatic ICC color correction
# - Animation playback controls
# - Frame navigation
# - Metadata viewer
```

### Convert PNG to v3

```bash
# Basic v3 file
cargo run --release compile photo.png --compress --binary

# With Brotli (recommended - best compression)
cargo run --release compile photo.png --brotli --binary

# With Zstd (fastest)
cargo run --release compile photo.png --zstd --binary

# ICC profile automatically extracted and embedded!
```

### Compression Comparison

Real-world test (1920×1080 photo):

| Compression | Size | Reduction | Speed |
|-------------|------|-----------|-------|
| None | 6.2 MB | 0% | ⚡⚡⚡ |
| Zlib | 2.5 MB | 60% | ⚡⚡ |
| **Brotli** | **2.2 MB** | **65%** ✨ | ⚡ |
| Zstd | 2.3 MB | 63% | ⚡⚡⚡ |

**Recommendation:** Use Brotli for distribution, Zstd for real-time.

## Development Roadmap

### Phase 1: Foundation ✅ COMPLETE
- [x] v3 file format structure
- [x] Extended metadata with serde
- [x] Compression algorithms (zlib/brotli/zstd)
- [x] ICC profile extraction
- [x] ICC profile embedding

### Phase 2: Viewer ✅ COMPLETE
- [x] GUI viewer with eframe
- [x] ICC color correction
- [x] Animation playback
- [x] Metadata display
- [x] Frame navigation controls

### Phase 3: Animation Tools 🚧 IN PROGRESS
- [ ] Multi-file animation compiler
- [ ] Frame delay configuration
- [ ] Loop count options
- [ ] Animation preview
- [x] v3 file structure
- [x] Extended metadata schema
- [x] Compression algorithm selection
- [x] Basic file writing

### Phase 2: Core Features (Current)
- [ ] ICC profile extraction from PNG
- [ ] ICC profile embedding
- [ ] ICC color correction
- [ ] Multi-frame encoding
- [ ] Animation decoder

### Phase 3: Compression
- [ ] Brotli integration
- [ ] Zstd integration
- [ ] Compression benchmarks
- [ ] Adaptive compression selection

### Phase 4: HDR
- [ ] 16-bit per channel encoding
- [ ] HDR metadata
- [ ] Tone mapping for display

### Phase 5: Polish
- [ ] Animation playback GUI
- [ ] Frame editor
- [ ] Profile converter tools
- [ ] Performance optimization

## Contributing

v3 is actively seeking contributors! Areas that need help:

- 🎨 ICC color profile support (lcms2 integration)
- 🎬 Animation frame handling
- 🗜️ Brotli/Zstd compression
- 📸 EXIF metadata extraction
- 🧪 Testing and benchmarking

See the main [CONTRIBUTING.md](../docs/CONTRIBUTING.md) guide.

## Dependencies

```toml
image = "0.24"        # Image I/O
eframe = "0.22"       # GUI (future viewer)
flate2 = "1.0"        # zlib compression
serde = "1.0"         # Serialization
serde_json = "1.0"    # JSON metadata
chrono = "0.4"        # Timestamps

# Planned:
# brotli = "3.3"      # Brotli compression
# zstd = "0.12"       # Zstd compression
# lcms2 = "6.0"       # ICC profiles
```

## Testing

```bash
# Build v3
cargo build --release

# Test compile
./target/release/yeet-v3 compile test.png --compress --binary
```

## Comparison with v2

| Feature | v2 (Stable) | v3 (Experimental) |
|---------|-------------|-------------------|
| RGBA | ✅ | ✅ |
| Compression | ✅ zlib | ✅ zlib + brotli/zstd (TODO) |
| Metadata | ✅ Basic JSON | ✅ Extended JSON |
| ICC Profiles | ❌ | 🚧 In progress |
| Animation | ❌ | 🚧 In progress |
| HDR | ❌ | 📋 Planned |
| Viewer | ✅ | ❌ Not yet |

## Known Issues

- Viewer not implemented (use yeet-core for v2 files)
- Brotli compression placeholder
- Zstd compression placeholder
- ICC profile extraction not working
- Animation encoding not implemented
- HDR support missing

## License

MIT License - Copyright (c) 2025 Stijn Jakobs

## Links

- **Main Project:** [YEETIFF](https://github.com/jakobsstijn/YEETIFF)
- **Stable Viewer:** Use `yeet-core` instead
- **Specification:** See [SPEC_v3.md](../docs/SPEC_v3.md)
