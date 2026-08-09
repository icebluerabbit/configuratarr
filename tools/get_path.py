#!/usr/bin/env python3
"""
Get full details for an API path from an OpenAPI spec.

Usage:
  get_path.py <spec.json> <path>

Example:
  get_path.py radarr-v3.json /api/v3/config/naming

Inline (non-$ref) request/response bodies are expanded to their fields, so
specs that don't name component schemas are still inspectable here.
"""
import sys

from common import load_spec, describe_type, schema_properties


def extract_schema_ref(schema: dict) -> str | None:
    if "$ref" in schema:
        return schema["$ref"]
    if schema.get("type") == "array" and "$ref" in schema.get("items", {}):
        return f"array<{schema['items']['$ref']}>"
    return None


def _describe_body(label: str, schema: dict, spec: dict) -> None:
    """Print a request/response body: its ref, or — if inline — its fields."""
    ref = extract_schema_ref(schema)
    if ref:
        print(f"    {label}: {ref}")
        return
    inner = schema.get("items", schema) if schema.get("type") == "array" else schema
    arr = "array of " if schema.get("type") == "array" else ""
    props, _ = schema_properties(inner, spec)
    if props:
        print(f"    {label}: {arr}inline object")
        for fname, prop in props.items():
            print(f"        {fname}: {describe_type(prop, spec, nullable_suffix=True)}")
    elif schema.get("type"):
        print(f"    {label}: {arr}{schema.get('type')}")


def main():
    args = sys.argv[1:]
    if len(args) < 2 or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    spec_path, target_path = args[0], args[1]
    spec = load_spec(spec_path)
    paths = spec.get("paths", {})

    path_item = paths.get(target_path)
    if path_item is None:
        matches = [p for p in paths if p.startswith(target_path)]
        if not matches:
            print(f"ERROR: path '{target_path}' not found", file=sys.stderr)
            sys.exit(1)
        if len(matches) > 1:
            print(f"Multiple matches (be more specific):")
            for m in sorted(matches):
                print(f"  {m}")
            sys.exit(1)
        target_path = matches[0]
        path_item = paths[target_path]

    print(f"Path: {target_path}")
    print()

    http_methods = [m for m in ("get", "post", "put", "delete", "patch") if m in path_item]

    for method in http_methods:
        op = path_item[method]
        print(f"  {method.upper()}")

        params = op.get("parameters", [])
        for p in params:
            loc = p.get("in", "?")
            pname = p.get("name", "?")
            pschema = p.get("schema", {})
            ptype = pschema.get("type", "?")
            default = pschema.get("default")
            default_str = f" = {default!r}" if default is not None else ""
            req_str = " (required)" if p.get("required", False) else ""
            print(f"    param [{loc}] {pname}: {ptype}{default_str}{req_str}")

        rb = op.get("requestBody", {})
        if rb:
            for ct_val in rb.get("content", {}).values():
                _describe_body("request body", ct_val.get("schema", {}), spec)

        for status, resp in op.get("responses", {}).items():
            content = resp.get("content", {})
            if content:
                for ct_val in content.values():
                    _describe_body(f"response {status}", ct_val.get("schema", {}), spec)
            elif resp.get("description"):
                print(f"    response {status}: {resp['description']}")

        print()


if __name__ == "__main__":
    main()
