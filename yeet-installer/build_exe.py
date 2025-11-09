#!/usr/bin/env python3
"""
Build script to create a Windows executable from yeet_viewer.py
"""

import os
import sys
import subprocess
import shutil


def main():
    """Build the YEET Viewer executable"""
    
    print("=" * 60)
    print("YEET Viewer - Build Script")
    print("=" * 60)
    
    # Check if PyInstaller is installed
    try:
        import PyInstaller
        print("✓ PyInstaller is installed")
    except ImportError:
        print("✗ PyInstaller is not installed")
        print("\nInstalling PyInstaller...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", "pyinstaller"])
        print("✓ PyInstaller installed successfully")
    
    # Check if Pillow is installed
    try:
        import PIL
        print("✓ Pillow is installed")
    except ImportError:
        print("✗ Pillow is not installed")
        print("\nInstalling Pillow...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", "Pillow"])
        print("✓ Pillow installed successfully")
    
    print("\n" + "=" * 60)
    print("Building executable...")
    print("=" * 60 + "\n")
    
    # Build command
    build_cmd = [
        "pyinstaller",
        "--onefile",           # Single executable
        "--windowed",          # No console window
        "--name", "YeetViewer", # Executable name
        "yeet_viewer.py"
    ]
    
    # Check if icon file exists (optional)
    if os.path.exists("icon.ico"):
        build_cmd.extend(["--icon", "icon.ico"])
        print("✓ Using icon.ico")
    else:
        print("ℹ No icon.ico found (optional)")
    
    print(f"\nRunning: {' '.join(build_cmd)}\n")
    
    try:
        subprocess.check_call(build_cmd)
        print("\n" + "=" * 60)
        print("✓ Build completed successfully!")
        print("=" * 60)
        print(f"\nExecutable location: {os.path.abspath('dist/YeetViewer.exe')}")
        print("\nYou can now:")
        print("1. Run the executable: dist\\YeetViewer.exe")
        print("2. Register .yeet file association (see README.md)")
        print("3. Distribute the executable to other Windows systems")
        
    except subprocess.CalledProcessError as e:
        print(f"\n✗ Build failed with error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
