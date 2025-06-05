#!/usr/bin/env python3
"""Script to remove unnecessary #[must_use] attributes from functions returning Result<T>"""

import os
import re
from pathlib import Path

def process_file(file_path):
    """Process a single Rust file to remove unnecessary #[must_use] attributes"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Pattern to match #[must_use] followed by pub fn that returns Result<T>
    # This is a simplified pattern - might need refinement
    pattern = r'(\s*)#\[must_use\]\s*\n(\s*pub fn [^{]*-> Result<[^>]+>)'
    
    def replace_func(match):
        indent = match.group(1)
        function_line = match.group(2)
        return f"{indent}{function_line}"
    
    new_content = re.sub(pattern, replace_func, content)
    
    if new_content != content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Updated: {file_path}")
        return True
    return False

def main():
    """Main function to process all Rust files in the workspace"""
    workspace_root = Path(".")
    rust_files = list(workspace_root.rglob("*.rs"))
    
    updated_count = 0
    for rust_file in rust_files:
        # Skip target directory
        if "target" in rust_file.parts:
            continue
        
        if process_file(rust_file):
            updated_count += 1
    
    print(f"Updated {updated_count} files")

if __name__ == "__main__":
    main()
