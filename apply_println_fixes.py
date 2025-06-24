#!/usr/bin/env python3
"""
apply_println_fixes.py - Apply println! to tracing macro corrections

**Brief:** Intelligently replace all println! calls with appropriate tracing macros

# ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
+ [Intelligent println! to tracing Converter]
 - [Context-Aware Macro Selection]
 - [Automatic Pattern Recognition]
 - [Safety-First Replacement Strategy]
# ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
# **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
# **Copyright:** (c) 2025 ArcMoon Studios
# **Author:** Lord Xyn
# **License:** MIT

Usage: python apply_println_fixes.py
- Finds all files with println! usage
- Intelligently replaces with appropriate tracing macros
- Creates backups before modification
"""

import os
import re
import sys
import shutil
from pathlib import Path
from datetime import datetime

def find_println_files(project_root):
    """Find all Rust files containing println! calls"""
    println_files = []
    
    for root, dirs, files in os.walk(project_root):
        # Skip target and backup directories
        dirs[:] = [d for d in dirs if d not in ['target', 'backups', '.git']]
        
        for file in files:
            if file.endswith('.rs'):
                file_path = Path(root) / file
                try:
                    content = file_path.read_text(encoding='utf-8')
                    if 'println!' in content:
                        println_files.append(file_path)
                        print(f"📍 Found println! in: {file_path}")
                except Exception as e:
                    print(f"⚠️ Error reading {file_path}: {e}")
    
    return println_files

def create_backup(file_path):
    """Create a backup of the file before modification"""
    backup_dir = Path("backups") / "println_fixes" / datetime.now().strftime("%Y%m%d_%H%M%S")
    backup_dir.mkdir(parents=True, exist_ok=True)
    
    backup_path = backup_dir / file_path.name
    shutil.copy2(file_path, backup_path)
    print(f"🛡️ Backup created: {backup_path}")
    return backup_path

def apply_intelligent_println_fixes(file_path):
    """Apply intelligent println! to tracing macro corrections"""
    content = file_path.read_text(encoding='utf-8')
    original_content = content
    corrections_applied = 0

    # Intelligent println! replacement patterns
    patterns = [
        # Error messages -> tracing::error!
        (r'println!\s*\(\s*"🚨([^"]*)"', r'tracing::error!("🚨\1"'),
        (r'println!\s*\(\s*"❌([^"]*)"', r'tracing::error!("❌\1"'),
        (r'println!\s*\(\s*"Error:([^"]*)"', r'tracing::error!("Error:\1"'),
        (r'println!\s*\(\s*"CRITICAL:([^"]*)"', r'tracing::error!("CRITICAL:\1"'),
        (r'println!\s*\(\s*"Failed([^"]*)"', r'tracing::error!("Failed\1"'),
        
        # Warning messages -> tracing::warn!
        (r'println!\s*\(\s*"⚠️([^"]*)"', r'tracing::warn!("⚠️\1"'),
        (r'println!\s*\(\s*"Warning:([^"]*)"', r'tracing::warn!("Warning:\1"'),
        (r'println!\s*\(\s*"WARN:([^"]*)"', r'tracing::warn!("WARN:\1"'),
        
        # Success messages -> tracing::info!
        (r'println!\s*\(\s*"🎉([^"]*)"', r'tracing::info!("🎉\1"'),
        (r'println!\s*\(\s*"✅([^"]*)"', r'tracing::info!("✅\1"'),
        (r'println!\s*\(\s*"Success:([^"]*)"', r'tracing::info!("Success:\1"'),
        (r'println!\s*\(\s*"Completed([^"]*)"', r'tracing::info!("Completed\1"'),
        
        # Debug/Info messages -> tracing::debug!
        (r'println!\s*\(\s*"🔍([^"]*)"', r'tracing::debug!("🔍\1"'),
        (r'println!\s*\(\s*"Debug:([^"]*)"', r'tracing::debug!("Debug:\1"'),
        (r'println!\s*\(\s*"INFO:([^"]*)"', r'tracing::info!("INFO:\1"'),
        
        # File/Path operations -> tracing::info!
        (r'println!\s*\(\s*"📁([^"]*)"', r'tracing::info!("📁\1"'),
        (r'println!\s*\(\s*"📄([^"]*)"', r'tracing::info!("📄\1"'),
        
        # Generic println! with format args -> tracing::info!
        (r'println!\s*\(\s*"([^"]*)",\s*([^)]+)\)', r'tracing::info!("\1", \2)'),
        
        # Simple println! -> tracing::info!
        (r'println!\s*\(\s*"([^"]+)"\s*\)', r'tracing::info!("\1")'),
    ]

    for pattern, replacement in patterns:
        regex = re.compile(pattern)
        matches = regex.findall(content)
        if matches:
            content = regex.sub(replacement, content)
            corrections_applied += len(matches)
            print(f"  ✅ Applied {len(matches)} corrections for pattern: {pattern[:30]}...")

    # Handle multiline println! patterns (like help text)
    multiline_pattern = r'println!\s*\(\s*r"\s*((?:[^"\\]|\\.|"[^"]*")*)\s*"\s*\)'
    multiline_regex = re.compile(multiline_pattern, re.DOTALL)
    
    def replace_multiline(match):
        nonlocal corrections_applied
        corrections_applied += 1
        content_text = match.group(1)
        return f'tracing::info!(r"{content_text}")'
    
    content = multiline_regex.sub(replace_multiline, content)

    if corrections_applied > 0:
        file_path.write_text(content, encoding='utf-8')
        print(f"🎯 Applied {corrections_applied} println! corrections to {file_path}")
        return True
    else:
        print(f"ℹ️ No println! patterns found in {file_path}")
        return False

def main():
    """Main entry point"""
    project_root = Path.cwd()
    print("🚀 Intelligent println! to tracing Converter")
    print("=" * 60)
    
    # Find all files with println! calls
    println_files = find_println_files(project_root)
    
    if not println_files:
        print("✅ No println! calls found in the project!")
        return
    
    print(f"\n📊 Found {len(println_files)} files with println! calls")
    
    # Apply corrections to each file
    total_corrections = 0
    corrected_files = 0
    
    for file_path in println_files:
        print(f"\n🔧 Processing: {file_path}")
        
        # Create backup
        backup_path = create_backup(file_path)
        
        # Apply corrections
        if apply_intelligent_println_fixes(file_path):
            corrected_files += 1
    
    print(f"\n🎉 Correction Summary:")
    print(f"   📁 Files processed: {len(println_files)}")
    print(f"   ✅ Files corrected: {corrected_files}")
    print(f"   🛡️ Backups created in: backups/println_fixes/")
    
    print(f"\n🚀 Next steps:")
    print(f"   1. Run: cargo check --workspace")
    print(f"   2. Run: cargo clippy --workspace")
    print(f"   3. Verify all println! warnings are resolved")

if __name__ == "__main__":
    main()
