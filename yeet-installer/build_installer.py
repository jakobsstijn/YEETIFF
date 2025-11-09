#!/usr/bin/env python3
"""
Build script to create a standalone installer executable
This bundles the installer GUI + viewer into one distributable EXE
"""

import os
import sys
import subprocess
import shutil

# Fix Windows console encoding for Unicode symbols
if sys.platform == 'win32':
    sys.stdout.reconfigure(encoding='utf-8')


def main():
    """Build the YEET Installer executable"""
    
    print("=" * 70)
    print("YEET Format - Complete Installer Builder")
    print("=" * 70)
    print()
    
    # Check if PyInstaller is installed
    try:
        import PyInstaller
        print("[OK] PyInstaller is installed")
    except ImportError:
        print("[!] PyInstaller not found")
        print("    Installing PyInstaller...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", "pyinstaller"])
        print("[OK] PyInstaller installed")
    
    # Check dependencies
    print("\nChecking dependencies...")
    dependencies = {
        "Pillow": "PIL",
        "pywin32": "win32com"
    }
    
    for dep_name, import_name in dependencies.items():
        try:
            __import__(import_name.split('.')[0])
            print(f"[OK] {dep_name} is installed")
        except ImportError:
            print(f"[!] {dep_name} not found")
            print(f"    Installing {dep_name}...")
            subprocess.check_call([sys.executable, "-m", "pip", "install", dep_name])
            print(f"[OK] {dep_name} installed")
    
    print("\n" + "=" * 70)
    print("Building standalone installer...")
    print("=" * 70 + "\n")
    
    # Clean previous builds
    for folder in ["build", "dist", "__pycache__"]:
        if os.path.exists(folder):
            print(f"Cleaning {folder}/...")
            shutil.rmtree(folder, ignore_errors=True)
    
    # Remove old spec file
    if os.path.exists("installer_gui.spec"):
        os.remove("installer_gui.spec")
    
    # Check if logo exists
    logo_exists = os.path.exists("logo.png")
    if logo_exists:
        print("[OK] Found logo.png - will be included\n")
    else:
        print("[i] No logo.png found - installer will use emoji fallback\n")
    
    # Check if YEET viewer exists
    parent_dir = os.path.dirname(os.path.abspath(__file__))
    viewer_paths = [
        os.path.join(parent_dir, "..", "yeet-format", "target", "release", "yeet.exe"),
        os.path.join(parent_dir, "..", "yeet-format", "target", "debug", "yeet.exe"),
    ]
    
    viewer_exe = None
    for path in viewer_paths:
        if os.path.exists(path):
            viewer_exe = path
            print(f"[OK] Found YEET viewer: {os.path.basename(os.path.dirname(path))}/yeet.exe")
            break
    
    if not viewer_exe:
        print("[!] WARNING: yeet.exe not found!")
        print("    Build the Rust project first:")
        print("      cd ../yeet-format")
        print("      cargo build --release")
        print("\n    Continuing anyway - installer will search at runtime...\n")
    else:
        print(f"    Will be bundled into installer\n")
    
    # Build command
    build_cmd = [
        "pyinstaller",
        "--onefile",                           # Single executable
        "--windowed",                          # No console window
        "--name", "YeetInstaller",            # Output name
        "--uac-admin",                         # Request admin on launch
        
        # Add Python files to bundle
        "--add-data", "yeet_viewer.py;.",     # Include viewer script
        
        # Icon (if available)
        # "--icon", "logo.ico",                # Uncomment if you have .ico file
        
        # Hidden imports (sometimes needed)
        "--hidden-import", "PIL._tkinter_finder",
        
        # Main script
        "installer_gui.py"
    ]
    
    # Add logo if it exists
    if logo_exists:
        build_cmd.insert(-1, "--add-data")
        build_cmd.insert(-1, "logo.png;.")
    
    # Add YEET viewer if found - EMBED IT IN THE INSTALLER!
    if viewer_exe:
        build_cmd.insert(-1, "--add-data")
        build_cmd.insert(-1, f"{viewer_exe};.")
    
    print("PyInstaller command:")
    print(" ".join(build_cmd))
    print("\nThis may take a few minutes...\n")
    
    try:
        # Run PyInstaller
        result = subprocess.run(build_cmd, check=True)
        
        print("\n" + "=" * 70)
        print("[OK] Build completed successfully!")
        print("=" * 70)
        
        exe_path = os.path.join("dist", "YeetInstaller.exe")
        if os.path.exists(exe_path):
            size_mb = os.path.getsize(exe_path) / (1024 * 1024)
            print(f"\nInstaller location: {os.path.abspath(exe_path)}")
            print(f"File size: {size_mb:.1f} MB")
            print("\nYou can now distribute 'YeetInstaller.exe'")
            print("\nHow it works:")
            print("  1. User runs YeetInstaller.exe")
            print("  2. Installer finds yeet.exe in ../yeet-format/target/")
            print("  3. Copies it to C:\\Program Files\\YeetViewer\\")
            print("  4. Registers .yeet extension in Windows Registry")
            print("  5. User can now double-click .yeet files!")
            
            # Check if Rust exe exists
            print("\n" + "=" * 70)
            print("Checking for YEET viewer executable...")
            print("=" * 70)
            
            parent_dir = os.path.dirname(os.path.abspath(__file__))
            rust_release = os.path.join(parent_dir, "..", "yeet-format", "target", "release", "yeet.exe")
            rust_debug = os.path.join(parent_dir, "..", "yeet-format", "target", "debug", "yeet.exe")
            
            if os.path.exists(rust_release):
                print(f"[OK] Found Rust release build: {rust_release}")
                print("     The installer will use this when run by users")
            elif os.path.exists(rust_debug):
                print(f"[OK] Found Rust debug build: {rust_debug}")
                print("     The installer will use this when run by users")
            else:
                print("[!] WARNING: No yeet.exe found!")
                print("    Build the Rust project first:")
                print("      cd ../yeet-format")
                print("      cargo build --release")
                print("\n    The installer will look for:")
                print(f"      {rust_release}")
                print(f"      {rust_debug}")
        
        print("\n" + "=" * 70)
        
        return 0
        
    except subprocess.CalledProcessError as e:
        print("\n" + "=" * 70)
        print("[X] Build failed!")
        print("=" * 70)
        print(f"\nError: {e}")
        print("\nTroubleshooting:")
        print("  - Make sure all dependencies are installed")
        print("  - Check that installer_gui.py has no syntax errors")
        print("  - Try running: pip install --upgrade pyinstaller")
        return 1


if __name__ == "__main__":
    sys.exit(main())
