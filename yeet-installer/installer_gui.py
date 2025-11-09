"""
YEET Image Format - Installation Wizard
Install .yeet file format support for Windows
"""

import os
import sys
import winreg
import subprocess
import tkinter as tk
from tkinter import ttk, messagebox
from pathlib import Path
import threading


class InstallerWizard:
    """Installation wizard for .yeet image format support"""
    
    def __init__(self):
        self.root = tk.Tk()
        self.root.title("YEET Image Format Setup")
        
        # Set icon if available
        try:
            icon_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "logo.png")
            if os.path.exists(icon_path):
                icon = tk.PhotoImage(file=icon_path)
                self.root.iconphoto(True, icon)
        except:
            pass
        
        # Make window NOT resizable
        self.root.resizable(False, False)
        
        # Don't set geometry yet - will be set dynamically per screen
        
        # Installer state
        self.current_step = 0
        self.install_path = None
        self.viewer_path = None
        
        # Create main container
        self.container = ttk.Frame(self.root)
        self.container.pack(fill='both', expand=True)
        
        # Configure container grid
        self.container.grid_rowconfigure(1, weight=1)  # Content expands
        self.container.grid_columnconfigure(0, weight=1)
        
        # Header
        self.create_header()
        
        # Content area
        self.content_frame = ttk.Frame(self.container)
        self.content_frame.grid(row=1, column=0, sticky='nsew', padx=20, pady=10)
        
        # Configure content frame to expand
        self.content_frame.grid_rowconfigure(0, weight=1)
        self.content_frame.grid_columnconfigure(0, weight=1)
        
        # Footer with buttons
        self.create_footer()
        
        # Show welcome screen
        self.show_welcome()
        
    def center_window(self):
        """Center the window on screen and resize based on content"""
        self.root.update_idletasks()
        
        # Get the required size based on content
        width = self.root.winfo_reqwidth()
        height = self.root.winfo_reqheight()
        
        # Set minimum and maximum constraints
        screen_width = self.root.winfo_screenwidth()
        screen_height = self.root.winfo_screenheight()
        
        min_width = 600
        min_height = 400
        max_width = int(screen_width * 0.8)
        max_height = int(screen_height * 0.85)
        
        # Constrain to limits
        width = max(min_width, min(width + 40, max_width))  # +40 for padding
        height = max(min_height, min(height + 40, max_height))
        
        # Center position
        x = (screen_width - width) // 2
        y = (screen_height - height) // 2
        
        self.root.geometry(f'{width}x{height}+{x}+{y}')
    
    def create_header(self):
        """Create header section"""
        header = ttk.Frame(self.container)
        header.grid(row=0, column=0, sticky='ew', padx=20, pady=20)
        
        title = ttk.Label(
            header, 
            text="YEET Image Format Setup",
            font=('Segoe UI', 18, 'bold')
        )
        title.pack(anchor='w')
        
        subtitle = ttk.Label(
            header,
            text="Install .yeet file format support for Windows",
            font=('Segoe UI', 10)
        )
        subtitle.pack(anchor='w', pady=(5, 0))
        
        # Author
        author = ttk.Label(
            header,
            text="This program was developed by Stijn Jakobs",
            font=('Segoe UI', 9),
            foreground='gray'
        )
        author.pack(anchor='w', pady=(2, 0))
        
        # Separator
        ttk.Separator(header, orient='horizontal').pack(fill='x', pady=(10, 0))
    
    def create_footer(self):
        """Create footer with navigation buttons"""
        footer = ttk.Frame(self.container)
        footer.grid(row=2, column=0, sticky='ew', padx=20, pady=20)
        
        # Separator
        ttk.Separator(footer, orient='horizontal').pack(fill='x', pady=(0, 10))
        
        button_frame = ttk.Frame(footer)
        button_frame.pack(fill='x')
        
        self.back_btn = ttk.Button(button_frame, text="< Back", command=self.go_back, width=12)
        self.back_btn.pack(side='left')
        
        self.cancel_btn = ttk.Button(button_frame, text="Cancel", command=self.cancel_install, width=12)
        self.cancel_btn.pack(side='right', padx=(5, 0))
        
        self.next_btn = ttk.Button(button_frame, text="Next >", command=self.go_next, width=12)
        self.next_btn.pack(side='right')
    
    def clear_content(self):
        """Clear content frame"""
        for widget in self.content_frame.winfo_children():
            widget.destroy()
    
    def show_welcome(self):
        """Show welcome screen"""
        self.clear_content()
        self.current_step = 0
        self.back_btn.config(state='disabled')
        self.next_btn.config(state='normal', text="Next >", command=self.go_next)
        
        # Logo image
        try:
            logo_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "logo.png")
            if os.path.exists(logo_path):
                from PIL import Image, ImageTk
                logo_img = Image.open(logo_path)
                # Resize logo to reasonable size (max 128x128)
                logo_img.thumbnail((128, 128), Image.Resampling.LANCZOS)
                logo_photo = ImageTk.PhotoImage(logo_img)
                
                logo_label = ttk.Label(self.content_frame, image=logo_photo)
                logo_label.image = logo_photo  # Keep reference
                logo_label.pack(pady=(20, 10))
            else:
                # Fallback to emoji if logo not found
                icon = ttk.Label(self.content_frame, text="🎨", font=('Segoe UI', 64))
                icon.pack(pady=(20, 10))
        except Exception as e:
            # Fallback to emoji on error
            icon = ttk.Label(self.content_frame, text="🎨", font=('Segoe UI', 64))
            icon.pack(pady=(20, 10))
        
        # Separator line
        separator = ttk.Separator(self.content_frame, orient='horizontal')
        separator.pack(fill='x', padx=50, pady=(0, 15))
        
        # Introduction text
        intro_text = """What is .YEET?

YEETIFF (Yet Even Expressier Transcoded Image File Format) 
is a simple, uncompressed image format that stores pixels as 
hexadecimal color codes.

This installer will set up YEET image format support on your system.
You'll be able to double-click .yeet files to view them!"""
        
        text = ttk.Label(
            self.content_frame,
            text=intro_text,
            justify='left',
            font=('Segoe UI', 10),
            wraplength=550
        )
        text.pack(pady=10, padx=30)
        
        # Resize window to fit content
        self.center_window()
    
    def show_license(self):
        """Show license agreement"""
        self.clear_content()
        self.current_step = 1
        self.back_btn.config(state='normal')
        self.next_btn.config(text="Next >", command=self.go_next)
        
        title = ttk.Label(
            self.content_frame,
            text="License Agreement",
            font=('Segoe UI', 12, 'bold')
        )
        title.pack(anchor='w', pady=(0, 10))
        
        license_text = """MIT License

Copyright (c) 2025 Stijn Jakobs

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."""
        
        text_frame = ttk.Frame(self.content_frame)
        text_frame.pack(fill='both', expand=True)
        
        scrollbar = ttk.Scrollbar(text_frame)
        scrollbar.pack(side='right', fill='y')
        
        text_widget = tk.Text(
            text_frame,
            wrap='word',
            height=12,
            font=('Consolas', 9),
            yscrollcommand=scrollbar.set
        )
        text_widget.pack(fill='both', expand=True)
        text_widget.insert('1.0', license_text)
        text_widget.config(state='disabled')
        scrollbar.config(command=text_widget.yview)
        
        self.accept_var = tk.BooleanVar(value=False)
        accept_cb = ttk.Checkbutton(
            self.content_frame,
            text="I accept the terms in the License Agreement",
            variable=self.accept_var,
            command=self.update_next_button
        )
        accept_cb.pack(anchor='w', pady=(10, 0))
        
        # Set initial button state
        self.update_next_button()
        
        # Resize window to fit content
        self.center_window()
    
    def show_installation_type(self):
        """Show installation type selection"""
        self.clear_content()
        self.current_step = 2
        self.next_btn.config(state='normal', text="Next >", command=self.go_next)
        self.back_btn.config(state='normal')
        
        title = ttk.Label(
            self.content_frame,
            text="Choose Installation Type",
            font=('Segoe UI', 12, 'bold')
        )
        title.pack(anchor='w', pady=(0, 20))
        
        self.install_type = tk.StringVar(value='install')
        
        # Install option
        install_frame = ttk.LabelFrame(self.content_frame, text="Install YEET Viewer", padding=15)
        install_frame.pack(fill='x', pady=10)
        
        ttk.Radiobutton(
            install_frame,
            text="Install YEET Viewer and register .yeet format",
            variable=self.install_type,
            value='install'
        ).pack(anchor='w')
        
        info_text = """• Installs YEET Viewer to Program Files
• Registers .yeet as an image format
• Associates .yeet files with YEET Viewer
• Double-click .yeet files to open them"""
        
        ttk.Label(
            install_frame,
            text=info_text,
            justify='left',
            font=('Segoe UI', 9),
            foreground='gray'
        ).pack(anchor='w', padx=20, pady=(5, 0))
        
        # Uninstall option
        uninstall_frame = ttk.LabelFrame(self.content_frame, text="Uninstall YEET Viewer", padding=15)
        uninstall_frame.pack(fill='x', pady=10)
        
        ttk.Radiobutton(
            uninstall_frame,
            text="Remove YEET Viewer and .yeet format",
            variable=self.install_type,
            value='uninstall'
        ).pack(anchor='w')
        
        ttk.Label(
            uninstall_frame,
            text="Removes the viewer from Program Files and unregisters .yeet format.\nYour .yeet files will remain on your computer.",
            font=('Segoe UI', 9),
            foreground='gray'
        ).pack(anchor='w', padx=20, pady=(5, 0))
        
        # Resize window to fit content
        self.center_window()
    
    def show_progress(self):
        """Show installation progress"""
        self.clear_content()
        self.current_step = 3
        self.back_btn.config(state='disabled')
        self.next_btn.config(state='disabled')
        self.cancel_btn.config(state='disabled')
        
        install_type = self.install_type.get()
        
        if install_type == 'install':
            title_text = "Installing YEET Viewer..."
        else:
            title_text = "Uninstalling YEET Viewer..."
        
        title = ttk.Label(
            self.content_frame,
            text=title_text,
            font=('Segoe UI', 12, 'bold')
        )
        title.pack(anchor='w', pady=(0, 20))
        
        # Progress bar
        self.progress = ttk.Progressbar(
            self.content_frame,
            mode='indeterminate',
            length=400
        )
        self.progress.pack(pady=20)
        self.progress.start(10)
        
        # Status label
        self.status_label = ttk.Label(
            self.content_frame,
            text="Initializing...",
            font=('Segoe UI', 9)
        )
        self.status_label.pack(pady=10)
        
        # Log text
        log_frame = ttk.Frame(self.content_frame)
        log_frame.pack(fill='both', expand=True, pady=10)
        
        scrollbar = ttk.Scrollbar(log_frame)
        scrollbar.pack(side='right', fill='y')
        
        self.log_text = tk.Text(
            log_frame,
            wrap='word',
            height=10,
            font=('Consolas', 8),
            yscrollcommand=scrollbar.set
        )
        self.log_text.pack(fill='both', expand=True)
        scrollbar.config(command=self.log_text.yview)
        
        # Resize window to fit content
        self.center_window()
        
        # Start installation in background
        if install_type == 'install':
            thread = threading.Thread(target=self.perform_installation)
        else:
            thread = threading.Thread(target=self.perform_uninstallation)
        thread.daemon = True
        thread.start()
    
    def log(self, message):
        """Add message to log"""
        self.log_text.insert('end', message + '\n')
        self.log_text.see('end')
        self.root.update()
    
    def update_status(self, message):
        """Update status label"""
        self.status_label.config(text=message)
        self.root.update()
    
    def perform_installation(self):
        """Perform the actual installation"""
        try:
            # Check admin rights
            self.update_status("Checking administrator privileges...")
            self.log("Checking for administrator privileges...")
            
            if not self.is_admin():
                self.log("✗ Administrator privileges required")
                self.progress.stop()
                messagebox.showerror(
                    "Admin Required",
                    "This installer requires administrator privileges.\n\n"
                    "Please run this installer as Administrator."
                )
                self.show_finish(success=False)
                return
            
            self.log("✓ Running with administrator privileges\n")
            
            # Create installation directory
            program_files = os.environ.get('ProgramFiles', 'C:\\Program Files')
            self.install_path = os.path.join(program_files, 'YeetViewer')
            
            self.update_status("Creating installation directory...")
            self.log(f"Creating directory: {self.install_path}")
            
            try:
                os.makedirs(self.install_path, exist_ok=True)
                self.log("✓ Installation directory created\n")
            except Exception as e:
                self.log(f"✗ Failed to create directory: {str(e)}")
                self.show_finish(success=False)
                return
            
            # Find and copy viewer
            self.update_status("Installing YEET Viewer...")
            self.log("Locating YEET Viewer...")
            
            viewer_source = self.find_viewer_source()
            if not viewer_source:
                self.log("✗ Could not find viewer executable")
                self.show_finish(success=False)
                return
            
            self.log(f"✓ Found viewer: {viewer_source}")
            
            # Copy viewer to Program Files
            import shutil
            viewer_dest = os.path.join(self.install_path, "YeetViewer.exe")
            
            self.log(f"Copying to: {viewer_dest}")
            try:
                shutil.copy2(viewer_source, viewer_dest)
                self.log("✓ Viewer installed\n")
                self.viewer_path = viewer_dest
            except Exception as e:
                self.log(f"✗ Failed to copy viewer: {str(e)}")
                self.show_finish(success=False)
                return
            
            # Register file extension
            self.update_status("Registering .yeet file format...")
            self.log("Registering .yeet file format...")
            
            if self.register_file_extension(viewer_dest):
                self.log("✓ File format registered\n")
            else:
                self.log("✗ Registration failed")
                self.show_finish(success=False)
                return
            
            # Create uninstaller
            self.update_status("Creating uninstaller...")
            self.log("Creating uninstaller...")
            
            if self.create_uninstaller():
                self.log("✓ Uninstaller created\n")
            else:
                self.log("⚠ Warning: Could not create uninstaller")
            
            self.log("\n✓ Installation completed successfully!")
            self.progress.stop()
            self.show_finish(success=True)
                
        except Exception as e:
            self.log(f"\n✗ Error: {str(e)}")
            self.progress.stop()
            messagebox.showerror("Installation Error", f"An error occurred:\n{str(e)}")
            self.show_finish(success=False)
    
    def perform_uninstallation(self):
        """Perform uninstallation"""
        try:
            self.update_status("Checking administrator privileges...")
            self.log("Checking for administrator privileges...")
            
            if not self.is_admin():
                self.log("✗ Administrator privileges required")
                self.progress.stop()
                messagebox.showerror(
                    "Admin Required",
                    "This installer requires administrator privileges.\n\n"
                    "Please run this installer as Administrator."
                )
                self.show_finish(success=False)
                return
            
            self.log("✓ Running with administrator privileges\n")
            
            # Unregister file extension
            self.update_status("Unregistering .yeet file format...")
            self.log("Removing .yeet file format...")
            
            if self.unregister_file_extension():
                self.log("✓ File format removed\n")
            else:
                self.log("⚠ Warning: Could not unregister format")
            
            # Remove from Program Files
            program_files = os.environ.get('ProgramFiles', 'C:\\Program Files')
            install_path = os.path.join(program_files, 'YeetViewer')
            
            self.update_status("Removing YEET Viewer...")
            self.log(f"Removing: {install_path}")
            
            import shutil
            try:
                if os.path.exists(install_path):
                    shutil.rmtree(install_path)
                    self.log("✓ Viewer removed\n")
                else:
                    self.log("ℹ Viewer was not installed\n")
            except Exception as e:
                self.log(f"⚠ Warning: Could not remove viewer: {str(e)}\n")
            
            # Remove uninstaller entry
            self.update_status("Removing uninstaller entry...")
            self.log("Removing from Windows Programs...")
            
            try:
                uninstall_key = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\YeetViewer"
                winreg.DeleteKey(winreg.HKEY_LOCAL_MACHINE, uninstall_key)
                self.log("✓ Uninstaller entry removed\n")
            except FileNotFoundError:
                self.log("ℹ Uninstaller entry was not found\n")
            except Exception as e:
                self.log(f"⚠ Warning: {str(e)}\n")
            
            self.log("✓ Uninstallation completed successfully!")
            self.progress.stop()
            self.show_finish(success=True, uninstall=True)
                
        except Exception as e:
            self.log(f"\n✗ Error: {str(e)}")
            self.progress.stop()
            messagebox.showerror("Uninstall Error", f"An error occurred:\n{str(e)}")
            self.show_finish(success=False)
    
    def show_finish(self, success=True, uninstall=False):
        """Show completion screen"""
        self.clear_content()
        self.current_step = 4
        self.back_btn.config(state='disabled')
        self.next_btn.config(text="Finish", command=self.root.quit, state='normal')
        self.cancel_btn.config(state='disabled')
        
        if success:
            if uninstall:
                icon = ttk.Label(self.content_frame, text="✓", font=('Segoe UI', 48), foreground='green')
                title_text = "Uninstallation Complete"
                message = """YEET Viewer has been successfully removed!

What was removed:
• YEET Viewer from Program Files
• .yeet file format registration
• Uninstaller entry from Windows Programs

Your .yeet files remain on your computer.

Click Finish to close the installer."""
            else:
                icon = ttk.Label(self.content_frame, text="✓", font=('Segoe UI', 48), foreground='green')
                title_text = "Installation Complete!"
                message = """YEET Viewer has been successfully installed!

What you can do now:
• Double-click any .yeet file to view it
• .yeet files will open automatically with YEET Viewer

Click Finish to close the installer."""
        else:
            icon = ttk.Label(self.content_frame, text="✗", font=('Segoe UI', 48), foreground='red')
            title_text = "Installation Failed"
            message = """Failed to install YEET Viewer.

The installation log has been saved and will open automatically.
Please check the log for details.

You may need to run the installer as Administrator.

Click Finish to close the installer."""
            
            # Save log to file
            self.save_error_log()
        
        icon.pack(pady=20)
        
        title = ttk.Label(
            self.content_frame,
            text=title_text,
            font=('Segoe UI', 14, 'bold')
        )
        title.pack(pady=10)
        
        text = ttk.Label(
            self.content_frame,
            text=message,
            justify='left',
            font=('Segoe UI', 10)
        )
        text.pack(pady=20)
        
        # Resize window to fit content
        self.center_window()
    
    def save_error_log(self):
        """Save installation log to file and open it"""
        try:
            import tempfile
            import subprocess
            from datetime import datetime
            
            # Get log content
            log_content = self.log_text.get('1.0', 'end-1c')
            
            # Create log file in temp directory
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            log_filename = f"YeetViewer_Install_Error_{timestamp}.txt"
            log_path = os.path.join(tempfile.gettempdir(), log_filename)
            
            # Write log to file
            with open(log_path, 'w', encoding='utf-8') as f:
                f.write("YEET Viewer Installation Error Log\n")
                f.write("=" * 50 + "\n")
                f.write(f"Date: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n")
                f.write("=" * 50 + "\n\n")
                f.write(log_content)
            
            # Open log file with notepad
            subprocess.Popen(['notepad.exe', log_path])
            
        except Exception as e:
            # If saving/opening fails, just show a message
            messagebox.showerror("Error", f"Could not save log file: {str(e)}")
    
    def update_next_button(self):
        """Update next button state based on license acceptance"""
        if hasattr(self, 'accept_var') and self.accept_var:
            if self.accept_var.get():
                self.next_btn.config(state='normal')
            else:
                self.next_btn.config(state='disabled')
        else:
            # If accept_var doesn't exist, enable the button
            self.next_btn.config(state='normal')
    
    def go_next(self):
        """Go to next step"""
        if self.current_step == 0:
            self.show_license()
        elif self.current_step == 1:
            # Check for admin rights after license acceptance
            if not self.is_admin():
                # Ask user if they want to restart as admin
                result = messagebox.askyesno(
                    "Administrator Rights Required",
                    "This installer needs administrator privileges to register the .yeet file format.\n\n"
                    "Would you like to restart the installer as Administrator?",
                    icon='warning'
                )
                if result:
                    self.restart_as_admin()
                    return
                else:
                    messagebox.showwarning(
                        "Limited Functionality",
                        "Without administrator rights, the installation cannot proceed.\n\n"
                        "Please run this installer as Administrator to continue."
                    )
                    return
            # If admin or user declined, continue
            self.show_installation_type()
        elif self.current_step == 2:
            self.show_progress()
    
    def go_back(self):
        """Go to previous step"""
        if self.current_step == 1:
            self.show_welcome()
        elif self.current_step == 2:
            self.show_license()
    
    def cancel_install(self):
        """Cancel installation"""
        if messagebox.askyesno("Cancel", "Are you sure you want to cancel the installation?"):
            self.root.quit()
    
    def is_admin(self):
        """Check if running with admin privileges"""
        try:
            import ctypes
            return ctypes.windll.shell32.IsUserAnAdmin()
        except:
            return False
    
    def restart_as_admin(self):
        """Restart the installer with administrator privileges"""
        try:
            import ctypes
            # Get the path to the script
            script_path = os.path.abspath(sys.argv[0])
            
            # Use pythonw.exe instead of python.exe to avoid console window
            python_exe = sys.executable
            if python_exe.endswith('python.exe'):
                python_exe = python_exe.replace('python.exe', 'pythonw.exe')
            
            # Use ShellExecuteW to restart as admin
            ctypes.windll.shell32.ShellExecuteW(
                None,
                "runas",  # Request admin elevation
                python_exe,  # Python interpreter (without console)
                f'"{script_path}"',  # This script
                None,
                1  # SW_SHOWNORMAL
            )
            
            # Close current instance
            self.root.quit()
        except Exception as e:
            messagebox.showerror(
                "Error",
                f"Failed to restart as administrator:\n{str(e)}\n\n"
                "Please manually run this installer as Administrator."
            )
    
    def find_viewer_source(self):
        """Find the viewer executable to install"""
        current_dir = os.path.dirname(os.path.abspath(__file__))
        parent_dir = os.path.dirname(current_dir)
        
        # Priority 1: Bundled with installer (when running as PyInstaller EXE)
        # PyInstaller extracts bundled files to sys._MEIPASS temp folder
        if getattr(sys, 'frozen', False):
            # Running as compiled EXE
            bundle_dir = sys._MEIPASS
            bundled_exe = os.path.join(bundle_dir, "yeet.exe")
            if os.path.exists(bundled_exe):
                self.log(f"✓ Using bundled viewer from installer")
                return bundled_exe
        
        # Priority 2: Workspace release build (new structure) - for development
        workspace_release = os.path.join(parent_dir, "target", "release", "yeet.exe")
        if os.path.exists(workspace_release):
            self.log(f"✓ Found workspace release build")
            return workspace_release
        
        # Priority 3: Component release build (yeet-core) - for development
        core_release = os.path.join(parent_dir, "yeet-core", "target", "release", "yeet.exe")
        if os.path.exists(core_release):
            self.log(f"✓ Found yeet-core release build")
            return core_release
        
        # Priority 4: Legacy release build (yeet-format folder) - backward compatibility
        rust_release = os.path.join(parent_dir, "yeet-format", "target", "release", "yeet.exe")
        if os.path.exists(rust_release):
            self.log(f"✓ Found legacy Rust release build")
            return rust_release
        
        # Priority 5: Legacy debug build (yeet-format folder) - backward compatibility
        rust_debug = os.path.join(parent_dir, "yeet-format", "target", "debug", "yeet.exe")
        if os.path.exists(rust_debug):
            self.log(f"✓ Found legacy Rust debug build")
            return rust_debug
        
        # Priority 6: Python built executable (current dir)
        exe_path = os.path.join(current_dir, "dist", "YeetViewer.exe")
        if os.path.exists(exe_path):
            self.log(f"✓ Found Python viewer build")
            return exe_path
        
        return None
    
    def build_exe_if_needed(self):
        """Build the executable if it doesn't exist"""
        exe_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "dist", "YeetViewer.exe")
        
        if os.path.exists(exe_path):
            self.log(f"✓ Found existing executable")
            return True
        
        self.log("Building executable (this may take a few minutes)...")
        build_script = os.path.join(os.path.dirname(os.path.abspath(__file__)), "build_exe.py")
        
        if os.path.exists(build_script):
            try:
                process = subprocess.Popen(
                    [sys.executable, build_script],
                    stdout=subprocess.PIPE,
                    stderr=subprocess.STDOUT,
                    text=True
                )
                
                for line in process.stdout:
                    self.log(line.rstrip())
                
                process.wait()
                
                if os.path.exists(exe_path) and process.returncode == 0:
                    self.log("✓ Executable built successfully")
                    return True
                else:
                    self.log("✗ Failed to build executable")
                    return False
            except Exception as e:
                self.log(f"✗ Build error: {str(e)}")
                return False
        
        self.log("✗ build_exe.py not found")
        return False
    
    def register_file_extension(self, viewer_path):
        """Register .yeet file extension in Windows Registry"""
        try:
            # Create or open .yeet extension key
            with winreg.CreateKey(winreg.HKEY_CLASSES_ROOT, ".yeet") as key:
                winreg.SetValue(key, "", winreg.REG_SZ, "YeetImageFile")
            self.log("✓ Registered .yeet extension")
            
            # Create or open YeetImageFile file type key
            with winreg.CreateKey(winreg.HKEY_CLASSES_ROOT, "YeetImageFile") as key:
                winreg.SetValue(key, "", winreg.REG_SZ, "YEET Image File")
            self.log("✓ Set file type name")
            
            # Set friendly type name
            with winreg.CreateKey(winreg.HKEY_CLASSES_ROOT, "YeetImageFile") as key:
                winreg.SetValueEx(key, "FriendlyTypeName", 0, winreg.REG_SZ, "YEET Image File")
            self.log("✓ Set display name")
            
            # Set default icon
            with winreg.CreateKey(winreg.HKEY_CLASSES_ROOT, r"YeetImageFile\DefaultIcon") as key:
                winreg.SetValue(key, "", winreg.REG_SZ, f'"{viewer_path}",0')
            self.log("✓ Set file icon")
            
            # Set open command
            with winreg.CreateKey(winreg.HKEY_CLASSES_ROOT, r"YeetImageFile\shell\open\command") as key:
                winreg.SetValue(key, "", winreg.REG_SZ, f'"{viewer_path}" "%1"')
            self.log("✓ Associated with viewer")
            
            # Set content type
            with winreg.CreateKey(winreg.HKEY_CLASSES_ROOT, ".yeet") as key:
                winreg.SetValueEx(key, "Content Type", 0, winreg.REG_SZ, "image/yeet")
            self.log("✓ Configured as image format")
            
            # Set perceived type as image
            with winreg.CreateKey(winreg.HKEY_CLASSES_ROOT, ".yeet") as key:
                winreg.SetValueEx(key, "PerceivedType", 0, winreg.REG_SZ, "image")
            self.log("✓ Applied image properties")
            
            # Notify Windows of the change
            import ctypes
            SHCNE_ASSOCCHANGED = 0x08000000
            SHCNF_IDLIST = 0x0000
            ctypes.windll.shell32.SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None)
            self.log("✓ Updated system")
            
            return True
            
        except Exception as e:
            self.log(f"✗ Error: {str(e)}")
            return False
    
    def unregister_file_extension(self):
        """Unregister .yeet file extension from Windows Registry"""
        try:
            # Delete .yeet extension key
            try:
                winreg.DeleteKey(winreg.HKEY_CLASSES_ROOT, ".yeet")
                self.log("✓ Removed .yeet extension")
            except FileNotFoundError:
                self.log("ℹ .yeet extension was not registered")
            
            # Delete YeetImageFile type keys
            try:
                with winreg.OpenKey(winreg.HKEY_CLASSES_ROOT, "YeetImageFile", 0, winreg.KEY_ALL_ACCESS) as key:
                    try:
                        winreg.DeleteKey(key, r"shell\open\command")
                    except:
                        pass
                    try:
                        winreg.DeleteKey(key, r"shell\open")
                    except:
                        pass
                    try:
                        winreg.DeleteKey(key, "shell")
                    except:
                        pass
                    try:
                        winreg.DeleteKey(key, "DefaultIcon")
                    except:
                        pass
                
                winreg.DeleteKey(winreg.HKEY_CLASSES_ROOT, "YeetImageFile")
                self.log("✓ Removed file type")
            except FileNotFoundError:
                self.log("ℹ File type was not registered")
            
            # Notify Windows of the change
            import ctypes
            SHCNE_ASSOCCHANGED = 0x08000000
            SHCNF_IDLIST = 0x0000
            ctypes.windll.shell32.SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None)
            self.log("✓ Updated system")
            
            return True
            
        except Exception as e:
            self.log(f"✗ Error: {str(e)}")
            return False
    
    def create_uninstaller(self):
        """Create uninstaller entry in Windows Programs"""
        try:
            # Add to Windows uninstall registry
            uninstall_key = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\YeetViewer"
            
            with winreg.CreateKey(winreg.HKEY_LOCAL_MACHINE, uninstall_key) as key:
                winreg.SetValueEx(key, "DisplayName", 0, winreg.REG_SZ, "YEET Image Viewer")
                winreg.SetValueEx(key, "Publisher", 0, winreg.REG_SZ, "Stijn Jakobs")
                winreg.SetValueEx(key, "DisplayVersion", 0, winreg.REG_SZ, "1.0.0")
                winreg.SetValueEx(key, "InstallLocation", 0, winreg.REG_SZ, self.install_path)
                
                # Create uninstall command (points to this installer)
                current_script = os.path.abspath(__file__)
                uninstall_cmd = f'"{sys.executable}" "{current_script}" /uninstall'
                winreg.SetValueEx(key, "UninstallString", 0, winreg.REG_SZ, uninstall_cmd)
                
            return True
        except Exception as e:
            self.log(f"⚠ Uninstaller error: {str(e)}")
            return False
    
    def run(self):
        """Run the installer"""
        self.root.mainloop()


def main():
    """Main entry point"""
    installer = InstallerWizard()
    installer.run()


if __name__ == "__main__":
    main()
