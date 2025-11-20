#!/usr/bin/env python3
"""
Create a simple animation example for YEET v3 testing
Generates rotating gradient frames
"""

from PIL import Image, ImageDraw
import math
import os

def create_rotating_gradient(size, angle, colors):
    """Create a frame with rotating gradient"""
    img = Image.new('RGBA', size, (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    center_x, center_y = size[0] // 2, size[1] // 2
    radius = min(size) // 2
    
    # Draw rotating gradient circle
    for i in range(360):
        current_angle = (i + angle) % 360
        rad = math.radians(current_angle)
        
        # Calculate color based on angle
        color_idx = int((current_angle / 360) * len(colors))
        color = colors[color_idx % len(colors)]
        
        # Draw radial line
        x = center_x + int(radius * math.cos(rad))
        y = center_y + int(radius * math.sin(rad))
        draw.line([(center_x, center_y), (x, y)], fill=color, width=2)
    
    return img

def main():
    print("🎬 Creating YEET v3 Animation Example...")
    
    # Animation parameters
    size = (400, 400)
    num_frames = 24
    colors = [
        (255, 0, 0, 255),      # Red
        (255, 127, 0, 255),    # Orange
        (255, 255, 0, 255),    # Yellow
        (0, 255, 0, 255),      # Green
        (0, 127, 255, 255),    # Blue
        (139, 0, 255, 255),    # Purple
    ]
    
    # Create output directory
    os.makedirs("frames", exist_ok=True)
    
    # Generate frames
    for frame_num in range(num_frames):
        angle = (frame_num * 360) // num_frames
        img = create_rotating_gradient(size, angle, colors)
        
        output_path = f"frames/frame_{frame_num:03d}.png"
        img.save(output_path)
        print(f"  ✅ Generated {output_path}")
    
    print(f"\n✨ Created {num_frames} frames")
    print("\nNext steps:")
    print("  1. Convert frames to YEET v3 animation:")
    print("     cargo run --release animate frames/*.png --delay 42 --brotli --output rotating_gradient.yeet")
    print("\n  2. View the animation:")
    print("     cargo run --release rotating_gradient.yeet")

if __name__ == "__main__":
    main()
