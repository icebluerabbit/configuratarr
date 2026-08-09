#!/usr/bin/env python3
"""
Compare two schemas across two OpenAPI specs.

Usage:
  compare_schemas.py <spec1.json> "#/components/schemas/Foo" <spec2.json> "#/components/schemas/Bar"

Output:
  - Fields only in spec1
  - Fields only in spec2
  - Fields in both with type differences
  - Fields in both with same type (shown with --all)

allOf is merged before comparing; oneOf/anyOf are flagged.
"""
import sys

from common import load_spec, resolve_ref, describe_type, schema_properties  # resolve_ref re-exported for parity


def get_fields(schema_ref: str, spec: dict) -> dict[str, str]:
    name = schema_ref.split("/")[-1]
    schema = spec.get("components", {}).get("schemas", {}).get(name)
    if schema is None:
        return {}
    props, _ = schema_properties(schema, spec)
    return {field: describe_type(prop, spec, nullable_suffix=True) for field, prop in props.items()}


def _notes(schema_ref: str, spec: dict) -> list[str]:
    name = schema_ref.split("/")[-1]
    schema = spec.get("components", {}).get("schemas", {}).get(name)
    return schema_properties(schema, spec)[1] if schema else []


def main():
    args = sys.argv[1:]
    show_all = "--all" in args
    args = [a for a in args if a != "--all"]

    if len(args) < 4 or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    spec1_path, ref1, spec2_path, ref2 = args[0], args[1], args[2], args[3]
    spec1 = load_spec(spec1_path)
    spec2 = load_spec(spec2_path)

    fields1 = get_fields(ref1, spec1)
    fields2 = get_fields(ref2, spec2)

    if not fields1:
        print(f"ERROR: '{ref1}' not found in {spec1_path}", file=sys.stderr)
        sys.exit(1)
    if not fields2:
        print(f"ERROR: '{ref2}' not found in {spec2_path}", file=sys.stderr)
        sys.exit(1)

    only_in_1 = sorted(set(fields1) - set(fields2))
    only_in_2 = sorted(set(fields2) - set(fields1))
    in_both   = sorted(set(fields1) & set(fields2))

    print(f"Comparing:")
    print(f"  A: {ref1}  ({spec1_path})")
    print(f"  B: {ref2}  ({spec2_path})")
    for side, ref, spec in (("A", ref1, spec1), ("B", ref2, spec2)):
        for note in _notes(ref, spec):
            print(f"  ⚠ {side}: {note}")
    print()

    if only_in_1:
        print(f"Only in A ({len(only_in_1)}):")
        for f in only_in_1:
            print(f"  + {f}: {fields1[f]}")
        print()

    if only_in_2:
        print(f"Only in B ({len(only_in_2)}):")
        for f in only_in_2:
            print(f"  + {f}: {fields2[f]}")
        print()

    diffs = [(f, fields1[f], fields2[f]) for f in in_both if fields1[f] != fields2[f]]
    same  = [(f, fields1[f]) for f in in_both if fields1[f] == fields2[f]]

    if diffs:
        print(f"Type differences ({len(diffs)}):")
        for f, t1, t2 in diffs:
            print(f"  ~ {f}: {t1}  vs  {t2}")
        print()

    if show_all and same:
        print(f"Identical ({len(same)}):")
        for f, t in same:
            print(f"    {f}: {t}")
        print()
    elif same:
        print(f"Identical fields: {len(same)} (use --all to show)")


if __name__ == "__main__":
    main()
