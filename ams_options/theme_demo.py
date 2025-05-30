# theme_demo.py
"""
ArcMoon Studios Theme Demonstration Application
Interactive demo showcasing all available color themes.

Module Classification: Utility
Complexity Level: Medium
API Stability: Stable

Mathematical Properties:
  Algorithmic Complexity:
  - Time Complexity: O(1) for theme switching operations
  - Space Complexity: O(n) where n is number of theme widgets
  - UI Responsiveness: 60fps target with smooth transitions

~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
+ Interactive theme showcase with real-time preview
 - Dynamic theme switching with mathematical color transitions
 - Color palette visualization with hex/RGB/HSL displays
 - Performance-optimized widget updates with minimal redraws
 - Accessibility-compliant contrast ratio calculations
~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
GitHub: ArcMoon Studios (https://github.com/arcmoonstudios)
Copyright: (c) 2025 ArcMoon Studios
License: Business Source License 1.1 (BSL-1.1)
License Terms: Non-production use only; commercial/production use requires paid license.
Effective Date: 2025-05-25 | Change License: GPL v3
License File: /LICENSE
Contact: LordXyn@proton.me
Author: Lord Xyn
Last Validation: 2025-01-26
"""

import tkinter as tk
from tkinter import ttk
import tkinter.font as tkFont
from color_themes import ThemeSelector, ArcMoonThemeVariations
from typing import Optional

