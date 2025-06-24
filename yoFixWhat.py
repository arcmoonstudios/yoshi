#!/usr/bin/env python3
"""
yoFixWhat.py

**Brief:** Advanced cargo diagnostics aggregator with structured parsing and comprehensive error reporting
# ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
+ [Advanced Diagnostic Parser & Multi-Format Support]
 - [Cargo Check/Clippy Analysis] - Direct cargo command execution with timeout control
 - [VS Code Problems JSON Parser] - Parse exported VS Code Problems panel data
 - [Structured Output Processing] - JSON and text format diagnostic parsing
 - [Intelligent Error Categorization] - Safety, Correctness, Performance, Documentation, Style
 - [Parallel Crate Analysis] - Multi-threaded workspace processing
 - [Detailed Output Mode] - Enhanced clippy warning context (default)
 - [Normal Output Mode] - Simplified diagnostic summaries (--norm flag)
 - [Workspace Auto-Discovery] - Automatic crate detection and metadata parsing
 - [Cross-Platform Encoding] - Robust UTF-8 handling with fallback strategies
 - [Backup Report Management] - Automatic timestamped report archiving
 - [AI-Augmented Derive Analysis] - Semantic pattern detection and trait suggestions
 - [Compilation Error Categorization] - Error code extraction and grouping (E0433, E0277, etc.)
 - [Custom Output Paths] - Configurable report file locations
 - [Selective Crate Analysis] - Target specific crates with --crate flag
 - [Timeout Configuration] - Adjustable per-command execution limits
 - [Logging Level Control] - DEBUG, INFO, WARNING, ERROR output modes
# ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
# **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
# **Copyright:** (c) 2025 ArcMoon Studios
# **Author:** Lord Xyn
# **License:** MIT
"""

import os
import sys
import json
import logging
import argparse
import subprocess
import asyncio
import multiprocessing
import time
from enum import Enum
from pathlib import Path
from datetime import datetime
from dataclasses import dataclass
from typing import Any, Dict, List, Optional, Set, Tuple
from concurrent.futures import ThreadPoolExecutor, as_completed, ProcessPoolExecutor

# Pre-compiled regex patterns for performance
import re
WARNING_PATTERN = re.compile(r"warning(?:\[([^\]]+)\])?: (.+)")
LOCATION_PATTERN = re.compile(r"^\s*--> ([^:]+):(\d+):(\d+)")
MESSAGE_PATTERN = re.compile(r"^\s*\|\s*(.*)$")

# Try to import tomllib (Python 3.11+) or fallback to tomli
try:
    import tomllib
except ImportError:
    try:
        import tomli as tomllib
    except ImportError:
        tomllib = None

# Optional progress bar
try:
    from tqdm import tqdm
except ImportError:
    # Fallback if tqdm not available
    def tqdm(iterable, desc=None, **kwargs):
        if desc:
            logging.info(f"🔄 {desc}...")
        return iterable

# AI-Augmented Derive Framework Integration
try:
    import ast
    import tokenize
    from io import StringIO
    DERIVE_ANALYSIS_AVAILABLE = True
except ImportError:
    DERIVE_ANALYSIS_AVAILABLE = False
    logging.warning("Derive analysis not available - install tokenize support")


@dataclass
class DiagnosticInfo:
    """Structured diagnostic information with derive framework integration"""
    message: str
    severity: str  # error, warning, note, help
    code: Optional[str] = None
    file_path: Optional[str] = None
    line_number: Optional[int] = None
    column: Optional[int] = None
    suggestion: Optional[str] = None
    full_context: str = ""
    # 🚀 AI-Augmented Derive Framework Extensions
    derive_suggestions: Optional[List[str]] = None
    semantic_category: Optional[str] = None
    trait_inference_confidence: float = 0.0
    optimization_potential: Optional[str] = None

    def __post_init__(self):
        if self.derive_suggestions is None:
            self.derive_suggestions = []


@dataclass
class DeriveFrameworkAnalysis:
    """AI-augmented derive analysis results"""
    struct_name: str
    field_analysis: Dict[str, 'FieldSemanticAnalysis']
    suggested_derives: List[str]
    semantic_patterns: List[str]
    optimization_hints: List[str]
    ai_confidence: float
    performance_impact: str


@dataclass
class FieldSemanticAnalysis:
    """Semantic analysis for individual struct/enum fields"""
    field_name: str
    field_type: str
    suggested_traits: List[str]
    semantic_role: str  # "id", "data", "config", "error_source", etc.
    size_category: str  # "small", "medium", "large", "dynamic"
    copy_eligible: bool
    display_priority: int  # 0-10 for Display formatting priority


class DiagnosticCategory(Enum):
    """Categories for diagnostic classification with derive framework integration"""
    SAFETY = "safety"           # unsafe, unwrap, expect, indexing
    PERFORMANCE = "performance" # inefficient patterns, allocations
    STYLE = "style"            # formatting, naming conventions
    CORRECTNESS = "correctness" # logic errors, type issues
    DOCUMENTATION = "docs"      # missing docs, doc formatting
    DEPRECATED = "deprecated"   # deprecated apis, patterns
    # 🚀 AI-Augmented Derive Framework Categories
    DERIVE_MISSING = "derive_missing"      # Missing beneficial derives
    DERIVE_REDUNDANT = "derive_redundant"  # Unnecessary derives detected
    SEMANTIC_MISMATCH = "semantic_mismatch" # Field semantics don't match usage
    TRAIT_INFERENCE = "trait_inference"     # AI-suggested trait implementations
    OPTIMIZATION_DERIVE = "optimization_derive" # Performance-oriented derives
    # ⚠️ ARCHITECTURE PROTECTION CATEGORY
    ARCHITECTURE_PROTECTED = "architecture_protected"  # Intentional design choices - SKIP REPORTING


