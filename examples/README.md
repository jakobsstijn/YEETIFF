# YEET Example Files

This directory contains example YEET files demonstrating different format features.

## Files

### simple.yeet
- **Format:** v2
- **Size:** 1×1 pixel
- **Features:** Uncompressed, RGB, hex text
- **Purpose:** Minimal example for learning

### compressed.yeet
- **Format:** v2
- **Size:** 100×100 pixels
- **Features:** Compressed, binary
- **Purpose:** Demonstrates compression

### transparent.yeet
- **Format:** v2
- **Size:** 50×50 pixels
- **Features:** RGBA with alpha channel
- **Purpose:** Transparency example

## Creating Your Own Examples

### Simple RGB Image

```bash
# Create a test PNG first
# Then convert:
yeet compile test.png
```

### Optimized Image

```bash
yeet compile photo.png --compress --binary
```

### Batch Convert

```bash
yeet batch ./your-images --compress --binary
```

## Viewing Examples

```bash
# View any example
yeet simple.yeet
yeet compressed.yeet
yeet transparent.yeet
```

## File Size Comparison

| File | Format | Compression | Size |
|------|--------|-------------|------|
| simple.yeet | v2 hex | None | ~50 bytes |
| compressed.yeet | v2 binary | zlib | ~500 bytes |
| transparent.yeet | v2 binary | zlib | ~300 bytes |

## Contributing Examples

Have an interesting YEET file? Submit a PR!

Guidelines:
- Keep files small (< 1 MB)
- Include description
- Demonstrate specific features
- Provide source PNG if possible
