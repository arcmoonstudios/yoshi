#!/usr/bin/env python3
"""
Fix yopost! macro syntax in restored files.
Converts yopost!(message: "text") to yopost!(message: "text".into())
and Ok() to Ok(()) where appropriate.
"""

import os
import re
import glob

def fix_yopost_syntax(content):
    """Fix yopost! macro syntax to use .into() for string literals."""
    # Pattern 1: yopost!(message: "literal_string") -> yopost!(message: "literal_string".into())
    # Only match string literals, not format! calls or variables
    pattern1 = r'yopost!\(message:\s*"([^"]*?)"\)'
    replacement1 = r'yopost!(message: "\1".into())'
    content = re.sub(pattern1, replacement1, content)
    
    # Pattern 2: Handle format! calls inside yopost!
    # yopost!(message: format!(...)) -> yopost!(message: format!(...).into())
    pattern2 = r'yopost!\(message:\s*(format!\([^)]+\))\)'
    replacement2 = r'yopost!(message: \1.into())'
    content = re.sub(pattern2, replacement2, content)
    
    return content

def fix_ok_syntax(content):
    """Fix Ok() to Ok(()) for unit return types."""
    # Pattern: Ok() -> Ok(()) but be careful not to change Ok(something)
    # Look for Ok() followed by => or at end of line/statement
    patterns = [
        (r'\bOk\(\)\s*=>', r'Ok(()) =>'),
        (r'\bOk\(\)\s*;', r'Ok(());'),
        (r'\bOk\(\)\s*$', r'Ok(())'),
        (r'return\s+Ok\(\)', r'return Ok(())'),
    ]
    
    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
    
    return content

def fix_file(filepath):
    """Fix a single file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        content = fix_yopost_syntax(content)
        content = fix_ok_syntax(content)
        
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Fixed: {filepath}")
            return True
        else:
            print(f"⏭️  No changes needed: {filepath}")
            return False
    except Exception as e:
        print(f"❌ Error fixing {filepath}: {e}")
        return False

def main():
    """Fix all Rust files in examples/ and tests/ directories."""
    print("🔧 Fixing yopost! macro syntax in restored files...")
    
    # Find all .rs files in examples and tests
    patterns = [
        "examples/*.rs",
        "tests/*.rs",
    ]
    
    files_fixed = 0
    total_files = 0
    
    for pattern in patterns:
        for filepath in glob.glob(pattern):
            total_files += 1
            if fix_file(filepath):
                files_fixed += 1
    
    print(f"\n📊 Summary: Fixed {files_fixed}/{total_files} files")

if __name__ == "__main__":
    main()
