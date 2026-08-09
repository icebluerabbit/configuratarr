#!/usr/bin/env python3
"""
Get full details for a schema from an OpenAPI spec.

Usage:
  get_resource.py <spec.json> "#/components/schemas/Foo"

Output includes:
  - All properties with types, nullability, readOnly, defaults, enum values
  - $ref children resolved one level (shows actual field types)
  - allOf merged in; oneOf/anyOf flagged (fields may be incomplete)
  - Which paths/methods reference this schema and in what role
  - Classification (CRUD / singleton / provider / embedded)
"""
import sys

from common import (  # re-exported for tests
    load_spec,
    resolve_ref,
    describe_type,
    find_path_usages,
    resolve_schema,
    build_schema_path_index,
    is_crud,
    is_singleton,
    is_provider,
)


def main():
    args = sys.argv[1:]
    if len(args) < 2 or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    spec_path, schema_ref = args[0], args[1]
    spec = load_spec(spec_path)
    schemas = spec.get("components", {}).get("schemas", {})

    name = schema_ref.split("/")[-1]
    schema = schemas.get(name)
    if schema is None:
        print(f"ERROR: schema '{schema_ref}' not found", file=sys.stderr)
        sys.exit(1)

    print(f"Schema: {schema_ref}")
    print()

    if "enum" in schema:
        print(f"Type: enum ({schema.get('type', 'string')})")
        print(f"Values: {schema['enum']}")
        return

    merged, notes = resolve_schema(schema, spec)
    props = merged.get("properties", {})
    required = set(merged.get("required", []))

    for note in notes:
        print(f"  ⚠ {note}")
    if notes:
        print()

    if props:
        print("Properties:")
        col = max((len(k) for k in props), default=10) + 2
        for field_name, prop in props.items():
            type_str = describe_type(prop, spec)
            tags = []
            if field_name not in required and prop.get("nullable") is not True:
                if prop.get("type") != "array":
                    tags.append("optional")
            if prop.get("nullable"):
                tags.append("nullable")
            if prop.get("readOnly"):
                tags.append("readOnly")
            if "default" in prop:
                tags.append(f"default={prop['default']!r}")
            tag_str = f"  # {', '.join(tags)}" if tags else ""
            print(f"  {field_name:<{col}} {type_str}{tag_str}")
    else:
        print("Properties: (none)")

    print()
    usages = sorted(find_path_usages(schema_ref, spec))
    if usages:
        print("Used in paths:")
        for path, method, role in usages:
            print(f"  {method:<7} {path}  ({role})")
    else:
        print("Used in paths: (none — may be embedded only)")

    print()
    # Sync kind and provider-ness are orthogonal (a provider is still a CRUD
    # resource) — report the sync classification, then note provider separately.
    index = build_schema_path_index(spec)
    if is_crud(schema_ref, index):
        sync = "CRUD resource"
    elif is_singleton(schema_ref, index):
        sync = "singleton config (GET + PUT only)"
    elif schema_ref not in index:
        sync = "read-only / embedded"
    else:
        sync = "mixed"
    provider_note = " + provider (dynamic fields[] blob)" if is_provider(schema) else ""
    print(f"Classification: {sync}{provider_note}")


if __name__ == "__main__":
    main()
