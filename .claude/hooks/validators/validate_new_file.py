#!/usr/bin/env python3
"""
Validator: Check that a new file was created in the specified directory.

Usage:
  python3 validate_new_file.py --directory <dir> --extension <ext>

Example:
  python3 validate_new_file.py --directory specs --extension .md
"""

import sys
import argparse
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description="Validate that a new file was created")
    parser.add_argument("--directory", required=True, help="Directory to check")
    parser.add_argument("--extension", required=True, help="File extension to look for")

    args = parser.parse_args()

    dir_path = Path(args.directory)

    if not dir_path.exists():
        print(f"❌ Directory not found: {dir_path}")
        sys.exit(1)

    # Find files with the specified extension
    files = list(dir_path.glob(f"*{args.extension}"))

    if not files:
        print(f"❌ No files with extension '{args.extension}' found in {dir_path}")
        sys.exit(1)

    print(f"✓ Found {len(files)} file(s) with extension '{args.extension}' in {dir_path}")
    for f in files:
        print(f"  - {f.name}")

    sys.exit(0)


if __name__ == "__main__":
    main()
