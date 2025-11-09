# YEET v3 Format Specification

**Status:** Planning / Development  
**Target Release:** Q1 2026  
**Backward Compatibility:** Full support for v1 and v2 files

---

## Overview

YEET v3 extends the format with professional features while maintaining the educational and experimental spirit of the project.

## New Features

### 1. ICC Color Profile Support

**Purpose:** Ensure accurate color reproduction across different displays and devices.

**Implementation:**
- Embedded ICC profile data in metadata
- Support for common profiles: sRGB, Adobe RGB, Display P3, ProPhoto RGB
- Profile can be extracted and applied during viewing

**Header Addition:**
```
- ICC Profile Length: u32 (4 bytes)
- ICC Profile Data: (variable, optional)
```

**Metadata Example:**
```json
{
  "color_profile": "sRGB IEC61966-2.1",
  "color_space": "RGB",
  "profile_embedded": true
}
```

### 2. Multi-Frame Animation Support

**Purpose:** Support animated images similar to GIF or APNG.

**Specification:**

**Header Changes:**
```
- Frame Count: u32 (4 bytes)
- Loop Count: u32 (4 bytes, 0 = infinite)
```

**Per-Frame Data:**
```
- Frame Index: u32 (4 bytes)
- Frame Delay: u32 (4 bytes, milliseconds)
- Frame Data Length: u32 (4 bytes)
- Frame Pixel Data: (variable)
```

**Metadata Example:**
```json
{
  "frame_count": 24,
  "frame_delay": 42,
  "loop_count": 0,
  "total_duration": 1008,
  "animation": true
}
```

**File Structure (Animated):**
```
┌─────────┬─────────┬───────┬────────┬────────┬────────────┬──────────┬─────────────┐
│  Magic  │ Version │ Flags │ Width  │ Height │ FrameCount │ LoopCnt  │  Metadata   │
└─────────┴─────────┴───────┴────────┴────────┴────────────┴──────────┴─────────────┘
┌──────────────┬─────────────┬──────────────┬─────────────┐
│  Frame 0     │  Frame 1    │  Frame 2     │  ...        │
└──────────────┴─────────────┴──────────────┴─────────────┘
```

### 3. Enhanced Compression

**New Algorithms:**
- **Brotli** - Better compression ratios than zlib
- **Zstd** - Fast compression/decompression
- **LZ4** - Ultra-fast decompression

**Flags Update:**
```
- Bit 0-1: Compression type
  - 00: None
  - 01: Zlib (v2 compatible)
  - 10: Brotli
  - 11: Zstd
- Bit 2: Alpha channel
- Bit 3: Binary mode
- Bit 4: Animation
- Bit 5: ICC Profile embedded
- Bit 6-7: Reserved
```

### 4. HDR Support

**16-bit Per Channel:**
- RGB16/RGBA16 support
- Extended dynamic range
- Better for professional photography

**Pixel Encoding:**
- Text mode: 12 or 16 hex chars (RRRRGGGGBBBB or RRRRGGGGBBBBAAAA)
- Binary mode: 6 or 8 bytes per pixel

### 5. Extended Metadata

**New Fields:**
```json
{
  "author": "Stijn Jakobs",
  "created": "2025-11-10T12:00:00Z",
  "software": "YEET v3.0",
  "color_profile": "sRGB",
  "frame_count": 1,
  "frame_delay": 0,
  "loop_count": 0,
  "bit_depth": 8,
  "hdr": false,
  "orientation": "normal",
  "dpi": [300, 300],
  "camera": {
    "make": "Canon",
    "model": "EOS R5",
    "iso": 100,
    "exposure": "1/1000",
    "aperture": "f/2.8"
  }
}
```

## Format Comparison

| Feature | v1 | v2 | v3 (Planned) |
|---------|----|----|--------------|
| Alpha Channel | ❌ | ✅ | ✅ |
| Compression | ❌ | ✅ (zlib) | ✅ (zlib/brotli/zstd) |
| Binary Mode | ❌ | ✅ | ✅ |
| Metadata | ❌ | ✅ (basic) | ✅ (extended) |
| ICC Profiles | ❌ | ❌ | ✅ |
| Animation | ❌ | ❌ | ✅ |
| HDR | ❌ | ❌ | ✅ |
| Bit Depth | 8-bit | 8-bit | 8/16-bit |

## Implementation Roadmap

### Phase 1: Foundation (Current)
- [x] v3 metadata structure
- [x] Forward compatibility fields
- [ ] ICC profile parsing library integration

### Phase 2: Core Features
- [ ] ICC profile embedding
- [ ] ICC profile extraction and application
- [ ] Multi-frame parsing
- [ ] Animation playback in viewer

### Phase 3: Enhancement
- [ ] Brotli/Zstd compression
- [ ] HDR support
- [ ] Extended EXIF-like metadata

### Phase 4: Polish
- [ ] Animation editor
- [ ] Profile conversion tools
- [ ] Performance optimization

## Example Usage (Future)

```bash
# Convert with ICC profile
cargo run compile photo.png --compress --binary --profile sRGB

# Create animation from frames
cargo run animate frames/*.png --delay 100 --loop 0 --compress

# Extract ICC profile
cargo run extract-profile image.yeet --output profile.icc

# Convert to HDR
cargo run compile hdr_photo.png --hdr --16bit
```

## File Size Impact

**Single Frame (1920×1080):**
- v2 (binary, compressed): ~500 KB
- v3 (binary, compressed, ICC): ~502 KB (+2 KB for profile)
- v3 (binary, brotli, ICC): ~450 KB (10% better)

**Animation (10 frames, 1920×1080):**
- v3 (binary, compressed): ~4.5 MB
- v3 (binary, brotli): ~4.0 MB
- GIF equivalent: ~2 MB (but lower quality)
- APNG equivalent: ~3.5 MB

## Migration Path

**v2 → v3 Converter:**
```bash
# Automatic upgrade
cargo run upgrade image_v2.yeet --output image_v3.yeet

# With new features
cargo run upgrade image_v2.yeet --add-profile sRGB --output image_v3.yeet
```

**Backward Compatibility:**
- v3 viewer reads v1, v2, v3
- v3 files with basic features work in v2 viewer
- Animation/ICC require v3 viewer

## Technical Notes

### ICC Profile Storage
- Stored after metadata, before frame data
- Optional - flag indicates presence
- Can be shared across frames in animations

### Animation Optimization
- Delta frames supported (store only changes)
- Optional frame disposal methods
- Configurable interpolation hints

### Color Space Conversion
- Automatic conversion on display
- Preserve original profile in file
- Fallback to sRGB if profile unsupported

---

**Contributors Welcome!** This is a community project. Submit PRs or open issues on GitHub!

**License:** MIT - Same as YEET v1/v2
