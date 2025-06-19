#!/usr/bin/env python3
"""
yoFixWhat.py

**Brief:** Advanced cargo diagnostics aggregator with structured parsing and comprehensive error reporting

# ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
+ [Advanced Diagnostic Parser]
 - [Structured Output Processing]
 - [JSON and Text Format Support]
 - [Intelligent Error Categorization]
# ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
# **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
# **Copyright:** (c) 2025 ArcMoon Studios
# **Author:** Lord Xyn
# **License:** MIT

Usage: python yoFixWhat.py
- Intelligently discovers all crates and generates comprehensive diagnostics
- Uses structured JSON output when available for precise parsing
- Fallback to advanced text parsing for maximum compatibility
- Creates detailed yoFixME.txt with actionable diagnostic information
"""

import argparse
import json
import logging
import os
import subprocess
import sys
import time
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple
from dataclasses import dataclass
from enum import Enum

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


@dataclass
class DiagnosticInfo:
    """Structured diagnostic information"""
    message: str
    severity: str  # error, warning, note, help
    code: Optional[str] = None
    file_path: Optional[str] = None
    line_number: Optional[int] = None
    column: Optional[int] = None
    suggestion: Optional[str] = None
    full_context: str = ""


class DiagnosticCategory(Enum):
    """Categories for diagnostic classification"""
    SAFETY = "safety"           # unsafe, unwrap, expect, indexing
    PERFORMANCE = "performance" # inefficient patterns, allocations
    STYLE = "style"            # formatting, naming conventions
    CORRECTNESS = "correctness" # logic errors, type issues
    DOCUMENTATION = "docs"      # missing docs, doc formatting
    DEPRECATED = "deprecated"   # deprecated apis, patterns


