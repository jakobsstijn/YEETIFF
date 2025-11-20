# YEET v3 Examples

This directory contains examples demonstrating YEET v3 features.

## Creating Animations

### Generate Animation Frames

```bash
# Generate example frames
python3 create_animation.py
```

This creates 24 frames of a rotating gradient animation in the `frames/` directory.

### Convert to YEET v3

Once multi-frame support is added, you'll be able to convert frames to an animation:

```bash
# Planned feature
cargo run --release animate frames/*.png --delay 42 --brotli --output animation.yeet
```

## Single Image with ICC Profile

```bash
# Convert PNG with ICC profile to YEET v3
cargo run --release compile photo_with_icc.png --brotli --binary

# View with color correction
cargo run --release photo_with_icc.yeet
```

## Compression Comparison

```bash
# No compression
cargo run --release compile photo.png --binary
# Size: ~6.2 MB

# Zlib (v2 compatible)
cargo run --release compile photo.png --compress --binary
# Size: ~2.5 MB (60% reduction)

# Brotli (better)
cargo run --release compile photo.png --brotli --binary
# Size: ~2.2 MB (65% reduction)

# Zstd (fastest)
cargo run --release compile photo.png --zstd --binary
# Size: ~2.3 MB (63% reduction)
```

## Features Demo

### ICC Color Profiles

The v3 viewer automatically:
1. Extracts ICC profile from source PNG
2. Embeds it in YEET file
3. Applies color correction on display
4. Shows ICC indicator in UI

### Animation Playback

When viewing multi-frame YEET v3 files:
- ▶ Auto-plays animations
- ⏸ Pause/resume button
- ⏮/⏭ Frame navigation
- Shows current frame number
- Respects per-frame delays

## Sample Files

Create some sample files:

```bash
# Simple gradient (no ICC)
convert -size 800x600 gradient:blue-yellow sample_gradient.png
cargo run --release compile sample_gradient.png --brotli --binary

# Photo with transparency
convert sample.jpg -alpha set -channel A -fx "j/h" sample_alpha.png
cargo run --release compile sample_alpha.png --zstd --binary
```

## Metadata

YEET v3 files contain rich metadata visible in the viewer:

- Author
- Creation date/time
- Software used
- Color space/profile
- Frame count & timing
- Bit depth
- HDR flag
- Camera info (if available)
- DPI/resolution
- Orientation

Access metadata by opening the "📊 Metadata" panel in the viewer.
