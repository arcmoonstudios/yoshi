#!/usr/bin/env python3
"""
**Brief:** ArcMoon Studios Enterprise Control Panel with Integrated GitHub Management.

**Module Classification:** Performance-Critical
**Complexity Level:** Expert
**API Stability:** Stable

## Mathematical Properties

**Algorithmic Complexity:**
- Time Complexity: O(1) for UI operations, O(n) for dependency operations
- Space Complexity: O(1) constant UI overhead, O(m) for command output buffering
- Concurrency Safety: Thread-safe GUI operations with async subprocess execution

**Performance Characteristics:**
- Expected Performance: Sub-millisecond UI responsiveness, variable command execution
- Worst-Case Scenarios: Network-dependent cargo operations
- Optimization Opportunities: Async command execution with real-time output streaming

~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
+ ArcMoon Studios Enterprise Control Panel with comprehensive development tools
 - Tkinter-based interface with custom ArcMoon Studios color theme
 - Real-time command execution with output streaming
 - Git safety integration with visual status indicators
 - Enterprise-grade error handling and user feedback
 - Cross-platform PowerShell and Bash script integration
 - Integrated comprehensive crate quality validation system
 - Professional crates.io release readiness validation
 - Complete GitHub CLI integration and repository management
 - SSH key management and authentication handling
~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>

GitHub: ArcMoon Studios (https://github.com/arcmoonstudios)
Copyright: (c) 2025 ArcMoon Studios
License: MIT OR Apache-2.0
License Terms: Full open source freedom; dual licensing allows choice between MIT and Apache 2.0
Effective Date: 2025-05-30 | Open Source Release
License File: /LICENSE
Contact: LordXyn@proton.me
Author: Lord Xyn
Last Validation: 2025-06-02
"""
import os
import sys
import json
import time
import queue
import signal
import logging
import colorsys
import platform
import threading
import subprocess
import tkinter as tk
from pathlib import Path
from datetime import datetime
from dataclasses import dataclass, asdict
from typing import Optional, Callable, NamedTuple, Tuple, List, Dict, Any
from tkinter import ttk, scrolledtext, messagebox, filedialog, simpledialog

# Configure logging for enterprise-grade error tracking
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

@dataclass
class AMSConfig:
    """Configuration settings for AMS Enterprise Control Panel"""
    github_username: str = ""
    default_clone_dir: str = ""
    ssh_key_path: str = ""
    auto_refresh: bool = True
    theme: str = "dark"
    terminal_font: str = "Consolas"
    terminal_font_size: int = 10
    workspace_path: str = ""
    ssh_username: str = ""
    remember_ssh_password: bool = False

    @classmethod
    def load(cls) -> 'AMSConfig':
        """Load configuration from file"""
        config_path = Path.home() / ".ams_config.json"
        if config_path.exists():
            try:
                with open(config_path, 'r') as f:
                    data = json.load(f)
                # Filter out any keys that aren't valid AMSConfig fields
                valid_keys = {field.name for field in cls.__dataclass_fields__.values()}
                filtered_data = {k: v for k, v in data.items() if k in valid_keys}
                return cls(**filtered_data)
            except Exception as e:
                logger.warning(f"Error loading config: {e}")
        return cls()

    def save(self) -> None:
        """Save configuration to file"""
        config_path = Path.home() / ".ams_config.json"
        try:
            with open(config_path, 'w') as f:
                json.dump(asdict(self), f, indent=2)
        except Exception as e:
            logger.error(f"Error saving config: {e}")

