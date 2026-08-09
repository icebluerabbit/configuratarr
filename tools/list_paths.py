#!/usr/bin/env python3
"""
List all API paths with their HTTP methods from an OpenAPI spec.

Usage:
  list_paths.py <spec.json> [prefix]

Examples:
  list_paths.py radarr-v3.json
  list_paths.py radarr-v3.json /api/v3/config
"""
import sys

from common import load_spec


def main():
    args = sys.argv[1:]
    if not args or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    spec_path = args[0]
    prefix = args[1] if len(args) > 1 else None

    spec = load_spec(spec_path)
    paths = spec.get("paths", {})

    for path in sorted(paths):
        if prefix and not path.startswith(prefix):
            continue
        methods = [m.upper() for m in paths[path] if isinstance(paths[path][m], dict)]
        print(f"{','.join(sorted(methods)):<30} {path}")


if __name__ == "__main__":
    main()
