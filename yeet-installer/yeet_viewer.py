#!/usr/bin/env python3
"""
YEET Image Viewer
A Python application to view .yeet image files
"""

import sys
import struct
import tkinter as tk
from tkinter import messagebox, filedialog, Menu
from PIL import Image, ImageTk
import io
import os


class YeetImageViewer:
    """Viewer for .yeet image files"""
    
    def __init__(self, file_path=None):
        self.file_path = file_path
        self.width = 0
        self.height = 0
        self.image = None
        self.root = None
        self.canvas = None
        self.photo = None
        
    def parse_yeet_file(self):
        """
        Parse .yeet file format:
        - First 4 bytes: width (unsigned 32-bit integer, native byte order)
        - Next 4 bytes: height (unsigned 32-bit integer, native byte order)
        - Remaining bytes: hex color codes (6 chars per pixel, newlines ignored)
        """
        if not self.file_path:
            return False
            
        try:
            with open(self.file_path, 'rb') as f:
                # Read width and height (8 bytes total)
                header = f.read(8)
                if len(header) < 8:
                    raise ValueError("Invalid .yeet file: header too short")
                
                # Unpack width and height as native byte order unsigned integers
                self.width, self.height = struct.unpack('=II', header)
                
                # Read the rest of the file (hex color data)
                color_data = f.read().decode('utf-8', errors='ignore')
                
                # Remove newlines and whitespace
                color_data = color_data.replace('\n', '').replace('\r', '').strip()
                
                # Create PIL Image
                self.image = Image.new('RGB', (self.width, self.height))
                pixels = self.image.load()
                
                # Parse hex colors (6 characters per pixel)
                expected_pixels = self.width * self.height
                pixel_index = 0
                
                for i in range(0, len(color_data), 6):
                    if pixel_index >= expected_pixels:
                        break
                        
                    hex_color = color_data[i:i+6]
                    
                    if len(hex_color) == 6:
                        try:
                            # Convert hex to RGB
                            r = int(hex_color[0:2], 16)
                            g = int(hex_color[2:4], 16)
                            b = int(hex_color[4:6], 16)
                            
                            # Calculate x, y coordinates
                            x = pixel_index % self.width
                            y = pixel_index // self.width
                            
                            pixels[x, y] = (r, g, b)
                            pixel_index += 1
                        except ValueError:
                            # Skip invalid hex values
                            continue
                
                return True
                
        except FileNotFoundError:
            messagebox.showerror("Error", f"File not found: {self.file_path}")
            return False
        except Exception as e:
            messagebox.showerror("Error", f"Failed to parse .yeet file: {str(e)}")
            return False
    
    def open_file(self):
        """Open a .yeet file via file dialog"""
        file_path = filedialog.askopenfilename(
            title="Open YEET Image",
            filetypes=[("YEET files", "*.yeet"), ("All files", "*.*")]
        )
        
        if file_path:
            self.file_path = file_path
            if self.parse_yeet_file():
                self.display_image()
                if self.root:
                    self.root.title(f"YEET Viewer - {os.path.basename(self.file_path)}")
    
    def save_as_png(self):
        """Save the current image as PNG"""
        if not self.image:
            messagebox.showwarning("Warning", "No image loaded to save")
            return
        
        file_path = filedialog.asksaveasfilename(
            title="Save as PNG",
            defaultextension=".png",
            filetypes=[("PNG files", "*.png"), ("All files", "*.*")]
        )
        
        if file_path:
            try:
                self.image.save(file_path, "PNG")
                messagebox.showinfo("Success", f"Image saved as {os.path.basename(file_path)}")
            except Exception as e:
                messagebox.showerror("Error", f"Failed to save image: {str(e)}")
    
    def save_as_yeet(self):
        """Save the current image as .yeet file"""
        if not self.image:
            messagebox.showwarning("Warning", "No image loaded to save")
            return
        
        file_path = filedialog.asksaveasfilename(
            title="Save as YEET",
            defaultextension=".yeet",
            filetypes=[("YEET files", "*.yeet"), ("All files", "*.*")]
        )
        
        if file_path:
            try:
                # Convert image to .yeet format
                width, height = self.image.size
                pixels = self.image.load()
                
                # Build hex string
                hex_data = ""
                for y in range(height):
                    for x in range(width):
                        r, g, b = pixels[x, y]
                        hex_data += f"{r:02x}{g:02x}{b:02x}"
                    if y < height - 1:
                        hex_data += "\n"
                
                # Write to file
                with open(file_path, 'wb') as f:
                    f.write(struct.pack('=II', width, height))
                    f.write(hex_data.encode('utf-8'))
                
                messagebox.showinfo("Success", f"Image saved as {os.path.basename(file_path)}")
            except Exception as e:
                messagebox.showerror("Error", f"Failed to save image: {str(e)}")
    
    def exit_app(self):
        """Exit the application"""
        if self.root:
            self.root.quit()
    
    def display_image(self):
        """Display or update the image in the canvas"""
        if not self.image or not self.canvas:
            return
        
        # Convert PIL Image to PhotoImage
        self.photo = ImageTk.PhotoImage(self.image)
        
        # Clear canvas and add new image
        self.canvas.delete("all")
        self.canvas.create_image(0, 0, anchor='nw', image=self.photo)
        
        # Update scrollable region
        self.canvas.configure(scrollregion=(0, 0, self.width, self.height))
    
    def show(self):
        """Display the image in a Tkinter window"""
        # Create main window
        self.root = tk.Tk()
        self.root.title("YEET Viewer")
        
        # Create menu bar
        menubar = Menu(self.root)
        self.root.config(menu=menubar)
        
        # File menu
        file_menu = Menu(menubar, tearoff=0)
        menubar.add_cascade(label="File", menu=file_menu)
        file_menu.add_command(label="Open...", command=self.open_file, accelerator="Ctrl+O")
        file_menu.add_separator()
        file_menu.add_command(label="Save as PNG...", command=self.save_as_png, accelerator="Ctrl+S")
        file_menu.add_command(label="Save as YEET...", command=self.save_as_yeet)
        file_menu.add_separator()
        file_menu.add_command(label="Exit", command=self.exit_app, accelerator="Alt+F4")
        
        # Keyboard shortcuts
        self.root.bind('<Control-o>', lambda e: self.open_file())
        self.root.bind('<Control-s>', lambda e: self.save_as_png())
        
        # Make window resizable
        self.root.resizable(True, True)
        
        # Create a canvas with scrollbars for large images
        self.canvas = tk.Canvas(self.root, highlightthickness=0)
        h_scrollbar = tk.Scrollbar(self.root, orient=tk.HORIZONTAL, command=self.canvas.xview)
        v_scrollbar = tk.Scrollbar(self.root, orient=tk.VERTICAL, command=self.canvas.yview)
        
        self.canvas.configure(xscrollcommand=h_scrollbar.set, yscrollcommand=v_scrollbar.set)
        
        # Grid layout for canvas and scrollbars
        self.canvas.grid(row=0, column=0, sticky='nsew')
        h_scrollbar.grid(row=1, column=0, sticky='ew')
        v_scrollbar.grid(row=0, column=1, sticky='ns')
        
        # Configure grid weights for resizing
        self.root.grid_rowconfigure(0, weight=1)
        self.root.grid_columnconfigure(0, weight=1)
        
        # Load and display initial image if provided
        if self.file_path and self.parse_yeet_file():
            self.display_image()
            self.root.title(f"YEET Viewer - {os.path.basename(self.file_path)}")
            
            # Set initial window size (constrain to screen size if needed)
            screen_width = self.root.winfo_screenwidth()
            screen_height = self.root.winfo_screenheight()
            
            # Use 80% of screen as max, or image size if smaller
            max_width = int(screen_width * 0.8)
            max_height = int(screen_height * 0.8)
            
            window_width = min(self.width + 20, max_width)  # +20 for scrollbar
            window_height = min(self.height + 20, max_height)
            
            # Center window on screen
            x = (screen_width - window_width) // 2
            y = (screen_height - window_height) // 2
            self.root.geometry(f"{window_width}x{window_height}+{x}+{y}")
        else:
            # Default window size if no image
            self.root.geometry("800x600")
        
        self.root.mainloop()


def main():
    """Main entry point"""
    file_path = None
    
    if len(sys.argv) >= 2:
        file_path = sys.argv[1]
    
    viewer = YeetImageViewer(file_path)
    viewer.show()


if __name__ == "__main__":
    main()