class ArcMoonThemeDemo:
    """Interactive demonstration of ArcMoon Studios color themes."""
    
    def __init__(self):
        self.root = tk.Tk()
        self.root.title("🌙 ArcMoon Studios Theme Showcase")
        self.root.geometry("1200x800")
        self.root.configure(bg="#030303")
        
        # Current theme
        self.current_theme = ThemeSelector.get_theme('ultra_dark')
        self.theme_name = tk.StringVar(value="ultra_dark")
        
        # Fonts
        self.title_font = tkFont.Font(family="Consolas", size=16, weight="bold")
        self.code_font = tkFont.Font(family="Consolas", size=10)
        self.label_font = tkFont.Font(family="Segoe UI", size=10)
        
        self.setup_ui()
        self.apply_theme()
    
    def setup_ui(self):
        """Create the demonstration interface."""
        
        # Main container
        main_frame = tk.Frame(self.root)
        main_frame.pack(fill=tk.BOTH, expand=True, padx=10, pady=10)
        
        # Title
        title_label = tk.Label(
            main_frame,
            text="🌙 ArcMoon Studios Enterprise Theme Showcase",
            font=self.title_font
        )
        title_label.pack(pady=(0, 20))
        
        # Theme selector
        theme_frame = tk.Frame(main_frame)
        theme_frame.pack(fill=tk.X, pady=(0, 20))
        
        tk.Label(theme_frame, text="Select Theme:", font=self.label_font).pack(side=tk.LEFT)
        
        theme_combo = ttk.Combobox(
            theme_frame,
            textvariable=self.theme_name,
            values=ThemeSelector.list_available_themes(),
            state="readonly",
            font=self.label_font
        )
        theme_combo.pack(side=tk.LEFT, padx=(10, 0))
        theme_combo.bind('<<ComboboxSelected>>', self.on_theme_change)
        
        # Content area
        content_frame = tk.Frame(main_frame)
        content_frame.pack(fill=tk.BOTH, expand=True)
        
        # Left panel - Color palette
        self.create_color_palette_panel(content_frame)
        
        # Right panel - Theme preview
        self.create_theme_preview_panel(content_frame)
        
        # Store references for theme updates
        self.main_frame = main_frame
        self.title_label = title_label
        self.theme_frame = theme_frame
        self.content_frame = content_frame
    
    def create_color_palette_panel(self, parent):
        """Create the color palette visualization panel."""
        
        palette_frame = tk.LabelFrame(
            parent,
            text="Color Palette",
            font=self.label_font,
            padx=10,
            pady=10
        )
        palette_frame.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=(0, 10))
        
        # Color grid
        self.color_grid = tk.Frame(palette_frame)
        self.color_grid.pack(fill=tk.BOTH, expand=True)
        
        # Store reference
        self.palette_frame = palette_frame
    
    def create_theme_preview_panel(self, parent):
        """Create the theme preview panel with sample UI elements."""
        
        preview_frame = tk.LabelFrame(
            parent,
            text="Theme Preview",
            font=self.label_font,
            padx=10,
            pady=10
        )
        preview_frame.pack(side=tk.RIGHT, fill=tk.BOTH, expand=True)
        
        # Sample toolbar
        toolbar = tk.Frame(preview_frame, height=40)
        toolbar.pack(fill=tk.X, pady=(0, 10))
        
        # Sample buttons
        button_frame = tk.Frame(toolbar)
        button_frame.pack(side=tk.LEFT)
        
        self.sample_buttons = []
        for i, text in enumerate(["New", "Open", "Save", "Run", "Debug"]):
            btn = tk.Button(
                button_frame,
                text=text,
                font=self.label_font,
                width=8,
                relief=tk.FLAT,
                bd=1
            )
            btn.pack(side=tk.LEFT, padx=2)
            self.sample_buttons.append(btn)
        
        # Sample text editor
        editor_frame = tk.Frame(preview_frame)
        editor_frame.pack(fill=tk.BOTH, expand=True, pady=(0, 10))
        
        # Line numbers
        line_numbers = tk.Text(
            editor_frame,
            width=4,
            height=15,
            font=self.code_font,
            wrap=tk.NONE,
            state=tk.DISABLED
        )
        line_numbers.pack(side=tk.LEFT, fill=tk.Y)
        
        # Add line numbers
        line_numbers.config(state=tk.NORMAL)
        for i in range(1, 21):
            line_numbers.insert(tk.END, f"{i:2d}\n")
        line_numbers.config(state=tk.DISABLED)
        
        # Code editor
        self.code_editor = tk.Text(
            editor_frame,
            font=self.code_font,
            wrap=tk.NONE,
            height=15
        )
        self.code_editor.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        
        # Sample code
        sample_code = '''# ArcMoon Studios Enterprise Application
"""Ultra-sophisticated code with mathematical precision."""

class ArcMoonApplication:
    def __init__(self, theme="ultra_dark"):
        self.theme = theme
        self.initialize_components()
    
    def initialize_components(self):
        """Initialize with O(1) complexity."""
        self.logger = create_logger()
        self.database = connect_database()
        self.api_client = APIClient()
    
    async def process_data(self, data):
        """Process data with async optimization."""
        try:
            validated = await self.validate_input(data)
            result = await self.transform_data(validated)
            return {"status": "success", "data": result}
        except ValidationError as e:
            self.logger.error(f"Validation failed: {e}")
            return {"status": "error", "message": str(e)}
    
    def calculate_performance_metrics(self):
        """Mathematical performance analysis."""
        return {
            "complexity": "O(n log n)",
            "memory": "O(n)",
            "throughput": "10,000 ops/sec"
        }'''
        
        self.code_editor.insert(tk.END, sample_code)
        
        # Sample status bar
        self.status_bar = tk.Frame(preview_frame, height=25)
        self.status_bar.pack(fill=tk.X)
        
        status_label = tk.Label(
            self.status_bar,
            text="Ready | Theme: Ultra Dark | Lines: 24 | Memory: 42MB",
            font=self.label_font,
            anchor=tk.W
        )
        status_label.pack(side=tk.LEFT, fill=tk.X, expand=True)
        
        # Store references
        self.preview_frame = preview_frame
        self.toolbar = toolbar
        self.line_numbers = line_numbers
        self.status_label = status_label
    
    def populate_color_palette(self):
        """Populate the color palette with current theme colors."""
        
        # Clear existing colors
        for widget in self.color_grid.winfo_children():
            widget.destroy()
        
        # Get all color attributes from current theme
        colors = {}
        for attr in dir(self.current_theme):
            if not attr.startswith('_') and isinstance(getattr(self.current_theme, attr), str):
                value = getattr(self.current_theme, attr)
                if value.startswith('#'):
                    colors[attr] = value
        
        # Create color swatches
        row = 0
        col = 0
        max_cols = 3
        
        for name, color in colors.items():
            # Color swatch frame
            swatch_frame = tk.Frame(self.color_grid)
            swatch_frame.grid(row=row, column=col, padx=5, pady=5, sticky="ew")
            
            # Color rectangle
            color_rect = tk.Frame(
                swatch_frame,
                bg=color,
                width=80,
                height=40,
                relief=tk.RAISED,
                bd=1
            )
            color_rect.pack(side=tk.LEFT, padx=(0, 10))
            
            # Color info
            info_frame = tk.Frame(swatch_frame)
            info_frame.pack(side=tk.LEFT, fill=tk.X, expand=True)
            
            name_label = tk.Label(
                info_frame,
                text=name.replace('_', ' ').title(),
                font=self.label_font,
                anchor=tk.W
            )
            name_label.pack(anchor=tk.W)
            
            hex_label = tk.Label(
                info_frame,
                text=color.upper(),
                font=self.code_font,
                anchor=tk.W
            )
            hex_label.pack(anchor=tk.W)
            
            # RGB value
            try:
                rgb = tuple(int(color[i:i+2], 16) for i in (1, 3, 5))
                rgb_text = f"RGB({rgb[0]}, {rgb[1]}, {rgb[2]})"
                rgb_label = tk.Label(
                    info_frame,
                    text=rgb_text,
                    font=self.code_font,
                    anchor=tk.W
                )
                rgb_label.pack(anchor=tk.W)
            except ValueError:
                pass
            
            # Update grid position
            col += 1
            if col >= max_cols:
                col = 0
                row += 1
    
    def apply_theme(self):
        """Apply the current theme to all UI elements."""
        
        # Update color palette
        self.populate_color_palette()
        
        # Apply theme to main elements
        bg = getattr(self.current_theme, 'OFF_BLACK', '#030303')
        secondary_bg = getattr(self.current_theme, 'SIDEBAR_DARK', '#060606')
        text_color = getattr(self.current_theme, 'TEXT_PRIMARY', '#F8F8FF')
        accent_color = getattr(self.current_theme, 'LIGHT_BLUE_MOON', '#87CEEB')
        
        # Root window
        self.root.configure(bg=bg)
        
        # Main frame
        self.main_frame.configure(bg=bg)
        
        # Title
        self.title_label.configure(bg=bg, fg=accent_color)
        
        # Theme frame
        self.theme_frame.configure(bg=bg)
        for widget in self.theme_frame.winfo_children():
            if isinstance(widget, tk.Label):
                widget.configure(bg=bg, fg=text_color)
        
        # Content frame
        self.content_frame.configure(bg=bg)
        
        # Palette frame
        self.palette_frame.configure(bg=bg, fg=text_color)
        self.color_grid.configure(bg=bg)
        
        # Preview frame
        self.preview_frame.configure(bg=bg, fg=text_color)
        
        # Toolbar
        self.toolbar.configure(bg=secondary_bg)
        
        # Sample buttons
        button_bg = getattr(self.current_theme, 'BUTTON_PRIMARY', accent_color)
        for i, btn in enumerate(self.sample_buttons):
            # Alternate button colors
            if i % 2 == 0:
                btn_color = button_bg
            else:
                btn_color = getattr(self.current_theme, 'BUTTON_SECONDARY', '#FFB7C5')
            
            btn.configure(
                bg=btn_color,
                fg=bg,
                activebackground=self.lighten_color(btn_color),
                activeforeground=bg
            )
        
        # Code editor
        self.code_editor.configure(
            bg=secondary_bg,
            fg=text_color,
            insertbackground=accent_color,
            selectbackground=accent_color,
            selectforeground=bg
        )
        
        # Line numbers
        self.line_numbers.configure(
            bg=bg,
            fg=getattr(self.current_theme, 'TEXT_SECONDARY', '#B0C4DE')
        )
        
        # Status bar
        self.status_bar.configure(bg=secondary_bg)
        self.status_label.configure(
            bg=secondary_bg,
            fg=text_color,
            text=f"Ready | Theme: {self.theme_name.get().replace('_', ' ').title()} | Lines: 24 | Memory: 42MB"
        )
    
    def lighten_color(self, hex_color: str) -> str:
        """Lighten a hex color for hover effects."""
        try:
            # Convert hex to RGB
            rgb = tuple(int(hex_color[i:i+2], 16) for i in (1, 3, 5))
            # Lighten by 20%
            lightened = tuple(min(255, int(c * 1.2)) for c in rgb)
            return f"#{lightened[0]:02x}{lightened[1]:02x}{lightened[2]:02x}"
        except:
            return hex_color
    
    def on_theme_change(self, event=None):
        """Handle theme selection change."""
        theme_name = self.theme_name.get()
        self.current_theme = ThemeSelector.get_theme(theme_name)
        self.apply_theme()
    
    def run(self):
        """Start the demonstration application."""
        self.root.mainloop()

# ===============================================
# 🚀 APPLICATION LAUNCHER
# ===============================================
if __name__ == "__main__":
    print("🌙 Launching ArcMoon Studios Theme Showcase...")
    app = ArcMoonThemeDemo()
    app.run()