def parse_args() -> argparse.Namespace:
    """Parse command line arguments"""
    parser = argparse.ArgumentParser(
        description="yoFixWhat – advanced cargo diagnostics aggregator",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python yoFixWhat.py                    # Analyze all crates
  python yoFixWhat.py --crate yoshi-core # Analyze specific crate
  python yoFixWhat.py --no-clippy        # Skip clippy analysis
  python yoFixWhat.py --timeout 600      # Increase timeout to 10 minutes
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
    return parser.parse_args()


class YoFixWhat:
    """Advanced cargo diagnostics collector with structured parsing"""

    def __init__(self, timeout: int = 300, crates: Optional[List[str]] = None,
                 skip_clippy: bool = False, parallel: bool = True):
        """Initialize with configurable options"""
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

        # AI strategy extension tracking
        self.pattern_tracker: Dict[str, Dict[str, int]] = {
            "compiler_errors": {},
            "clippy_lints": {},
            "recurring_patterns": {}
        }
        self.correction_counter = 0

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

                # Skip non-diagnostic messages
                if data.get('reason') != 'compiler-message':
                    continue

                message_data = data.get('message', {})

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
                    # Check if it's a cargo diagnostic message
                    if isinstance(data, dict) and data.get('reason') == 'compiler-message':
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
        """Parse text format diagnostics with advanced regex"""
        diagnostics = []
        lines = text_output.split('\n')
        i = 0

        # Patterns for different diagnostic formats
        warning_pattern = re.compile(r'^(warning|error|note|help):\s*(.+)$')
        location_pattern = re.compile(r'^\s*-->\s*([^:]+):(\d+):(\d+)$')
        code_pattern = re.compile(r'= note: `#\[warn\(([^)]+)\)\]`|= help: for further information visit.*#([^)]+)\)')

        while i < len(lines):
            line = lines[i].strip()

            # Skip empty lines and compilation status
            if not line or any(skip in line.lower() for skip in [
                'compiling', 'finished', 'checking', 'building', 'generated', 'warnings'
            ]):
                i += 1
                continue

            # Match diagnostic lines
            warning_match = warning_pattern.match(line)
            if warning_match:
                severity = warning_match.group(1)
                message = warning_match.group(2)

                diagnostic = DiagnosticInfo(
                    message=message,
                    severity=severity,
                    full_context=line
                )

                # Look for location information in subsequent lines
                j = i + 1
                context_lines = [line]

                while j < len(lines) and j < i + 10:  # Look ahead max 10 lines
                    next_line = lines[j].strip()

                    if not next_line:
                        j += 1
                        continue

                    # Check for location
                    location_match = location_pattern.match(next_line)
                    if location_match:
                        diagnostic.file_path = location_match.group(1)
                        diagnostic.line_number = int(location_match.group(2))
                        diagnostic.column = int(location_match.group(3))
                        context_lines.append(next_line)

                    # Check for error code
                    code_match = code_pattern.search(next_line)
                    if code_match:
                        diagnostic.code = code_match.group(1) or code_match.group(2)
                        context_lines.append(next_line)

                    # Check for suggestions
                    if 'help:' in next_line and ('consider' in next_line or 'try' in next_line):
                        diagnostic.suggestion = next_line.split('help:')[1].strip()
                        context_lines.append(next_line)

                    # Stop if we hit another diagnostic or empty section
                    if warning_pattern.match(next_line) or (next_line == '' and j > i + 3):
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
        """Categorize diagnostic based on content"""
        message = diagnostic.message.lower()
        code = (diagnostic.code or '').lower()

        # Safety issues
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
                print(f" ⚠️ Compilation issues detected")
                # Parse compilation errors from stderr
                if stderr:
                    compile_diagnostics = self._parse_text_diagnostics(stderr)
                    if compile_diagnostics:
                        diagnostics.extend(compile_diagnostics)
                        print(f" ✅ Compilation errors parsed")
                        return self._filter_diagnostics(diagnostics)
        except Exception as e:
            print(f" ⚠️ Basic check failed: {e}")

        # Strategy 2: JSON format with encoding-safe execution (only if compilation works)
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
        """Collect comprehensive diagnostics for a specific crate"""
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

        # Deduplicate diagnostics (check may repeat clippy issues)
        existing_messages = {diag.message for diag in all_diagnostics}
        for diag in check_results:
            if diag.message not in existing_messages:
                all_diagnostics.append(diag)

        # Categorize diagnostics for statistics
        category_counts = {category: 0 for category in DiagnosticCategory}
        for diag in all_diagnostics:
            category = self._categorize_diagnostic(diag)
            category_counts[category] += 1

        self.category_stats[crate_name] = category_counts

        # Store results
        if all_diagnostics:
            self.diagnostics[crate_name] = all_diagnostics
            print(f"   📊 Found {len(all_diagnostics)} issues")
        else:
            self.diagnostics[crate_name] = []
            print(f"   ✅ No issues found!")

    def _run_parallel_analysis(self, crates: List[str]) -> None:
        """Run analysis in parallel for faster execution"""
        print(f"🚀 Running parallel analysis on {len(crates)} crates...")

        with ThreadPoolExecutor(max_workers=os.cpu_count() or 4) as executor:
            # Submit all crate analysis tasks
            futures = {executor.submit(self._collect_diagnostics_for_crate, crate): crate
                      for crate in crates}

            # Process results as they complete
            for future in tqdm(as_completed(futures), total=len(futures), desc="Analyzing crates"):
                crate = futures[future]
                try:
                    future.result()  # This will raise any exceptions that occurred
                except Exception as e:
                    print(f"❌ Analysis failed for crate {crate}: {e}")

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
            if count > 0:
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

            # Category breakdown for this crate
            if crate_name in self.category_stats:
                crate_categories = self.category_stats[crate_name]
                category_summary = []
                for category, count in crate_categories.items():
                    if count > 0:
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

                # Display by category
                for category in DiagnosticCategory:
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
        """Generate meta-prompt for IDE AI integration with ACTUAL WORKING PROCESS and systematic methodology"""
        return [
            "```rust",
            "/*",
            " *    ╔═══════════════════════════════════════════════════════════════════════════════╗",
            " *    ║                   🤖 AI META-PROMPT FOR YOSHI DEVELOPMENT 🤖                 ║",
            " *    ╠═══════════════════════════════════════════════════════════════════════════════╣",
            " *    ║                                                                               ║",
            " *    ║ 🎯 CRITICAL OBJECTIVE:  Apply YUM Binary Strategies & Extend Capabilities 🎯 ║",
            " *    ║                                                                               ║",
            " *    ║                     📂 ACTUAL WORKING STRATEGY SYSTEM 📂                     ║",
            " *    ║                                                                               ║",
            " *    ║  ✅ WORKING STRATEGIES: PanicUsed, UnnecessaryToOwned, ComparisonToEmpty ✅  ║",
            " *    ║  ✅ IMPLEMENTED IN: yoshi-deluxe/src/strategies/flawless_clippy.rs       ✅  ║",
            " *    ║  ✅ ACCESSIBLE VIA: yoshi facade crate (use yoshi::*;)                   ✅  ║",
            " *    ║  ✅ EXECUTABLE VIA: cargo run --bin yum -- apply-strategies --clippy     ✅  ║",
            " *    ║                                                                               ║",
            " *    ║                       🔄 STRATEGY-FIRST METHODOLOGY 🔄                       ║",
            " *    ║                                                                               ║",
            " *    ║     1. CREATE STRATEGY: Design in yoshi-deluxe/src/strategies/                ║",
            " *    ║        - flawless_clippy.rs for clippy lints                                  ║",
            " *    ║        - error_correction.rs for compiler errors                              ║",
            " *    ║     2. MAKE PUBLIC: Change pub(super) to pub for strategy structs             ║",
            " *    ║     3. RE-EXPORT: Add to strategies/mod.rs and yoshi-deluxe/lib.rs            ║",
            " *    ║     4. FACADE INTEGRATION: Add to yoshi/src/lib.rs re-exports                 ║",
            " *    ║     5. YUM INTEGRATION: Add strategy to yum binary apply_*_strategies()       ║",
            " *    ║     6. TEST EXECUTION: cargo run --bin yum -- apply-strategy strategy_name    ║",
            " *    ║     7. VALIDATE RESULTS: Ensure compilation and functionality                 ║",
            " *    ║     8. RINSE & REPEAT: Rerun python yoFixWhat.py to measure progress          ║",
            " *    ║                                                                               ║",
            " *    ║                  🚀 ACTUAL WORKING  APPLICATION PROTOCOL 🚀                  ║",
            " *    ║                                                                               ║",
            " *    ║     1. IDENTIFY PATTERN: Find issues in yoFixME.txt matching strategies       ║",
            " *    ║     2. APPLY VIA YUM: cargo run --bin yum -- apply-strategy <name>            ║",
            " *    ║     3. BATCH PROCESS: cargo run --bin yum -- apply-strategies --clippy        ║",
            " *    ║     4. VALIDATE FIXES: cargo check && cargo clippy                            ║",
            " *    ║     5. MEASURE PROGRESS: python yoFixWhat.py (compare issue counts)           ║",
            " *    ║     6. ITERATE: Create new strategies for remaining patterns                  ║",
            " *    ║                                                                               ║",
            " *    ║                        🎯 CURRENT  ISSUE BREAKDOWN 🎯                        ║",
            " *    ║                                                                               ║",
            " *    ║     🛡️ SAFETY: panic!, expect(), unwrap(), indexing_slicing           🛡️     ║",
            " *    ║     ✅ CORRECTNESS: manual_let_else, unnecessary_wraps, type issues   ✅     ║",
            " *    ║     🎨 STYLE: uninlined_format_args, redundant_field_names            🎨     ║",
            " *    ║     📚 DOCS: missing_docs, doc_markdown                               📚     ║",
            " *    ║                                                                               ║",
            " *    ║                        📚 PROVEN STRATEGY REFERENCE 📚                       ║",
            " *    ║                                                                               ║",
            " *    ║  🔗           COMPLETE STRATEGY DOCUMENTATION: yoStrats.md               🔗  ║",
            " *    ║  📋  10+ TESTED STRATEGIES:  PanicUsed, ExpectUsed, UninlinedFormatArgs  📋  ║",
            " *    ║  ✅         SUCCESS RATE: 95%+ across all proven implementations         ✅  ║",
            " *    ║                                                                               ║",
            " *    ║                    🚨 ACTUAL WORKING EXECUTION PROTOCOL 🚨                   ║",
            " *    ║                                                                               ║",
            " *    ║ ❌ FORBIDDEN: Manual str-replace-editor for issues w/ existing strategies ❌ ║",
            " *    ║ ✅ REQUIRED: Use YUM binary for strategy application                      ✅ ║",
            " *    ║ ✅ REQUIRED: cargo run --bin yum -- apply-strategies --clippy             ✅ ║",
            " *    ║ ✅ REQUIRED: cargo run --bin yum -- apply-strategy <strategy_name>        ✅ ║",
            " *    ║ ✅ REQUIRED: Reference yoStrats.md for strategy checklist                 ✅ ║",
            " *    ║                                                                               ║",
            " *    ║                      📂 ACTUAL WORKING FILE STRUCTURE 📂                     ║",
            " *    ║                                                                               ║",
            " *    ║     • yoshi-deluxe/src/strategies/flawless_clippy.rs (clippy strategies)      ║",
            " *    ║     • yoshi-deluxe/src/strategies/error_correction.rs (compiler errors)       ║",
            " *    ║     • yoshi-deluxe/src/strategies/mod.rs (re-exports)                         ║",
            " *    ║     • yoshi-deluxe/src/lib.rs (public API)                                    ║",
            " *    ║     • yoshi/src/lib.rs (facade re-exports)                                    ║",
            " *    ║     • yoshi/src/main.rs (yum binary implementation)                           ║",
            " *    ║     • yoStrats.md (simple strategy checklist)                                 ║",
            " *    ║                                                                               ║",
            " *    ║                          🔄 INTEGRATION  PROTOCOL 🔄                         ║",
            " *    ║                                                                               ║",
            " *    ║     Every 5 corrections you make, analyze the error patterns and extend       ║",
            " *    ║     the strategy files with generalized correction strategies that can        ║",
            " *    ║     be applied agnostically across similar error types.                       ║",
            " *    ║                                                                               ║",
            " *    ║                         📋 STRATEGY  REQUIREMENTS 📋                         ║",
            " *    ║                                                                               ║",
            " *    ║     ✅      Follow the existing CorrectionStrategy trait pattern      ✅     ║",
            " *    ║     ✅          Use yoshi_af! macro for all implementations           ✅     ║",
            " *    ║     ✅          Provide comprehensive Rustdoc documentation           ✅     ║",
            " *    ║     ✅  Include multiple correction proposals with confidence scores  ✅     ║",
            " *    ║     ✅       Add helper functions with proper error handling          ✅     ║",
            " *    ║     ✅    Only record generalizable strategies (skip unique bugs)     ✅     ║",
            " *    ║     ✅         Ensure no duplication of existing error codes          ✅     ║",
            " *    ║                                                                               ║",
            " *    ║                    🎯 ACTUAL WORKING APPLICATION GUIDE 🎯                    ║",
            " *    ║                                                                               ║",
            " *    ║     1. IDENTIFY PATTERN: Find issues matching strategies in yoStrats.md       ║",
            " *    ║     2. USE YUM BINARY: Apply via cargo run --bin yum commands                 ║",
            " *    ║     3. BATCH PROCESS: Fix all instances of same pattern systematically        ║",
            " *    ║     4. VALIDATE COMPILATION: cargo check && cargo clippy                      ║",
            " *    ║     5. MEASURE IMPACT: python yoFixWhat.py (compare issue counts)             ║",
            " *    ║     6. UPDATE STRATEGIES: Add new patterns every 5 fixes                      ║",
            " *    ║                                                                               ║",
            " *    ║                     🚀 ACTUAL WORKING USAGE EXAMPLES 🚀                      ║",
            " *    ║                                                                               ║",
            " *    ║     # Apply all clippy strategies                                             ║",
            " *    ║     cargo run --bin yum -- apply-strategies --clippy                          ║",
            " *    ║                                                                               ║",
            " *    ║     # Apply specific strategy                                                 ║",
            " *    ║     cargo run --bin yum -- apply-strategy panic_used                          ║",
            " *    ║     cargo run --bin yum -- apply-strategy unnecessary_to_owned                ║",
            " *    ║                                                                               ║",
            " *    ║              🏗️ IMPLEMENTATION PATTERN (error_correction.rs) 🏗️              ║",
            " *    ║                                                                               ║",
            " *    ║     #[derive(Debug, yoshi_derive::YoshiError)]                                ║",
            " *    ║     pub(super) struct E####DescriptiveName;                                   ║",
            " *    ║                                                                               ║",
            " *    ║     yoshi_af! {                                                               ║",
            " *    ║         impl CorrectionStrategy for E####DescriptiveName {                    ║",
            " *    ║             fn error_code(&self) -> ErrorCode { ErrorCode::E#### }            ║",
            " *    ║             fn generate_proposals(&self, context: &ASTContext)                ║",
            " *    ║                 -> Hatch<Vec<CorrectionProposal>> {                           ║",
            " *    ║                 // Implementation with multiple strategies                    ║",
            " *    ║             }                                                                 ║",
            " *    ║         }                                                                     ║",
            " *    ║     }                                                                         ║",
            " *    ║                                                                               ║",
            " *    ║              🎯 CLIPPY STRATEGY PATTERN (flawless_clippy.rs) 🎯              ║",
            " *    ║                                                                               ║",
            " *    ║     #[derive(Debug, yoshi_derive::YoshiError)]                                ║",
            " *    ║     pub(super) struct ClippyLintName;                                         ║",
            " *    ║                                                                               ║",
            " *    ║     yoshi_af! {                                                               ║",
            " *    ║         impl ClippyStrategy for ClippyLintName {                              ║",
            " *    ║             fn lint_name(&self) -> &'static str { lint_name }                 ║",
            " *    ║             fn generate_fixes(&self, context: &ClippyContext)                 ║",
            " *    ║                 -> Hatch<Vec<ClippyFix>> {                                    ║",
            " *    ║                 // Implementation with automated fixes                        ║",
            " *    ║             }                                                                 ║",
            " *    ║         }                                                                     ║",
            " *    ║     }                                                                         ║",
            " *    ║                                                                               ║",
            " *    ║                            📊 QUALITY METRICS 📊                             ║",
            " *    ║                                                                               ║",
            " *    ║     • Confidence Score: 0.0-1.0 (use 0.9+ for high-confidence fixes)          ║",
            " *    ║     • Safety Level: Safe | RequiresReview | Unsafe                            ║",
            " *    ║     • Coverage: Aim for 95%+ of common error patterns                         ║",
            " *    ║                                                                               ║",
            " *    ║                           🔍 ANALYSIS TRIGGER 🔍                             ║",
            " *    ║                                                                               ║",
            " *    ║     After every 5 corrections, examine the error patterns below and           ║",
            " *    ║     determine if any represent generalizable correction strategies.           ║",
            " *    ║     Focus on patterns that appear multiple times or have clear                ║",
            " *    ║     algorithmic solutions.                                                    ║",
            " *    ║                                                                               ║",
            " *    ║                   💡 STRATEGY CATEGORIES TO PRIORITIZE 💡                    ║",
            " *    ║                                                                               ║",
            " *    ║     🛡️      Memory Safety: Ownership, borrowing, lifetime issues      🛡️     ║",
            " *    ║     🔧     Type System: Trait bounds, generics, associated types      🔧     ║",
            " *    ║     ⚡  Performance: Iterator chains, allocations, algorithmic fixes  ⚡     ║",
            " *    ║     🎨   Style: Formatting, naming conventions, idiomatic patterns    🎨     ║",
            " *    ║     📚       Documentation: Missing docs, incorrect examples          📚     ║",
            " *    ║     🚫       Deprecation: Outdated APIs, superseded patterns          🚫     ║",
            " *    ║                                                                               ║",
            " *    ║                       📝 DOCUMENTATION REQUIREMENTS 📝                       ║",
            " *    ║                                                                               ║",
            " *    ║     /// **Strategy for E####: descriptive name.**                             ║",
            " *    ║     ///                                                                       ║",
            " *    ║     /// Handles cases where [specific scenario]. This strategy provides       ║",
            " *    ║     /// solutions for [list of approaches].                                   ║",
            " *    ║     ///                                                                       ║",
            " *    ║     /// # Examples                                                            ║",
            " *    ║     ///                                                                       ║",
            " *    ║     /// ```rust,ignore                                                        ║",
            " *    ║     /// // Error case and suggested fix                                       ║",
            " *    ║     /// ```                                                                   ║",
            " *    ║                                                                               ║",
            " *    ║                          🚨 YUM BINARY-FIRST MANDATE 🚨                      ║",
            " *    ║                                                                               ║",
            " *    ║     The YUM binary exists to automate strategy application. Using manual      ║",
            " *    ║     str-replace-editor for issues with existing strategies defeats the        ║",
            " *    ║     purpose and wastes the proven automation capabilities.                    ║",
            " *    ║                                                                               ║",
            " *    ║   ✅ CORRECT: cargo run --bin yum -- apply-strategies --clippy          ✅   ║",
            " *    ║   ✅ CORRECT: cargo run --bin yum -- apply-strategy panic_used          ✅   ║",
            " *    ║   ❌ WRONG: Manual str-replace-editor for existing strategies           ❌   ║",
            " *    ║                                                                               ║",
            " *    ║                    🔍 VALIDATION & VERIFICATION  STEPS 🔍                    ║",
            " *    ║                                                                               ║",
            " *    ║     1. COMPILE CHECK: cargo check (must pass with 0 errors)                   ║",
            " *    ║     2. CLIPPY CHECK: cargo clippy (must pass with 0 warnings)                 ║",
            " *    ║     3. TEST EXECUTION: cargo test --all (all tests must pass)                 ║",
            " *    ║     4. PROGRESS MEASUREMENT: python yoFixWhat.py (compare issue counts)       ║",
            " *    ║     5. STRATEGY VERIFICATION: Test yum commands work correctly                ║",
            " *    ║                                                                               ║",
            " *    ║                      📊 SUCCESS METRICS  &  TRACKING 📊                      ║",
            " *    ║                                                                               ║",
            " *    ║     • Issue Count Reduction: Track before/after yoFixME.txt                   ║",
            " *    ║     • Compilation Success: Zero errors after strategy application             ║",
            " *    ║     • Strategy Coverage: Number of patterns with working strategies           ║",
            " *    ║     • Automation Rate: Percentage of issues fixed via strategies vs manual    ║",
            " *    ║                                                                               ║",
            " *    ║                                🧠 REMEMBER 🧠                                ║",
            " *    ║                                                                               ║",
            " *    ║     You are extending Yoshi's intelligence. Each strategy you                 ║",
            " *    ║     add makes the system smarter and more capable of autonomous fixes.        ║",
            " *    ║     Think like a Rust expert teaching the system to recognize and solve       ║",
            " *    ║     problems independently through the YUM binary interface.                  ║",
            " *    ║                                                                               ║",
            " *    ╚═══════════════════════════════════════════════════════════════════════════════╝",
            " */",
            "```",
            "",
            "      <!-- 🎯 CORRECTION COUNTER: Track your fixes here -->",
            "      <!-- When you reach multiples of 5, extend the strategy files -->",
            "      <!-- Last strategy update: [DATE] -->",
            "      <!-- Next update due at correction: [NUMBER] -->",
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
                print("   💡 Make sure you're in a directory containing Cargo.toml files")
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
                print(f"   📁 Moved previous report to: yoFixed/{archived_file.name}")

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

def main():
    """Entry point for enhanced yoFixWhat"""
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
        parallel=args.parallel
    )

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
