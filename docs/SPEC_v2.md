# YEET v2 Format Specification

**Version:** 2.0  
**Status:** Stable  
**Date:** November 2025

---

## Overview

YEET v2 is the current stable version of the YEET image format, supporting RGBA transparency, compression, binary encoding, and rich metadata.

## File Structure

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

**Minimum Header Size:** 20 bytes + metadata length

---

## Header Details

### Magic Bytes (4 bytes)

Fixed sequence identifying YEET files:

```
0x59 0x45 0x45 0x54  ("YEET" in ASCII)
```

### Version (1 byte)

```
0x02  (Version 2)
```

### Flags (1 byte)

Bitfield controlling format features:

```
┌───┬───┬───┬───┬───┬───┬───┬───┐
│ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 │ 0 │
└───┴───┴───┴───┴───┴───┴───┴───┘
  │   │   │   │   │   │   │   └─── Bit 0: Compression (0=none, 1=zlib)
  │   │   │   │   │   │   └─────── Bit 1: Alpha (0=RGB, 1=RGBA)
  │   │   │   │   │   └─────────── Bit 2: Binary (0=hex text, 1=binary)
  │   │   │   │   └─────────────── Bits 3-7: Reserved (must be 0)
  └───┴───┴───┴─────────────────── 
```

**Examples:**
- `0x00` = No compression, RGB, hex text
- `0x07` = Compressed, RGBA, binary
- `0x01` = Compressed, RGB, hex text
- `0x06` = No compression, RGBA, binary

### Dimensions (8 bytes total)

- **Width:** 4 bytes, unsigned 32-bit integer, little-endian
- **Height:** 4 bytes, unsigned 32-bit integer, little-endian

**Maximum:** 4,294,967,295 × 4,294,967,295 pixels (theoretical)

**Practical limit:** Constrained by available memory

---

## Metadata

### Length (2 bytes)

Unsigned 16-bit integer (little-endian) specifying JSON string length.

**Maximum:** 65,535 bytes

### Format

UTF-8 encoded JSON object:

```json
{
  "author": "string (optional)",
  "created": "ISO 8601 timestamp (optional)",
  "software": "YEET v2.0"
}
```

**Minimal Example:**
```json
{"software":"YEET v2.0"}
```

**Full Example:**
```json
{
  "author": "Stijn Jakobs",
  "created": "2025-11-10T12:00:00Z",
  "software": "YEET v2.0"
}
```

### Forward Compatibility (v3 fields)

v2 metadata may include v3-compatible fields for future use:

```json
{
  "software": "YEET v2.0",
  "color_profile": "sRGB",
  "frame_count": 1,
  "frame_delay": 0
}
```

v2 parsers should ignore unknown fields.

---

## Pixel Data

### Data Length (4 bytes)

Unsigned 32-bit integer (little-endian) specifying pixel data size in bytes.

### Encoding Modes

#### Hex Text Mode (Flag bit 2 = 0)

Human-readable hexadecimal ASCII:

**RGB (6 characters per pixel):**
```
RRGGBB RRGGBB RRGGBB ...
```

**RGBA (8 characters per pixel):**
```
RRGGBBAA RRGGBBAA RRGGBBAA ...
```

**Example (2×2 red square with transparency):**
```
FF0000FF  FF000080
FF000080  FF0000FF
```

#### Binary Mode (Flag bit 2 = 1)

Raw byte encoding:

**RGB (3 bytes per pixel):**
```
[R][G][B] [R][G][B] ...
```

**RGBA (4 bytes per pixel):**
```
[R][G][B][A] [R][G][B][A] ...
```

**Example (2×2 red square):**
```
0xFF 0x00 0x00  0xFF 0x00 0x00
0xFF 0x00 0x00  0xFF 0x00 0x00
```

### Compression

When flag bit 0 is set, pixel data is compressed using **zlib** (DEFLATE algorithm).

**Process:**
1. Encode pixels (hex or binary)
2. Compress with zlib
3. Write compressed data length
4. Write compressed bytes

**Decompression:**
1. Read compressed data
2. Decompress with zlib
3. Parse according to encoding mode

**Typical Compression Ratios:**
- Photos: 40-60% reduction
- Graphics: 50-70% reduction
- Text/UI: 60-80% reduction

---

## Pixel Ordering

Pixels are stored in **row-major order** (left-to-right, top-to-bottom):

```
For a 3×2 image:
┌───┬───┬───┐
│ 0 │ 1 │ 2 │  ← Row 0
├───┼───┼───┤
│ 3 │ 4 │ 5 │  ← Row 1
└───┴───┴───┘

Data order: pixel[0], pixel[1], pixel[2], pixel[3], pixel[4], pixel[5]
```

---

## Complete Examples

### Example 1: Minimal 1×1 Red Pixel (Uncompressed, RGB, Hex)

```
Offset | Hex                 | Description
-------+---------------------+---------------------------
0x00   | 59 45 45 54         | Magic "YEET"
0x04   | 02                  | Version 2
0x05   | 00                  | Flags (no compression, RGB, hex)
0x06   | 01 00 00 00         | Width = 1
0x0A   | 01 00 00 00         | Height = 1
0x0E   | 18 00               | Metadata length = 24
0x10   | {"software":"YEET v2.0"} | Metadata (24 bytes)
0x28   | 06 00 00 00         | Data length = 6
0x2C   | FF 30 30 30 30 30   | "FF0000" (red)
```

