#!/usr/bin/env python3
"""
List all schema names from an OpenAPI spec.

Usage:
  list_resources.py <spec.json> [--provider] [--singleton] [--crud] [--enums] [--no-enums]

Flags:
  --provider    Only schemas with fields+implementation+configContract (provider-style)
  --singleton   Only schemas that appear in GET+PUT but never POST (config singletons)
  --crud        Only schemas that appear in POST+PUT+DELETE (full CRUD)
  --enums       Only enum schemas
  --no-enums    Exclude enum schemas (useful default for resource scanning)

If the spec names few/no component schemas (some non-*arr specs inline their
request/response bodies in paths), the inline bodies are listed too, tagged
[inline] — inspect them with get_path.
"""
import sys

from common import (  # re-exported for tests
    load_spec,
    is_enum,
    is_provider,
    is_singleton,
    is_crud,
    build_schema_path_index,
    inline_path_schemas,
)


def main():
    args = sys.argv[1:]
    if not args or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    spec_path = args[0]
    flags = set(args[1:])

    spec = load_spec(spec_path)
    schemas = spec.get("components", {}).get("schemas", {})
    index = build_schema_path_index(spec)

    want_provider  = "--provider"  in flags
    want_singleton = "--singleton" in flags
    want_crud      = "--crud"      in flags
    want_enums     = "--enums"     in flags
    no_enums       = "--no-enums"  in flags

    active_filter = want_provider or want_singleton or want_crud or want_enums

    results = []
    for name, schema in sorted(schemas.items()):
        ref = f"#/components/schemas/{name}"
        enum = is_enum(schema)

        if no_enums and enum:
            continue

        if active_filter:
            match = False
            if want_enums and enum:
                match = True
            if want_provider and is_provider(schema):
                match = True
            if want_singleton and is_singleton(ref, index):
                match = True
            if want_crud and is_crud(ref, index):
                match = True
            if not match:
                continue

        tags = []
        if enum:
            tags.append("enum")
        elif is_provider(schema):
            tags.append("provider")
        elif is_singleton(ref, index):
            tags.append("singleton")
        elif is_crud(ref, index):
            tags.append("crud")

        tag_str = f"  [{', '.join(tags)}]" if tags else ""
        results.append(f"#/components/schemas/{name}{tag_str}")

    for r in results:
        print(r)

    # Inline (path-defined) bodies — only when no positive filter is active, so
    # filtered runs stay focused on named component schemas.
    if not active_filter:
        inline = inline_path_schemas(spec)
        if inline:
            print()
            print("Inline (path-defined) schemas — inspect with get_path:")
            for label, _ in inline:
                print(f"  {label}  [inline]")


if __name__ == "__main__":
    main()