@dataclass
class SSHPasswordManager:
    """Secure SSH password management with session caching"""

    def __init__(self):
        self._cached_passwords = {}  # hostname -> password mapping
        self._ssh_usernames = {}     # hostname -> username mapping

    def get_cached_password(self, hostname: str) -> Optional[str]:
        """Get cached password for hostname"""
        return self._cached_passwords.get(hostname)

    def cache_password(self, hostname: str, username: Optional[str], password: Optional[str]) -> None:
        """Cache password for hostname"""
        if password is not None:
            self._cached_passwords[hostname] = password
        if username is not None:
            self._ssh_usernames[hostname] = username

    def get_cached_username(self, hostname: str) -> Optional[str]:
        """Get cached username for hostname"""
        return self._ssh_usernames.get(hostname, None)

    def clear_cache(self) -> None:
        """Clear all cached passwords"""
        self._cached_passwords.clear()
        self._ssh_usernames.clear()

    def prompt_for_ssh_passphrase(self, key_path: str, parent_window=None) -> Optional[str]:
        """Prompt user for SSH key passphrase"""
        try:
            # Create passphrase dialog
            dialog = tk.Toplevel(parent_window) if parent_window else tk.Tk()
            dialog.title("SSH Key Passphrase")
            dialog.geometry("450x180")
            dialog.configure(bg=ArcMoonTheme.DARK_BG)
            dialog.transient(parent_window)
            dialog.grab_set()
            dialog.resizable(False, False)

            # Center the dialog
            dialog.update_idletasks()
            x = (dialog.winfo_screenwidth() // 2) - (450 // 2)
            y = (dialog.winfo_screenheight() // 2) - (180 // 2)
            dialog.geometry(f"450x180+{x}+{y}")

            result = {"passphrase": "", "cancelled": True}

            # Main frame
            main_frame = tk.Frame(dialog, bg=ArcMoonTheme.DARK_BG)
            main_frame.pack(fill='both', expand=True, padx=20, pady=20)

            # Title
            title_label = tk.Label(main_frame,
                                 text="🔐 SSH Key Passphrase Required",
                                 bg=ArcMoonTheme.DARK_BG,
                                 fg=ArcMoonTheme.TEXT_PRIMARY,
                                 font=('Segoe UI', 12, 'bold'))
            title_label.pack(pady=(0, 10))

            key_label = tk.Label(main_frame,
                                text=f"Key: {key_path}",
                                bg=ArcMoonTheme.DARK_BG,
                                fg=ArcMoonTheme.TEXT_SECONDARY,
                                font=('Segoe UI', 9))
            key_label.pack(pady=(0, 15))

            # Passphrase field
            tk.Label(main_frame, text="Passphrase:",
                    bg=ArcMoonTheme.DARK_BG,
                    fg=ArcMoonTheme.TEXT_PRIMARY).pack(anchor='w')

            passphrase_var = tk.StringVar()
            passphrase_entry = tk.Entry(main_frame,
                                      textvariable=passphrase_var,
                                      show="*",
                                      bg=ArcMoonTheme.DARK_TERTIARY,
                                      fg=ArcMoonTheme.TEXT_PRIMARY,
                                      insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON,
                                      relief='flat',
                                      bd=5,
                                      width=40)
            passphrase_entry.pack(fill='x', pady=(2, 15))

            # Buttons
            button_frame = tk.Frame(main_frame, bg=ArcMoonTheme.DARK_BG)
            button_frame.pack(fill='x')

            def on_ok():
                result["passphrase"] = passphrase_var.get()
                result["cancelled"] = False
                dialog.destroy()

            def on_cancel():
                result["cancelled"] = True
                dialog.destroy()

            ok_btn = tk.Button(button_frame, text="Unlock",
                             bg=ArcMoonTheme.BUTTON_SUCCESS,
                             fg=ArcMoonTheme.OFF_BLACK,
                             command=on_ok)
            ok_btn.pack(side='left', padx=5)

            cancel_btn = tk.Button(button_frame, text="Cancel",
                                 bg=ArcMoonTheme.BUTTON_DANGER,
                                 fg=ArcMoonTheme.OFF_BLACK,
                                 command=on_cancel)
            cancel_btn.pack(side='right', padx=5)

            # Bind Enter key
            passphrase_entry.bind('<Return>', lambda e: on_ok())
            passphrase_entry.focus()

            # Wait for dialog completion
            dialog.wait_window()

            return None if result["cancelled"] else result["passphrase"]

        except Exception as e:
            logger.error(f"Error creating passphrase dialog: {e}")
            return None

class Colors:
    """Color codes for terminal output (converted to GUI colors using new palette)"""
    RED = '#FFB6C1'      # Light pink (matches our error color)
    GREEN = '#90EE90'    # Light green (matches our success color)
    YELLOW = '#F0E68C'   # Khaki (matches our warning color)
    BLUE = '#87CEEB'     # Light blue moon (our primary accent)
    MAGENTA = '#FFB7C5'  # Cherry blossom pink
    CYAN = '#B0C4DE'     # Pale blue gray
    WHITE = '#F8F8FF'    # Ghost white

class ArcMoonTheme:
    """ArcMoon Studios Enterprise Color Theme with Light Blue Moon Palette."""    # New Light Blue Moon Color Palette - UPDATED (PURE BLACK)
    LIGHT_BLUE_MOON = "#87CEEB"        # 0 - Light Blue Moon (primary accent)
    OFF_BLACK = "#000000"              # 1 - Pure black (deep backgrounds) - DARKEST
    MEDIUM_DARK_GRAY = "#020202"       # 2 - Almost pure black (secondary elements)
    SIDEBAR_DARK = "#010101"           # 1 - Darker than medium, lighter than main background
    WORKSPACE_BG = "#020202"           # Workspace background (customizable per theme)
    SECTION_ACCENT_1 = "#0A0A0F"       # Section accent 1 (slightly blue-tinted dark)
    SECTION_ACCENT_2 = "#0F0A0A"       # Section accent 2 (slightly red-tinted dark)
    SECTION_ACCENT_3 = "#0A0F0A"       # Section accent 3 (slightly green-tinted dark)
    CHERRY_BLOSSOM_PINK = "#FFB7C5"    # 3 - Cherry blossom pink (highlights)
    PALE_BLUE_GRAY = "#B0C4DE"         # 4 - Pale blue/grayish (tertiary elements)

    # Backward compatibility aliases for any remaining references
    ARCMOON_PURPLE = CHERRY_BLOSSOM_PINK  # Legacy compatibility
    ARCMOON_CYAN = LIGHT_BLUE_MOON        # Legacy compatibility
    ARCMOON_DEEP_PURPLE = CHERRY_BLOSSOM_PINK  # Legacy compatibility
    ARCMOON_TEAL = PALE_BLUE_GRAY         # Legacy compatibility
      # Dark Theme Base (using new palette - REALLY REALLY DARK)
    DARK_BG = OFF_BLACK                   # Primary background (nearly pure black)
    DARK_SECONDARY = SIDEBAR_DARK         # Secondary dark (darker than medium, lighter than main)
    DARK_TERTIARY = MEDIUM_DARK_GRAY      # Tertiary elements
    DARK_BORDER = "#030303"             # Border color (slightly lighter than backgrounds)

    # Text Colors
    TEXT_PRIMARY = "#F8F8FF"           # Primary text (ghost white)
    TEXT_SECONDARY = PALE_BLUE_GRAY      # Secondary text (pale blue gray)
    TEXT_SUCCESS = "#90EE90"           # Success light green
    TEXT_ERROR = "#FFB6C1"             # Error light pink
    TEXT_WARNING = "#F0E68C"           # Warning khaki

    # Interactive Elements (using new palette)
    BUTTON_PRIMARY = CHERRY_BLOSSOM_PINK
    BUTTON_SECONDARY = LIGHT_BLUE_MOON
    BUTTON_SUCCESS = TEXT_SUCCESS
    BUTTON_WARNING = TEXT_WARNING
    BUTTON_DANGER = TEXT_ERROR
      # Hover States
    BUTTON_PRIMARY_HOVER = "#FFD0DC"   # Lighter cherry blossom
    BUTTON_SECONDARY_HOVER = "#ADD8E6" # Light blue

    # Retractable Overlay
    OVERLAY_BG = OFF_BLACK               # Off black (transparency via attributes)
    OVERLAY_PANEL = DARK_SECONDARY       # Panel background
    OVERLAY_ACCENT = LIGHT_BLUE_MOON     # Accent color

    # Button color sets for alternating patterns
    BUTTON_SET_A = [CHERRY_BLOSSOM_PINK, TEXT_SUCCESS, TEXT_WARNING, LIGHT_BLUE_MOON]
    BUTTON_SET_B = [LIGHT_BLUE_MOON, CHERRY_BLOSSOM_PINK, TEXT_SUCCESS, TEXT_WARNING]

class ArcMoonStyles:
    """Enterprise-grade styling configuration with enhanced error handling."""

    @staticmethod
    def configure_styles() -> None:
        """Configure ttk styles with ArcMoon theme and comprehensive error handling."""
        try:
            style = ttk.Style()

            # Configure overall theme with fallback
            try:
                style.theme_use('clam')
            except tk.TclError:
                logger.warning("Clam theme not available, using default")
                try:
                    style.theme_use('default')
                except tk.TclError:
                    logger.error("No TTK themes available, using basic styling")
                    return

            # Main frame styles
            style.configure('ArcMoon.TFrame',
                           background=ArcMoonTheme.DARK_BG,
                           borderwidth=0)

            # Tab content frame (slightly lighter than main bg)
            style.configure('TabContent.TFrame',
                           background=ArcMoonTheme.DARK_SECONDARY,
                           borderwidth=0)

            # Notebook styles (tabs)
            style.configure('TNotebook',
                           background=ArcMoonTheme.DARK_BG,
                           borderwidth=0,
                           tabposition='n')

            style.configure('TNotebook.Tab',
                           background=ArcMoonTheme.DARK_TERTIARY,
                           foreground=ArcMoonTheme.TEXT_PRIMARY,
                           padding=[10, 5],
                           borderwidth=0,
                           focuscolor='none')

            style.map('TNotebook.Tab',
                    background=[('selected', ArcMoonTheme.DARK_SECONDARY),
                               ('active', ArcMoonTheme.DARK_SECONDARY)])            # Workspace frame style (using dedicated workspace background) - Enhanced error handling
            try:
                # First, create the layout for the custom LabelFrame
                style.layout('Workspace.TLabelFrame', [
                    ('Labelframe.border', {
                        'sticky': 'nswe',
                        'children': [
                            ('Labelframe.padding', {
                                'sticky': 'nswe',
                                'children': [
                                    ('Labelframe.label', {'side': 'top', 'sticky': ''}),
                                    ('Labelframe.focus', {
                                        'sticky': 'nswe',
                                        'children': [
                                            ('Labelframe.text', {'sticky': 'nswe'})
                                        ]
                                    })
                                ]
                            })
                        ]
                    })
                ])

                # Now configure the style
                style.configure('Workspace.TLabelFrame',
                               background=ArcMoonTheme.WORKSPACE_BG,
                               borderwidth=1,
                               relief='flat',
                               bordercolor=ArcMoonTheme.LIGHT_BLUE_MOON,
                               lightcolor=ArcMoonTheme.WORKSPACE_BG,
                               darkcolor=ArcMoonTheme.WORKSPACE_BG)

                style.configure('Workspace.TLabelFrame.Label',
                               background=ArcMoonTheme.WORKSPACE_BG,
                               foreground=ArcMoonTheme.TEXT_PRIMARY,
                               font=('Segoe UI', 9, 'bold'))

                # Configure the border element
                style.configure('Workspace.TLabelFrame.Border',
                               background=ArcMoonTheme.WORKSPACE_BG,
                               borderwidth=1,
                               relief='flat')

                logger.debug("Workspace.TLabelFrame style and layout configured successfully")
            except Exception as workspace_error:
                logger.error(f"Failed to configure Workspace.TLabelFrame: {workspace_error}")
                logger.error(f"Workspace error details: {type(workspace_error).__name__}: {str(workspace_error)}")
                # Fallback to basic TLabelFrame style
                try:
                    style.configure('TLabelFrame',
                                   background=ArcMoonTheme.WORKSPACE_BG,
                                   borderwidth=1,
                                   relief='flat')
                    logger.debug("Fallback to basic TLabelFrame style")
                except Exception as fallback_error:
                    logger.error(f"Even basic TLabelFrame configuration failed: {fallback_error}")
                # Continue with other styles even if this one fails

            # Button styles
            style.configure('ArcMoon.TButton',
                           background=ArcMoonTheme.BUTTON_PRIMARY,
                           foreground=ArcMoonTheme.OFF_BLACK,  # Dark text on light button
                           borderwidth=0,
                           focuscolor='none',
                           font=('Segoe UI', 10, 'bold'))

            style.map('ArcMoon.TButton',
                     background=[('active', ArcMoonTheme.BUTTON_PRIMARY_HOVER),
                               ('pressed', ArcMoonTheme.CHERRY_BLOSSOM_PINK)])

            # Secondary button style
            style.configure('ArcMoonSecondary.TButton',
                           background=ArcMoonTheme.BUTTON_SECONDARY,
                           foreground=ArcMoonTheme.OFF_BLACK,  # Dark text on light button
                           borderwidth=0,
                           focuscolor='none',
                           font=('Segoe UI', 10, 'bold'))

            style.map('ArcMoonSecondary.TButton',
                    background=[('active', ArcMoonTheme.BUTTON_SECONDARY_HOVER),
                                ('pressed', ArcMoonTheme.LIGHT_BLUE_MOON)])

            # Success button style
            style.configure('ArcMoonSuccess.TButton',
                           background=ArcMoonTheme.BUTTON_SUCCESS,
                           foreground=ArcMoonTheme.OFF_BLACK,
                           borderwidth=0,
                           focuscolor='none',
                           font=('Segoe UI', 10, 'bold'))

            # Warning button style
            style.configure('ArcMoonWarning.TButton',
                           background=ArcMoonTheme.BUTTON_WARNING,
                           foreground=ArcMoonTheme.OFF_BLACK,
                           borderwidth=0,
                           focuscolor='none',
                           font=('Segoe UI', 10, 'bold'))

            # Danger button style
            style.configure('ArcMoonDanger.TButton',
                           background=ArcMoonTheme.BUTTON_DANGER,
                           foreground=ArcMoonTheme.OFF_BLACK,
                           borderwidth=0,
                           focuscolor='none',
                           font=('Segoe UI', 10, 'bold'))

            # Label styles
            style.configure('ArcMoon.TLabel',
                           background=ArcMoonTheme.DARK_BG,
                           foreground=ArcMoonTheme.TEXT_PRIMARY,
                           font=('Segoe UI', 10))

            style.configure('ArcMoonTitle.TLabel',
                           background=ArcMoonTheme.DARK_BG,
                           foreground=ArcMoonTheme.LIGHT_BLUE_MOON,
                           font=('Segoe UI', 16, 'bold'))
            style.configure('ArcMoonSubtitle.TLabel',
                           background=ArcMoonTheme.DARK_BG,
                           foreground=ArcMoonTheme.PALE_BLUE_GRAY,
                           font=('Segoe UI', 10, 'italic'))

            # Try to style scrollbars (limited in tkinter)
            style.configure('Vertical.TScrollbar',
                           background=ArcMoonTheme.DARK_TERTIARY,
                           troughcolor=ArcMoonTheme.DARK_BG,
                           borderwidth=0,
                           arrowcolor=ArcMoonTheme.PALE_BLUE_GRAY)

            style.map('Vertical.TScrollbar',
                    background=[('active', ArcMoonTheme.MEDIUM_DARK_GRAY)])

        except Exception as e:
            logger.error(f"Failed to configure styles: {e}")
            logger.error(f"Style error details: {type(e).__name__}: {str(e)}")
            # Don't raise - allow GUI to continue with default styles
            # Instead, try to configure minimal fallback styles
            try:
                ArcMoonStyles._configure_fallback_styles()
            except Exception as fallback_error:
                logger.error(f"Fallback style configuration also failed: {fallback_error}")

    @staticmethod
    def _configure_fallback_styles() -> None:
        """Configure minimal fallback styles if main configuration fails."""
        try:
            style = ttk.Style()

            # Basic fallback styles that should always work
            style.configure('Workspace.TLabelFrame',
                           background='#000000',
                           borderwidth=1)

            style.configure('ArcMoon.TFrame',
                           background='#000000')

            style.configure('TabContent.TFrame',
                           background='#010101')

            logger.info("Fallback styles configured successfully")
        except Exception as e:
            logger.error(f"Even fallback styles failed: {e}")

class ColorRGB(NamedTuple):
    """RGB color representation with mathematical operations."""
    r: int
    g: int
    b: int

    def to_hex(self) -> str:
        """Convert RGB to hex format."""
        return f"#{self.r:02x}{self.g:02x}{self.b:02x}"

    def to_hsl(self) -> Tuple[float, float, float]:
        """Convert RGB to HSL color space."""
        return colorsys.rgb_to_hls(self.r/255, self.g/255, self.b/255)

    @classmethod
    def from_hex(cls, hex_color: str) -> 'ColorRGB':
        """Create RGB from hex string."""
        hex_color = hex_color.lstrip('#')
        return cls(
            int(hex_color[0:2], 16),
            int(hex_color[2:4], 16),
            int(hex_color[4:6], 16)
        )

@dataclass
class ArcMoonThemeVariations:
    """Extended ArcMoon Studios theme variations with mathematical precision."""

    # ===============================================
    # 🌙 ORIGINAL ARCMOON ULTRA DARK THEME
    # ===============================================
    class UltraDark:
        """Original ultra-dark theme - nearly pure black backgrounds."""

        # Core backgrounds (extremely dark)
        OFF_BLACK = "#030303"              # Nearly pure black
        SIDEBAR_DARK = "#060606"           # Subtle sidebar differentiation
        MEDIUM_DARK_GRAY = "#0A0A0A"       # Secondary elements
        DARK_BORDER = "#0D0D0D"            # Minimal border visibility
        WORKSPACE_BG = "#020202"           # Ultra dark workspace background

        # Accent colors (light blue moon palette)
        LIGHT_BLUE_MOON = "#87CEEB"        # Primary accent
        CHERRY_BLOSSOM_PINK = "#FFB7C5"    # Secondary accent
        PALE_BLUE_GRAY = "#B0C4DE"         # Tertiary elements

        # Text colors
        TEXT_PRIMARY = "#F8F8FF"           # Ghost white
        TEXT_SECONDARY = "#B0C4DE"         # Pale blue gray
        TEXT_SUCCESS = "#90EE90"           # Light green
        TEXT_ERROR = "#FFB6C1"             # Light pink
        TEXT_WARNING = "#F0E68C"           # Khaki

    # ===============================================
    # 🌌 COSMIC VOID THEME (Even Darker!)
    # ===============================================
    class CosmicVoid:
        """Cosmic void theme - absolute darkness with stellar accents."""

        # Void backgrounds (maximum darkness)
        VOID_BLACK = "#000000"             # Pure black
        SHADOW_GRAY = "#020202"            # Barely visible gray
        NEBULA_DARK = "#040404"            # Faint nebula
        ASTEROID_GRAY = "#080808"          # Asteroid belt
        WORKSPACE_BG = "#000000"           # Pure black workspace

        # Stellar accents
        NEUTRON_BLUE = "#4169E1"           # Royal blue neutron star
        PULSAR_CYAN = "#00CED1"            # Dark turquoise pulsar
        QUASAR_PURPLE = "#9370DB"          # Medium purple quasar
        SOLAR_GOLD = "#FFD700"             # Gold solar flare

        # Cosmic text
        STARLIGHT = "#FFFFFF"              # Pure white starlight
        MOONBEAM = "#F0F8FF"               # Alice blue moonbeam
        AURORA_GREEN = "#00FF7F"           # Spring green aurora
        COMET_TAIL = "#87CEEB"             # Sky blue comet

    # ===============================================
    # 🎯 MATRIX NOIR THEME
    # ===============================================
    class MatrixNoir:
        """Matrix-inspired noir theme with green phosphor accents."""

        # Matrix backgrounds
        MATRIX_BLACK = "#000000"           # Pure black matrix
        TERMINAL_DARK = "#001100"          # Dark green tint
        CODE_RAIN_BG = "#002200"           # Code rain background
        CONSOLE_GRAY = "#003300"           # Console background
        WORKSPACE_BG = "#001100"           # Dark green workspace (special for Matrix)

        # Phosphor greens
        PHOSPHOR_GREEN = "#00FF00"         # Classic matrix green
        TERMINAL_GREEN = "#00CC00"         # Terminal text green
        DATA_STREAM = "#009900"            # Data stream green
        GHOST_GREEN = "#006600"            # Faded green

        # Noir accents
        NEON_CYAN = "#00FFFF"              # Neon cyan highlights
        WARNING_AMBER = "#FFAA00"          # Amber warnings
        ERROR_RED = "#FF3333"              # Error red
        WHITE_NOISE = "#CCCCCC"            # Static white

    # ===============================================
    # 🔥 EMBER STORM THEME
    # ===============================================
    class EmberStorm:
        """Ember storm theme - dark with warm fire accents."""

        # Storm backgrounds
        STORM_BLACK = "#0A0A0A"            # Storm cloud black
        ASH_GRAY = "#1A1A1A"               # Volcanic ash
        EMBER_DARK = "#2A1A1A"             # Dark ember glow
        SMOKE_GRAY = "#3A2A2A"             # Smoke gray
        WORKSPACE_BG = "#050505"           # Ultra dark storm workspace

        # Fire accents
        EMBER_ORANGE = "#FF6600"           # Bright ember
        FLAME_RED = "#FF3300"              # Flame red
        COAL_GLOW = "#CC3300"              # Glowing coal
        SUNSET_GOLD = "#FFCC00"            # Sunset gold

        # Storm colors
        LIGHTNING_WHITE = "#FFFFFF"        # Lightning flash
        RAIN_BLUE = "#336699"              # Rain blue
        THUNDER_PURPLE = "#663399"         # Thunder purple
        MIST_GRAY = "#999999"              # Mist gray

    # ===============================================
    # ❄️ ARCTIC FROST THEME
    # ===============================================
    class ArcticFrost:
        """Arctic frost theme - cool blues and whites."""

        # Arctic backgrounds
        ARCTIC_BLACK = "#0A0F1A"           # Arctic night
        ICE_BLUE = "#1A2F3A"               # Deep ice blue
        GLACIER_GRAY = "#2A3F4A"           # Glacier gray
        SNOW_DRIFT = "#3A4F5A"             # Snow drift
        WORKSPACE_BG = "#040608"           # Ultra dark arctic workspace

        # Frost accents
        ICE_CRYSTAL = "#87CEEB"            # Ice crystal blue
        AURORA_BLUE = "#4169E1"            # Aurora blue
        FROST_WHITE = "#F0F8FF"            # Frost white
        ARCTIC_CYAN = "#00CED1"            # Arctic cyan

        # Winter colors
        SNOW_WHITE = "#FFFFFF"             # Pure snow
        BLIZZARD_GRAY = "#E6E6FA"          # Blizzard gray
        POLAR_BLUE = "#B0E0E6"             # Polar blue
        TUNDRA_GREEN = "#2E8B57"           # Tundra green

    # ===============================================
    # 🌙 MATHEMATICAL COLOR GENERATION
    # ===============================================
    @staticmethod
    def generate_analogous_colors(base_hex: str, count: int = 5) -> list[str]:
        """Generate analogous colors using mathematical color theory."""
        base_rgb = ColorRGB.from_hex(base_hex)
        h, l, s = base_rgb.to_hsl()

        colors = []
        for i in range(count):
            # Generate analogous hues (±30 degrees)
            hue_shift = (i - count//2) * 30 / 360
            new_h = (h + hue_shift) % 1.0

            # Convert back to RGB
            r, g, b = colorsys.hls_to_rgb(new_h, l, s)
            rgb = ColorRGB(int(r*255), int(g*255), int(b*255))
            colors.append(rgb.to_hex())

        return colors

    @staticmethod
    def generate_triadic_colors(base_hex: str) -> Tuple[str, str, str]:
        """Generate triadic color harmony (120° apart)."""
        base_rgb = ColorRGB.from_hex(base_hex)
        h, l, s = base_rgb.to_hsl()

        colors = []
        for shift in [0, 120/360, 240/360]:
            new_h = (h + shift) % 1.0
            r, g, b = colorsys.hls_to_rgb(new_h, l, s)
            rgb = ColorRGB(int(r*255), int(g*255), int(b*255))
            colors.append(rgb.to_hex())

        return tuple(colors)

    @staticmethod
    def darken_color(hex_color: str, factor: float = 0.2) -> str:
        """Mathematically darken a color by reducing lightness."""
        rgb = ColorRGB.from_hex(hex_color)
        h, l, s = rgb.to_hsl()

        # Reduce lightness
        new_l = max(0, l - factor)

        r, g, b = colorsys.hls_to_rgb(h, new_l, s)
        darkened = ColorRGB(int(r*255), int(g*255), int(b*255))
        return darkened.to_hex()

    @staticmethod
    def create_gradient(start_hex: str, end_hex: str, steps: int = 10) -> list[str]:
        """Create a smooth color gradient between two colors."""
        start_rgb = ColorRGB.from_hex(start_hex)
        end_rgb = ColorRGB.from_hex(end_hex)

        gradient = []
        for i in range(steps):
            factor = i / (steps - 1)

            r = int(start_rgb.r + (end_rgb.r - start_rgb.r) * factor)
            g = int(start_rgb.g + (end_rgb.g - start_rgb.g) * factor)
            b = int(start_rgb.b + (end_rgb.b - start_rgb.b) * factor)

            gradient.append(ColorRGB(r, g, b).to_hex())

        return gradient

# ===============================================
# 🎨 THEME SELECTOR UTILITY
# ===============================================
class ThemeSelector:
    """Dynamic theme selection with mathematical optimization."""

    @classmethod
    def get_themes_dict(cls):
        """Get available themes dictionary."""
        return {
            'ultra_dark': ArcMoonThemeVariations.UltraDark,
            'cosmic_void': ArcMoonThemeVariations.CosmicVoid,
            'matrix_noir': ArcMoonThemeVariations.MatrixNoir,
            'ember_storm': ArcMoonThemeVariations.EmberStorm,
            'arctic_frost': ArcMoonThemeVariations.ArcticFrost,
        }

    @classmethod
    def get_theme(cls, theme_name: str):
        """Get theme by name with validation."""
        themes = cls.get_themes_dict()
        return themes.get(theme_name, themes['ultra_dark'])

    @classmethod
    def list_available_themes(cls) -> list[str]:
        """List all available theme names."""
        return list(cls.get_themes_dict().keys())

    @classmethod
    def create_custom_theme(cls, base_theme: str, customizations: Dict[str, str]):
        """Create a custom theme based on an existing theme."""
        base = cls.get_theme(base_theme)

        # Create a new class dynamically
        class CustomTheme:
            pass

        # Copy all attributes from base theme
        for attr in dir(base):
            if not attr.startswith('_'):
                setattr(CustomTheme, attr, getattr(base, attr))

        # Apply customizations
        for attr, value in customizations.items():
            setattr(CustomTheme, attr, value)

        return CustomTheme

# ===============================================
# 🧪 EXAMPLE USAGE AND DEMONSTRATIONS
# ===============================================
if __name__ == "__main__":
    # Demonstrate color generation
    print("🌙 ArcMoon Studios Color Theme Variations")
    print("=" * 50)

    # Show available themes
    themes = ThemeSelector.list_available_themes()
    print(f"Available themes: {', '.join(themes)}")

    # Demonstrate color mathematics
    base_color = "#87CEEB"  # Light Blue Moon
    print(f"\nBase color: {base_color}")

    # Generate analogous colors
    analogous = ArcMoonThemeVariations.generate_analogous_colors(base_color)
    print(f"Analogous colors: {analogous}")

    # Generate triadic harmony
    triadic = ArcMoonThemeVariations.generate_triadic_colors(base_color)
    print(f"Triadic harmony: {triadic}")

    # Create gradient
    gradient = ArcMoonThemeVariations.create_gradient("#000000", base_color, 5)
    print(f"Gradient to black: {gradient}")

    # Demonstrate theme usage
    cosmic_theme = ThemeSelector.get_theme('cosmic_void')
    print(f"\nCosmic Void primary background: {cosmic_theme.VOID_BLACK}")
    print(f"Cosmic Void stellar accent: {cosmic_theme.NEUTRON_BLUE}")

class CrateChecker:
    """Comprehensive Rust Crate Quality Validation System"""

    def __init__(self, output_callback: Optional[Callable[[str], None]] = None,
                 working_dir: Optional[str] = None):
        self.passed_checks = 0
        self.total_checks = 0
        self.failed_checks = []
        self.warnings = []
        self.output_callback = output_callback
        self.working_dir = working_dir or os.getcwd()
        self._cancelled = False
        self.workspace_info = self._detect_workspace()
        self.packages = self._get_workspace_packages()

    def output(self, text: str, color: str = Colors.WHITE) -> None:
        """Output text with optional color formatting"""
        if self.output_callback:
            # Remove ANSI color codes for GUI output and add timestamp
            clean_text = text.replace('\033[91m', '').replace('\033[92m', '').replace('\033[93m', '')
            clean_text = clean_text.replace('\033[94m', '').replace('\033[95m', '').replace('\033[96m', '')
            clean_text = clean_text.replace('\033[97m', '').replace('\033[1m', '').replace('\033[4m', '')
            clean_text = clean_text.replace('\033[0m', '')
            self.output_callback(clean_text)

    def _detect_workspace(self) -> Dict[str, Any]:
        """Detect if this is a workspace project and get workspace information."""
        try:
            cargo_toml_path = Path(self.working_dir) / "Cargo.toml"
            if not cargo_toml_path.exists():
                return {"is_workspace": False, "members": []}

            with open(cargo_toml_path, 'r') as f:
                content = f.read()

            # Simple detection - look for [workspace] section
            is_workspace = "[workspace]" in content and "members" in content

            return {
                "is_workspace": is_workspace,
                "root_dir": self.working_dir,
                "cargo_toml": str(cargo_toml_path)
            }
        except Exception as e:
            logger.warning(f"Error detecting workspace: {e}")
            return {"is_workspace": False, "members": []}

    def _get_workspace_packages(self) -> List[str]:
        """Get list of packages in workspace."""
        try:
            if not self.workspace_info.get("is_workspace", False):
                # Single package project
                cargo_toml_path = Path(self.working_dir) / "Cargo.toml"
                if cargo_toml_path.exists():
                    # Try to extract package name from Cargo.toml
                    with open(cargo_toml_path, 'r') as f:
                        content = f.read()
                        import re
                        name_match = re.search(r'name\s*=\s*"([^"]+)"', content)
                        if name_match:
                            return [name_match.group(1)]
                return ["current_package"]

            # For workspace, use cargo metadata to get package list
            result = subprocess.run(
                ["cargo", "metadata", "--format-version", "1", "--no-deps"],
                cwd=self.working_dir,
                capture_output=True,
                text=True,
                timeout=30
            )

            if result.returncode == 0:
                import json
                metadata = json.loads(result.stdout)
                packages = []
                for package in metadata.get("packages", []):
                    packages.append(package["name"])
                return packages
            else:
                # Fallback: try to find packages manually
                return self._find_packages_manually()

        except Exception as e:
            logger.warning(f"Error getting workspace packages: {e}")
            return ["unknown_package"]

    def _find_packages_manually(self) -> List[str]:
        """Manually find packages by looking for Cargo.toml files."""
        packages = []
        try:
            for root, dirs, files in os.walk(self.working_dir):
                if "Cargo.toml" in files:
                    # Skip the workspace root
                    if root != self.working_dir:
                        package_name = os.path.basename(root)
                        packages.append(package_name)
        except Exception as e:
            logger.warning(f"Error finding packages manually: {e}")

        return packages if packages else ["unknown_package"]

    def print_header(self, title: str) -> None:
        """Print a formatted section header"""
        self.output(f"\n{'='*60}\n")
        self.output(f"{title.center(60)}\n", Colors.CYAN)
        self.output(f"{'='*60}\n")

    def print_step(self, step: str) -> None:
        """Print a step description"""
        self.output(f"\n🔍 {step}\n", Colors.BLUE)

    def run_command(self, cmd: List[str], description: str, critical: bool = True) -> Tuple[bool, str]:
        """Run a command and return success status and output"""
        self.total_checks += 1
        original_dir = None

        try:
            if self._cancelled:
                self.output("   Operation cancelled\n", Colors.YELLOW)
                return False, "Operation cancelled"

            self.output(f"   Running: {' '.join(cmd)}\n")

            # Change to working directory for command execution
            original_dir = os.getcwd()
            if self.working_dir and os.path.exists(self.working_dir):
                os.chdir(self.working_dir)

            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=300  # 5 minute timeout
            )

            if result.returncode == 0:
                self.output(f"   ✅ {description} - PASSED\n", Colors.GREEN)
                self.passed_checks += 1
                return True, result.stdout
            else:
                error_msg = f"{description} - FAILED"
                self.output(f"   ❌ {error_msg}\n", Colors.RED)
                if result.stderr:
                    self.output(f"   Error: {result.stderr.strip()}\n", Colors.RED)

                if critical:
                    self.failed_checks.append(error_msg)
                else:
                    self.warnings.append(error_msg)

                return False, result.stderr

        except subprocess.TimeoutExpired:
            error_msg = f"{description} - TIMEOUT"
            self.output(f"   ⏰ {error_msg}\n", Colors.RED)
            if critical:
                self.failed_checks.append(error_msg)
            else:
                self.warnings.append(error_msg)
            return False, "Command timed out"

        except Exception as e:
            error_msg = f"{description} - ERROR: {str(e)}"
            self.output(f"   💥 {error_msg}\n", Colors.RED)
            if critical:
                self.failed_checks.append(error_msg)
            else:
                self.warnings.append(error_msg)
            return False, str(e)
        finally:
            # Ensure we're back in the original directory
            if original_dir is not None:
                try:
                    os.chdir(original_dir)
                except Exception as e:
                    logger.error(f"Failed to restore directory: {e}")

    def check_prerequisites(self) -> bool:
        """Check if required tools are available"""
        self.print_header("PREREQUISITE CHECKS")

        tools = [
            (["cargo", "--version"], "Cargo availability"),
            (["rustc", "--version"], "Rust compiler availability"),
            (["git", "--version"], "Git availability"),
        ]

        all_good = True
        for cmd, desc in tools:
            success, _ = self.run_command(cmd, desc, critical=True)
            if not success:
                all_good = False

        return all_good

    def core_compilation_tests(self) -> bool:
        """Run core compilation and testing checks"""
        if self._cancelled:
            return False

        self.print_header("CORE COMPILATION & TESTING")

        checks = [
            (["cargo", "check"], "Basic compilation check"),
            (["cargo", "build"], "Full build"),
            (["cargo", "build", "--release"], "Release build"),
            (["cargo", "test"], "All tests"),
            (["cargo", "test", "--all-targets"], "All targets tests"),
            (["cargo", "bench", "--no-run"], "Benchmark compilation"),
        ]

        all_passed = True
        for cmd, desc in checks:
            if self._cancelled:
                return False
            self.print_step(desc)
            success, _ = self.run_command(cmd, desc, critical=True)
            if not success:
                all_passed = False

        return all_passed

    def code_quality_checks(self) -> bool:
        """Run code quality and linting checks"""
        if self._cancelled:
            return False

        self.print_header("CODE QUALITY CHECKS")

        checks = [
            (["cargo", "fmt", "--all", "--check"], "Code formatting check"),
            (["cargo", "fmt", "--all"], "Code formatting (auto-fix)", False),
            (["cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings"], "Clippy linting"),
            (["cargo", "check", "--all-targets", "--all-features"], "All features check"),
        ]

        all_passed = True
        for cmd, desc, *critical_flag in checks:
            if self._cancelled:
                return False
            critical = critical_flag[0] if critical_flag else True
            self.print_step(desc)
            success, _ = self.run_command(cmd, desc, critical=critical)
            if not success and critical:
                all_passed = False

        return all_passed

    def security_checks(self) -> bool:
        """Run security audit checks"""
        if self._cancelled:
            return False

        self.print_header("SECURITY AUDIT CHECKS")

        # First try to install cargo-audit if not available
        self.print_step("Checking cargo-audit availability")
        check_result = subprocess.run(
            ["cargo", "audit", "--version"],
            capture_output=True,
            text=True,
            cwd=self.working_dir
        )

        if check_result.returncode != 0:
            self.output("   cargo-audit not found, attempting to install...\n", Colors.YELLOW)
            install_success, _ = self.run_command(
                ["cargo", "install", "cargo-audit"],
                "Install cargo-audit",
                critical=False
            )
            if not install_success:
                self.output("   ⚠️ cargo-audit installation failed, skipping security audit\n", Colors.YELLOW)
                self.warnings.append("cargo-audit not available")
                return True  # Not critical

        checks = [
            (["cargo", "audit"], "Security vulnerability audit"),
            (["cargo", "audit", "--deny", "warnings"], "Security audit (deny warnings)", False),
        ]

        all_passed = True
        for cmd, desc, *critical_flag in checks:
            if self._cancelled:
                return False
            critical = critical_flag[0] if critical_flag else True
            self.print_step(desc)
            success, _ = self.run_command(cmd, desc, critical=critical)
            if not success and critical:
                all_passed = False

        return all_passed

    def documentation_checks(self) -> bool:
        """Run documentation generation checks"""
        if self._cancelled:
            return False

        self.print_header("DOCUMENTATION CHECKS")

        checks = [
            (["cargo", "doc", "--no-deps"], "Basic documentation"),
            (["cargo", "doc", "--all-features", "--no-deps"], "Documentation with all features"),
            (["cargo", "doc", "--document-private-items", "--no-deps"], "Private items documentation", False),
        ]

        all_passed = True
        for cmd, desc, *critical_flag in checks:
            if self._cancelled:
                return False
            critical = critical_flag[0] if critical_flag else True
            self.print_step(desc)
            success, _ = self.run_command(cmd, desc, critical=critical)
            if not success and critical:
                all_passed = False

        return all_passed

    def benchmark_checks(self) -> bool:
        """Run benchmark checks"""
        if self._cancelled:
            return False

        self.print_header("BENCHMARK CHECKS")

        # Check if benchmarks exist
        bench_dir = Path(self.working_dir) / "benches"
        cargo_toml = Path(self.working_dir) / "Cargo.toml"

        has_benches = False
        if bench_dir.exists() and any(bench_dir.glob("*.rs")):
            has_benches = True
        elif cargo_toml.exists():
            try:
                with open(cargo_toml, 'r') as f:
                    content = f.read()
                    if "[[bench]]" in content or "[bench]" in content:
                        has_benches = True
            except Exception:
                pass

        if not has_benches:
            self.output("   ℹ️ No benchmarks found, skipping benchmark checks\n", Colors.CYAN)
            return True

        checks = [
            (["cargo", "bench", "--no-run"], "Benchmark compilation"),
            (["cargo", "bench", "--", "--test"], "Benchmark validation", False),
        ]

        all_passed = True
        for cmd, desc, *critical_flag in checks:
            if self._cancelled:
                return False
            critical = critical_flag[0] if critical_flag else True
            self.print_step(desc)
            success, _ = self.run_command(cmd, desc, critical=critical)
            if not success and critical:
                all_passed = False

        return all_passed

    def package_validation(self) -> bool:
        """Run package validation for crates.io"""
        if self._cancelled:
            return False

        self.print_header("PACKAGE VALIDATION")

        if self.workspace_info.get("is_workspace", False):
            self.output("   📦 Workspace detected - validating all packages\n", Colors.CYAN)
            all_passed = True

            for package in self.packages:
                if self._cancelled:
                    return False

                self.output(f"\n   🎯 Validating package: {package}\n", Colors.BLUE)

                checks = [
                    (["cargo", "package", "--list", "-p", package], f"Package file list for {package}"),
                    (["cargo", "package", "--allow-dirty", "-p", package], f"Package creation for {package}"),
                    (["cargo", "publish", "--dry-run", "-p", package], f"Publish dry run for {package}", False),
                ]

                for cmd, desc, *critical_flag in checks:
                    if self._cancelled:
                        return False
                    critical = critical_flag[0] if critical_flag else True
                    self.print_step(desc)
                    success, _ = self.run_command(cmd, desc, critical=critical)
                    if not success and critical:
                        all_passed = False
        else:
            # Single package validation
            checks = [
                (["cargo", "package", "--list"], "Package file list"),
                (["cargo", "package", "--allow-dirty"], "Package creation"),
                (["cargo", "publish", "--dry-run"], "Publish dry run", False),
            ]

            all_passed = True
            for cmd, desc, *critical_flag in checks:
                if self._cancelled:
                    return False
                critical = critical_flag[0] if critical_flag else True
                self.print_step(desc)
                success, _ = self.run_command(cmd, desc, critical=critical)
                if not success and critical:
                    all_passed = False

        return all_passed

    def metadata_checks(self) -> bool:
        """Check required metadata files"""
        if self._cancelled:
            return False

        self.print_header("METADATA & FILES CHECK")

        required_files = [
            ("Cargo.toml", "Cargo manifest", True),
            ("README.md", "README file", False),
            ("LICENSE", "License file", False),
            ("CHANGELOG.md", "Changelog file", False),
        ]

        all_passed = True
        for filename, desc, critical in required_files:
            if self._cancelled:
                return False
            self.print_step(f"Checking {desc}")
            file_path = Path(self.working_dir) / filename
            if file_path.exists():
                self.output(f"   ✅ {desc} - EXISTS\n", Colors.GREEN)
                self.passed_checks += 1
            else:
                if critical:
                    self.output(f"   ❌ {desc} - MISSING (CRITICAL)\n", Colors.RED)
                    self.failed_checks.append(f"{desc} missing")
                    all_passed = False
                else:
                    self.output(f"   ⚠️ {desc} - MISSING\n", Colors.YELLOW)
                    self.warnings.append(f"{desc} missing")
            self.total_checks += 1

        return all_passed

    def print_summary(self) -> bool:
        """Print final summary of all checks"""
        self.print_header("FINAL SUMMARY")

        self.output(f"\n📊 RESULTS SUMMARY:\n")
        self.output(f"   Total Checks: {self.total_checks}\n")
        self.output(f"   Passed: {self.passed_checks}\n", Colors.GREEN)
        self.output(f"   Failed: {len(self.failed_checks)}\n", Colors.RED)
        self.output(f"   Warnings: {len(self.warnings)}\n", Colors.YELLOW)

        if self.failed_checks:
            self.output(f"\n❌ CRITICAL FAILURES:\n", Colors.RED)
            for failure in self.failed_checks:
                self.output(f"   • {failure}\n", Colors.RED)

        if self.warnings:
            self.output(f"\n⚠️ WARNINGS:\n", Colors.YELLOW)
            for warning in self.warnings:
                self.output(f"   • {warning}\n", Colors.YELLOW)

        # Final verdict
        if not self.failed_checks and not self.warnings:
            self.output(f"\n🎉 CRATE IS READY FOR CRATES.IO RELEASE! 🎉\n", Colors.GREEN)
            self.output(f"All checks passed with no failures or warnings. You can proceed with publishing.\n", Colors.GREEN)
            return True
        elif not self.failed_checks and self.warnings:
            self.output(f"\n⚠️ CRATE NOT READY - WARNINGS MUST BE FIXED\n", Colors.YELLOW)
            self.output(f"All critical checks passed, but warnings must be addressed before release.\n", Colors.YELLOW)
            return False
        else:
            self.output(f"\n🚫 CRATE NOT READY FOR RELEASE\n", Colors.RED)
            self.output(f"Please fix the critical failures and warnings before publishing.\n", Colors.RED)
            return False

    def cancel(self) -> None:
        """Cancel the current validation process"""
        self._cancelled = True

    def run_all_checks(self) -> bool:
        """Run all checks in sequence"""
        self.output("🦀 RUST CRATE QUALITY CHECKER 🦀\n", Colors.MAGENTA)
        self.output("Comprehensive validation for crates.io release\n")
        self.output("="*50 + "\n")

        start_time = time.time()

        # Reset cancellation flag
        self._cancelled = False

        # Run all check categories
        checks_passed = True

        if not self.check_prerequisites():
            self.output("\n❌ Prerequisites failed. Cannot continue.\n", Colors.RED)
            return False

        if self._cancelled:
            self.output("\n🛑 Validation cancelled by user.\n", Colors.YELLOW)
            return False

        checks_passed &= self.core_compilation_tests()
        if not self._cancelled:
            checks_passed &= self.code_quality_checks()
        if not self._cancelled:
            checks_passed &= self.security_checks()
        if not self._cancelled:
            checks_passed &= self.documentation_checks()
        if not self._cancelled:
            checks_passed &= self.benchmark_checks()
        if not self._cancelled:
            checks_passed &= self.package_validation()
        if not self._cancelled:
            checks_passed &= self.metadata_checks()

        if self._cancelled:
            self.output("\n🛑 Validation cancelled by user.\n", Colors.YELLOW)
            return False

        # Print timing
        elapsed = time.time() - start_time
        self.output(f"\n⏱️ Total execution time: {elapsed:.2f} seconds\n", Colors.CYAN)

        # Print summary
        return self.print_summary()

class GitHubOperations:
    """Core GitHub operations using gh CLI and git commands"""

    def __init__(self, executor):
        self.executor = executor
        self.ssh_manager: Optional[SSHPasswordManager] = None  # Will be set by the main GUI class
        self.command_executor = executor
        self.root: Optional[tk.Tk] = None  # Will be set by the main GUI class

    def _execute_git_with_ssh_auth(self, cmd: str, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Execute git command with SSH authentication support"""
        try:
            # Try SSH command with GUI passphrase handling
            return self._execute_ssh_with_gui_passphrase(cmd, cwd)
        except Exception as e:
            return 1, "", f"Git command error: {str(e)}"

    # Repository Operations
    def create_repo(self, name: str, private: bool = False, description: str = "") -> Tuple[int, str, str]:
        """Create a new repository"""
        cmd = f"gh repo create {name}"
        if private:
            cmd += " --private"
        else:
            cmd += " --public"
        if description:
            cmd += f" --description '{description}'"
        return self.executor.execute_command(cmd)

    def delete_repo(self, repo_name: str) -> Tuple[int, str, str]:
        """Delete a repository"""
        cmd = f"gh repo delete {repo_name} --yes"
        return self.executor.execute_command(cmd)

    def fork_repo(self, repo_name: str) -> Tuple[int, str, str]:
        """Fork a repository"""
        cmd = f"gh repo fork {repo_name}"
        return self.executor.execute_command(cmd)

    def list_repos(self, user: str = "", limit: int = 30) -> Tuple[int, str, str]:
        """List repositories"""
        cmd = f"gh repo list"
        if user:
            cmd += f" {user}"
        cmd += f" --limit {limit}"
        return self.executor.execute_command(cmd)

    # Branch Operations
    def create_branch(self, branch_name: str, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Create a new branch"""
        cmd = f"git checkout -b {branch_name}"
        return self.executor.execute_command(cmd, cwd)

    def switch_branch(self, branch_name: str, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Switch to a branch"""
        cmd = f"git checkout {branch_name}"
        return self.executor.execute_command(cmd, cwd)

    def delete_branch(self, branch_name: str, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Delete a branch"""
        cmd = f"git branch -d {branch_name}"
        return self.executor.execute_command(cmd, cwd)

    def list_branches(self, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """List branches"""
        cmd = "git branch -a"
        return self.executor.execute_command(cmd, cwd)

    # Issue Operations
    def create_issue(self, title: str, body: str = "", labels: Optional[List[str]] = None) -> Tuple[int, str, str]:
        """Create an issue"""
        cmd = f"gh issue create --title '{title}'"
        if body:
            cmd += f" --body '{body}'"
        if labels:
            cmd += f" --label {','.join(labels)}"
        return self.executor.execute_command(cmd)

    def list_issues(self, state: str = "open", limit: int = 30) -> Tuple[int, str, str]:
        """List issues"""
        cmd = f"gh issue list --state {state} --limit {limit}"
        return self.executor.execute_command(cmd)

    def close_issue(self, issue_number: int) -> Tuple[int, str, str]:
        """Close an issue"""
        cmd = f"gh issue close {issue_number}"
        return self.executor.execute_command(cmd)

    # Pull Request Operations
    def create_pr(self, title: str, body: str = "", base: str = "main") -> Tuple[int, str, str]:
        """Create a pull request"""
        cmd = f"gh pr create --title '{title}' --base {base}"
        if body:
            cmd += f" --body '{body}'"
        return self.executor.execute_command(cmd)

    def list_prs(self, state: str = "open", limit: int = 30) -> Tuple[int, str, str]:
        """List pull requests"""
        cmd = f"gh pr list --state {state} --limit {limit}"
        return self.executor.execute_command(cmd)

    def merge_pr(self, pr_number: int, method: str = "merge") -> Tuple[int, str, str]:
        """Merge a pull request"""
        cmd = f"gh pr merge {pr_number} --{method}"
        return self.executor.execute_command(cmd)

    # Release Operations
    def create_release(self, tag: str, title: str, notes: str = "") -> Tuple[int, str, str]:
        """Create a release"""
        cmd = f"gh release create {tag} --title '{title}'"
        if notes:
            cmd += f" --notes '{notes}'"
        return self.executor.execute_command(cmd)

    def list_releases(self, limit: int = 30) -> Tuple[int, str, str]:
        """List releases"""
        cmd = f"gh release list --limit {limit}"
        return self.executor.execute_command(cmd)

    # Gist Operations
    def create_gist(self, filename: str, description: str = "", public: bool = True) -> Tuple[int, str, str]:
        """Create a gist"""
        cmd = f"gh gist create {filename}"
        if description:
            cmd += f" --desc '{description}'"
        if not public:
            cmd += " --secret"
        return self.executor.execute_command(cmd)

    def list_gists(self, limit: int = 30) -> Tuple[int, str, str]:
        """List gists"""
        cmd = f"gh gist list --limit {limit}"
        return self.executor.execute_command(cmd)

    # SSH Operations
    def list_ssh_keys(self) -> Tuple[int, str, str]:
        """List SSH keys"""
        cmd = "gh ssh-key list"
        return self.executor.execute_command(cmd)

    def add_ssh_key(self, key_file: str, title: str = "") -> Tuple[int, str, str]:
        """Add SSH key"""
        cmd = f"gh ssh-key add {key_file}"
        if title:
            cmd += f" --title '{title}'"
        return self.executor.execute_command(cmd)

    def clone_repo(self, repo_url: str, destination: str = "") -> Tuple[int, str, str]:
        """Clone a repository using SSH"""
        # Convert HTTPS URL to SSH if needed
        ssh_url = self._convert_to_ssh_url(repo_url)
        cmd = f"git clone {ssh_url}"
        if destination:
            cmd += f" {destination}"
        return self._execute_git_with_ssh_auth(cmd)

    def _convert_to_ssh_url(self, url: str) -> str:
        """Convert HTTPS GitHub URL to SSH format"""
        if url.startswith("https://github.com/"):
            # Convert https://github.com/user/repo.git to git@github.com:user/repo.git
            path = url.replace("https://github.com/", "")
            if not path.endswith(".git"):
                path += ".git"
            return f"git@github.com:{path}"
        elif url.startswith("http://github.com/"):
            # Convert http://github.com/user/repo.git to git@github.com:user/repo.git
            path = url.replace("http://github.com/", "")
            if not path.endswith(".git"):
                path += ".git"
            return f"git@github.com:{path}"
        else:
            # Already SSH or other format, return as-is
            return url

    def _ensure_ssh_remote(self, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Ensure git remote origin is using SSH instead of HTTPS"""
        try:
            # Get current remote URL
            result = subprocess.run(
                ["git", "remote", "get-url", "origin"],
                cwd=cwd,
                capture_output=True,
                text=True,
                timeout=10
            )

            if result.returncode == 0:
                current_url = result.stdout.strip()

                # If it's HTTPS, convert to SSH
                if current_url.startswith("https://github.com/") or current_url.startswith("http://github.com/"):
                    ssh_url = self._convert_to_ssh_url(current_url)

                    # Set the new SSH URL
                    set_result = subprocess.run(
                        ["git", "remote", "set-url", "origin", ssh_url],
                        cwd=cwd,
                        capture_output=True,
                        text=True,
                        timeout=10
                    )

                    if set_result.returncode == 0:
                        return 0, f"Converted remote URL to SSH: {ssh_url}", ""
                    else:
                        return set_result.returncode, set_result.stdout, set_result.stderr
                else:
                    return 0, f"Remote already using SSH: {current_url}", ""
            else:
                return result.returncode, result.stdout, result.stderr

        except Exception as e:
            return 1, "", f"Error ensuring SSH remote: {str(e)}"

    def convert_remote_to_ssh(self, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Manually convert git remote from HTTPS to SSH"""
        return self._ensure_ssh_remote(cwd)

    # Git Operations
    def git_status(self, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Get git status"""
        cmd = "git status"
        return self.executor.execute_command(cmd, cwd)

    def git_add_all(self, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Add all changes to staging"""
        cmd = "git add ."
        return self.executor.execute_command(cmd, cwd)

    def git_commit(self, message: str, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Commit changes with message"""
        cmd = f'git commit -m "{message}"'
        return self.executor.execute_command(cmd, cwd)

    def git_pull(self, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Pull changes from remote with SSH authentication support"""
        # Ensure SSH remote before pulling
        self._ensure_ssh_remote(cwd)
        cmd = "git pull"
        return self._execute_git_with_ssh_auth(cmd, cwd)

    def git_push(self, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Push changes to remote with SSH authentication support"""
        # Ensure SSH remote before pushing
        self._ensure_ssh_remote(cwd)
        cmd = "git push"
        return self._execute_git_with_ssh_auth(cmd, cwd)


    def git_commit_and_push(self, message: str, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Add all, commit, and push in one operation"""
        # First add all changes
        add_result = self.git_add_all(cwd)
        if add_result[0] != 0:
            return add_result

        # Then commit
        commit_result = self.git_commit(message, cwd)
        if commit_result[0] != 0:
            return commit_result

        # Finally push
        return self.git_push(cwd)

    # Authentication
    def auth_status(self) -> Tuple[int, str, str]:
        """Check authentication status"""
        cmd = "gh auth status"
        return self.executor.execute_command(cmd)

    def auth_login(self) -> Tuple[int, str, str]:
        """Login to GitHub"""
        cmd = "gh auth login"
        return self.executor.execute_command(cmd)

    def auth_logout(self) -> Tuple[int, str, str]:
        """Logout from GitHub"""
        cmd = "gh auth logout"
        return self.executor.execute_command(cmd)

    def _execute_ssh_with_gui_passphrase(self, cmd: str, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Execute SSH command with GUI passphrase prompt support"""
        try:
            import tempfile
            import stat
            import re

            # Create expect script that will handle passphrase prompts
            expect_script = f'''#!/usr/bin/expect -f
set timeout 30
spawn {cmd}
expect {{
    "Enter passphrase for key*:" {{
        puts "PASSPHRASE_PROMPT:[lindex $expect_out(0,string) end-1]"
        interact
    }}
    "Permission denied*" {{
        puts "PERMISSION_DENIED"
        exit 1
    }}
    "Hi*" {{
        puts $expect_out(buffer)
        expect eof
        exit 0
    }}
    timeout {{
        puts "TIMEOUT"
        exit 1
    }}
    eof {{
        exit 0
    }}
}}
'''

            with tempfile.NamedTemporaryFile(mode='w', suffix='.exp', delete=False) as f:
                f.write(expect_script)
                script_path = f.name

            os.chmod(script_path, stat.S_IRWXU)

            try:
                # First try without interaction to see if passphrase is needed
                result = subprocess.run(
                    ["expect", script_path],
                    cwd=cwd,
                    capture_output=True,
                    text=True,
                    timeout=60,
                    input="\n"  # Send enter to trigger any prompts
                )

                output = result.stdout + result.stderr

                # Check if passphrase was requested
                if "Enter passphrase for key" in output:
                    # Extract key path from the output
                    match = re.search(r"Enter passphrase for key '([^']+)':", output)
                    key_path = match.group(1) if match else "SSH key"

                    # Show GUI passphrase dialog on main thread
                    if self.ssh_manager is None or self.root is None:
                        return 1, "", "SSH manager or GUI root not available"

                    passphrase_result: Dict[str, Optional[str]] = {"passphrase": None}
                    def prompt_passphrase():
                        if self.ssh_manager is not None and self.root is not None:
                            result: Optional[str] = self.ssh_manager.prompt_for_ssh_passphrase(key_path, self.root)
                            if result is not None:
                                passphrase_result["passphrase"] = result

                    while passphrase_result.get("passphrase") is None:
                        time.sleep(0.1)

                    passphrase = passphrase_result.get("passphrase")
                    if passphrase is None:
                        return 1, "", "Passphrase entry cancelled"

                    # Create new expect script with the passphrase
                    expect_with_passphrase = f'''#!/usr/bin/expect -f
set timeout 30
spawn {cmd}
expect {{
    "Enter passphrase for key*:" {{
        send "{passphrase}\\r"
        exp_continue
    }}
    "Permission denied*" {{
        exit 1
    }}
    "Hi*" {{
        puts $expect_out(buffer)
        expect eof
        exit 0
    }}
    timeout {{
        exit 1
    }}
    eof {{
        exit 0
    }}
}}
'''

                    with tempfile.NamedTemporaryFile(mode='w', suffix='.exp', delete=False) as f2:
                        f2.write(expect_with_passphrase)
                        script_path_2 = f2.name

                    os.chmod(script_path_2, stat.S_IRWXU)

                    # Execute with passphrase
                    final_result = subprocess.run(
                        ["expect", script_path_2],
                        cwd=cwd,
                        capture_output=True,
                        text=True,
                        timeout=60
                    )

                    os.unlink(script_path_2)
                    os.unlink(script_path)

                    return final_result.returncode, final_result.stdout, final_result.stderr

                else:
                    # No passphrase needed, return original result
                    os.unlink(script_path)
                    return result.returncode, result.stdout, result.stderr

            except subprocess.TimeoutExpired:
                return 1, "", "SSH command timed out"
            except FileNotFoundError:
                # expect not available, fall back to basic execution
                return self.command_executor.execute_command(cmd, cwd)
            finally:
                try:
                    os.unlink(script_path)
                except:
                    pass

        except Exception as e:
            return 1, "", f"SSH command error: {str(e)}"

    def _convert_to_https_auth(self, cmd: str, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Convert SSH command to HTTPS authentication as fallback"""
        try:
            # Simple fallback to basic git command execution
            return self.command_executor.execute_command(cmd, cwd)
        except Exception as e:
            return 1, "", f"HTTPS auth fallback error: {str(e)}"

class CommandExecutor:
    """Enterprise-grade command execution with real-time output streaming."""

    def __init__(self, output_callback: Optional[Callable[[str], None]] = None):
        self.output_callback = output_callback
        self.process: Optional[subprocess.Popen] = None
        self.is_running = False
        self._lock = threading.Lock()
        self.current_crate_checker: Optional[CrateChecker] = None

    def execute_command(self, command: str, cwd: Optional[str] = None, shell: bool = True) -> Tuple[int, str, str]:
        """Execute a command and return (returncode, stdout, stderr)"""
        try:
            if self.output_callback:
                self.output_callback(f"$ {command}\n")

            self.process = subprocess.Popen(
                command,
                shell=shell,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                cwd=cwd,  # cwd can be None, subprocess handles this correctly
                universal_newlines=True
            )

            stdout, stderr = self.process.communicate()
            returncode = self.process.returncode

            if self.output_callback:
                if stdout:
                    self.output_callback(stdout)
                if stderr:
                    self.output_callback(f"ERROR: {stderr}")

            return returncode, stdout, stderr

        except Exception as e:
            error_msg = f"Command execution failed: {str(e)}"
            if self.output_callback:
                self.output_callback(f"EXCEPTION: {error_msg}\n")
            return 1, "", error_msg

    def execute_async(self, command: str, shell_type: str = "auto", # Renamed 'shell' to 'shell_type' to avoid conflict
                     working_dir: Optional[str] = None) -> threading.Thread:
        """Execute command asynchronously with real-time output."""
        def run_command():
            with self._lock:
                self.is_running = True

            original_dir = None
            try:
                # Store original directory and change if needed
                if working_dir is not None:
                    original_dir = os.getcwd()
                    if not os.path.exists(working_dir):
                        raise FileNotFoundError(f"Working directory does not exist: {working_dir}")
                    os.chdir(working_dir)

                # Determine shell based on platform and shell_type
                cmd_list: List[str]
                use_shell = False
                if shell_type == "pwsh" or (shell_type == "auto" and platform.system() == "Windows" and command.endswith(".ps1")):
                    cmd_list = ["pwsh", "-Command", command]
                elif shell_type == "cmd" or (shell_type == "auto" and platform.system() == "Windows"):
                    cmd_list = ["cmd", "/c", command]
                elif shell_type == "bash" or (shell_type == "auto" and platform.system() != "Windows"):
                    cmd_list = ["bash", "-c", command]
                else: # Default to shell=False for direct execution if no specific shell is implied
                    cmd_list = command.split() # Simple split, might need more robust parsing for complex commands
                    use_shell = True # Revert to shell=True if custom shell behavior is implied or not auto-detected

                # Execute command with proper error handling
                self.process = subprocess.Popen(
                    cmd_list if not use_shell else command, # Pass list or string based on use_shell
                    shell=use_shell,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.STDOUT, # Redirect stderr to stdout for combined output
                    text=True,
                    bufsize=1,
                    universal_newlines=True,
                    creationflags=subprocess.CREATE_NEW_PROCESS_GROUP if platform.system() == "Windows" else 0
                )

                # Ensure stdout is not None before reading
                if self.process.stdout is not None:
                    # Stream output with proper error handling
                    for line in iter(self.process.stdout.readline, ''):
                        if not line:
                            break
                        if self.output_callback:
                            try:
                                self.output_callback(line)
                            except Exception as e:
                                logger.error(f"Error in output callback: {e}")

                    # Close stdout
                    self.process.stdout.close()

                # Wait for process completion
                return_code = self.process.wait()

                # Final status with comprehensive feedback
                if self.output_callback:
                    try:
                        if return_code == 0:
                            self.output_callback(f"\n✅ Command completed successfully (exit code: 0)\n")
                        else:
                            self.output_callback(f"\n❌ Command failed (exit code: {return_code})\n")
                    except Exception as e:
                        logger.error(f"Error in final status callback: {e}")

            except FileNotFoundError as e:
                error_msg = f"\n💥 Command not found: {e}\n"
                logger.error(error_msg)
                if self.output_callback:
                    try:
                        self.output_callback(error_msg)
                    except Exception as callback_error:
                        logger.error(f"Error in error callback: {callback_error}")

            except PermissionError as e:
                error_msg = f"\n🚫 Permission denied: {e}\n"
                logger.error(error_msg)
                if self.output_callback:
                    try:
                        self.output_callback(error_msg)
                    except Exception as callback_error:
                        logger.error(f"Error in error callback: {callback_error}")

            except Exception as e:
                error_msg = f"\n💥 Execution error: {str(e)}\n"
                logger.error(error_msg)
                if self.output_callback:
                    try:
                        self.output_callback(error_msg)
                    except Exception as callback_error:
                        logger.error(f"Error in error callback: {callback_error}")

            finally:
                # Restore original directory
                if original_dir is not None:
                    try:
                        os.chdir(original_dir)
                    except Exception as e:
                        logger.error(f"Failed to restore original directory: {e}")

                with self._lock:
                    self.is_running = False

        thread = threading.Thread(target=run_command, daemon=True)
        thread.start()
        return thread

    def execute_crate_check_async(self, working_dir: Optional[str] = None) -> threading.Thread:
        """Execute comprehensive crate validation asynchronously."""
        def run_crate_check():
            with self._lock:
                self.is_running = True
                self.current_crate_checker = CrateChecker(
                    output_callback=self.output_callback,
                    working_dir=working_dir
                )

            try:
                success = self.current_crate_checker.run_all_checks()

                if self.output_callback:
                    try:
                        if success:
                            self.output_callback(f"\n🎉 Crate validation completed successfully!\n")
                        else:
                            self.output_callback(f"\n⚠️ Crate validation completed with issues. See details above.\n")
                    except Exception as e:
                        logger.error(f"Error in final validation callback: {e}")

            except Exception as e:
                error_msg = f"\n💥 Crate validation error: {str(e)}\n"
                logger.error(error_msg)
                if self.output_callback:
                    try:
                        self.output_callback(error_msg)
                    except Exception as callback_error:
                        logger.error(f"Error in error callback: {callback_error}")

            finally:
                with self._lock:
                    self.is_running = False
                    self.current_crate_checker = None

        thread = threading.Thread(target=run_crate_check, daemon=True)
        thread.start()
        return thread

    def terminate(self) -> None:
        """Terminate running process with proper cleanup."""
        with self._lock:
            # Cancel crate checker if running
            if self.current_crate_checker:
                self.current_crate_checker.cancel()

            if self.process and self.is_running:
                try:
                    if platform.system() == "Windows":
                        # Windows-specific termination
                        self.process.send_signal(signal.CTRL_BREAK_EVENT)
                    else:
                        # Unix-like systems
                        self.process.terminate()

                    # Wait for graceful termination
                    try:
                        self.process.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        # Force kill if graceful termination fails
                        self.process.kill()
                        self.process.wait()

                except Exception as e:
                    logger.error(f"Error terminating process: {e}")
                    try:
                        self.process.kill()
                    except:
                        pass
                finally:
                    self.is_running = False

class RetractableOverlay:
    """Retractable overlay panel with smooth animations and error handling."""

    def __init__(self, parent: tk.Tk, width: int = 400, height: int = 600):
        self.parent = parent
        self.width = width
        self.height = height
        self.is_visible = False
        self.animation_speed = 10
        self._animation_lock = threading.Lock()

        try:
            # Create overlay frame with proper error handling
            self.overlay = tk.Toplevel(parent)
            self.overlay.overrideredirect(True)
            self.overlay.configure(bg=ArcMoonTheme.OVERLAY_BG)
            self.overlay.attributes('-alpha', 0.95)
            self.overlay.attributes('-topmost', True)

            # Position overlay with screen boundary validation
            screen_width = parent.winfo_screenwidth()
            screen_height = parent.winfo_screenheight()

            # Ensure overlay fits on screen
            if width > screen_width:
                width = screen_width - 50
            if height > screen_height:
                height = screen_height - 50

            self.x_hidden = screen_width
            self.x_visible = screen_width - width
            self.y_pos = max(0, (screen_height - height) // 2)

            self.overlay.geometry(f"{width}x{height}+{self.x_hidden}+{self.y_pos}")

            # Create main panel with error handling
            self.panel = tk.Frame(self.overlay,
                                 bg=ArcMoonTheme.OVERLAY_PANEL,
                                 relief='flat',
                                 bd=2)
            self.panel.pack(fill='both', expand=True, padx=2, pady=2)

            # Hide initially
            self.overlay.withdraw()

        except Exception as e:
            logger.error(f"Failed to create overlay: {e}")
            raise

    def show(self) -> None:
        """Show overlay with slide-in animation."""
        if not self.is_visible and not self._animation_lock.locked():
            try:
                self.overlay.deiconify()
                self._animate_slide_in()
            except Exception as e:
                logger.error(f"Error showing overlay: {e}")

    def hide(self) -> None:
        """Hide overlay with slide-out animation."""
        if self.is_visible and not self._animation_lock.locked():
            try:
                self._animate_slide_out()
            except Exception as e:
                logger.error(f"Error hiding overlay: {e}")

    def toggle(self) -> None:
        """Toggle overlay visibility."""
        if self.is_visible:
            self.hide()
        else:
            self.show()

    def _animate_slide_in(self) -> None:
        """Animate slide-in effect with thread safety."""
        with self._animation_lock:
            current_x = self.x_hidden
            target_x = self.x_visible

            def animate():
                nonlocal current_x
                try:
                    if current_x > target_x:
                        current_x -= self.animation_speed
                        self.overlay.geometry(f"{self.width}x{self.height}+{current_x}+{self.y_pos}")
                        self.overlay.after(1, animate)
                    else:
                        self.is_visible = True
                        self.overlay.geometry(f"{self.width}x{self.height}+{target_x}+{self.y_pos}")
                except Exception as e:
                    logger.error(f"Animation error: {e}")
                    self.is_visible = True

            animate()

    def _animate_slide_out(self) -> None:
        """Animate slide-out effect with thread safety."""
        with self._animation_lock:
            current_x = self.x_visible
            target_x = self.x_hidden

            def animate():
                nonlocal current_x
                try:
                    if current_x < target_x:
                        current_x += self.animation_speed
                        self.overlay.geometry(f"{self.width}x{self.height}+{current_x}+{self.y_pos}")
                        self.overlay.after(1, animate)
                    else:
                        self.is_visible = False
                        self.overlay.withdraw()
                except Exception as e:
                    logger.error(f"Animation error: {e}")
                    self.is_visible = False
                    try:
                        self.overlay.withdraw()
                    except:
                        pass

            animate()

class ArcMoonSystemGUI:
    """ArcMoon Studios Enterprise GUI Control Panel with comprehensive error handling."""

    def __init__(self):
        try:
            self.root = tk.Tk()
            self.root.title("🌙 ArcMoon Studios Enterprise Control Panel")
            self.root.configure(bg=ArcMoonTheme.DARK_BG)

            # Configure window with error handling
            self.root.geometry("1600x1000")
            self.root.minsize(1200, 800)

            # Configure styles
            ArcMoonStyles.configure_styles()

            # Load configuration
            self.config = AMSConfig.load()

            # Initialize SSH password manager
            self.ssh_manager = SSHPasswordManager()            # Initialize variables with proper defaults
            self.workspace_path = self._detect_workspace()
            self.git_status_var = tk.StringVar(value="Ready")
            self.upgrade_enabled_var = tk.BooleanVar(value=False)
            self.custom_command_var = tk.StringVar(value="")

            # Command queue for terminal output
            self.command_queue = queue.Queue()

            # Initialize command executor and GitHub operations
            self.command_executor = CommandExecutor(self._append_output_queued)
            self.github_ops = GitHubOperations(self.command_executor)

            # Set up SSH manager and root references
            if hasattr(self, 'ssh_manager'):
                self.github_ops.ssh_manager = self.ssh_manager
                self.github_ops.root = self.root

            self.current_status = "Ready"
            self.overlay: Optional[RetractableOverlay] = None

            # Initialize UI components
            self._create_main_interface()
            self._create_retractable_overlay()

            # Initialize status
            self._update_status_display()

            # Start terminal output processor
            self._process_queue()

            # Bind events with error handling
            self.root.protocol("WM_DELETE_WINDOW", self._on_closing)

        except Exception as e:
            logger.error(f"Failed to initialize GUI: {e}")
            raise

    def _detect_workspace(self) -> str:
        """Detect workspace path automatically with comprehensive error handling."""
        try:
            # Use saved workspace path if available
            if self.config.workspace_path and os.path.exists(self.config.workspace_path):
                return self.config.workspace_path

            current_dir = Path.cwd()

            # Look for Cargo.toml in current directory or parents
            for path in [current_dir] + list(current_dir.parents):
                cargo_toml = path / "Cargo.toml"
                if cargo_toml.exists() and cargo_toml.is_file():
                    workspace = str(path)
                    self.config.workspace_path = workspace
                    self.config.save()
                    return workspace

            # Default to current directory if no Cargo.toml found
            workspace = str(current_dir)
            self.config.workspace_path = workspace
            self.config.save()
            return workspace

        except Exception as e:
            logger.warning(f"Error detecting workspace: {e}")
            # Fallback to home directory if current directory is inaccessible
            try:
                return str(Path.home())
            except Exception as fallback_error:
                logger.error(f"Fallback workspace detection failed: {fallback_error}")
                return "."

    def _create_main_interface(self) -> None:
        """Create the main GUI interface with comprehensive error handling."""
        try:
            # Main container
            main_frame = ttk.Frame(self.root, style='ArcMoon.TFrame')
            main_frame.pack(fill='both', expand=True, padx=10, pady=10)

            # Create interface components
            self._create_header(main_frame)
            self._create_tabbed_interface(main_frame)
            self._create_status_bar(main_frame)

        except Exception as e:
            logger.error(f"Failed to create main interface: {e}")
            raise

    def _create_header(self, parent: ttk.Frame) -> None:
        """Create application header with error handling."""
        try:
            header_frame = ttk.Frame(parent, style='ArcMoon.TFrame')
            header_frame.pack(fill='x', pady=(0, 20))

            # Title
            title_label = ttk.Label(header_frame,
                                   text="🌙 ArcMoon Studios Enterprise Control Panel",
                                   style='ArcMoonTitle.TLabel')
            title_label.pack(side='left')

            # Control buttons frame
            controls_frame = ttk.Frame(header_frame, style='ArcMoon.TFrame')
            controls_frame.pack(side='right')

            # Stop button for cancelling operations
            self.stop_btn = ttk.Button(controls_frame,
                                      text="🛑 Stop",
                                      style='ArcMoonDanger.TButton',
                                      command=self._stop_current_operation,
                                      state='disabled')
            self.stop_btn.pack(side='right', padx=(5, 0))

            # Settings button
            settings_btn = ttk.Button(controls_frame,
                                     text="⚙️ Settings",
                                     style='ArcMoonSecondary.TButton',
                                     command=self._open_settings)
            settings_btn.pack(side='right', padx=(5, 0))

            # Overlay toggle button
            overlay_btn = ttk.Button(controls_frame,
                                    text="🔧 Advanced",
                                    style='ArcMoonSecondary.TButton',
                                    command=self._toggle_overlay)
            overlay_btn.pack(side='right', padx=(5, 0))

            # Subtitle
            subtitle_label = ttk.Label(header_frame,
                                      text="Enterprise Development • Mathematical Precision • Quality Assurance • GitHub Integration",
                                      style='ArcMoonSubtitle.TLabel')
            subtitle_label.pack(side='left', padx=(20, 0))

        except Exception as e:
            logger.error(f"Failed to create header: {e}")

    def _create_tabbed_interface(self, parent: ttk.Frame) -> None:
        """Create the main tabbed interface."""
        try:
            # Create main paned window for tabs and terminal
            main_paned = ttk.PanedWindow(parent, orient=tk.HORIZONTAL)
            main_paned.pack(fill='both', expand=True)

            # Left side - tabbed operations
            left_frame = ttk.Frame(main_paned)
            main_paned.add(left_frame, weight=1)

            # Create notebook for tabs
            self.notebook = ttk.Notebook(left_frame)
            self.notebook.pack(fill='both', expand=True, padx=(0, 5))

            # Create tabs
            self._create_rust_tab()
            self._create_github_tab()
            self._create_tools_tab()

            # Right side - terminal output
            right_frame = ttk.Frame(main_paned)
            main_paned.add(right_frame, weight=1)

            self._create_terminal_panel(right_frame)

        except Exception as e:
            logger.error(f"Failed to create tabbed interface: {e}")
            raise

    def _create_rust_tab(self) -> None:
        """Create the Rust development tab."""
        try:
            rust_frame = ttk.Frame(self.notebook, style='TabContent.TFrame')
            self.notebook.add(rust_frame, text="🦀 Rust")
              # Workspace section
            workspace_frame = ttk.LabelFrame(rust_frame, text="📁 Workspace", style='Workspace.TLabelFrame')
            workspace_frame.pack(fill='x', padx=5, pady=5)

            path_frame = tk.Frame(workspace_frame, bg=ArcMoonTheme.WORKSPACE_BG)
            path_frame.pack(fill='x', padx=5, pady=5)

            ttk.Label(path_frame, text="Path:", style='ArcMoon.TLabel').pack(side='left')

            self.path_var = tk.StringVar(value=self.workspace_path)
            path_entry = tk.Entry(path_frame, textvariable=self.path_var,
                                 bg=ArcMoonTheme.DARK_TERTIARY,
                                 fg=ArcMoonTheme.TEXT_PRIMARY,
                                 insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON,
                                 relief='flat', bd=5)
            path_entry.pack(side='left', fill='x', expand=True, padx=(10, 0))

            browse_btn = ttk.Button(path_frame, text="📂", command=self._browse_workspace)
            browse_btn.pack(side='right', padx=(5, 0))

            # Git status
            git_status_label = ttk.Label(workspace_frame, textvariable=self.git_status_var, style='ArcMoon.TLabel')
            git_status_label.pack(pady=5)

            refresh_btn = ttk.Button(workspace_frame, text="🔄 Refresh Status", command=self._refresh_status)
            refresh_btn.pack(pady=5)

            # Quick Actions section
            actions_frame = ttk.LabelFrame(rust_frame, text="🚀 Quick Actions", style='Workspace.TLabelFrame')
            actions_frame.pack(fill='x', padx=5, pady=5)
              # Action buttons grid with alternating colors (Set A)
            buttons_frame = tk.Frame(actions_frame, bg=ArcMoonTheme.WORKSPACE_BG)
            buttons_frame.pack(fill='x', padx=5, pady=5)

            quick_buttons = [
                ("🔍 Check", self._run_standard_check),
                ("🎯 Full Validation", self._run_full_crate_validation),
                ("⬆️ Upgrade", self._run_upgrade),
                ("🧪 Test", self._run_tests),
            ]

            for i, (text, command) in enumerate(quick_buttons):
                # Map to ArcMoon button styles
                style_map = ['ArcMoon.TButton', 'ArcMoonSecondary.TButton', 'ArcMoonSuccess.TButton', 'ArcMoonWarning.TButton']
                style = style_map[i % len(style_map)]

                btn = ttk.Button(buttons_frame, text=text, command=command, style=style)
                btn.grid(row=i//2, column=i%2, padx=5, pady=5, sticky='ew')

            buttons_frame.columnconfigure(0, weight=1)
            buttons_frame.columnconfigure(1, weight=1)            # Quality Tools section
            quality_frame = ttk.LabelFrame(rust_frame, text="🔧 Quality Tools", style='Workspace.TLabelFrame')
            quality_frame.pack(fill='x', padx=5, pady=5)

            quality_buttons_frame = tk.Frame(quality_frame, bg=ArcMoonTheme.WORKSPACE_BG)
            quality_buttons_frame.pack(fill='x', padx=5, pady=5)

            quality_buttons = [
                ("📝 Format", self._run_format),
                ("📎 Clippy", self._run_clippy),
                ("🔒 Audit", self._run_audit),
                ("⚡ Bench", self._run_benchmarks),
            ]

            for i, (text, command) in enumerate(quality_buttons):
                # Map to ArcMoon button styles
                style_map = ['ArcMoonSecondary.TButton', 'ArcMoon.TButton', 'ArcMoonSuccess.TButton', 'ArcMoonWarning.TButton']
                style = style_map[i % len(style_map)]

                btn = ttk.Button(quality_buttons_frame, text=text, command=command, style=style)
                btn.grid(row=i//2, column=i%2, padx=5, pady=5, sticky='ew')

            quality_buttons_frame.columnconfigure(0, weight=1)
            quality_buttons_frame.columnconfigure(1, weight=1)

        except Exception as e:
            logger.error(f"Failed to create Rust tab: {e}")

    def _create_github_tab(self) -> None:
        """Create the GitHub operations tab."""
        try:
            github_frame = ttk.Frame(self.notebook, style='TabContent.TFrame')
            self.notebook.add(github_frame, text="🐙 GitHub")

            # Create scrollable frame
            canvas = tk.Canvas(github_frame, bg=ArcMoonTheme.DARK_SECONDARY, highlightthickness=0)
            scrollbar = ttk.Scrollbar(github_frame, orient="vertical", command=canvas.yview)
            scrollable_frame = ttk.Frame(canvas, style='TabContent.TFrame')

            scrollable_frame.bind(
                "<Configure>",
                lambda e: canvas.configure(scrollregion=canvas.bbox("all"))
            )

            canvas.create_window((0, 0), window=scrollable_frame, anchor="nw")
            canvas.configure(yscrollcommand=scrollbar.set)

            canvas.pack(side="left", fill="both", expand=True)
            scrollbar.pack(side="right", fill="y")

            # Authentication section
            auth_frame = ttk.LabelFrame(scrollable_frame, text="🔐 Authentication", style='Workspace.TLabelFrame')
            auth_frame.pack(fill='x', padx=5, pady=5)

            auth_buttons = ttk.Frame(auth_frame)
            auth_buttons.pack(fill='x', padx=5, pady=5)

            ttk.Button(auth_buttons, text="Check Status",
                      command=lambda: self._execute_github_async(self.github_ops.auth_status)).pack(side='left', padx=5)

            ttk.Button(auth_buttons, text="Login",
                      command=lambda: self._execute_github_async(self.github_ops.auth_login)).pack(side='left', padx=5)

            ttk.Button(auth_buttons, text="Logout",
                      command=lambda: self._execute_github_async(self.github_ops.auth_logout)).pack(side='left', padx=5)

            # Git Operations section
            git_frame = ttk.LabelFrame(scrollable_frame, text="📝 Git Operations", style='Workspace.TLabelFrame')
            git_frame.pack(fill='x', padx=5, pady=5)

            # Git status
            git_status_frame = ttk.Frame(git_frame)
            git_status_frame.pack(fill='x', padx=5, pady=5)

            ttk.Button(git_status_frame, text="📊 Git Status",
                      command=lambda: self._execute_github_async(self.github_ops.git_status, self.workspace_path)).pack(side='left', padx=5)

            ttk.Button(git_status_frame, text="⬇️ Pull",
                      command=lambda: self._execute_github_async(self.github_ops.git_pull, self.workspace_path)).pack(side='left', padx=5)

            ttk.Button(git_status_frame, text="🔐 Use SSH",
                      command=lambda: self._execute_github_async(self.github_ops.convert_remote_to_ssh, self.workspace_path)).pack(side='left', padx=5)

            # Commit section
            commit_section = ttk.Frame(git_frame)
            commit_section.pack(fill='x', padx=5, pady=5)

            ttk.Label(commit_section, text="Commit Message:").pack(anchor='w')
            self.commit_message_var = tk.StringVar()
            commit_entry = tk.Entry(commit_section, textvariable=self.commit_message_var,
                                   bg=ArcMoonTheme.DARK_TERTIARY,
                                   fg=ArcMoonTheme.TEXT_PRIMARY,
                                   insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON,
                                   relief='flat', bd=5, width=50)
            commit_entry.pack(fill='x', pady=2)
            commit_entry.bind('<Return>', lambda event: self._commit_and_push())

            # Commit buttons
            commit_buttons = ttk.Frame(commit_section)
            commit_buttons.pack(fill='x', pady=5)

            ttk.Button(commit_buttons, text="➕ Add All",
                      command=lambda: self._execute_github_async(self.github_ops.git_add_all, self.workspace_path)).pack(side='left', padx=5)

            ttk.Button(commit_buttons, text="💾 Commit",
                      command=self._commit_changes).pack(side='left', padx=5)

            ttk.Button(commit_buttons, text="⬆️ Push",
                      command=lambda: self._execute_github_async(self.github_ops.git_push, self.workspace_path)).pack(side='left', padx=5)

            ttk.Button(commit_buttons, text="🚀 Commit & Push",
                      command=self._commit_and_push).pack(side='left', padx=5)

            # Quick commit templates
            quick_commits = ttk.Frame(commit_section)
            quick_commits.pack(fill='x', pady=5)

            ttk.Label(quick_commits, text="Quick Templates:").pack(anchor='w')

            templates = [
                "🎉 Initial commit",
                "✨ Add new feature",
                "🐛 Fix bug",
                "📝 Update documentation",
                "🔧 Update configuration",
                "♻️ Refactor code"
            ]

            for template in templates:
                ttk.Button(quick_commits, text=template,
                          command=lambda t=template: self.commit_message_var.set(t)).pack(side='left', padx=2)

            # Repository section
            repo_frame = ttk.LabelFrame(scrollable_frame, text="📁 Repository Operations", style='Workspace.TLabelFrame')
            repo_frame.pack(fill='x', padx=5, pady=5)

            # Clone section
            clone_section = ttk.Frame(repo_frame)
            clone_section.pack(fill='x', padx=5, pady=5)

            ttk.Label(clone_section, text="Clone Repository:").pack(anchor='w')
            self.clone_url_var = tk.StringVar()
            ttk.Entry(clone_section, textvariable=self.clone_url_var, width=50).pack(fill='x', pady=2)

            ttk.Label(clone_section, text="Destination (optional):").pack(anchor='w')
            dest_frame = ttk.Frame(clone_section)
            dest_frame.pack(fill='x', pady=2)

            self.clone_dest_var = tk.StringVar()
            ttk.Entry(dest_frame, textvariable=self.clone_dest_var).pack(side='left', fill='x', expand=True)
            ttk.Button(dest_frame, text="Browse", command=self._browse_clone_destination).pack(side='right', padx=(5, 0))

            ttk.Button(clone_section, text="Clone Repository",
                      command=self._clone_repo_command).pack(pady=5)

            # Create repository section
            create_section = ttk.Frame(repo_frame)
            create_section.pack(fill='x', padx=5, pady=5)

            ttk.Label(create_section, text="Create Repository:").pack(anchor='w')
            self.create_name_var = tk.StringVar()
            ttk.Entry(create_section, textvariable=self.create_name_var, width=50).pack(fill='x', pady=2)

            ttk.Label(create_section, text="Description:").pack(anchor='w')
            self.create_desc_var = tk.StringVar()
            ttk.Entry(create_section, textvariable=self.create_desc_var, width=50).pack(fill='x', pady=2)

            self.create_private_var = tk.BooleanVar()
            ttk.Checkbutton(create_section, text="Private Repository",
                           variable=self.create_private_var).pack(anchor='w')

            ttk.Button(create_section, text="Create Repository",
                      command=self._create_repo_command).pack(pady=5)

            # List repositories
            list_section = ttk.Frame(repo_frame)
            list_section.pack(fill='x', padx=5, pady=5)

            list_controls = ttk.Frame(list_section)
            list_controls.pack(fill='x')

            ttk.Label(list_controls, text="User (optional):").pack(side='left')
            self.list_user_var = tk.StringVar()
            ttk.Entry(list_controls, textvariable=self.list_user_var, width=20).pack(side='left', padx=5)

            ttk.Button(list_controls, text="List Repositories",
                      command=self._list_repos_command).pack(side='right')

            # Issues section
            issues_frame = ttk.LabelFrame(scrollable_frame, text="🐛 Issues", style='Workspace.TLabelFrame')
            issues_frame.pack(fill='x', padx=5, pady=5)

            ttk.Label(issues_frame, text="Create Issue:").pack(anchor='w')
            self.issue_title_var = tk.StringVar()
            ttk.Entry(issues_frame, textvariable=self.issue_title_var, width=50).pack(fill='x', padx=5, pady=2)

            ttk.Label(issues_frame, text="Body:").pack(anchor='w')
            self.issue_body_text = tk.Text(issues_frame, height=3, width=50)
            self.issue_body_text.pack(fill='x', padx=5, pady=2)

            issue_buttons = ttk.Frame(issues_frame)
            issue_buttons.pack(fill='x', padx=5, pady=5)

            ttk.Button(issue_buttons, text="Create Issue",
                      command=self._create_issue_command).pack(side='left', padx=5)

            ttk.Button(issue_buttons, text="List Issues",
                      command=lambda: self._execute_github_async(self.github_ops.list_issues)).pack(side='left', padx=5)

            # SSH Keys section
            ssh_frame = ttk.LabelFrame(scrollable_frame, text="🔑 SSH Keys", style='Workspace.TLabelFrame')
            ssh_frame.pack(fill='x', padx=5, pady=5)

            ttk.Button(ssh_frame, text="List SSH Keys",
                      command=lambda: self._execute_github_async(self.github_ops.list_ssh_keys)).pack(pady=5)

            ssh_add_frame = ttk.Frame(ssh_frame)
            ssh_add_frame.pack(fill='x', padx=5, pady=5)

            ttk.Label(ssh_add_frame, text="Key File:").pack(anchor='w')
            key_file_frame = ttk.Frame(ssh_add_frame)
            key_file_frame.pack(fill='x')

            self.ssh_key_file_var = tk.StringVar()
            ttk.Entry(key_file_frame, textvariable=self.ssh_key_file_var).pack(side='left', fill='x', expand=True)
            ttk.Button(key_file_frame, text="Browse", command=self._browse_ssh_key).pack(side='right', padx=(5, 0))

            ttk.Label(ssh_add_frame, text="Title (optional):").pack(anchor='w')
            self.ssh_key_title_var = tk.StringVar()
            ttk.Entry(ssh_add_frame, textvariable=self.ssh_key_title_var, width=50).pack(fill='x')

            ttk.Button(ssh_add_frame, text="Add SSH Key",
                      command=self._add_ssh_key_command).pack(pady=5)

        except Exception as e:
            logger.error(f"Failed to create GitHub tab: {e}")

    def _create_tools_tab(self) -> None:
        """Create the general tools tab."""
        try:
            tools_frame = ttk.Frame(self.notebook, style='TabContent.TFrame')
            self.notebook.add(tools_frame, text="🛠️ Tools")

            # Command input section
            command_frame = ttk.LabelFrame(tools_frame, text="💻 Command Execution", style='Workspace.TLabelFrame')
            command_frame.pack(fill='x', padx=5, pady=5)

            cmd_input_frame = ttk.Frame(command_frame)
            cmd_input_frame.pack(fill='x', padx=5, pady=5)

            ttk.Label(cmd_input_frame, text="$").pack(side='left')
            self.command_var = tk.StringVar()
            self.command_entry = tk.Entry(cmd_input_frame, textvariable=self.command_var,
                                         bg=ArcMoonTheme.DARK_TERTIARY,
                                         fg=ArcMoonTheme.TEXT_PRIMARY,
                                         insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON,
                                         relief='flat', bd=5)
            self.command_entry.pack(side='left', fill='x', expand=True, padx=5)
            self.command_entry.bind('<Return>', self._execute_command_from_entry)

            ttk.Button(cmd_input_frame, text="Execute",
                      command=self._execute_command_from_entry).pack(side='right')

            # Working directory
            wd_frame = ttk.Frame(command_frame)
            wd_frame.pack(fill='x', padx=5, pady=(0, 5))

            ttk.Label(wd_frame, text="Working Directory:").pack(side='left')
            self.working_dir_var = tk.StringVar(value=self.workspace_path)
            ttk.Label(wd_frame, textvariable=self.working_dir_var, foreground=ArcMoonTheme.LIGHT_BLUE_MOON).pack(side='left', padx=5)
            ttk.Button(wd_frame, text="Change", command=self._change_working_directory).pack(side='right')

            # Utilities section
            utilities_frame = ttk.LabelFrame(tools_frame, text="🔧 Utilities", style='Workspace.TLabelFrame')
            utilities_frame.pack(fill='x', padx=5, pady=5)

            util_buttons = ttk.Frame(utilities_frame)
            util_buttons.pack(fill='x', padx=5, pady=5)
            ttk.Button(util_buttons, text="Generate SSH Key", command=self._generate_ssh_key).pack(side='left', padx=5)
            ttk.Button(util_buttons, text="Test GitHub Connection", command=self._test_github_connection).pack(side='left', padx=5)
            ttk.Button(util_buttons, text="Clear Terminal", command=self._clear_output).pack(side='left', padx=5)

        except Exception as e:
            logger.error(f"Failed to create tools tab: {e}")

    def _create_terminal_panel(self, parent: ttk.Frame) -> None:
        """Create the terminal output panel with bulletproof widget creation."""
        try:
            # Terminal output section - try custom style first, fallback to standard
            try:
                terminal_frame = ttk.LabelFrame(parent, text="📝 Terminal Output", style='Workspace.TLabelFrame')
            except tk.TclError:
                logger.warning("Custom style failed, using fallback for terminal frame")
                terminal_frame = ttk.LabelFrame(parent, text="📝 Terminal Output")

            terminal_frame.pack(fill='both', expand=True, padx=(5, 0), pady=5)

            # Terminal text widget with scrollbar - ensure this always succeeds
            terminal_container = ttk.Frame(terminal_frame)
            terminal_container.pack(fill='both', expand=True, padx=5, pady=5)

            try:
                self.terminal_text = self._create_syntax_highlighted_terminal(terminal_container)
            except Exception as terminal_error:
                logger.error(f"Syntax highlighted terminal failed: {terminal_error}")
                # Fallback to basic terminal if syntax highlighting fails
                self.terminal_text = self._create_fallback_terminal(terminal_container)

            if self.terminal_text:
                self.terminal_text.pack(fill='both', expand=True)
            else:
                # Last resort - create minimal working terminal
                self.terminal_text = tk.Text(terminal_container,
                                          bg='#1a1a1a', fg='#ffffff',
                                          state='disabled', wrap=tk.WORD)
                self.terminal_text.pack(fill='both', expand=True)

            # Terminal controls
            controls_frame = ttk.Frame(terminal_frame)
            controls_frame.pack(fill='x', padx=5, pady=(0, 5))

            ttk.Button(controls_frame, text="🗑️ Clear", command=self._clear_output).pack(side='left')
            ttk.Button(controls_frame, text="📄 Save Log", command=self._save_log).pack(side='left', padx=(5, 0))

        except Exception as e:
            logger.error(f"Failed to create terminal panel: {e}")
            # Ensure terminal_text exists even if everything else fails
            if not hasattr(self, 'terminal_text') or self.terminal_text is None:
                self.terminal_text = tk.Text(parent, state='disabled')
                self.terminal_text.pack(fill='both', expand=True)

    def _create_status_bar(self, parent: ttk.Frame) -> None:
        """Create status bar with error handling."""
        try:
            status_frame = tk.Frame(parent, bg=ArcMoonTheme.DARK_BORDER, height=30)
            status_frame.pack(fill='x', side='bottom')
            status_frame.pack_propagate(False)

            self.status_var = tk.StringVar(value=self.current_status)
            status_label = tk.Label(status_frame,
                                   textvariable=self.status_var,
                                   bg=ArcMoonTheme.DARK_BORDER,
                                   fg=ArcMoonTheme.TEXT_SECONDARY,
                                   font=('Segoe UI', 9))
            status_label.pack(side='left', padx=10, pady=5)

            # Progress indicator
            self.progress_var = tk.DoubleVar()
            self.progress_bar = ttk.Progressbar(status_frame, variable=self.progress_var, mode='indeterminate')
            self.progress_bar.pack(side='right', padx=10, pady=5, fill='x', expand=True)

            # Time display
            self.time_var = tk.StringVar()
            time_label = tk.Label(status_frame,
                                 textvariable=self.time_var,
                                 bg=ArcMoonTheme.DARK_BORDER,
                                 fg=ArcMoonTheme.TEXT_SECONDARY,
                                 font=('Segoe UI', 9))
            time_label.pack(side='right', padx=10, pady=5)

            self._update_time()

        except Exception as e:
            logger.error(f"Failed to create status bar: {e}")

    def _create_retractable_overlay(self) -> None:
        """Create retractable overlay panel with error handling."""
        try:
            self.overlay = RetractableOverlay(self.root, width=500, height=800)

            # Overlay header
            overlay_header = tk.Frame(self.overlay.panel, bg=ArcMoonTheme.DARK_TERTIARY, height=50)
            overlay_header.pack(fill='x')
            overlay_header.pack_propagate(False)

            overlay_title = tk.Label(overlay_header,
                                    text="🔧 Advanced Control Panel",
                                    bg=ArcMoonTheme.DARK_TERTIARY,
                                    fg=ArcMoonTheme.CHERRY_BLOSSOM_PINK, # Using CHERRY_BLOSSOM_PINK directly
                                    font=('Segoe UI', 14, 'bold'))
            overlay_title.pack(side='left', padx=10, pady=10)

            close_btn = tk.Button(overlay_header,
                                 text="✕",
                                 bg=ArcMoonTheme.BUTTON_DANGER,
                                 fg=ArcMoonTheme.TEXT_PRIMARY,
                                 relief='flat',
                                 font=('Segoe UI', 12, 'bold'),
                                 command=self._toggle_overlay)
            close_btn.pack(side='right', padx=10, pady=10)

            # Overlay content
            self._create_overlay_content()

        except Exception as e:
            logger.error(f"Failed to create overlay: {e}")

    def _create_overlay_content(self) -> None:
        """Create overlay panel content with comprehensive error handling."""
        try:
            if self.overlay is None:
                raise RuntimeError("Overlay not initialized")

            # Create scrollable content frame
            canvas = tk.Canvas(self.overlay.panel, bg=ArcMoonTheme.OVERLAY_PANEL, highlightthickness=0)
            scrollbar = ttk.Scrollbar(self.overlay.panel, orient="vertical", command=canvas.yview)
            scrollable_frame = tk.Frame(canvas, bg=ArcMoonTheme.OVERLAY_PANEL)

            scrollable_frame.bind(
                "<Configure>",
                lambda e: canvas.configure(scrollregion=canvas.bbox("all"))
            )

            canvas.create_window((0, 0), window=scrollable_frame, anchor="nw")
            canvas.configure(yscrollcommand=scrollbar.set)

            canvas.pack(side="left", fill="both", expand=True, padx=10, pady=10)
            scrollbar.pack(side="right", fill="y")

            content_frame = scrollable_frame

            # Advanced commands section
            adv_label = tk.Label(content_frame,
                                text="🔧 Advanced Commands",
                                bg=ArcMoonTheme.OVERLAY_PANEL,
                                fg=ArcMoonTheme.TEXT_PRIMARY,
                                font=('Segoe UI', 12, 'bold'))
            adv_label.pack(anchor='w', pady=(0, 10))

            # Advanced command buttons with safe command execution
            adv_commands = [
                ("🔄 Dry Run Upgrade", lambda: self._run_custom_command("scripts/arcmoon-upgrade.ps1 -DryRun", shell_type="pwsh")),
                ("💪 Force Upgrade", lambda: self._run_custom_command("scripts/arcmoon-upgrade.ps1 -Force", shell_type="pwsh")),
                ("⏩ Skip Git Check", lambda: self._run_custom_command("scripts/arcmoon-upgrade.ps1 -SkipGitCheck", shell_type="pwsh")),
                ("📊 Performance Check", lambda: self._run_custom_command("make performance-check", shell_type="bash")),
                ("📖 Generate Docs", lambda: self._run_custom_command("cargo doc --open", shell_type="auto")),
                ("🧹 Clean Build", lambda: self._run_custom_command("cargo clean", shell_type="auto")),
                ("🎯 Publish Dry Run", lambda: self._run_custom_command("cargo publish --dry-run", shell_type="auto")),
                ("📦 Package", lambda: self._run_custom_command("cargo package", shell_type="auto")),
            ]

            for text, command in adv_commands:
                btn = tk.Button(content_frame,
                               text=text,
                               bg=ArcMoonTheme.BUTTON_SECONDARY,
                               fg=ArcMoonTheme.OFF_BLACK,
                               relief='flat',
                               font=('Segoe UI', 10, 'bold'),
                               command=command)
                btn.pack(fill='x', pady=2)
              # Visual Themes section
            themes_label = tk.Label(content_frame,
                                   text="🎨 Visual Themes",
                                   bg=ArcMoonTheme.OVERLAY_PANEL,
                                   fg=ArcMoonTheme.TEXT_PRIMARY,
                                   font=('Segoe UI', 12, 'bold'))
            themes_label.pack(anchor='w', pady=(20, 10))

            # Simple theme switching buttons
            theme_commands = [
                ("🌙 Ultra Dark", lambda: self._apply_theme('ultra_dark')),
                ("🌌 Cosmic Void", lambda: self._apply_theme('cosmic_void')),
                ("🎯 Matrix Noir", lambda: self._apply_theme('matrix_noir')),
                ("🔥 Ember Storm", lambda: self._apply_theme('ember_storm')),
                ("❄️ Arctic Frost", lambda: self._apply_theme('arctic_frost')),
            ]

            for text, command in theme_commands:
                btn = tk.Button(content_frame,
                               text=text,
                               bg=ArcMoonTheme.BUTTON_SECONDARY,
                               fg=ArcMoonTheme.OFF_BLACK,
                               relief='flat',
                               font=('Segoe UI', 10, 'bold'),
                               command=command)
                btn.pack(fill='x', pady=2)

            # System info section
            info_label = tk.Label(content_frame,
                                 text="ℹ️ System Information",
                                 bg=ArcMoonTheme.OVERLAY_PANEL,
                                 fg=ArcMoonTheme.TEXT_PRIMARY,
                                 font=('Segoe UI', 12, 'bold'))
            info_label.pack(anchor='w', pady=(20, 10))

            # System info display with error handling
            try:
                workspace_name = Path(self.workspace_path).name if self.workspace_path else "Unknown"
                system_info = f"""Platform: {platform.system()} {platform.release()}
Python: {sys.version.split()[0]}
Workspace: {workspace_name}
Time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}
Features: Integrated CrateCheck • GitHub CLI Integration"""
            except Exception as e:
                logger.warning(f"Error generating system info: {e}")
                system_info = "System information unavailable"

            info_text = tk.Text(content_frame,
                               bg=ArcMoonTheme.DARK_TERTIARY,
                               fg=ArcMoonTheme.TEXT_SECONDARY,
                               relief='flat',
                               font=('Consolas', 9),
                               height=8,
                               wrap=tk.WORD)
            info_text.pack(fill='x')
            info_text.insert('1.0', system_info)
            info_text.config(state='disabled')

        except Exception as e:
            logger.error(f"Failed to create overlay content: {e}")

    def _create_fallback_terminal(self, parent) -> tk.Text:
        """Create basic fallback terminal if syntax highlighting fails."""
        try:
            terminal = tk.Text(
                parent,
                wrap=tk.WORD,
                font=('Consolas', 10),
                bg='#1a1a1a',
                fg='#ffffff',
                insertbackground='#87CEEB',
                selectbackground='#FFB7C5',
                selectforeground='#000000',
                state='disabled'
            )
            return terminal
        except Exception as e:
            logger.error(f"Even fallback terminal creation failed: {e}")
            # Return basic text widget as last resort
            return tk.Text(parent, state='disabled')

    def _create_syntax_highlighted_terminal(self, parent) -> scrolledtext.ScrolledText:
        """Create terminal with basic syntax highlighting support"""
        terminal = scrolledtext.ScrolledText(
            parent,
            wrap=tk.WORD,
            font=(self.config.terminal_font, self.config.terminal_font_size),
            bg=ArcMoonTheme.DARK_TERTIARY,
            fg=ArcMoonTheme.TEXT_PRIMARY,
            insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON,
            selectbackground=ArcMoonTheme.CHERRY_BLOSSOM_PINK,
            selectforeground=ArcMoonTheme.OFF_BLACK,
            state='disabled'
        )
          # Configure syntax highlighting tags
        terminal.tag_configure("command", foreground=ArcMoonTheme.LIGHT_BLUE_MOON)
        terminal.tag_configure("success", foreground=ArcMoonTheme.TEXT_SUCCESS)
        terminal.tag_configure("error", foreground=ArcMoonTheme.TEXT_ERROR)
        terminal.tag_configure("warning", foreground=ArcMoonTheme.TEXT_WARNING)
        terminal.tag_configure("path", foreground=ArcMoonTheme.PALE_BLUE_GRAY)
        terminal.tag_configure("number", foreground=ArcMoonTheme.CHERRY_BLOSSOM_PINK)
        terminal.tag_configure("keyword", foreground=ArcMoonTheme.LIGHT_BLUE_MOON, font=(self.config.terminal_font, self.config.terminal_font_size, 'bold'))

        return terminal

    def _apply_syntax_highlighting(self, text: str) -> None:
        """Apply basic syntax highlighting to terminal text without regex dependencies"""
        try:
            # Get current position to apply tags only to the new text
            current_pos = self.terminal_text.index(tk.END + "-1c")

            self.terminal_text.insert(tk.END, text)

            # Find patterns and apply highlighting
            lines = text.split('\n')

            # Calculate the starting line index for the newly inserted text
            first_new_line_index = int(current_pos.split('.')[0])

            for i, line in enumerate(lines):
                if not line.strip():  # Skip empty lines
                    continue

                line_number = first_new_line_index + i
                line_start_idx = f"{line_number}.0"

                # Calculate line end position properly
                line_length = len(line)
                line_end_idx = f"{line_number}.{line_length}"

                # Highlight commands starting with $
                if line.strip().startswith('$'):
                    self.terminal_text.tag_add("command", line_start_idx, line_end_idx)

                # Highlight success messages
                elif any(word in line.lower() for word in ['✅', 'success', 'passed', 'completed']):
                    self.terminal_text.tag_add("success", line_start_idx, line_end_idx)

                # Highlight errors
                elif any(word in line.lower() for word in ['❌', 'error', 'failed', 'exception']):
                    self.terminal_text.tag_add("error", line_start_idx, line_end_idx)

                # Highlight warnings
                elif any(word in line.lower() for word in ['⚠️', 'warning', 'warn']):
                    self.terminal_text.tag_add("warning", line_start_idx, line_end_idx)

                # Highlight common file extensions (simple string matching)
                words = line.split()
                for word in words:
                    if any(word.endswith(ext) for ext in ['.toml', '.rs', '.py', '.md', '.txt', '.json']):
                        word_start = line.find(word)
                        if word_start >= 0:
                            start_idx = f"{line_number}.{word_start}"
                            end_idx = f"{line_number}.{word_start + len(word)}"
                            self.terminal_text.tag_add("path", start_idx, end_idx)

                # Highlight Rust/Cargo keywords (simple word matching)
                rust_keywords = ['cargo', 'rustc', 'clippy', 'fmt', 'test', 'bench', 'check', 'build', 'audit', 'doc', 'package', 'publish']
                for keyword in rust_keywords:
                    if keyword.lower() in line.lower():
                        keyword_start = line.lower().find(keyword.lower())
                        if keyword_start >= 0:
                            start_idx = f"{line_number}.{keyword_start}"
                            end_idx = f"{line_number}.{keyword_start + len(keyword)}"
                            self.terminal_text.tag_add("keyword", start_idx, end_idx)

            self.terminal_text.see(tk.END)

        except Exception as e:
            logger.error(f"Error applying syntax highlighting: {e}")
            # Fallback to normal insertion if highlighting fails
            try:
                self.terminal_text.insert(tk.END, text)
                self.terminal_text.see(tk.END)
            except Exception:
                pass  # Silence any secondary errors

    def _toggle_overlay(self) -> None:
        """Toggle overlay visibility with error handling."""
        try:
            if self.overlay:
                self.overlay.toggle()
        except Exception as e:
            logger.error(f"Error toggling overlay: {e}")

    def _append_output_queued(self, text: str) -> None:
        """Append text to output display by queuing it for thread safety."""
        try:
            self.command_queue.put(text)
        except Exception as e:
            logger.error(f"Error queuing output: {e}")

    def _process_queue(self) -> None:
        """Process the command output queue on the main thread."""
        try:
            while not self.command_queue.empty():
                text = self.command_queue.get_nowait()
                self.terminal_text.config(state='normal')
                self._apply_syntax_highlighting(text) # Apply highlighting
                self.terminal_text.config(state='disabled')
                self.root.update_idletasks() # Ensure GUI updates immediately
        except queue.Empty:
            pass # No items in queue
        except Exception as e:
            logger.error(f"Error processing output queue: {e}")
        finally:
            self.root.after(100, self._process_queue) # Schedule next check

    def _clear_output(self) -> None:
        """Clear output display with error handling."""
        try:
            self.terminal_text.config(state='normal')
            self.terminal_text.delete('1.0', tk.END)
            self.terminal_text.config(state='disabled')
        except Exception as e:
            logger.error(f"Error clearing output: {e}")

    def _save_log(self) -> None:
        """Save terminal output to file."""
        try:
            filename = filedialog.asksaveasfilename(
                title="Save Terminal Log",
                defaultextension=".log",
                filetypes=[("Log files", "*.log"), ("Text files", "*.txt"), ("All files", "*.*")]
            )
            if filename:
                content = self.terminal_text.get('1.0', tk.END)
                with open(filename, 'w', encoding='utf-8') as f:
                    f.write(content)
                self._append_output_queued(f"\n📄 Terminal log saved to: {filename}\n")
        except Exception as e:
            logger.error(f"Error saving log: {e}")
            messagebox.showerror("Error", f"Failed to save log: {str(e)}")

    def _update_status(self, status: str) -> None:
        """Update status display with error handling. Called via root.after()."""
        try:
            self.current_status = status
            self.status_var.set(status)

            # Enable/disable stop button and progress bar based on operation status
            if "Running" in status or "Executing" in status:
                self.stop_btn.config(state='normal')
                self.progress_bar.start()
            else:
                self.stop_btn.config(state='disabled')
                self.progress_bar.stop()

        except Exception as e:
            logger.error(f"Error updating status: {e}")

    def _update_time(self) -> None:
        """Update time display with error handling."""
        try:
            current_time = datetime.now().strftime('%H:%M:%S')
            self.time_var.set(current_time)
            self.root.after(1000, self._update_time)
        except Exception as e:
            logger.error(f"Error updating time: {e}")
            # Retry in 5 seconds if there's an error
            self.root.after(5000, self._update_time)

    def _browse_workspace(self) -> None:
        """Browse for workspace directory with validation."""
        try:
            directory = filedialog.askdirectory(initialdir=self.workspace_path)
            if directory and os.path.exists(directory):
                self.workspace_path = directory
                self.path_var.set(directory)
                self.working_dir_var.set(directory)
                self.config.workspace_path = directory
                self.config.save()
                self._refresh_status()
            elif directory:
                messagebox.showerror("Invalid Directory", f"Selected directory does not exist: {directory}")
        except Exception as e:
            logger.error(f"Error browsing workspace: {e}")
            messagebox.showerror("Error", f"Failed to browse workspace: {str(e)}")

    def _refresh_status(self) -> bool:
        """Refresh git and workspace status with comprehensive error handling."""
        self.git_status_var.set("Checking...")

        def check_status_thread():
            try:
                if not os.path.exists(self.workspace_path):
                    self.root.after(0, lambda: self.git_status_var.set("❌ Workspace directory does not exist"))
                    return

                # Check git status
                result = subprocess.run(
                    ["git", "status", "--porcelain", "Cargo.toml"],
                    cwd=self.workspace_path,
                    capture_output=True,
                    text=True,
                    timeout=30  # Add timeout to prevent hanging
                )

                if result.returncode == 0:
                    if result.stdout.strip():
                        self.root.after(0, lambda: self.git_status_var.set("⚠️ Uncommitted changes in Cargo.toml"))
                    else:
                        self.root.after(0, lambda: self.git_status_var.set("✅ Git status clean"))
                else:
                    self.root.after(0, lambda: self.git_status_var.set("❓ Not a git repository"))

            except subprocess.TimeoutExpired:
                self.root.after(0, lambda: self.git_status_var.set("⏱️ Git check timed out"))
            except FileNotFoundError:
                self.root.after(0, lambda: self.git_status_var.set("❌ Git not found in PATH"))
            except PermissionError:
                self.root.after(0, lambda: self.git_status_var.set("🚫 Permission denied accessing git"))
            except Exception as e:
                logger.error(f"Error checking git status: {e}")
                self.root.after(0, lambda: self.git_status_var.set(f"❌ Error: {str(e)[:50]}"))

        threading.Thread(target=check_status_thread, daemon=True).start()
        return True

    def _update_status_display(self) -> None:
        """Update initial status display."""
        try:
            self._refresh_status()
        except Exception as e:
            logger.error(f"Error updating status display: {e}")

    def _stop_current_operation(self) -> None:
        """Stop the currently running operation."""
        try:
            if self.command_executor.is_running:
                self.command_executor.terminate()
                self._append_output_queued("\n🛑 Operation stopped by user.\n")
                self.root.after(0, lambda: self._update_status("Operation stopped"))
        except Exception as e:
            logger.error(f"Error stopping operation: {e}")
            self._append_output_queued(f"\n❌ Error stopping operation: {str(e)}\n")

    # Rust-specific methods
    def _run_standard_check(self) -> None:
        """Run standard cargo check with error handling."""
        try:
            self.root.after(0, lambda: self._update_status("Running standard check..."))
            self._append_output_queued(f"\n🔍 Starting standard check at {datetime.now().strftime('%H:%M:%S')}\n")

            command = "cargo check --verbose"
            self.command_executor.execute_async(command, working_dir=self.workspace_path)
        except Exception as e:
            logger.error(f"Error running standard check: {e}")
            self._append_output_queued(f"\n❌ Error starting standard check: {str(e)}\n")

    def _run_full_crate_validation(self) -> None:
        """Run comprehensive crate validation with all checks."""
        try:
            self.root.after(0, lambda: self._update_status("Running comprehensive crate validation..."))
            self._append_output_queued(f"\n🎯 Starting comprehensive crate validation at {datetime.now().strftime('%H:%M:%S')}\n")

            self.command_executor.execute_crate_check_async(working_dir=self.workspace_path)
        except Exception as e:
            logger.error(f"Error running crate validation: {e}")
            self._append_output_queued(f"\n❌ Error starting crate validation: {str(e)}\n")

    def _run_format(self) -> None:
        """Run cargo fmt with error handling."""
        try:
            self.root.after(0, lambda: self._update_status("Running cargo fmt..."))
            self._append_output_queued(f"\n📝 Starting code formatting at {datetime.now().strftime('%H:%M:%S')}\n")

            command = "cargo fmt --all"
            self.command_executor.execute_async(command, working_dir=self.workspace_path)
        except Exception as e:
            logger.error(f"Error running format: {e}")
            self._append_output_queued(f"\n❌ Error starting format: {str(e)}\n")

    def _run_clippy(self) -> None:
        """Run cargo clippy with error handling."""
        try:
            self.root.after(0, lambda: self._update_status("Running cargo clippy..."))
            self._append_output_queued(f"\n📎 Starting clippy analysis at {datetime.now().strftime('%H:%M:%S')}\n")

            command = "cargo clippy --all-targets --all-features -- -D warnings"
            self.command_executor.execute_async(command, working_dir=self.workspace_path)
        except Exception as e:
            logger.error(f"Error running clippy: {e}")
            self._append_output_queued(f"\n❌ Error starting clippy: {str(e)}\n")

    def _run_audit(self) -> None:
        """Run cargo audit with error handling."""
        try:
            self.root.after(0, lambda: self._update_status("Running security audit..."))
            self._append_output_queued(f"\n🔒 Starting security audit at {datetime.now().strftime('%H:%M:%S')}\n")

            command = "cargo audit"
            self.command_executor.execute_async(command, working_dir=self.workspace_path)
        except Exception as e:
            logger.error(f"Error running audit: {e}")
            self._append_output_queued(f"\n❌ Error starting audit: {str(e)}\n")

    def _run_benchmarks(self) -> None:
        """Run cargo bench with error handling."""
        try:
            self.root.after(0, lambda: self._update_status("Running benchmarks..."))
            self._append_output_queued(f"\n⚡ Starting benchmark compilation at {datetime.now().strftime('%H:%M:%S')}\n")

            command = "cargo bench --no-run"
            self.command_executor.execute_async(command, working_dir=self.workspace_path)
        except Exception as e:
            logger.error(f"Error running benchmarks: {e}")
            self._append_output_queued(f"\n❌ Error starting benchmarks: {str(e)}\n")

    def _run_upgrade(self) -> None:
        """Run dependency upgrade with confirmation and error handling."""
        try:
            if not self._confirm_upgrade():
                return

            self.root.after(0, lambda: self._update_status("Running dependency upgrade..."))
            self._append_output_queued(f"\n⬆️ Starting dependency upgrade at {datetime.now().strftime('%H:%M:%S')}\n")

            env_var = "$env:CARGO_ARCMOON_UPGRADE='true'; " if self.upgrade_enabled_var.get() else ""
            command = f"{env_var}.\\scripts\\arcmoon-upgrade.ps1"
            self.command_executor.execute_async(command, shell_type="pwsh", working_dir=self.workspace_path)
        except Exception as e:
            logger.error(f"Error running upgrade: {e}")
            self._append_output_queued(f"\n❌ Error starting upgrade: {str(e)}\n")

    def _run_tests(self) -> None:
        """Run test suite with error handling."""
        try:
            self.root.after(0, lambda: self._update_status("Running tests..."))
            self._append_output_queued(f"\n🧪 Starting test suite at {datetime.now().strftime('%H:%M:%S')}\n")

            command = "cargo test --verbose"
            self.command_executor.execute_async(command, working_dir=self.workspace_path)
        except Exception as e:
            logger.error(f"Error running tests: {e}")
            self._append_output_queued(f"\n❌ Error starting tests: {str(e)}\n")

    def _run_custom_command(self, command: str, shell_type: str = "auto") -> None:
        """Run a custom command with validation and error handling."""
        try:
            if not command or not command.strip():
                self._append_output_queued("\n⚠️ Empty command provided\n")
                return

            self.root.after(0, lambda: self._update_status(f"Running: {command}"))
            self._append_output_queued(f"\n💻 Executing: {command} at {datetime.now().strftime('%H:%M:%S')}\n")

            self.command_executor.execute_async(command, shell_type=shell_type, working_dir=self.workspace_path)
        except Exception as e:
            logger.error(f"Error running custom command: {e}")
            self._append_output_queued(f"\n❌ Error executing command: {str(e)}\n")

    def _execute_custom_command(self) -> None:
        """Execute custom command from entry with validation."""
        try:
            command = self.custom_command_var.get().strip()
            if command:
                self._run_custom_command(command)
                self.custom_command_var.set("")
            else:
                self._append_output_queued("\n⚠️ Please enter a command\n")
        except Exception as e:
            logger.error(f"Error executing custom command: {e}")
            self._append_output_queued(f"\n❌ Error: {str(e)}\n")

    def _execute_command_from_entry(self, event=None) -> None:
        """Execute command from command entry"""
        try:
            command = self.command_var.get().strip()
            if command:
                self.command_var.set("")
                cwd = self.working_dir_var.get()
                self._execute_async_general(lambda: self.command_executor.execute_command(command, cwd))
        except Exception as e:
            logger.error(f"Error executing command from entry: {e}")
            self._append_output_queued(f"\n❌ Error: {str(e)}\n")

    def _execute_async_general(self, func: Callable[[], Any]) -> None:
        """Execute a general function asynchronously, updating status."""
        def run():
            self.root.after(0, lambda: self._update_status("Executing..."))
            try:
                # Assuming func returns a tuple (returncode, stdout, stderr) for logging
                return_code, stdout, stderr = func()
                if return_code != 0:
                    self.root.after(0, lambda: self._append_output_queued(f"ERROR: {stdout}\n{stderr}\n"))
                    self.root.after(0, lambda: self._update_status("Error"))
                else:
                    self.root.after(0, lambda: self._update_status("Ready"))
            except Exception as e:
                self.root.after(0, lambda: self._append_output_queued(f"EXCEPTION: {str(e)}\n"))
                self.root.after(0, lambda: self._update_status("Error"))

        thread = threading.Thread(target=run, daemon=True)
        thread.start()

    def _execute_github_async(self, func: Callable[..., Any], *args, **kwargs) -> None:
        """Execute GitHub operation asynchronously"""
        def run():
            self.root.after(0, lambda: self._update_status("Executing GitHub operation..."))

            try:
                # Assuming func returns a tuple (returncode, stdout, stderr)
                return_code, stdout, stderr = func(*args, **kwargs)
                if return_code != 0:
                    self.root.after(0, lambda: self._append_output_queued(f"ERROR: {stdout}\n{stderr}\n"))
                    self.root.after(0, lambda: self._update_status("Error"))
                else:
                    self.root.after(0, lambda: self._update_status("Ready"))
            except Exception as e:
                self.root.after(0, lambda: self._append_output_queued(f"EXCEPTION: {str(e)}\n"))
                self.root.after(0, lambda: self._update_status("Error"))

        thread = threading.Thread(target=run, daemon=True)
        thread.start()

    # GitHub-specific command methods
    def _clone_repo_command(self) -> None:
        """Clone repository command wrapper"""
        try:
            repo_url = self.clone_url_var.get().strip()
            destination = self.clone_dest_var.get().strip()

            if repo_url:
                self._execute_github_async(self.github_ops.clone_repo, repo_url, destination)
            else:
                messagebox.showerror("Error", "Repository URL is required")
        except Exception as e:
            logger.error(f"Error in clone command: {e}")
            self._append_output_queued(f"\n❌ Error: {str(e)}\n")

    def _create_repo_command(self) -> None:
        """Create repository command wrapper"""
        try:
            name = self.create_name_var.get().strip()
            description = self.create_desc_var.get().strip()
            private = self.create_private_var.get()

            if name:
                self._execute_github_async(self.github_ops.create_repo, name, private, description)
            else:
                messagebox.showerror("Error", "Repository name is required")
        except Exception as e:
            logger.error(f"Error in create repo command: {e}")
            self._append_output_queued(f"\n❌ Error: {str(e)}\n")

    def _list_repos_command(self) -> None:
        """List repositories command wrapper"""
        try:
            user = self.list_user_var.get().strip()
            self._execute_github_async(self.github_ops.list_repos, user, 30)
        except Exception as e:
            logger.error(f"Error in list repos command: {e}")
            self._append_output_queued(f"\n❌ Error: {str(e)}\n")

    def _create_issue_command(self) -> None:
        """Create issue command wrapper"""
        try:
            title = self.issue_title_var.get().strip()
            body = self.issue_body_text.get("1.0", tk.END).strip()

            if title:
                self._execute_github_async(self.github_ops.create_issue, title, body)
            else:
                messagebox.showerror("Error", "Issue title is required")
        except Exception as e:
            logger.error(f"Error in create issue command: {e}")
            self._append_output_queued(f"\n❌ Error: {str(e)}\n")

    def _commit_changes(self) -> None:
        """Commit changes with message"""
        try:
            message = self.commit_message_var.get().strip()
            if message:
                self._execute_github_async(self.github_ops.git_commit, message, self.workspace_path)
            else:
                messagebox.showerror("Error", "Commit message is required")
        except Exception as e:
            logger.error(f"Error in commit changes: {e}")
            self._append_output_queued(f"\n❌ Error: {str(e)}\n")

    def _commit_and_push(self) -> None:
        """Commit and push changes"""
        try:
            message = self.commit_message_var.get().strip()
            if message:
                self._execute_github_async(self.github_ops.git_commit_and_push, message, self.workspace_path)
                self.commit_message_var.set("")  # Clear after successful commit
            else:
                messagebox.showerror("Error", "Commit message is required")
        except Exception as e:
            logger.error(f"Error in commit and push: {e}")
            self._append_output_queued(f"\n❌ Error: {str(e)}\n")

    def _add_ssh_key_command(self) -> None:
        """Add SSH key command wrapper"""
        try:
            key_file = self.ssh_key_file_var.get().strip()
            title = self.ssh_key_title_var.get().strip()

            if key_file and os.path.exists(key_file):
                self._execute_github_async(self.github_ops.add_ssh_key, key_file, title)
            else:
                messagebox.showerror("Error", "Valid SSH key file is required")
        except Exception as e:
            logger.error(f"Error in add SSH key command: {e}")
            self._append_output_queued(f"\n❌ Error: {str(e)}\n")

    # File dialog methods
    def _browse_clone_destination(self) -> None:
        """Browse for clone destination"""
        try:
            directory = filedialog.askdirectory(title="Select Clone Destination")
            if directory:
                self.clone_dest_var.set(directory)
        except Exception as e:
            logger.error(f"Error browsing clone destination: {e}")

    def _browse_ssh_key(self) -> None:
        """Browse for SSH key file"""
        try:
            filename = filedialog.askopenfilename(
                title="Select SSH Key File",
                filetypes=[("Public Keys", "*.pub"), ("All Files", "*.*")]
            )
            if filename:
                self.ssh_key_file_var.set(filename)
        except Exception as e:
            logger.error(f"Error browsing SSH key: {e}")

    def _change_working_directory(self) -> None:
        """Change working directory"""
        try:
            directory = filedialog.askdirectory(title="Select Working Directory")
            if directory and os.path.exists(directory):
                os.chdir(directory)
                self.working_dir_var.set(directory)
                self._append_output_queued(f"Changed working directory to: {directory}\n")
            elif directory:
                messagebox.showerror("Invalid Directory", f"Selected directory does not exist: {directory}")
        except Exception as e:
            logger.error(f"Error changing working directory: {e}")
            messagebox.showerror("Error", f"Failed to change directory: {str(e)}")

    def _generate_ssh_key(self) -> None:
        """Generate SSH key"""
        try:
            email = simpledialog.askstring("SSH Key Generation", "Enter your email:")
            if email:
                filename = simpledialog.askstring("SSH Key Generation", "Enter filename (default: id_rsa):")
                if not filename:
                    filename = "id_rsa"

                cmd = f'ssh-keygen -t rsa -b 4096 -C "{email}" -f ~/.ssh/{filename}'
                self._execute_async_general(lambda: self.command_executor.execute_command(cmd))
        except Exception as e:
            logger.error(f"Error generating SSH key: {e}")
            self._append_output_queued(f"\n❌ Error generating SSH key: {str(e)}\n")

    def _test_github_connection(self) -> None:
        """Test GitHub connection with GUI passphrase support"""
        try:
            self._append_output_queued("\n🔑 Testing GitHub SSH connection...\n")

            def test_connection():
                try:
                    # Try SSH connection with GUI passphrase handling
                    result = self._execute_ssh_with_gui_passphrase("ssh -T git@github.com")

                    if result[0] == 0:
                        self.root.after(0, lambda: self._append_output_queued("✅ GitHub SSH connection successful!\n"))
                        # Parse the response to show username
                        if "successfully authenticated" in result[1]:
                            username_line = [line for line in result[1].split('\n') if 'Hi ' in line]
                            if username_line:
                                self.root.after(0, lambda: self._append_output_queued(f"🐙 {username_line[0]}\n"))
                    else:
                        self.root.after(0, lambda: self._append_output_queued(f"❌ GitHub SSH connection failed\n"))
                        self.root.after(0, lambda: self._append_output_queued(f"Error: {result[2]}\n"))

                        # Provide helpful guidance
                        guidance = """
💡 To fix SSH connection issues:
1. Ensure SSH key is added to GitHub: gh ssh-key add ~/.ssh/id_ed25519.pub
2. Test SSH agent: ssh-add -l
3. Add key to agent: ssh-add ~/.ssh/id_ed25519
4. Check GitHub settings: https://github.com/settings/keys
"""
                        self.root.after(0, lambda: self._append_output_queued(guidance))

                    self.root.after(0, lambda: self._update_status("Ready"))

                except Exception as e:
                    self.root.after(0, lambda: self._append_output_queued(f"❌ Error testing connection: {str(e)}\n"))
                    self.root.after(0, lambda: self._update_status("Error"))

            self.root.after(0, lambda: self._update_status("Testing GitHub connection..."))
            thread = threading.Thread(target=test_connection, daemon=True)
            thread.start()

        except Exception as e:
            logger.error(f"Error testing GitHub connection: {e}")
            self._append_output_queued(f"\n❌ Error testing connection: {str(e)}\n")

    def _execute_ssh_with_gui_passphrase(self, cmd: str, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Execute SSH command with GUI passphrase prompt support"""
        try:
            import tempfile
            import stat
            import re

            # Create expect script that will handle passphrase prompts
            expect_script = f'''#!/usr/bin/expect -f
set timeout 30
spawn {cmd}
expect {{
    "Enter passphrase for key*:" {{
        puts "PASSPHRASE_PROMPT:[lindex $expect_out(0,string) end-1]"
        interact
    }}
    "Permission denied*" {{
        puts "PERMISSION_DENIED"
        exit 1
    }}
    "Hi*" {{
        puts $expect_out(buffer)
        expect eof
        exit 0
    }}
    timeout {{
        puts "TIMEOUT"
        exit 1
    }}
    eof {{
        exit 0
    }}
}}
'''

            with tempfile.NamedTemporaryFile(mode='w', suffix='.exp', delete=False) as f:
                f.write(expect_script)
                script_path = f.name

            os.chmod(script_path, stat.S_IRWXU)

            try:
                # First try without interaction to see if passphrase is needed
                result = subprocess.run(
                    ["expect", script_path],
                    cwd=cwd,
                    capture_output=True,
                    text=True,
                    timeout=60,
                    input="\n"  # Send enter to trigger any prompts
                )

                output = result.stdout + result.stderr

                # Check if passphrase was requested
                if "Enter passphrase for key" in output:
                    # Extract key path from the output
                    match = re.search(r"Enter passphrase for key '([^']+)':", output)
                    key_path = match.group(1) if match else "SSH key"# Show GUI passphrase dialog on main thread
                    passphrase_result: Dict[str, Optional[str]] = {"passphrase": None}  # Initialize with None
                    def prompt_passphrase():
                        result: Optional[str] = self.ssh_manager.prompt_for_ssh_passphrase(key_path, self.root)
                        if result is not None:
                            passphrase_result["passphrase"] = result

                    while passphrase_result.get("passphrase") is None:
                        time.sleep(0.1)

                    passphrase = passphrase_result.get("passphrase")
                    if passphrase is None:
                        return 1, "", "Passphrase entry cancelled"

                    # Create new expect script with the passphrase
                    expect_with_passphrase = f'''#!/usr/bin/expect -f
set timeout 30
spawn {cmd}
expect {{
    "Enter passphrase for key*:" {{
        send "{passphrase}\\r"
        exp_continue
    }}
    "Permission denied*" {{
        exit 1
    }}
    "Hi*" {{
        puts $expect_out(buffer)
        expect eof
        exit 0
    }}
    timeout {{
        exit 1
    }}
    eof {{
        exit 0
    }}
}}
'''

                    with tempfile.NamedTemporaryFile(mode='w', suffix='.exp', delete=False) as f2:
                        f2.write(expect_with_passphrase)
                        script_path_2 = f2.name

                    os.chmod(script_path_2, stat.S_IRWXU)

                    # Execute with passphrase
                    final_result = subprocess.run(
                        ["expect", script_path_2],
                        cwd=cwd,
                        capture_output=True,
                        text=True,
                        timeout=60
                    )

                    os.unlink(script_path_2)
                    os.unlink(script_path)

                    return final_result.returncode, final_result.stdout, final_result.stderr

                else:
                    # No passphrase needed, return original result
                    os.unlink(script_path)
                    return result.returncode, result.stdout, result.stderr

            except subprocess.TimeoutExpired:
                return 1, "", "SSH command timed out"
            except FileNotFoundError:
                # expect not available, fall back to basic execution
                return self.command_executor.execute_command(cmd, cwd)
            finally:
                try:
                    os.unlink(script_path)
                except:
                    pass

        except Exception as e:
            return 1, "", f"SSH command error: {str(e)}"

    def _convert_to_https_auth(self, cmd: str, cwd: Optional[str] = None) -> Tuple[int, str, str]:
        """Convert SSH command to HTTPS authentication as fallback"""
        try:
            # Simple fallback to basic git command execution
            return self.command_executor.execute_command(cmd, cwd)
        except Exception as e:
            return 1, "", f"HTTPS auth fallback error: {str(e)}"

    def _test_ssh_with_password(self) -> None:
        """Test SSH connection with password authentication option"""
        try:
            # First try key-based authentication
            result = self.github_ops.git_push(self.workspace_path)
            if result[0] == 0:
                self._append_output_queued("✅ SSH connection successful with key authentication\n")
            else:
                self._append_output_queued("🔐 Key authentication failed, testing with password option available...\n")
                # The SSH manager will prompt for password if needed during actual operations
        except Exception as e:
            logger.error(f"Error testing SSH with password: {e}")
            self._append_output_queued(f"\n❌ Error testing SSH: {str(e)}\n")

    def _open_settings(self) -> None:
        """Open settings dialog"""
        try:
            # Create settings window
            settings_window = tk.Toplevel(self.root)
            settings_window.title("Settings")
            settings_window.geometry("500x400")
            settings_window.configure(bg=ArcMoonTheme.DARK_BG)
            settings_window.transient(self.root)
            settings_window.grab_set()

            # GitHub username
            ttk.Label(settings_window, text="GitHub Username:", style='ArcMoon.TLabel').pack(anchor=tk.W, padx=10, pady=5)
            username_var = tk.StringVar(value=self.config.github_username or "")
            username_entry = tk.Entry(settings_window, textvariable=username_var,
                                     bg=ArcMoonTheme.DARK_TERTIARY,
                                     fg=ArcMoonTheme.TEXT_PRIMARY,
                                     insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON,
                                     relief='flat', bd=5, width=40)
            username_entry.pack(padx=10, pady=5)

            # Default clone directory
            ttk.Label(settings_window, text="Default Clone Directory:", style='ArcMoon.TLabel').pack(anchor=tk.W, padx=10, pady=5)
            clone_dir_var = tk.StringVar(value=self.config.default_clone_dir)
            clone_dir_frame = ttk.Frame(settings_window)
            clone_dir_frame.pack(fill=tk.X, padx=10, pady=5)

            clone_dir_entry = tk.Entry(clone_dir_frame, textvariable=clone_dir_var,
                                      bg=ArcMoonTheme.DARK_TERTIARY,
                                      fg=ArcMoonTheme.TEXT_PRIMARY,
                                      insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON,
                                      relief='flat', bd=5)
            clone_dir_entry.pack(side=tk.LEFT, fill=tk.X, expand=True)

            ttk.Button(clone_dir_frame, text="Browse",
                      command=lambda: clone_dir_var.set(filedialog.askdirectory() or clone_dir_var.get())).pack(side=tk.RIGHT, padx=(5, 0))

            # Terminal font
            ttk.Label(settings_window, text="Terminal Font:", style='ArcMoon.TLabel').pack(anchor=tk.W, padx=10, pady=5)
            font_var = tk.StringVar(value=self.config.terminal_font)
            font_entry = tk.Entry(settings_window, textvariable=font_var,
                                 bg=ArcMoonTheme.DARK_TERTIARY,
                                 fg=ArcMoonTheme.TEXT_PRIMARY,
                                 insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON,
                                 relief='flat', bd=5, width=40)
            font_entry.pack(padx=10, pady=5)

            # Terminal font size
            ttk.Label(settings_window, text="Terminal Font Size:", style='ArcMoon.TLabel').pack(anchor=tk.W, padx=10, pady=5)
            font_size_var = tk.StringVar(value=str(self.config.terminal_font_size))
            font_size_entry = tk.Entry(settings_window, textvariable=font_size_var,
                                      bg=ArcMoonTheme.DARK_TERTIARY,
                                      fg=ArcMoonTheme.TEXT_PRIMARY,
                                      insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON,
                                      relief='flat', bd=5, width=40)
            font_size_entry.pack(padx=10, pady=5)

            # Buttons
            button_frame = ttk.Frame(settings_window)
            button_frame.pack(side=tk.BOTTOM, fill=tk.X, padx=10, pady=10)

            def save_settings():
                try:
                    self.config.github_username = username_var.get()
                    self.config.default_clone_dir = clone_dir_var.get()
                    self.config.terminal_font = font_var.get()
                    self.config.terminal_font_size = int(font_size_var.get() or 10)
                    self.config.save()
                    settings_window.destroy()
                    messagebox.showinfo("Settings", "Settings saved successfully!")
                except Exception as e:
                    logger.error(f"Error saving settings: {e}")
                    messagebox.showerror("Error", f"Failed to save settings: {str(e)}")

            ttk.Button(button_frame, text="Save", command=save_settings).pack(side=tk.RIGHT, padx=5)
            ttk.Button(button_frame, text="Cancel", command=settings_window.destroy).pack(side=tk.RIGHT)

        except Exception as e:
            logger.error(f"Error opening settings: {e}")
            messagebox.showerror("Error", f"Failed to open settings: {str(e)}")

    def _confirm_upgrade(self) -> bool:
        """Confirm upgrade operation with comprehensive information."""
        try:
            return messagebox.askyesno(
                "Confirm Upgrade",
                "Are you sure you want to upgrade dependencies?\n\n"
                "This will modify your Cargo.toml file.\n"
                "Ensure you have committed any important changes first.\n\n"
                "Current workspace: " + os.path.basename(self.workspace_path),
                icon='warning'
            )
        except Exception as e:
            logger.error(f"Error showing confirmation dialog: {e}")
            return False
    def _on_closing(self) -> None:
        """Handle application closing with proper cleanup."""
        try:
            if self.command_executor.is_running:
                if not messagebox.askokcancel("Quit", "A command is still running. Force quit?"):
                    return
                self.command_executor.terminate()

            self.config.save()
            self.root.destroy()
        except Exception as e:
            logger.error(f"Error during application closing: {e}")
            # Force close if there's an error
            try:
                self.root.destroy()
            except:
                pass

    def _open_theme_demo(self) -> None:
        """Open the interactive theme demo application."""
        try:
            demo_path = os.path.join(os.path.dirname(__file__), "ams_options", "theme_demo.py")
            if os.path.exists(demo_path):
                self._append_output_queued("🎨 Opening Interactive Theme Demo...\n")
                subprocess.Popen([sys.executable, demo_path])
            else:
                self._append_output_queued(f"❌ Theme demo not found at: {demo_path}\n")
                messagebox.showerror("Error", f"Theme demo file not found:\n{demo_path}")
        except Exception as e:
            logger.error(f"Error opening theme demo: {e}")
            self._append_output_queued(f"❌ Error opening theme demo: {str(e)}\n")
            messagebox.showerror("Error", f"Failed to open theme demo:\n{str(e)}")

    def _open_ams_options_folder(self) -> None:
        """Open the AMS options folder in file explorer."""
        try:
            options_path = os.path.join(os.path.dirname(__file__), "ams_options")
            if os.path.exists(options_path):
                self._append_output_queued(f"📁 Opening AMS Options folder: {options_path}\n")

                # Cross-platform file explorer opening
                if platform.system() == "Windows":
                    os.startfile(options_path)
                elif platform.system() == "Darwin":  # macOS
                    subprocess.Popen(["open", options_path])
                else:  # Linux
                    subprocess.Popen(["xdg-open", options_path])
            else:
                self._append_output_queued(f"❌ AMS Options folder not found at: {options_path}\n")
                messagebox.showerror("Error", f"AMS Options folder not found:\n{options_path}")
        except Exception as e:
            logger.error(f"Error opening AMS options folder: {e}")
            self._append_output_queued(f"❌ Error opening AMS options folder: {str(e)}\n")
            messagebox.showerror("Error", f"Failed to open AMS options folder:\n{str(e)}")

    def _open_theme_integration(self) -> None:
        """Open the theme integration system file."""
        try:
            integration_path = os.path.join(os.path.dirname(__file__), "ams_options", "ams_theme_integration.py")
            if os.path.exists(integration_path):
                self._append_output_queued(f"🔧 Opening Theme Integration: {integration_path}\n")

                # Cross-platform file opening
                if platform.system() == "Windows":
                    os.startfile(integration_path)
                elif platform.system() == "Darwin":  # macOS
                    subprocess.Popen(["open", integration_path])
                else:  # Linux
                    subprocess.Popen(["xdg-open", integration_path])
            else:
                self._append_output_queued(f"❌ Theme integration file not found at: {integration_path}\n")
                messagebox.showerror("Error", f"Theme integration file not found:\n{integration_path}")
        except Exception as e:
            logger.error(f"Error opening theme integration: {e}")
            self._append_output_queued(f"❌ Error opening theme integration: {str(e)}\n")
            messagebox.showerror("Error", f"Failed to open theme integration:\n{str(e)}")

    def _open_color_themes(self) -> None:
        """Open the color themes definition file."""
        try:
            themes_path = os.path.join(os.path.dirname(__file__), "ams_options", "color_themes.py")
            if os.path.exists(themes_path):
                self._append_output_queued(f"🎨 Opening Color Themes: {themes_path}\n")

                # Cross-platform file opening
                if platform.system() == "Windows":
                    os.startfile(themes_path)
                elif platform.system() == "Darwin":  # macOS
                    subprocess.Popen(["open", themes_path])
                else:  # Linux
                    subprocess.Popen(["xdg-open", themes_path])
            else:
                self._append_output_queued(f"❌ Color themes file not found at: {themes_path}\n")
                messagebox.showerror("Error", f"Color themes file not found:\n{themes_path}")
        except Exception as e:
            logger.error(f"Error opening color themes: {e}")
            self._append_output_queued(f"❌ Error opening color themes: {str(e)}\n")
            messagebox.showerror("Error", f"Failed to open color themes:\n{str(e)}")

    def _apply_theme(self, theme_name: str) -> None:
        """Apply a selected theme by updating the ArcMoonTheme class attributes."""
        try:
            # Get the selected theme
            theme_class = ThemeSelector.get_theme(theme_name)
            if theme_class is None:
                self._append_output_queued(f"❌ Theme '{theme_name}' not found\n")
                messagebox.showerror("Error", f"Theme '{theme_name}' not found")
                return

            self._append_output_queued(f"🎨 Applying theme: {theme_name}\n")

            # Create comprehensive theme mapping based on available attributes
            theme_mappings = {
                # Ultra Dark theme mappings
                'ultra_dark': {
                    'OFF_BLACK': 'DARK_BG',
                    'SIDEBAR_DARK': 'DARK_SECONDARY',
                    'MEDIUM_DARK_GRAY': 'DARK_TERTIARY',
                    'DARK_BORDER': 'DARK_BORDER',
                    'WORKSPACE_BG': 'WORKSPACE_BG',
                    'LIGHT_BLUE_MOON': 'LIGHT_BLUE_MOON',
                    'CHERRY_BLOSSOM_PINK': 'CHERRY_BLOSSOM_PINK',
                    'PALE_BLUE_GRAY': 'PALE_BLUE_GRAY',
                    'TEXT_PRIMARY': 'TEXT_PRIMARY',
                    'TEXT_SECONDARY': 'TEXT_SECONDARY',
                    'TEXT_SUCCESS': 'TEXT_SUCCESS',
                    'TEXT_ERROR': 'TEXT_ERROR',
                    'TEXT_WARNING': 'TEXT_WARNING',
                },
                # Cosmic Void theme mappings
                'cosmic_void': {
                    'VOID_BLACK': 'DARK_BG',
                    'SHADOW_GRAY': 'DARK_SECONDARY',
                    'NEBULA_DARK': 'DARK_TERTIARY',
                    'ASTEROID_GRAY': 'DARK_BORDER',
                    'WORKSPACE_BG': 'WORKSPACE_BG',
                    'NEUTRON_BLUE': 'LIGHT_BLUE_MOON',
                    'PULSAR_CYAN': 'CHERRY_BLOSSOM_PINK',
                    'QUASAR_PURPLE': 'PALE_BLUE_GRAY',
                    'STARLIGHT': 'TEXT_PRIMARY',
                    'MOONBEAM': 'TEXT_SECONDARY',
                    'AURORA_GREEN': 'TEXT_SUCCESS',
                    'COMET_TAIL': 'TEXT_ERROR',
                    'SOLAR_GOLD': 'TEXT_WARNING',
                },
                # Matrix Noir theme mappings
                'matrix_noir': {
                    'MATRIX_BLACK': 'DARK_BG',
                    'TERMINAL_DARK': 'DARK_SECONDARY',
                    'CODE_RAIN_BG': 'DARK_TERTIARY',
                    'CONSOLE_GRAY': 'DARK_BORDER',
                    'WORKSPACE_BG': 'WORKSPACE_BG',
                    'PHOSPHOR_GREEN': 'LIGHT_BLUE_MOON',
                    'TERMINAL_GREEN': 'CHERRY_BLOSSOM_PINK',
                    'DATA_STREAM': 'PALE_BLUE_GRAY',
                    'WHITE_NOISE': 'TEXT_PRIMARY',
                    'GHOST_GREEN': 'TEXT_SECONDARY',
                    'PHOSPHOR_GREEN': 'TEXT_SUCCESS',
                    'ERROR_RED': 'TEXT_ERROR',
                    'WARNING_AMBER': 'TEXT_WARNING',
                },
                # Ember Storm theme mappings
                'ember_storm': {
                    'STORM_BLACK': 'DARK_BG',
                    'ASH_GRAY': 'DARK_SECONDARY',
                    'EMBER_DARK': 'DARK_TERTIARY',
                    'SMOKE_GRAY': 'DARK_BORDER',
                    'WORKSPACE_BG': 'WORKSPACE_BG',
                    'EMBER_ORANGE': 'LIGHT_BLUE_MOON',
                    'FLAME_RED': 'CHERRY_BLOSSOM_PINK',
                    'COAL_GLOW': 'PALE_BLUE_GRAY',
                    'LIGHTNING_WHITE': 'TEXT_PRIMARY',
                    'MIST_GRAY': 'TEXT_SECONDARY',
                    'SUNSET_GOLD': 'TEXT_SUCCESS',
                    'FLAME_RED': 'TEXT_ERROR',
                    'EMBER_ORANGE': 'TEXT_WARNING',
                },
                # Arctic Frost theme mappings
                'arctic_frost': {
                    'ARCTIC_BLACK': 'DARK_BG',
                    'ICE_BLUE': 'DARK_SECONDARY',
                    'GLACIER_GRAY': 'DARK_TERTIARY',
                    'SNOW_DRIFT': 'DARK_BORDER',
                    'WORKSPACE_BG': 'WORKSPACE_BG',
                    'ICE_CRYSTAL': 'LIGHT_BLUE_MOON',
                    'AURORA_BLUE': 'CHERRY_BLOSSOM_PINK',
                    'ARCTIC_CYAN': 'PALE_BLUE_GRAY',
                    'SNOW_WHITE': 'TEXT_PRIMARY',
                    'BLIZZARD_GRAY': 'TEXT_SECONDARY',
                    'TUNDRA_GREEN': 'TEXT_SUCCESS',
                    'POLAR_BLUE': 'TEXT_ERROR',
                    'FROST_WHITE': 'TEXT_WARNING',
                }
            }

            # Get the mapping for this specific theme
            theme_mapping = theme_mappings.get(theme_name, theme_mappings['ultra_dark'])

            # Apply theme colors to ArcMoonTheme
            for theme_attr, arcmoon_attr in theme_mapping.items():
                if hasattr(theme_class, theme_attr):
                    new_color = getattr(theme_class, theme_attr)
                    setattr(ArcMoonTheme, arcmoon_attr, new_color)
                    self._append_output_queued(f"   {arcmoon_attr} = {new_color}\n")

            # Update derived colors
            ArcMoonTheme.BUTTON_PRIMARY = ArcMoonTheme.CHERRY_BLOSSOM_PINK
            ArcMoonTheme.BUTTON_SECONDARY = ArcMoonTheme.LIGHT_BLUE_MOON
            ArcMoonTheme.BUTTON_SUCCESS = ArcMoonTheme.TEXT_SUCCESS
            ArcMoonTheme.BUTTON_WARNING = ArcMoonTheme.TEXT_WARNING
            ArcMoonTheme.BUTTON_DANGER = ArcMoonTheme.TEXT_ERROR
            ArcMoonTheme.OVERLAY_BG = ArcMoonTheme.DARK_BG
            ArcMoonTheme.OVERLAY_PANEL = ArcMoonTheme.DARK_SECONDARY
            ArcMoonTheme.OVERLAY_ACCENT = ArcMoonTheme.LIGHT_BLUE_MOON

            # Reconfigure styles with new colors
            ArcMoonStyles.configure_styles()

            # Update root window and existing components
            self._update_gui_theme()

            self._append_output_queued(f"✅ Theme '{theme_name}' applied successfully\n")

        except Exception as e:
            logger.error(f"Error applying theme {theme_name}: {e}")
            self._append_output_queued(f"❌ Error applying theme {theme_name}: {str(e)}\n")

    def _update_gui_theme(self) -> None:
        """Update GUI components with new theme colors."""
        try:
            # Update root window
            self.root.configure(bg=ArcMoonTheme.DARK_BG)

            # Update terminal text widget
            if hasattr(self, 'terminal_text'):
                self.terminal_text.configure(
                    bg=ArcMoonTheme.DARK_TERTIARY,
                    fg=ArcMoonTheme.TEXT_PRIMARY,
                    insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON,
                    selectbackground=ArcMoonTheme.CHERRY_BLOSSOM_PINK,
                    selectforeground=ArcMoonTheme.OFF_BLACK
                )

            # Update overlay if it exists
            if hasattr(self, 'overlay') and self.overlay:
                self.overlay.overlay.configure(bg=ArcMoonTheme.OVERLAY_BG)
                self.overlay.panel.configure(bg=ArcMoonTheme.OVERLAY_PANEL)

            # Update entry widgets recursively
            self._update_entry_widgets_recursive(self.root)

            self._append_output_queued("🎨 GUI components updated with new theme\n")

        except Exception as e:
            logger.error(f"Error updating GUI theme: {e}")
            self._append_output_queued(f"⚠️ Some GUI components may need restart to fully update\n")

    def _update_entry_widgets_recursive(self, widget) -> None:
        """Recursively update entry widgets with new theme colors."""
        try:
            for child in widget.winfo_children():
                if isinstance(child, tk.Entry):
                    child.configure(
                        bg=ArcMoonTheme.DARK_TERTIARY,
                        fg=ArcMoonTheme.TEXT_PRIMARY,
                        insertbackground=ArcMoonTheme.LIGHT_BLUE_MOON
                    )
                elif isinstance(child, tk.Text):
                    child.configure(
                        bg=ArcMoonTheme.DARK_TERTIARY,
                        fg=ArcMoonTheme.TEXT_SECONDARY
                    )
                elif isinstance(child, tk.Frame):
                    # Update frame backgrounds
                    try:
                        child.configure(bg=ArcMoonTheme.DARK_BG)
                    except:
                        pass  # Some frames may not support bg

                # Recursively update children
                self._update_entry_widgets_recursive(child)
        except Exception as e:
            logger.debug(f"Error updating widget {widget}: {e}")

    def run(self) -> None:
        """Run the GUI application with comprehensive initialization."""
        try:
            # Display startup information
            startup_time = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
            self._append_output_queued("🌙 ArcMoon Studios Enterprise Control Panel\n")
            self._append_output_queued("=" * 80 + "\n")
            self._append_output_queued("✅ System initialized successfully\n")
            self._append_output_queued(f"📁 Workspace: {self.workspace_path}\n")
            self._append_output_queued(f"⏰ Started at: {startup_time}\n")
            self._append_output_queued(f"🐍 Python: {sys.version.split()[0]}\n")
            self._append_output_queued(f"💻 Platform: {platform.system()} {platform.release()}\n")
            self._append_output_queued(f"🎯 Features: Crate Quality Validation • GitHub CLI Integration • SSH Management\n")
            self._append_output_queued("\n💡 Use the tabs above for different operations:\n")
            self._append_output_queued("   🦀 Rust: Cargo operations and crate validation\n")
            self._append_output_queued("   🐙 GitHub: Repository management and CI/CD\n")
            self._append_output_queued("   🛠️ Tools: General utilities and command execution\n")
            self._append_output_queued("🎯 Use 'Full Validation' for comprehensive crates.io release readiness check.\n\n")

            # Check GitHub CLI availability
            try:
                result = subprocess.run(["gh", "--version"], capture_output=True, text=True, timeout=10)
                if result.returncode == 0:
                    self._append_output_queued("✅ GitHub CLI (gh) detected and ready\n")
                    # Check authentication status
                    self._execute_github_async(self.github_ops.auth_status)
                else:
                    self._append_output_queued("⚠️ GitHub CLI (gh) not found - GitHub features will be limited\n")
                    self._append_output_queued("   Install from: https://cli.github.com/\n")
            except Exception:
                self._append_output_queued("⚠️ GitHub CLI (gh) not available - GitHub features will be limited\n")

            self.root.after(0, lambda: self._update_status("Ready")) # Update status on main thread

            # Start the main event loop
            self.root.mainloop()

        except Exception as e:
            logger.error(f"Error running application: {e}")
            try:
                messagebox.showerror("Runtime Error", f"Application error: {str(e)}")
            except:
                pass
            raise

def main() -> None:
    """Main entry point with comprehensive error handling."""
    try:
        # Set up proper exception handling
        def handle_exception(exc_type, exc_value, exc_traceback):
            if issubclass(exc_type, KeyboardInterrupt):
                sys.__excepthook__(exc_type, exc_value, exc_traceback)
                return

            logger.error("Uncaught exception", exc_info=(exc_type, exc_value, exc_traceback))
            try:
                messagebox.showerror("Fatal Error",
                                   f"An unexpected error occurred:\n{exc_type.__name__}: {exc_value}")
            except:
                print(f"Fatal Error: {exc_type.__name__}: {exc_value}", file=sys.stderr)

        sys.excepthook = handle_exception

        # Create and run the application
        app = ArcMoonSystemGUI()
        app.run()

    except Exception as e:
        logger.error(f"Fatal error in main: {e}")
        try:
            messagebox.showerror("Fatal Error", f"Application failed to start:\n{str(e)}")
        except:
            print(f"Fatal Error: Application failed to start: {str(e)}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