def parse_args() -> argparse.Namespace:
    """Parse command line arguments"""
    parser = argparse.ArgumentParser(
        description="yoFixWhat – advanced cargo diagnostics aggregator",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
🚀 AVAILABLE COMMANDS & FEATURES:

ANALYSIS MODES:
  python yoFixWhat.py                           # Full workspace analysis (detailed mode default)
  python yoFixWhat.py --norm                    # Normal mode (simplified output)
  python yoFixWhat.py --parse-vscode xUNFIXED   # Parse VS Code Problems JSON export

TARGETING OPTIONS:
  python yoFixWhat.py --crate yoshi-core        # Analyze specific crate only
  python yoFixWhat.py --crate yoshi-derive --crate yoshi-std  # Multiple crates
  python yoFixWhat.py --no-clippy               # Skip clippy analysis (cargo check only)

PERFORMANCE TUNING:
  python yoFixWhat.py --timeout 600             # Increase timeout to 10 minutes
  python yoFixWhat.py --no-parallel             # Disable parallel processing
  python yoFixWhat.py --log-level DEBUG         # Verbose debugging output

OUTPUT CONTROL:
  python yoFixWhat.py --out custom_report.txt   # Custom output file
  python yoFixWhat.py --deets                   # Force detailed mode (explicit)

AI-ENHANCED FEATURES:
  python yoFixWhat.py --derive-analysis         # Enable AI derive framework analysis
  python yoFixWhat.py --derive-corrections      # Apply automatic corrections
  python yoFixWhat.py --backup-all              # Comprehensive backup creation (default)
  python yoFixWhat.py --no-backup               # Disable backups (DANGEROUS)

ADVANCED WORKFLOWS:
  python yoFixWhat.py --derive-analysis --backup-all --timeout 900
  python yoFixWhat.py --parse-vscode problems.json --out vscode_analysis.txt
  python yoFixWhat.py --crate yoshi-derive --derive-corrections --backup-dir ./safety_backups

📊 OUTPUT CATEGORIES:
  🛡️ Safety Issues    - unwrap(), expect(), panic!(), indexing, unreachable!()
  ✅ Correctness      - Compilation errors, logic issues, type mismatches
  ⚡ Performance      - Inefficient patterns, unnecessary allocations
  📚 Documentation    - Missing docs, malformed documentation
  🎨 Style Issues     - Formatting, idioms, clippy style suggestions
        """
    )
    parser.add_argument("--crate", action="append", dest="crates",
                       help="Limit analysis to specific crate(s) (repeatable)")
    parser.add_argument("--no-clippy", action="store_true",
                       help="Skip clippy analysis")
    parser.add_argument("--timeout", type=int, default=300,
                       help="Per-command timeout in seconds (default: 300)")
    parser.add_argument("--out", type=Path, default=Path("yoFixME.txt"),
                       help="Output report path (default: yoFixME.txt)")
    parser.add_argument("--log-level", default="INFO",
                       choices=["DEBUG", "INFO", "WARNING", "ERROR"],
                       help="Logging level (default: INFO)")
    parser.add_argument("--parallel", action="store_true", default=True,
                       help="Enable parallel crate analysis (default: True)")
    parser.add_argument("--no-parallel", dest="parallel", action="store_false",
                       help="Disable parallel analysis")
    parser.add_argument("--deets", action="store_true", default=True,
                       help="Include detailed clippy warning output in report (default)")
    parser.add_argument("--norm", dest="deets", action="store_false",
                       help="Use normal mode without detailed output")
    parser.add_argument("--derive-analysis", action="store_true",
                       help="Enable AI-augmented derive framework analysis")
    parser.add_argument("--backup-all", action="store_true", default=True,
                       help="Create comprehensive backups before any corrections (default: True)")
    parser.add_argument("--no-backup", dest="backup_all", action="store_false",
                       help="Disable backup creation (NOT RECOMMENDED - DANGEROUS)")
    parser.add_argument("--derive-corrections", action="store_true",
                       help="Apply automatic derive error corrections with mandatory backup")
    parser.add_argument("--backup-dir", type=Path, default=Path("backups"),
                       help="Directory for storing safety backups (default: backups/)")
    parser.add_argument("--parse-vscode", type=Path, metavar="FILE",
                       help="Parse VS Code Problems JSON file (e.g., xUNFIXED) instead of running cargo")
    return parser.parse_args()


class YoFixWhat:
    """Advanced cargo diagnostics collector with structured parsing"""

    def __init__(self, timeout: int = 300, crates: Optional[List[str]] = None,
                 skip_clippy: bool = False, parallel: bool = True, detailed_output: bool = False):
        """Initialize with configurable options and derive framework integration"""
        self.project_root = self._find_project_root()
        self.workspace_members: Set[str] = set()
        self.all_crates: Dict[str, Path] = {}
        self.diagnostics: Dict[str, List[DiagnosticInfo]] = {}
        self.category_stats: Dict[str, Dict[DiagnosticCategory, int]] = {}

        # Configuration options
        self.timeout = timeout
        self.target_crates = set(crates) if crates else None
        self.skip_clippy = skip_clippy
        self.parallel = parallel
        self.detailed_output = detailed_output

        # AI strategy extension tracking
        self.pattern_tracker: Dict[str, Dict[str, int]] = {
            "compiler_errors": {},
            "clippy_lints": {},
            "recurring_patterns": {}
        }
        self.correction_counter = 0

        # 🚀 AI-Augmented Derive Framework Integration
        self.derive_analysis: Dict[str, DeriveFrameworkAnalysis] = {}
        self.semantic_patterns: Dict[str, List[str]] = {}
        self.trait_suggestions: Dict[str, Dict[str, float]] = {}  # crate -> {trait: confidence}
        self.optimization_opportunities: List[str] = []
        self.ai_derive_engine = AIAugmentedDeriveEngine() if DERIVE_ANALYSIS_AVAILABLE else None

        # Ensure all attributes are properly initialized
        if not hasattr(self, 'project_root'):
            self.project_root = self._find_project_root()
        if not hasattr(self, 'workspace_members'):
            self.workspace_members = set()
        if not hasattr(self, 'all_crates'):
            self.all_crates = {}
        if not hasattr(self, 'diagnostics'):
            self.diagnostics = {}
        if not hasattr(self, 'category_stats'):
            self.category_stats = {}

    def _find_project_root(self) -> Path:
        """Auto-discover project root by walking up directory tree"""
        current = Path.cwd()

        while current != current.parent:
            # Check for Cargo.toml (workspace or single crate)
            if (current / "Cargo.toml").exists():
                return current
            # Check for .git directory as fallback
            if (current / ".git").exists():
                # Look for Cargo.toml in this git repo
                for cargo_toml in current.rglob("Cargo.toml"):
                    if cargo_toml.parent == current:
                        return current
            current = current.parent

        # Fallback to current directory
        return Path.cwd()

    def _cargo_env(self) -> Dict[str, str]:
        """Get consistent cargo environment variables"""
        return {
            **os.environ,
            'RUST_BACKTRACE': '0',
            'CARGO_TERM_COLOR': 'never',  # Disable ANSI colors
            'TERM': 'dumb'  # Disable terminal features
        }

    def _run_command(self, cmd: List[str], cwd: Optional[Path] = None) -> Tuple[str, str, int]:
        """Execute command with robust encoding handling for cross-platform compatibility"""
        try:
            env = self._cargo_env()

            # Primary attempt with UTF-8 encoding
            try:
                result = subprocess.run(
                    cmd,
                    cwd=cwd or self.project_root,
                    capture_output=True,
                    text=True,
                    encoding='utf-8',
                    errors='replace',  # Replace invalid characters instead of failing
                    timeout=self.timeout,
                    env=env
                )
                return result.stdout, result.stderr, result.returncode

            except UnicodeDecodeError:
                # Fallback: Try with system encoding and error replacement
                result = subprocess.run(
                    cmd,
                    cwd=cwd or self.project_root,
                    capture_output=True,
                    text=True,
                    encoding=sys.getdefaultencoding(),
                    errors='replace',
                    timeout=self.timeout,
                    env=env
                )
                return result.stdout, result.stderr, result.returncode

        except subprocess.TimeoutExpired:
            logging.warning(f"Command timed out after {self.timeout}s: {' '.join(cmd)}")
            return "", "Command timed out", 1
        except UnicodeDecodeError as e:
            logging.warning(f"Encoding error (trying fallback): {e}")
            # Final fallback: binary mode with manual decoding
            return self._run_command_binary_fallback(cmd, cwd)
        except Exception as e:
            logging.error(f"Command execution failed: {e}")
            return "", f"Command failed: {e}", 1

    def _run_command_binary_fallback(self, cmd: List[str], cwd: Optional[Path] = None) -> Tuple[str, str, int]:
        """Fallback method using binary output with manual UTF-8 decoding"""
        try:
            env = self._cargo_env()

            result = subprocess.run(
                cmd,
                cwd=cwd or self.project_root,
                capture_output=True,
                timeout=self.timeout,
                env=env
            )

            # Manual decoding with multiple fallback strategies
            stdout = self._safe_decode(result.stdout)
            stderr = self._safe_decode(result.stderr)

            return stdout, stderr, result.returncode

        except Exception as e:
            logging.error(f"Binary fallback failed: {e}")
            return "", f"Binary fallback failed: {e}", 1

    def _safe_decode(self, data: bytes) -> str:
        """Safely decode bytes to string with multiple encoding strategies"""
        if not data:
            return ""

        # Try UTF-8 first (most likely for Rust tools)
        try:
            return data.decode('utf-8')
        except UnicodeDecodeError:
            pass

        # Try UTF-8 with error replacement
        try:
            return data.decode('utf-8', errors='replace')
        except UnicodeDecodeError:
            pass

        # Try system default encoding
        try:
            return data.decode(sys.getdefaultencoding(), errors='replace')
        except UnicodeDecodeError:
            pass

        # Final fallback: latin1 (accepts any byte sequence)
        try:
            logging.warning("Using latin-1 fallback for decoding - potential encoding issue")
            # Dump raw bytes for debugging if needed
            if len(data) < 1000:  # Only for small data to avoid huge logs
                logging.debug(f"Raw bytes: {data[:100]}...")
            return data.decode('latin1', errors='replace')
        except UnicodeDecodeError:
            # This should never happen with latin1, but just in case
            logging.error("All decoding strategies failed, using str() fallback")
            return str(data, errors='replace')

    def _discover_workspace_members(self) -> None:
        """Discover workspace members using cargo metadata"""
        try:
            stdout, _stderr, code = self._run_command([
                "cargo", "metadata", "--format-version", "1", "--no-deps", "--quiet"
            ])

            if code == 0:
                metadata = json.loads(stdout)
                workspace_root = Path(metadata.get("workspace_root", self.project_root))

                for package in metadata.get("packages", []):
                    manifest_path = Path(package["manifest_path"])
                    if manifest_path.is_relative_to(workspace_root):
                        crate_name = package["name"]
                        self.workspace_members.add(crate_name)
                        self.all_crates[crate_name] = manifest_path.parent

            else:
                # Fallback: parse Cargo.toml manually
                self._parse_workspace_toml_fallback()

        except (json.JSONDecodeError, Exception):
            self._parse_workspace_toml_fallback()

    def _parse_workspace_toml_fallback(self) -> None:
        """Fallback method to parse workspace Cargo.toml"""
        workspace_toml = self.project_root / "Cargo.toml"
        try:
            with open(workspace_toml, 'r', encoding='utf-8') as f:
                content = f.read()

            # Simple parsing for workspace members
            in_workspace = False
            in_members = False

            for line in content.split('\n'):
                line = line.strip()

                if line.startswith('[workspace'):
                    in_workspace = True
                elif line.startswith('[') and line != '[workspace]':
                    in_workspace = False
                    in_members = False
                elif in_workspace and line.startswith('members'):
                    in_members = True
                elif in_members and line.startswith('"'):
                    # Extract member path
                    member = line.strip('", ')
                    member_path = self.project_root / member
                    if member_path.exists() and (member_path / "Cargo.toml").exists():
                        # Get crate name from member's Cargo.toml
                        crate_name = self._get_crate_name_from_toml(member_path / "Cargo.toml")
                        if crate_name:
                            self.workspace_members.add(crate_name)
                            self.all_crates[crate_name] = member_path

        except Exception:
            pass

    def _get_crate_name_from_toml(self, toml_path: Path) -> Optional[str]:
        """Extract crate name from Cargo.toml with robust parsing"""
        try:
            # Try modern tomllib/tomli first if available
            if tomllib:
                try:
                    content = toml_path.read_text(encoding='utf-8')
                    data = tomllib.loads(content)
                    return data.get("package", {}).get("name")
                except Exception as e:
                    logging.debug(f"TOML parsing failed, falling back to regex: {e}")

            # Fallback to regex-based parsing
            content = toml_path.read_text(encoding='utf-8')

            # Look for [package] section and name field
            in_package_section = False
            for line in content.split('\n'):
                line = line.strip()

                # Check for section headers
                if line.startswith('['):
                    in_package_section = line.startswith('[package]')
                    continue

                # Look for name field in package section
                if in_package_section and line.startswith('name'):
                    # Handle various formats: name = "value", name="value", etc.
                    if '=' in line:
                        value_part = line.split('=', 1)[1].strip()
                        # Remove quotes and whitespace
                        name = value_part.strip('"\'').strip()
                        if name:
                            return name

            return None

        except Exception as e:
            logging.warning(f"Failed to parse {toml_path}: {e}")
            return None

    def _discover_all_crates(self) -> None:
        """Discover all crates in the project"""
        print(f"🔍 Discovering crates in: {self.project_root}")

        # First discover workspace members
        self._discover_workspace_members()

        # If no workspace members found, look for individual Cargo.toml files
        if not self.workspace_members:
            for cargo_toml in self.project_root.rglob("Cargo.toml"):
                if cargo_toml.name == "Cargo.toml":
                    crate_name = self._get_crate_name_from_toml(cargo_toml)
                    if crate_name and crate_name not in self.all_crates:
                        self.all_crates[crate_name] = cargo_toml.parent

        # If still no crates found, check if project root itself is a crate
        if not self.all_crates:
            root_toml = self.project_root / "Cargo.toml"
            if root_toml.exists():
                crate_name = self._get_crate_name_from_toml(root_toml)
                if crate_name:
                    self.all_crates[crate_name] = self.project_root

        print(f"   📦 Found {len(self.all_crates)} crates: {', '.join(self.all_crates.keys())}")

    def _parse_json_diagnostics(self, json_output: str) -> List[DiagnosticInfo]:
        """Parse JSON format diagnostics with enhanced error handling"""
        diagnostics = []

        if not json_output or not json_output.strip():
            return diagnostics

        # Clean the output of any potential encoding artifacts
        json_output = self._clean_json_output(json_output)

        for line_num, line in enumerate(json_output.strip().split('\n'), 1):
            line = line.strip()
            if not line:
                continue

            try:
                data = json.loads(line)

                # Handle different JSON message formats
                message_data = None
                if data.get('reason') == 'compiler-message':
                    message_data = data.get('message', {})
                elif 'message' in data:
                    # Direct message format (sometimes used by clippy)
                    message_data = data
                else:
                    # Skip non-diagnostic messages
                    continue

                # Extract diagnostic information with safe string handling
                diagnostic = DiagnosticInfo(
                    message=self._safe_string(message_data.get('message', '')),
                    severity=self._safe_string(message_data.get('level', 'unknown')),
                    code=self._extract_error_code(message_data),
                    full_context=self._safe_string(line)
                )

                # Extract primary span information with error handling
                spans = message_data.get('spans', [])
                if spans and isinstance(spans, list):
                    try:
                        primary_span = next((s for s in spans if s.get('is_primary')), spans[0])
                        if isinstance(primary_span, dict):
                            diagnostic.file_path = self._safe_string(primary_span.get('file_name'))
                            diagnostic.line_number = self._safe_int(primary_span.get('line_start'))
                            diagnostic.column = self._safe_int(primary_span.get('column_start'))
                            diagnostic.suggestion = self._safe_string(primary_span.get('suggested_replacement'))
                    except (StopIteration, IndexError, TypeError):
                        pass  # Continue without span information

                if diagnostic.message and diagnostic.message.strip():
                    diagnostics.append(diagnostic)

            except json.JSONDecodeError as e:
                print(f"   🔤 JSON decode error on line {line_num}: {e}")
                continue
            except (KeyError, TypeError, AttributeError) as e:
                print(f"   ⚠️  Data structure error on line {line_num}: {e}")
                continue
            except Exception as e:
                print(f"   ❌ Unexpected error parsing line {line_num}: {e}")
                continue

        return diagnostics

    def _clean_json_output(self, output: str) -> str:
        """Clean JSON output of encoding artifacts and invalid characters"""
        if not output:
            return ""

        # Remove common encoding artifacts
        cleaned = output.replace('\ufffd', '')  # Unicode replacement character
        cleaned = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f\x7f-\x9f]', '', cleaned)  # Control characters

        return cleaned

    def _is_valid_json_output(self, output: str) -> bool:
        """Check if output contains valid JSON diagnostic messages"""
        if not output or not output.strip():
            return False

        # Check if any line looks like JSON
        for line in output.strip().split('\n'):
            line = line.strip()
            if line.startswith('{') and line.endswith('}'):
                try:
                    data = json.loads(line)
                    # Check if it's a cargo diagnostic message (broader check)
                    if isinstance(data, dict) and data.get('reason') in [
                        'compiler-message', 'compiler-artifact', 'build-script-executed', 'build-finished'
                    ]:
                        return True
                    # Also check for direct message structure (clippy sometimes uses this)
                    if isinstance(data, dict) and 'message' in data:
                        return True
                except json.JSONDecodeError:
                    continue

        return False

    def _safe_string(self, value) -> str:
        """Safely convert value to string with encoding cleanup"""
        if value is None:
            return ""
        if isinstance(value, str):
            return value.encode('utf-8', errors='replace').decode('utf-8')
        return str(value)

    def _safe_int(self, value) -> Optional[int]:
        """Safely convert value to int"""
        if value is None:
            return None
        try:
            return int(value)
        except (ValueError, TypeError):
            return None

    def _extract_error_code(self, message_data: dict) -> Optional[str]:
        """Safely extract error code from message data"""
        try:
            code_data = message_data.get('code')
            if isinstance(code_data, dict):
                return self._safe_string(code_data.get('code'))
            elif isinstance(code_data, str):
                return self._safe_string(code_data)
        except (TypeError, AttributeError):
            pass
        return None

    def _parse_text_diagnostics(self, text_output: str) -> List[DiagnosticInfo]:
        """Parse text format diagnostics with enhanced context capture for --deets mode"""
        diagnostics = []
        lines = text_output.split('\n')
        i = 0

        # Patterns for different diagnostic formats
        warning_pattern = re.compile(r'^(warning|error|note|help):\s*(.+)$')
        location_pattern = re.compile(r'^\s*-->\s*([^:]+):(\d+):(\d+)$')
        code_pattern = re.compile(r'= note: `#\[warn\(([^)]+)\)\]`|= help: for further information visit.*#([^)]+)\)')
        # Enhanced pattern for compilation error codes (e.g., E0308, E0277)
        error_code_pattern = re.compile(r'error\[([EW]\d+)\]:')
        # Pattern for standalone error lines with codes
        standalone_error_pattern = re.compile(r'^error\[([EW]\d+)\]:\s*(.+)$')

        while i < len(lines):
            line = lines[i].strip()

            # Skip empty lines and compilation status
            if not line or any(skip in line.lower() for skip in [
                'compiling', 'finished', 'checking', 'building', 'generated', 'warnings'
            ]):
                i += 1
                continue

            # Match diagnostic lines (including standalone error patterns)
            warning_match = warning_pattern.match(line)
            standalone_error_match = standalone_error_pattern.match(line)

            if warning_match or standalone_error_match:
                if warning_match:
                    severity = warning_match.group(1)
                    message = warning_match.group(2)
                    # Check for compilation error codes (E0308, E0277, etc.)
                    error_code_match = error_code_pattern.search(line)
                    error_code = error_code_match.group(1) if error_code_match else None
                elif standalone_error_match:  # standalone_error_match
                    severity = "error"
                    error_code = standalone_error_match.group(1)
                    message = standalone_error_match.group(2)
                else:
                    continue  # Should not happen, but safety check

                diagnostic = DiagnosticInfo(
                    message=message,
                    severity=severity,
                    code=error_code,
                    full_context=line
                )

                # Look for location information in subsequent lines
                j = i + 1
                context_lines = [line]

                # Enhanced context capture for detailed output
                max_lookahead = 50 if self.detailed_output else 10
                empty_line_tolerance = 5 if self.detailed_output else 3

                while j < len(lines) and j < i + max_lookahead:
                    next_line = lines[j]  # Keep original line with whitespace for context
                    next_line_stripped = next_line.strip()

                    # Check for location
                    location_match = location_pattern.match(next_line_stripped)
                    if location_match:
                        diagnostic.file_path = location_match.group(1)
                        diagnostic.line_number = int(location_match.group(2))
                        diagnostic.column = int(location_match.group(3))
                        context_lines.append(next_line)

                    # Check for error code
                    code_match = code_pattern.search(next_line_stripped)
                    if code_match:
                        diagnostic.code = code_match.group(1) or code_match.group(2)
                        context_lines.append(next_line)

                    # Check for suggestions
                    if 'help:' in next_line_stripped and ('consider' in next_line_stripped or 'try' in next_line_stripped):
                        diagnostic.suggestion = next_line_stripped.split('help:')[1].strip()
                        context_lines.append(next_line)

                    # Enhanced stopping conditions for detailed mode
                    if self.detailed_output:
                        # In detailed mode, capture more context including code snippets
                        # Stop only when we hit another diagnostic
                        if warning_pattern.match(next_line_stripped):
                            break
                        # Allow more empty lines in detailed mode
                        context_lines.append(next_line)
                    else:
                        # Original behavior for non-detailed mode
                        if not next_line_stripped:
                            j += 1
                            continue

                        # Stop if we hit another diagnostic or empty section
                        if warning_pattern.match(next_line_stripped) or (next_line_stripped == '' and j > i + empty_line_tolerance):
                            break

                        context_lines.append(next_line)

                    j += 1

                diagnostic.full_context = '\n'.join(context_lines)
                diagnostics.append(diagnostic)
                i = j
            else:
                i += 1

        return diagnostics

    def _categorize_diagnostic(self, diagnostic: DiagnosticInfo) -> DiagnosticCategory:
        """Categorize diagnostic based on content and error codes"""
        message = diagnostic.message.lower()
        code = (diagnostic.code or '').lower()

        # ⚠️ **ARCHITECTURE EXCLUSION LIST** ⚠️
        # These warnings are INTENTIONAL DESIGN CHOICES and should be IGNORED by our tools
        # DO NOT categorize or report these - they are protected by cargo config
        ARCHITECTURE_PROTECTED_WARNINGS = {
            'unnecessary_wraps',           # REQUIRED: Result wrappers essential for error system
            'needless_pass_by_value',      # REQUIRED: Owned values needed for corrections
            'single_match_else',           # REQUIRED: Match patterns clearer than if-let
            'manual_let_else',             # REQUIRED: Manual patterns more explicit
            'match_wildcard_for_single_variants',  # REQUIRED: Wildcards intentional
            'ptr_arg',                     # REQUIRED: Vec parameters needed for strategies
            'too_many_arguments',          # REQUIRED: Error correction needs many params
            'module_name_repetitions',     # REQUIRED: Yoshi naming patterns intentional
            'similar_names',               # REQUIRED: Error/correction naming patterns
            'struct_excessive_bools',      # REQUIRED: Configuration structs need bools
            'fn_params_excessive_bools',   # REQUIRED: Correction functions need bool flags
        }

        # Check if this diagnostic is architecture-protected
        if code and any(protected in code for protected in ARCHITECTURE_PROTECTED_WARNINGS):
            # SKIP this diagnostic entirely - it's an intentional design choice
            logging.debug(f"🛡️ ARCHITECTURE PROTECTED: Skipping {code} - {message[:50]}...")
            return DiagnosticCategory.ARCHITECTURE_PROTECTED  # Signal to skip this diagnostic

        if any(protected in message for protected in ARCHITECTURE_PROTECTED_WARNINGS):
            # SKIP this diagnostic entirely - it's an intentional design choice
            logging.debug(f"🛡️ ARCHITECTURE PROTECTED: Skipping message - {message[:50]}...")
            return DiagnosticCategory.ARCHITECTURE_PROTECTED  # Signal to skip this diagnostic

        # Compilation error codes categorization
        if diagnostic.code:
            error_code = diagnostic.code.upper()

            # Safety-critical compilation errors
            if error_code in ['E0133', 'E0506', 'E0507', 'E0382', 'E0384', 'E0596']:
                return DiagnosticCategory.SAFETY

            # Type/correctness errors
            if error_code in ['E0308', 'E0277', 'E0271', 'E0282', 'E0283', 'E0599', 'E0609']:
                return DiagnosticCategory.CORRECTNESS

            # Performance-related errors
            if error_code in ['E0507', 'E0382']:  # Move/borrow issues that affect performance
                return DiagnosticCategory.PERFORMANCE

        # Safety issues (clippy and general)
        if any(keyword in message for keyword in [
            'unwrap', 'expect', 'unsafe', 'indexing', 'panic', 'unreachable'
        ]) or any(keyword in code for keyword in [
            'unwrap', 'indexing', 'panic'
        ]):
            return DiagnosticCategory.SAFETY

        # Performance issues
        if any(keyword in message for keyword in [
            'inefficient', 'allocation', 'clone', 'copy', 'capacity', 'slow'
        ]) or any(keyword in code for keyword in [
            'performance', 'inefficient', 'clone'
        ]):
            return DiagnosticCategory.PERFORMANCE

        # Documentation issues
        if any(keyword in message for keyword in [
            'missing documentation', 'doc comment', 'backticks'
        ]) or any(keyword in code for keyword in [
            'missing_docs', 'doc_markdown'
        ]):
            return DiagnosticCategory.DOCUMENTATION

        # Style issues
        if any(keyword in message for keyword in [
            'format', 'style', 'naming', 'convention', 'redundant'
        ]) or any(keyword in code for keyword in [
            'style', 'format', 'naming'
        ]):
            return DiagnosticCategory.STYLE

        # Deprecated issues
        if any(keyword in message for keyword in [
            'deprecated', 'obsolete', 'superseded'
        ]):
            return DiagnosticCategory.DEPRECATED

        # Default to correctness
        return DiagnosticCategory.CORRECTNESS

    def _run_cargo_diagnostics(self, crate_name: str, command_type: str) -> List[DiagnosticInfo]:
        """Run cargo command with robust error handling and multiple parsing strategies"""
        print(f"   🔧 Running {command_type} on {crate_name}...", end='')

        diagnostics = []

        # Strategy 1: Check if crate compiles first
        try:
            # First try a basic compilation check
            basic_cmd = ["cargo", "check", "-p", crate_name, "--quiet"]
            stdout, stderr, code = self._run_command(basic_cmd)

            # If compilation fails, we need to handle it differently
            if code != 0:
                print(f" ⚠️ Compilation issues detected, trying workspace fallback...")

                # Try workspace-wide check as fallback
                try:
                    # Don't use --quiet for workspace check to get full error details
                    workspace_cmd = ["cargo", "check", "--workspace"]
                    ws_stdout, ws_stderr, ws_code = self._run_command(workspace_cmd)

                    # Parse workspace compilation errors from both stdout and stderr
                    full_workspace_output = f"{ws_stdout}\n{ws_stderr}".strip()
                    if full_workspace_output:
                        workspace_diagnostics = self._parse_text_diagnostics(full_workspace_output)
                        if workspace_diagnostics:
                            # Filter errors for the specific crate or return all if filtering fails
                            crate_diagnostics = [diag for diag in workspace_diagnostics
                                               if diag.file_path and crate_name in diag.file_path]
                            if crate_diagnostics:
                                diagnostics.extend(crate_diagnostics)
                                print(f" ✅ Workspace compilation errors parsed ({len(crate_diagnostics)} for {crate_name})")
                                return self._filter_diagnostics(diagnostics)
                            else:
                                # If no crate-specific errors found, include all workspace errors
                                diagnostics.extend(workspace_diagnostics)
                                print(f" ✅ Workspace compilation errors parsed (all {len(workspace_diagnostics)} issues)")
                                return self._filter_diagnostics(diagnostics)

                except Exception as ws_e:
                    print(f" ⚠️ Workspace check also failed: {ws_e}")

                # If workspace check fails, still parse the original compilation errors
                if stderr:
                    compile_diagnostics = self._parse_text_diagnostics(stderr)
                    if compile_diagnostics:
                        diagnostics.extend(compile_diagnostics)
                        print(f" ✅ Original compilation errors parsed")
                        return self._filter_diagnostics(diagnostics)
        except Exception as e:
            print(f" ⚠️ Basic check failed: {e}")

        # Strategy 2: JSON format with encoding-safe execution (only if compilation works)
        # Strategy 2: JSON format (preferred for structured parsing)
        try:
            if command_type == "clippy":
                cmd = ["cargo", "clippy", "-p", crate_name, "--message-format=json", "--", "-D", "warnings"]
            else:  # check
                cmd = ["cargo", "check", "-p", crate_name, "--message-format=json"]

            stdout, stderr, code = self._run_command(cmd)

            # Check if we got valid JSON output
            if stdout and stdout.strip():
                # Validate that it's actually JSON before parsing
                if self._is_valid_json_output(stdout):
                    json_diagnostics = self._parse_json_diagnostics(stdout)
                    if json_diagnostics:
                        diagnostics.extend(json_diagnostics)
                        print(f" ✅ JSON parsed")
                        return self._filter_diagnostics(diagnostics)
                else:
                    # If not JSON, treat as text
                    text_diagnostics = self._parse_text_diagnostics(stdout)
                    if text_diagnostics:
                        diagnostics.extend(text_diagnostics)
                        print(f" ✅ Text parsed (stdout)")
                        return self._filter_diagnostics(diagnostics)

            # Check stderr for diagnostics
            if stderr and stderr.strip():
                if self._is_valid_json_output(stderr):
                    json_diagnostics_stderr = self._parse_json_diagnostics(stderr)
                    if json_diagnostics_stderr:
                        diagnostics.extend(json_diagnostics_stderr)
                        print(f" ✅ JSON parsed (stderr)")
                        return self._filter_diagnostics(diagnostics)
                else:
                    text_diagnostics_stderr = self._parse_text_diagnostics(stderr)
                    if text_diagnostics_stderr:
                        diagnostics.extend(text_diagnostics_stderr)
                        print(f" ✅ Text parsed (stderr)")
                        return self._filter_diagnostics(diagnostics)

        except Exception as e:
            print(f" ⚠️ JSON strategy failed: {e}")

        # Strategy 3: Text format fallback
        try:
            if command_type == "clippy":
                cmd = ["cargo", "clippy", "-p", crate_name]
            else:  # check
                cmd = ["cargo", "check", "-p", crate_name]

            stdout, stderr, code = self._run_command(cmd)
            full_output = f"{stdout}\n{stderr}".strip()

            if full_output:
                text_diagnostics = self._parse_text_diagnostics(full_output)
                if text_diagnostics:
                    diagnostics.extend(text_diagnostics)
                    print(f" ✅ Text parsed")
                    return self._filter_diagnostics(diagnostics)

        except Exception as e:
            print(f" ⚠️ Text strategy failed: {e}")

        # If no diagnostics found, that might mean the crate is clean
        print(f" ✅ No issues found")
        return []

    def _filter_diagnostics(self, diagnostics: List[DiagnosticInfo]) -> List[DiagnosticInfo]:
        """Filter out low-value diagnostics while preserving actionable ones"""
        filtered_diagnostics = []
        for diag in diagnostics:
            # Skip help messages and notes that don't provide actionable information
            if diag.severity in ['help', 'note'] and not any(keyword in diag.message.lower() for keyword in [
                'consider', 'try', 'use', 'change', 'remove', 'add', 'replace', 'instead'
            ]):
                continue

            # Skip overly verbose internal compiler messages
            if any(skip_phrase in diag.message.lower() for skip_phrase in [
                'the lint level is defined here',
                'for more information about this error',
                'run with `rust_backtrace=1`'
            ]):
                continue

            filtered_diagnostics.append(diag)

        return filtered_diagnostics

    def parse_vscode_problems_json(self, json_file: Path) -> Dict[str, List[DiagnosticInfo]]:
        """Parse VS Code Problems JSON file (like xUNFIXED) into categorized diagnostics"""
        print(f"🔍 Parsing VS Code Problems JSON: {json_file}")

        try:
            with open(json_file, 'r', encoding='utf-8') as f:
                content = f.read().strip()

            # Handle both array format and line-by-line JSON objects
            if content.startswith('['):
                # Array format
                problems = json.loads(content)
            else:
                # Line-by-line JSON objects (like cargo JSON output)
                problems = []
                for line in content.split('\n'):
                    line = line.strip()
                    if line and line.startswith('{'):
                        try:
                            problems.append(json.loads(line))
                        except json.JSONDecodeError:
                            continue

            # Convert VS Code problems to our DiagnosticInfo format
            diagnostics_by_crate = {}

            for problem in problems:
                if not isinstance(problem, dict):
                    continue

                # Extract file path and determine crate
                resource = problem.get('resource', '')
                if not resource:
                    continue

                # Normalize path and extract crate name
                file_path = resource.replace('\\', '/').replace('/c:/_Repos/Yoshi Copilot/', '')
                crate_name = self._extract_crate_from_path(file_path)

                if not crate_name:
                    continue

                # Convert VS Code severity to our format
                severity_map = {
                    8: 'error',    # VS Code Error
                    4: 'warning',  # VS Code Warning
                    2: 'info',     # VS Code Info
                    1: 'hint'      # VS Code Hint
                }

                severity = severity_map.get(problem.get('severity', 4), 'warning')

                # Extract error code from message if present
                message = problem.get('message', '')
                error_code = None
                if '[E' in message and ']' in message:
                    # Extract error codes like [E0433]
                    import re
                    code_match = re.search(r'\[([EW]\d+)\]', message)
                    if code_match:
                        error_code = code_match.group(1)
                        # Clean the message
                        message = re.sub(r'\[[EW]\d+\]\s*', '', message)

                # Create diagnostic info
                diagnostic = DiagnosticInfo(
                    message=message,
                    severity=severity,
                    code=error_code,
                    file_path=file_path,
                    line_number=problem.get('startLineNumber'),
                    column=problem.get('startColumn'),
                    full_context=json.dumps(problem)
                )

                # Add to crate's diagnostics
                if crate_name not in diagnostics_by_crate:
                    diagnostics_by_crate[crate_name] = []
                diagnostics_by_crate[crate_name].append(diagnostic)

            print(f"   📊 Parsed {sum(len(diags) for diags in diagnostics_by_crate.values())} problems from {len(diagnostics_by_crate)} crates")
            return diagnostics_by_crate

        except Exception as e:
            print(f"   ❌ Failed to parse VS Code JSON: {e}")
            return {}

    def _extract_crate_from_path(self, file_path: str) -> Optional[str]:
        """Extract crate name from file path"""
        # Handle paths like "yoshi-derive/tests/test.rs" -> "yoshi-derive"
        parts = file_path.split('/')
        if len(parts) > 0:
            first_part = parts[0]
            # Check if it's a known crate name pattern
            if any(first_part.startswith(prefix) for prefix in ['yoshi-', 'yoshi']):
                return first_part
            # Handle nested paths like "yoshi-derive/yoshi-derive/tests/test.rs"
            if len(parts) > 1 and parts[1].startswith('yoshi'):
                return parts[1]
        return None

    def _parse_simple_diagnostics(self, output: str, _crate_name: str) -> List[DiagnosticInfo]:
        """Simple diagnostic parsing for when other methods fail"""
        diagnostics = []

        # Basic patterns that should work even with encoding issues
        for line in output.split('\n'):
            line = line.strip()
            if not line:
                continue

            # Look for basic warning/error patterns
            if any(pattern in line.lower() for pattern in ['warning:', 'error:']):
                severity = 'warning' if 'warning:' in line.lower() else 'error'

                # Extract message (everything after the severity marker)
                if ':' in line:
                    message = line.split(':', 1)[1].strip()
                    if message:
                        diagnostics.append(DiagnosticInfo(
                            message=message,
                            severity=severity,
                            full_context=line
                        ))

        return diagnostics

    def _collect_diagnostics_for_crate(self, crate_name: str) -> None:
        """Collect comprehensive diagnostics for a specific crate with derive framework analysis"""
        print(f"🔧 Analyzing crate: {crate_name}")

        all_diagnostics = []

        # Run clippy (unless skipped)
        if not self.skip_clippy:
            clippy_results = self._run_cargo_diagnostics(crate_name, "clippy")
            all_diagnostics.extend(clippy_results)

            # Track patterns for AI strategy extension
            self._track_diagnostic_patterns(clippy_results, "clippy")
        else:
            logging.debug(f"Skipping clippy analysis for {crate_name}")

        # Run check (may catch different issues)
        check_results = self._run_cargo_diagnostics(crate_name, "check")

        # Track patterns for AI strategy extension
        self._track_diagnostic_patterns(check_results, "compiler")

        # 🚀 AI-Augmented Derive Framework Analysis
        derive_analysis = self._analyze_derive_opportunities(crate_name)
        if derive_analysis:
            self.derive_analysis[crate_name] = derive_analysis
            # Convert derive suggestions to diagnostics
            derive_diagnostics = self._convert_derive_analysis_to_diagnostics(derive_analysis)
            all_diagnostics.extend(derive_diagnostics)
            print(f"   🧬 AI Derive Analysis: {len(derive_analysis.suggested_derives)} suggestions")

        # Deduplicate diagnostics (check may repeat clippy issues)
        existing_messages = {diag.message for diag in all_diagnostics}
        for diag in check_results:
            if diag.message not in existing_messages:
                all_diagnostics.append(diag)

        # Categorize diagnostics for statistics and FILTER OUT architecture-protected ones
        category_counts = {category: 0 for category in DiagnosticCategory}
        filtered_diagnostics = []

        for diag in all_diagnostics:
            category = self._categorize_diagnostic(diag)

            # SKIP architecture-protected diagnostics entirely
            if category == DiagnosticCategory.ARCHITECTURE_PROTECTED:
                logging.debug(f"🛡️ FILTERED OUT architecture-protected diagnostic: {diag.message[:50]}...")
                continue

            category_counts[category] += 1
            filtered_diagnostics.append(diag)

        self.category_stats[crate_name] = category_counts

        # Store results (ONLY non-architecture-protected diagnostics)
        if filtered_diagnostics:
            self.diagnostics[crate_name] = filtered_diagnostics
            print(f"   📊 Found {len(filtered_diagnostics)} issues")
        else:
            self.diagnostics[crate_name] = []
            print(f"   ✅ No issues found!")

    def _analyze_derive_opportunities(self, crate_name: str) -> Optional[DeriveFrameworkAnalysis]:
        """Analyze derive opportunities for a specific crate using AI"""
        if not self.ai_derive_engine:
            return None

        try:
            # Find Rust source files for this crate
            crate_path = self.all_crates.get(crate_name)
            if not crate_path:
                return None

            rust_files = list(crate_path.rglob("*.rs"))
            if not rust_files:
                return None

            # Analyze the main lib.rs or src/main.rs
            main_file = None
            for candidate in ["lib.rs", "main.rs", "mod.rs"]:
                potential = crate_path / "src" / candidate
                if potential.exists():
                    main_file = potential
                    break

            if not main_file:
                main_file = rust_files[0]  # Fallback to first .rs file

            with open(main_file, 'r', encoding='utf-8') as f:
                rust_code = f.read()

            # Find struct definitions
            import re
            struct_pattern = r'struct\s+(\w+)\s*[<\w\s,>]*\s*{'
            structs = re.findall(struct_pattern, rust_code)

            if not structs:
                return None

            # Analyze the first struct found (could be extended to analyze all)
            first_struct = structs[0]
            analysis = self.ai_derive_engine.analyze_struct_semantics(rust_code, first_struct)

            return analysis

        except Exception as e:
            logging.warning(f"Derive analysis failed for {crate_name}: {e}")
            return None

    def _convert_derive_analysis_to_diagnostics(self, analysis: DeriveFrameworkAnalysis) -> List[DiagnosticInfo]:
        """Convert derive framework analysis to diagnostic format"""
        diagnostics = []

        # Create diagnostic for missing derives
        if analysis.suggested_derives:
            diagnostic = DiagnosticInfo(
                message=f"Consider adding derives for {analysis.struct_name}: {', '.join(analysis.suggested_derives)}",
                severity="info",
                code="derive_suggestion",
                suggestion=f"Add #[derive({', '.join(analysis.suggested_derives)})] to {analysis.struct_name}",
                derive_suggestions=analysis.suggested_derives,
                semantic_category="trait_inference",
                trait_inference_confidence=analysis.ai_confidence,
                optimization_potential=analysis.performance_impact
            )
            diagnostics.append(diagnostic)

        # Create diagnostics for optimization hints
        for hint in analysis.optimization_hints:
            diagnostic = DiagnosticInfo(
                message=hint,
                severity="note",
                code="optimization_hint",
                suggestion=hint,
                semantic_category="optimization_derive",
                trait_inference_confidence=analysis.ai_confidence,
                optimization_potential="performance"
            )
            diagnostics.append(diagnostic)

        return diagnostics

    def _track_diagnostic_patterns(self, diagnostics: List[DiagnosticInfo], diagnostic_type: str) -> None:
        """Track diagnostic patterns for AI strategy extension"""
        for diag in diagnostics:
            # Extract error/lint codes for pattern tracking
            if diag.code:
                key = f"{diagnostic_type}_lints" if diagnostic_type == "clippy" else "compiler_errors"
                self.pattern_tracker[key][diag.code] = self.pattern_tracker[key].get(diag.code, 0) + 1

            # Track recurring message patterns
            msg_pattern = self._extract_message_pattern(diag.message)
            if msg_pattern:
                self.pattern_tracker["recurring_patterns"][msg_pattern] = \
                    self.pattern_tracker["recurring_patterns"].get(msg_pattern, 0) + 1

    def _extract_message_pattern(self, message: str) -> str:
        """Extract generalizable pattern from error/lint message"""
        # Remove specific identifiers to find patterns
        import re
        # Replace specific names with placeholders
        pattern = re.sub(r'`[^`]+`', '`<IDENTIFIER>`', message)
        pattern = re.sub(r'\d+', '<NUMBER>', pattern)
        pattern = re.sub(r'"[^"]*"', '"<STRING>"', pattern)
        return pattern[:100]  # Limit length for pattern matching

    def _generate_strategy_extension_suggestions(self) -> List[str]:
        """Generate suggestions for extending strategy files based on tracked patterns"""
        suggestions = []

        # Find frequently occurring patterns
        for category, patterns in self.pattern_tracker.items():
            sorted_patterns = sorted(patterns.items(), key=lambda x: x[1], reverse=True)
            for pattern, count in sorted_patterns:
                if count >= 3:  # Threshold for pattern significance
                    if category == "compiler_errors":
                        suggestions.append(f"Consider extending error_correction.rs with strategy for {pattern} (occurred {count} times)")
                    elif category == "clippy_lints":
                        suggestions.append(f"Consider extending flawless_clippy.rs with strategy for {pattern} (occurred {count} times)")
                    elif category == "recurring_patterns":
                        suggestions.append(f"Recurring pattern: {pattern} (occurred {count} times)")

        return suggestions

    def _run_parallel_analysis(self, crates: List[str]) -> None:
        """🚀 TURBO MODE: Ultra-fast parallel analysis using async processing"""
        print(f"🚀 TURBO MODE: Running ultra-fast parallel analysis on {len(crates)} crates...")
        print(f"💻 Using {multiprocessing.cpu_count()} CPU cores for maximum performance")

        start_time = time.time()

        # Run async turbo analysis
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        try:
            loop.run_until_complete(self._turbo_analyze_all_crates(crates))
        finally:
            loop.close()

        analysis_time = time.time() - start_time
        print(f"⚡ TURBO analysis completed in {analysis_time:.2f}s ({len(crates)/analysis_time:.1f} crates/sec)")

    async def _turbo_analyze_all_crates(self, crates: List[str]) -> None:
        """🚀 TURBO: Async parallel analysis of all crates"""
        semaphore = asyncio.Semaphore(multiprocessing.cpu_count() * 2)  # Allow oversubscription

        tasks = []
        for crate in crates:
            task = self._turbo_analyze_single_crate(crate, semaphore)
            tasks.append(task)

        # Run all crates in parallel
        results = await asyncio.gather(*tasks, return_exceptions=True)

        # Process results
        for i, result in enumerate(results):
            crate = crates[i]
            if isinstance(result, Exception):
                print(f"❌ TURBO analysis failed for {crate}: {result}")
                self.diagnostics[crate] = []
            elif isinstance(result, list):
                self.diagnostics[crate] = result
                if result:
                    print(f"   📊 Found {len(result)} issues")
                else:
                    print(f"   ✅ No issues found!")
            else:
                self.diagnostics[crate] = []

    async def _turbo_analyze_single_crate(self, crate: str, semaphore: asyncio.Semaphore) -> List[DiagnosticInfo]:
        """🚀 TURBO: Async analysis of a single crate"""
        async with semaphore:
            try:
                print(f"🔧 TURBO analyzing crate: {crate}")

                # Run clippy and check in parallel
                tasks = []
                if not self.skip_clippy:
                    clippy_task = self._run_cargo_async(['cargo', 'clippy', '-p', crate, '--message-format=json'])
                    tasks.append(clippy_task)

                check_task = self._run_cargo_async(['cargo', 'check', '-p', crate, '--message-format=json'])
                tasks.append(check_task)

                results = await asyncio.gather(*tasks, return_exceptions=True)

                # Parse all results
                all_diagnostics = []
                for result in results:
                    if not isinstance(result, Exception) and isinstance(result, tuple) and len(result) >= 3:
                        stdout, stderr, _ = result
                        # Parse JSON output super fast
                        diagnostics = self._parse_json_turbo(stdout) + self._parse_json_turbo(stderr)
                        all_diagnostics.extend(diagnostics)

                return all_diagnostics

            except Exception as e:
                print(f"❌ TURBO error for {crate}: {e}")
                return []

    async def _run_cargo_async(self, cmd: List[str]) -> tuple:
        """🚀 TURBO: Async cargo command execution"""
        process = await asyncio.create_subprocess_exec(
            *cmd,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
            cwd=self.project_root
        )

        stdout, stderr = await process.communicate()

        return (
            stdout.decode('utf-8', errors='ignore'),
            stderr.decode('utf-8', errors='ignore'),
            process.returncode
        )

    def _parse_json_turbo(self, json_output: str) -> List[DiagnosticInfo]:
        """🚀 TURBO: Ultra-fast JSON parsing optimized for cargo output"""
        diagnostics = []

        if not json_output.strip():
            return diagnostics

        # Process line by line for memory efficiency
        for line in json_output.strip().split('\n'):
            line = line.strip()
            if not line or not line.startswith('{'):
                continue

            try:
                data = json.loads(line)

                # Handle different JSON message formats
                message_data = None
                if data.get('reason') == 'compiler-message':
                    message_data = data.get('message', {})
                elif 'message' in data:
                    message_data = data
                else:
                    continue

                # Fast diagnostic creation
                if message_data and message_data.get('level') in ['error', 'warning']:
                    diagnostic = DiagnosticInfo(
                        message=message_data.get('message', ''),
                        severity=message_data.get('level', 'unknown'),
                        code=self._extract_error_code_turbo(message_data),
                        full_context=line
                    )
                    diagnostics.append(diagnostic)

            except json.JSONDecodeError:
                continue

        return diagnostics

    def _extract_error_code_turbo(self, message: dict) -> str:
        """🚀 TURBO: Fast error code extraction"""
        code = message.get('code')
        if isinstance(code, dict):
            return code.get('code', '')
        return str(code) if code else ''

    def _run_sequential_analysis(self, crates: List[str]) -> None:
        """Run analysis sequentially with progress tracking"""
        for crate_name in tqdm(crates, desc="Analyzing crates"):
            self._collect_diagnostics_for_crate(crate_name)
            print()

    def _format_diagnostic(self, diag: DiagnosticInfo) -> str:
        """Format a single diagnostic for display"""
        lines = []

        # Header with severity and message
        severity_emoji = {
            'error': '❌',
            'warning': '⚠️',
            'note': '📝',
            'help': '💡'
        }.get(diag.severity, '🔹')

        header = f"{severity_emoji} {diag.severity.upper()}"
        if diag.code:
            header += f" [{diag.code}]"
        header += f": {diag.message}"
        lines.append(header)

        # Location information
        if diag.file_path and diag.line_number:
            location = f"   📍 {diag.file_path}:{diag.line_number}"
            if diag.column:
                location += f":{diag.column}"
            lines.append(location)

        # Include detailed output if requested
        if self.detailed_output and diag.full_context:
            lines.append("")
            lines.append("   📋 DETAILED CLIPPY OUTPUT:")
            lines.append("")
            # Preserve original formatting for code snippets
            context_lines = diag.full_context.split('\n')
            for line in context_lines:
                # Only add minimal indentation to preserve code structure
                if line.strip():
                    lines.append(f"   {line}")
                else:
                    lines.append("")  # Preserve empty lines for readability
            lines.append("")

        # Suggestion if available
        if diag.suggestion and diag.suggestion != diag.message:
            lines.append(f"   💡 Suggestion: {diag.suggestion}")

        return '\n'.join(lines)

    def _generate_report(self) -> str:
        """Generate comprehensive yoFixME.txt report with enhanced formatting"""
        report_lines = []

        # Meta-prompt for IDE AI integration
        meta_prompt = self._generate_ai_meta_prompt()
        report_lines.extend(meta_prompt)

        # Header
        report_lines.extend([
            "# yoFixME.txt - Comprehensive Cargo Diagnostics Report",
            f"# Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}",
            f"# Project Root: {self.project_root}",
            f"# Crates Analyzed: {len(self.all_crates)}",
            "",
            "═" * 80,
            ""
        ])

        # Enhanced summary with categorization
        total_issues = sum(len(diags) for diags in self.diagnostics.values())
        clean_crates = len([k for k, v in self.diagnostics.items() if not v])

        # Category statistics
        total_by_category = {category: 0 for category in DiagnosticCategory}
        for crate_stats in self.category_stats.values():
            for category, count in crate_stats.items():
                total_by_category[category] += count

        report_lines.extend([
            "## EXECUTIVE SUMMARY",
            f"   📦 Total Crates: {len(self.all_crates)}",
            f"   🎯 Total Issues: {total_issues}",
            f"   ✅ Clean Crates: {clean_crates}",
            f"   🔧 Crates Needing Attention: {len(self.all_crates) - clean_crates}",
            "",
            "### ISSUE BREAKDOWN BY CATEGORY",
        ])

        for category, count in total_by_category.items():
            # SKIP architecture-protected category from reports
            if count > 0 and category != DiagnosticCategory.ARCHITECTURE_PROTECTED:
                category_emoji = {
                    DiagnosticCategory.SAFETY: "🛡️",
                    DiagnosticCategory.PERFORMANCE: "⚡",
                    DiagnosticCategory.STYLE: "🎨",
                    DiagnosticCategory.CORRECTNESS: "✅",
                    DiagnosticCategory.DOCUMENTATION: "📚",
                    DiagnosticCategory.DEPRECATED: "🚫"
                }.get(category, "🔹")

                report_lines.append(f"   {category_emoji} {category.value.title()}: {count} issues")

        report_lines.extend(["", "═" * 80, ""])

        # Per-crate sections with enhanced formatting
        for crate_name, diagnostics in self.diagnostics.items():
            crate_path = self.all_crates.get(crate_name, "Unknown")

            # Crate header
            report_lines.extend([
                f"## CRATE: {crate_name}",
                f"   📂 Path: {crate_path}",
                f"   🎯 Total Issues: {len(diagnostics)}",
            ])

            # Category breakdown for this crate (EXCLUDE architecture-protected)
            if crate_name in self.category_stats:
                crate_categories = self.category_stats[crate_name]
                category_summary = []
                for category, count in crate_categories.items():
                    # SKIP architecture-protected category from crate breakdowns
                    if count > 0 and category != DiagnosticCategory.ARCHITECTURE_PROTECTED:
                        category_summary.append(f"{category.value}: {count}")

                if category_summary:
                    report_lines.append(f"   📊 Breakdown: {', '.join(category_summary)}")

            report_lines.extend(["", "─" * 60, ""])

            if not diagnostics:
                report_lines.append("✅ No issues found!")
            else:
                # Group diagnostics by category for better organization
                by_category = {category: [] for category in DiagnosticCategory}
                for diag in diagnostics:
                    category = self._categorize_diagnostic(diag)
                    by_category[category].append(diag)

                # Display by category (EXCLUDE architecture-protected)
                for category in DiagnosticCategory:
                    # SKIP architecture-protected category from detailed display
                    if category == DiagnosticCategory.ARCHITECTURE_PROTECTED:
                        continue

                    category_diags = by_category[category]
                    if not category_diags:
                        continue

                    category_emoji = {
                        DiagnosticCategory.SAFETY: "🛡️",
                        DiagnosticCategory.PERFORMANCE: "⚡",
                        DiagnosticCategory.STYLE: "🎨",
                        DiagnosticCategory.CORRECTNESS: "✅",
                        DiagnosticCategory.DOCUMENTATION: "📚",
                        DiagnosticCategory.DEPRECATED: "🚫"
                    }.get(category, "🔹")

                    report_lines.extend([
                        f"### {category_emoji} {category.value.upper()} ISSUES ({len(category_diags)})",
                        ""
                    ])

                    for i, diag in enumerate(category_diags, 1):
                        report_lines.append(f"{i}. {self._format_diagnostic(diag)}")
                        report_lines.append("")

            report_lines.extend(["═" * 80, ""])

        # Enhanced footer with actionable recommendations
        report_lines.extend([
            "",
            "## RECOMMENDATIONS",
            "",
            "### PRIORITY ORDER FOR FIXES:",
            "1. 🛡️  **Safety Issues** - Address unwrap(), indexing, and unsafe patterns first",
            "2. ✅ **Correctness Issues** - Fix logic errors and type problems",
            "3. ⚡ **Performance Issues** - Optimize inefficient patterns and allocations",
            "4. 📚 **Documentation Issues** - Add missing docs and fix formatting",
            "5. 🎨 **Style Issues** - Apply formatting and naming convention fixes",
            "6. 🚫 **Deprecated Issues** - Update to modern APIs and patterns",
            "",
            "### AUTOMATION SUGGESTIONS:",
            "• Run `cargo clippy --fix` for auto-fixable issues",
            "• Use `cargo fmt` for formatting standardization",
            "• Consider enabling `#![deny(clippy::unwrap_used)]` for safety",
            "• Set up pre-commit hooks for continuous quality enforcement",
            "",
            f"### GENERATION COMPLETE",
            f"   🕒 Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}",
            f"   🔄 Re-run 'python yoFixWhat.py' to refresh after fixes",
            f"   📈 Track progress by comparing issue counts over time",
            ""
        ])

        return '\n'.join(report_lines)

    def _generate_ai_meta_prompt(self) -> List[str]:
        """Generate meta-prompt for IDE AI integration with proven strategy workflow and systematic methodology"""
        return [
            "```rust",
            "/*",
            " *╔═════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗",
            " *║                                 🧬 YoshiAF Continuous Extension Protocol 🧬                                        ║",
            " *╠═════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╣",
            " *║                                                                                                                     ║",
            " *║                                  🚀 MANDATORY WORKFLOW -  FOLLOW EXACTLY 🚀                                        ║",
            " *║                                                                                                                     ║",
            " *║ ## STEP 1: RUN DIAGNOSTICS (ALWAYS FIRST)                                                                           ║",
            " *║    ```bash                                                                                                          ║",
            " *║    python yoFixWhat.py                                                                                              ║",
            " *║    ```                                                                                                              ║",
            " *║    📊 Generates: yoFixME.txt with categorized warnings and fixes.                                                   ║"
            " *║                                                                                                                     ║",
            " *║ ## STEP 2: EXECUTE ALL OF OUR AUTOFIX METHODS                                                                       ║",
            " *║     + Discover them in: yoshi/src/auto_fix/mod.rs                                                                   ║",
            " *║        - yoshi/src/auto_fix/mod.rs                                                                                  ║",
            " *║        - yoshi/src/main.rs                                                                                          ║",
            " *║        - and possibly in yoshi/src/auto_fix/auto_fix.txt  (may not be fully implemented- this was the plan)         ║",
            " *║     **This will save you priceless debugging time and file documenting time (yes we have a auto-documentator)**     ║",
            " *║                                                                                                                     ║",
            " *║ ## STEP 3: EXTEND YoshiAF MODULES (MANDATORY IMPLEMENTATION)                                                        ║",
            " *║                                                                                                                     ║",
            " *║ ### 🚨 CLIPPY FIXES MODULE - auto_fix/unclipped.rs                                                                  ║",
            " *║    📍 File: yoshi/src/auto_fix/unclipped.rs                                                                         ║",
            " *║    🔧 Parse docs/unclipped_References.md for ALL 500+ Clippy lint patterns                                          ║",
            " *║    🛡️ Implement comprehensive pattern matching for each Clippy lint type against remaining stubs in unclipped.rs    ║",
            " *║    ⚡ Add automated fixes for ALL documented Clippy patterns that are not yet implemented, but stubbed              ║",
            " *║    📊 Replace placeholder comment with full implementation based on docs/unclipped_References.md                    ║",
            " *║    📍 Update docs/unclipped_References.md using the Legend and updated error codes as an example                    ║",
            " *║        ```rust                                                                                                      ║",
            " *║        _ => {                                                                                                       ║",
            " *║            // Other fix types can be implemented here                                                               ║",
            " *║            // This is where we'll add the 500+ Clippy patterns!                                                     ║",
            " *║        }                                                                                                            ║",
            " *║        ```                                                                                                          ║",
            " *║                                                                                                                     ║",
            " *║ ### 🚨 AUTO-CORRECTIONS MODULE - auto_fix/flawless.rs                                                               ║",
            " *║    📍 File: yoshi/src/auto_fix/flawless.rs                                                                          ║",
            " *║    🔧 Use SAME methodology as yoshi/src/auto_fix/mod.rs                                                             ║",
            " *║    🛡️ Implement auto-correction patterns using consistent architecture                                              ║",
            " *║    ⚡ Avoid making mod.rs megalithic by proper module separation                                                    ║",
            " *║    📊 Create stubs for each undeprecated error code in flawless.rs based on docs/flawless_References.md             ║",
            " *║    📍 Update docs/flawless_References.md using the Legend and updated error codes as an example                     ║",
            " *║                                                                                                                     ║",
            " *║ ### 🚨 INTEGRATION REQUIREMENTS                                                                                     ║",
            " *║    📍 Update yoshi/src/auto_fix/mod.rs to export new modules                                                        ║",
            " *║    🧠 Ensure semanticator.rs is fulfills its role elegantly for 100 percent safe, semantic modifications per run    ║",
            " *║    🔧 Add pub use statements for unclipped and flawless modules                                                     ║",
            " *║    🛡️ Maintain consistent API surface across all auto_fix modules                                                   ║",
            " *║                                                                                                                     ║",
            " *╚═════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝",
            " */",
            "```",
            "",
            "      <!-- 🧬 YOSHI FRAMEWORK TRACKING: Track corrections here -->",
            "      <!-- Last correction run: [TIMESTAMP] -->",
            "      <!-- Strategies used: [STRATEGY_LIST] -->",
            "      <!-- Warnings fixed: [COUNT] -->",
            "",
        ]

    def run(self) -> None:
        """Main execution pipeline with enhanced error handling"""
        print("🚀 YoFixWhat: Advanced Cargo Diagnostics Aggregator")
        print("=" * 60)

        try:
            # Discover all crates
            self._discover_all_crates()

            if not self.all_crates:
                print("❌ No Rust crates found in project!")
                print("💡 Make sure you're in a directory containing Cargo.toml files")
                sys.exit(1)

            # Filter crates if specific ones were requested
            crates_to_analyze = list(self.all_crates.keys())
            if self.target_crates:
                crates_to_analyze = [c for c in crates_to_analyze if c in self.target_crates]
                if not crates_to_analyze:
                    print(f"❌ None of the specified crates found: {self.target_crates}")
                    sys.exit(1)
                print(f"🎯 Analyzing specific crates: {', '.join(crates_to_analyze)}")
            else:
                print(f"\n📊 Analyzing {len(self.all_crates)} crates...")

            # Collect diagnostics for each crate (parallel or sequential)
            if self.parallel and len(crates_to_analyze) > 1:
                self._run_parallel_analysis(crates_to_analyze)
            else:
                self._run_sequential_analysis(crates_to_analyze)

            # Generate and write report
            print("📝 Generating comprehensive yoFixME.txt report...")
            report_content = self._generate_report()

            output_file = self.project_root / "yoFixME.txt"

            # Create yoFixed directory and move existing file if it exists
            if output_file.exists():
                yofixed_dir = self.project_root / "yoFixed"
                yofixed_dir.mkdir(exist_ok=True)

                timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
                archived_file = yofixed_dir / f"yoFixME_{timestamp}.txt"
                output_file.rename(archived_file)
                print(f"📁 Moved previous report to: yoFixed/{archived_file.name}")

            # Write new report
            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(report_content)

            # Final summary
            total_issues = sum(len(diags) for diags in self.diagnostics.values())
            total_by_category = {category: 0 for category in DiagnosticCategory}
            for crate_stats in self.category_stats.values():
                for category, count in crate_stats.items():
                    total_by_category[category] += count

            print(f"✅ Report generated: {output_file}")
            print(f"📊 Summary: {total_issues} total issues found")

            if total_issues > 0:
                print("🎯 Priority categories:")
                for category in [DiagnosticCategory.SAFETY, DiagnosticCategory.CORRECTNESS,
                               DiagnosticCategory.PERFORMANCE]:
                    count = total_by_category[category]
                    if count > 0:
                        emoji = {"safety": "🛡️", "correctness": "✅", "performance": "⚡"}[category.value]
                        print(f"   {emoji} {category.value.title()}: {count} issues")

            print("\n🚀 Next steps:")
            print("   1. Review yoFixME.txt for detailed issue breakdown")
            print("   2. Address safety issues first, then correctness")
            print("   3. Use `cargo clippy --fix` for auto-fixable issues")
            print("   4. Re-run this script to track progress")

        except KeyboardInterrupt:
            print("\n⚠️  Operation cancelled by user")
            sys.exit(1)
        except Exception as e:
            print(f"\n❌ Error during analysis: {e}")
            print("💡 Try running with individual crate names if workspace analysis fails")
            sys.exit(1)




class AIAugmentedDeriveEngine:
    """AI-powered semantic derive analysis and suggestion engine"""

    def __init__(self):
        self.field_type_patterns = {
            # ID patterns
            r'\b(u|i)(8|16|32|64|128|size)\b': {'traits': ['Copy', 'Clone', 'Debug', 'PartialEq', 'Eq', 'Hash'], 'role': 'id'},
            r'\bUuid\b': {'traits': ['Copy', 'Clone', 'Debug', 'Display', 'PartialEq', 'Eq', 'Hash'], 'role': 'id'},

            # Data patterns
            r'\bVec<.*>\b': {'traits': ['Clone', 'Debug', 'PartialEq'], 'role': 'data', 'avoid': ['Copy']},
            r'\bHashMap<.*>\b': {'traits': ['Clone', 'Debug', 'PartialEq'], 'role': 'data', 'avoid': ['Copy']},
            r'\bString\b': {'traits': ['Clone', 'Debug', 'Display', 'PartialEq', 'Eq'], 'role': 'data'},

            # Error patterns
            r'\bio::Error\b': {'traits': ['Debug', 'Display'], 'role': 'error_source'},
            r'\b.*Error\b': {'traits': ['Debug', 'Display'], 'role': 'error'},

            # Config patterns
            r'\bPathBuf\b': {'traits': ['Clone', 'Debug', 'PartialEq'], 'role': 'config'},
            r'\bDuration\b': {'traits': ['Copy', 'Clone', 'Debug', 'PartialEq', 'Eq'], 'role': 'config'},

            # Optional patterns
            r'\bOption<.*>\b': {'traits': ['Clone', 'Debug', 'PartialEq'], 'role': 'optional'},
        }

        self.semantic_derive_mapping = {
            'id': ['Display', 'Hash', 'Eq', 'PartialEq', 'Copy', 'Clone'],
            'data': ['Debug', 'Clone', 'PartialEq'],
            'config': ['Debug', 'Clone', 'PartialEq', 'Default'],
            'error': ['Debug', 'Display', 'Error'],
            'error_source': ['Debug', 'Display'],
            'optional': ['Debug', 'Clone', 'PartialEq', 'Default'],
        }

    def analyze_struct_semantics(self, rust_code: str, struct_name: str) -> DeriveFrameworkAnalysis:
        """Analyze a struct for semantic derive opportunities"""
        field_analysis = {}
        suggested_derives = set()
        semantic_patterns = []
        optimization_hints = []

        # Parse struct fields using regex (simplified approach)
        import re

        # Find struct definition
        struct_pattern = rf'struct\s+{re.escape(struct_name)}\s*{{([^}}]*)}}'
        struct_match = re.search(struct_pattern, rust_code, re.DOTALL)

        if not struct_match:
            return DeriveFrameworkAnalysis(
                struct_name=struct_name,
                field_analysis={},
                suggested_derives=[],
                semantic_patterns=[],
                optimization_hints=['Could not parse struct definition'],
                ai_confidence=0.0,
                performance_impact='unknown'
            )

        fields_text = struct_match.group(1)

        # Parse individual fields
        field_pattern = r'(\w+):\s*([^,\n]+)'
        fields = re.findall(field_pattern, fields_text)

        total_confidence = 0.0
        field_count = len(fields)

        for field_name, field_type in fields:
            field_type = field_type.strip().rstrip(',')

            # Analyze field semantics
            semantic_analysis = self._analyze_field_semantics(field_name, field_type)
            field_analysis[field_name] = semantic_analysis

            # Accumulate suggested traits
            suggested_derives.update(semantic_analysis.suggested_traits)
            total_confidence += 0.8  # Base confidence per field

            # Generate semantic patterns
            semantic_patterns.append(f"{field_name}: {semantic_analysis.semantic_role}")

            # Generate optimization hints
            if semantic_analysis.copy_eligible and 'Copy' not in semantic_analysis.suggested_traits:
                optimization_hints.append(f"Field '{field_name}' could derive Copy for better performance")

            if semantic_analysis.size_category == 'large' and 'Copy' in semantic_analysis.suggested_traits:
                optimization_hints.append(f"Field '{field_name}' is large - consider Clone instead of Copy")

        ai_confidence = min(total_confidence / max(field_count, 1), 1.0) if field_count > 0 else 0.0

        # Determine performance impact
        large_fields = sum(1 for analysis in field_analysis.values() if analysis.size_category == 'large')
        if large_fields > field_count // 2:
            performance_impact = 'significant_memory_usage'
        elif any('Copy' in analysis.suggested_traits for analysis in field_analysis.values()):
            performance_impact = 'optimized_copying'
        else:
            performance_impact = 'standard'

        return DeriveFrameworkAnalysis(
            struct_name=struct_name,
            field_analysis=field_analysis,
            suggested_derives=sorted(suggested_derives),
            semantic_patterns=semantic_patterns,
            optimization_hints=optimization_hints,
            ai_confidence=ai_confidence,
            performance_impact=performance_impact
        )

    def _analyze_field_semantics(self, field_name: str, field_type: str) -> FieldSemanticAnalysis:
        """Analyze individual field semantics"""
        import re

        suggested_traits = ['Debug']  # Default trait for all fields
        semantic_role = 'data'  # Default role
        copy_eligible = False
        size_category = 'medium'  # Default size
        display_priority = 5  # Default priority

        # Pattern matching for semantic analysis
        for pattern, pattern_info in self.field_type_patterns.items():
            if re.search(pattern, field_type):
                suggested_traits.extend(pattern_info['traits'])
                semantic_role = pattern_info['role']

                # Check if copy-eligible
                if 'Copy' in pattern_info['traits']:
                    copy_eligible = True

                # Remove avoided traits
                if 'avoid' in pattern_info:
                    for avoid_trait in pattern_info['avoid']:
                        if avoid_trait in suggested_traits:
                            suggested_traits.remove(avoid_trait)
                break

        # Determine size category
        if any(large_type in field_type for large_type in ['Vec', 'HashMap', 'String', 'PathBuf']):
            size_category = 'large'
            copy_eligible = False
        elif any(small_type in field_type for small_type in ['u8', 'u16', 'i8', 'i16', 'bool']):
            size_category = 'small'
            copy_eligible = True

        # Determine display priority based on field name
        if any(id_name in field_name.lower() for id_name in ['id', 'uuid', 'key']):
            display_priority = 10  # High priority
            semantic_role = 'id'
        elif any(name_pattern in field_name.lower() for name_pattern in ['name', 'title', 'desc']):
            display_priority = 8
        elif field_name.lower() in ['data', 'content', 'body']:
            display_priority = 3  # Low priority for large data

        # Remove duplicates while preserving order
        suggested_traits = list(dict.fromkeys(suggested_traits))

        return FieldSemanticAnalysis(
            field_name=field_name,
            field_type=field_type,
            suggested_traits=suggested_traits,
            semantic_role=semantic_role,
            size_category=size_category,
            copy_eligible=copy_eligible,
            display_priority=display_priority
        )


def main():
    """Entry point for enhanced yoFixWhat with derive framework integration"""
    # Parse command line arguments
    args = parse_args()

    # Configure logging
    logging.basicConfig(
        level=getattr(logging, args.log_level),
        format='%(message)s',  # Simple format for user-friendly output
        handlers=[logging.StreamHandler()]
    )

    # Create analyzer with configuration
    fixer = YoFixWhat(
        timeout=args.timeout,
        crates=args.crates,
        skip_clippy=args.no_clippy,
        parallel=args.parallel,
        detailed_output=args.deets
    )

    # Handle VS Code JSON parsing mode
    if args.parse_vscode:
        if not args.parse_vscode.exists():
            print(f"❌ VS Code JSON file not found: {args.parse_vscode}")
            return 1

        print("🚀 YoFixWhat: VS Code Problems JSON Parser")
        print("=" * 60)

        # Parse the VS Code JSON file
        diagnostics_by_crate = fixer.parse_vscode_problems_json(args.parse_vscode)

        if not diagnostics_by_crate:
            print("❌ No diagnostics found in VS Code JSON file")
            return 1

        # Set the diagnostics in the fixer
        fixer.diagnostics = diagnostics_by_crate

        # Calculate category stats
        for crate_name, diagnostics in diagnostics_by_crate.items():
            fixer.category_stats[crate_name] = {}
            for diagnostic in diagnostics:
                category = fixer._categorize_diagnostic(diagnostic)
                fixer.category_stats[crate_name][category] = fixer.category_stats[crate_name].get(category, 0) + 1

        # Generate report
        print("📝 Generating comprehensive yoFixME.txt report...")
        report_content = fixer._generate_report()

        # Write report to specified output file
        output_file = args.out
        if output_file.exists():
            # Move existing report to backup
            backup_dir = output_file.parent / "yoFixed"
            backup_dir.mkdir(exist_ok=True)
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            backup_file = backup_dir / f"{output_file.stem}_{timestamp}.txt"
            output_file.rename(backup_file)
            print(f"📁 Moved previous report to: {backup_file}")

        output_file.write_text(report_content, encoding='utf-8')
        print(f"✅ Report generated: {output_file.absolute()}")

        # Print summary
        total_issues = sum(len(diags) for diags in diagnostics_by_crate.values())
        print(f"📊 Summary: {total_issues} total issues found")

        return 0

    # Run analysis
    fixer.run()

    # If custom output path specified, copy the report
    if args.out != Path("yoFixME.txt"):
        default_report = fixer.project_root / "yoFixME.txt"
        if default_report.exists():
            import shutil
            shutil.copy2(default_report, args.out)
            logging.info(f"📄 Report also saved to: {args.out}")


if __name__ == "__main__":
    main()
