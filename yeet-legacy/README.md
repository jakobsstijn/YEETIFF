# YEET Legacy (v1)

Legacy support for the original YEET v1 format.

## ⚠️ Deprecated

**This format is deprecated.** Use `yeet-core` (v2) for modern features.

## v1 Limitations

- ❌ No alpha channel (RGB only)
- ❌ No compression
- ❌ No metadata
- ❌ No binary mode
- ❌ Larger file sizes

## v1 Format Specification

```
┌─────────┬─────────┬──────────────────┐
│  Width  │ Height  │  Pixel Data      │
│  (4B)   │  (4B)   │  (6 chars/pixel) │
└─────────┴─────────┴──────────────────┘
```

**Header:** 8 bytes (width + height, native-endian u32)

**Pixel Data:** Hex color codes `RRGGBB` (6 ASCII characters per pixel)

## Usage

### Convert PNG to v1

```bash
yeet-legacy compile image.png
```

### View v1 File

```bash
yeet-legacy image.yeet
```

## Why v1 Exists

The v1 format was the original prototype created for educational purposes. It demonstrated the basic concept of a custom image format using human-readable hex codes.

**Use v2 instead** - it's backward compatible and has all the modern features.

## Migration to v2

To convert v1 files to v2 format:

1. Open in v1 viewer
2. Save as PNG
3. Convert to v2 using `yeet-core`:

```bash
# View v1 file (creates temp PNG)
yeet-legacy old_image.yeet

# Convert the PNG to v2
yeet compile old_image.png --compress --binary
```

## Code

The v1 parser is included in `yeet-core` for backward compatibility. This standalone tool is provided for archival purposes.

## License

MIT License - Copyright (c) 2025 Stijn Jakobs