**Total size:** 50 bytes

### Example 2: 2×2 RGBA Compressed Binary

```
Flags: 0x07 (compressed + alpha + binary)

Uncompressed pixel data (16 bytes):
  0xFF 0x00 0x00 0xFF  (red, opaque)
  0x00 0xFF 0x00 0x80  (green, 50% transparent)
  0x00 0x00 0xFF 0x80  (blue, 50% transparent)
  0xFF 0xFF 0xFF 0xFF  (white, opaque)

Compressed data: ~12 bytes (after zlib)
```

---

## Size Calculations

### Uncompressed Sizes

**Hex Text Mode:**
- RGB: `width × height × 6` bytes
- RGBA: `width × height × 8` bytes

**Binary Mode:**
- RGB: `width × height × 3` bytes
- RGBA: `width × height × 4` bytes

**Total File Size:**
```
20 + metadata_length + pixel_data_length
```

### Example: 1920×1080 Image

| Mode | Alpha | Compression | Size |
|------|-------|-------------|------|
| Hex | No | No | ~12.4 MB |
| Hex | Yes | No | ~16.6 MB |
| Binary | No | No | ~6.2 MB |
| Binary | Yes | No | ~8.3 MB |
| Binary | Yes | **Yes** | **~500 KB** ⭐ |

---

## Implementation Guidelines

### Writing YEET v2 Files

```python
1. Open output file
2. Write magic bytes "YEET"
3. Write version byte (0x02)
4. Calculate and write flags
5. Write width (u32 little-endian)
6. Write height (u32 little-endian)
7. Prepare metadata JSON
8. Write metadata length (u16 little-endian)
9. Write metadata bytes
10. Encode pixel data (hex or binary)
11. Compress if flag set
12. Write data length (u32 little-endian)
13. Write pixel data
14. Close file
```

### Reading YEET v2 Files

```python
1. Read and verify magic bytes
2. Read version (should be 0x02)
3. Read flags byte
4. Parse compression, alpha, binary flags
5. Read width and height
6. Read metadata length
7. Read and parse metadata JSON
8. Read data length
9. Read pixel data
10. Decompress if flag set
11. Parse pixels based on mode
12. Construct image buffer
```

---

## Error Handling

### Invalid Files

- **Wrong magic bytes:** Not a YEET file
- **Version mismatch:** Use appropriate parser (v1, v2, v3)
- **Invalid flags:** Unknown bits set (reject or ignore)
- **Truncated data:** File corrupted

### Validation Checklist

- ✅ Magic bytes == "YEET"
- ✅ Version == 2
- ✅ Reserved flag bits == 0
- ✅ Metadata length ≤ 65535
- ✅ Valid UTF-8 metadata
- ✅ Pixel data length matches dimensions
- ✅ Successful decompression (if compressed)

---

## Backward Compatibility

### v1 Files

v2 parsers should detect v1 files and fall back to v1 parser:

- v1 has no magic bytes
- First 8 bytes are width + height
- Followed by hex color codes (RRGGBB)

### Forward Compatibility

v2 files may include v3-compatible metadata fields. v2 parsers should:

- Parse known fields
- Ignore unknown fields
- Preserve metadata when re-encoding

---

## Best Practices

### Recommended Settings

**For storage/distribution:**
```bash
yeet compile image.png --compress --binary
```
Flags: `0x07` (compressed + RGBA + binary)

**For debugging:**
```bash
yeet compile image.png
```
Flags: `0x00` (uncompressed + RGB + hex) - human-readable

**For web transparency:**
```bash
yeet compile logo.png --compress --binary
```
Ensures alpha channel preservation

### Optimization Tips

1. **Always compress** for non-debugging use (40-60% smaller)
2. **Use binary mode** for smaller files
3. **Strip unnecessary metadata** for minimal size
4. **Batch convert** for efficiency

---

## Tools

### Official Implementation

**yeet-core** (Rust):
- Fast viewer with OpenGL acceleration
- Efficient converter
- Batch processing support

### Command Reference

```bash
# View
yeet image.yeet

# Convert (optimized)
yeet compile photo.png --compress --binary

# Batch convert
yeet batch ./folder --compress --binary
```

---

## Appendix: Technical Details

### Endianness

All multi-byte integers use **little-endian** byte order:

```
Value 305 (0x00000131) as u32:
Bytes: 0x31 0x01 0x00 0x00
```

### Character Encoding

- Metadata: UTF-8
- Hex mode: ASCII (characters 0-9, A-F)

### Compression Algorithm

- **Algorithm:** zlib (DEFLATE)
- **Level:** Maximum compression (level 9)
- **Library:** flate2 (Rust), zlib (C), gzip (Python)

---

## Change Log

**v2.0 (November 2025):**
- Initial stable release
- RGBA support
- zlib compression
- Binary encoding mode
- JSON metadata

**Future (v3.0):**
- ICC color profiles
- Multi-frame animation
- Brotli/Zstd compression
- HDR support

---

**Author:** Stijn Jakobs  
**License:** MIT  
**Repository:** [github.com/jakobsstijn/YEETIFF](https://github.com/jakobsstijn/YEETIFF)
