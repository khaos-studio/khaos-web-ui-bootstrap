#!/usr/bin/env python3
"""
Validator: Check that files in a directory contain specified text patterns.

Usage:
  python3 validate_file_contains.py --directory <dir> --extension <ext> --contains <pattern> [--contains <pattern>...]

Example:
  python3 validate_file_contains.py --directory specs --extension .md \\
    --contains '## Task Description' \\
    --contains '## Objective' \\
    --contains '## Acceptance Criteria'
"""

import sys
import argparse
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description="Validate file contents")
    parser.add_argument("--directory", required=True, help="Directory to check")
    parser.add_argument("--extension", required=True, help="File extension to look for")
    parser.add_argument(
        "--contains",
        action="append",
        required=True,
        help="Text pattern that must be present (can be used multiple times)",
    )

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

    all_passed = True

    for file_path in files:
        print(f"\nChecking: {file_path.name}")
        content = file_path.read_text(encoding="utf-8")

        for pattern in args.contains:
            if pattern in content:
                print(f"  ✓ Found: '{pattern}'")
            else:
                print(f"  ❌ Missing: '{pattern}'")
                all_passed = False

    if not all_passed:
        print("\n❌ Validation failed: Some required patterns are missing")
        sys.exit(1)

    print("\n✓ All validations passed")
    sys.exit(0)


if __name__ == "__main__":
    main()
