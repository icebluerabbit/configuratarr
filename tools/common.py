"""Shared helpers for the OpenAPI spec-exploration tools.

One home for the logic that was copy-pasted across the per-command scripts:
spec loading, `$ref` / `allOf` / `oneOf` resolution, type description, the
schema→path-method index, and the crud/singleton/provider classification.
Each `*.py` tool re-exports the names its tests import, so this module is the
single source of truth and the tools stay thin.

Inspection only — reads the JSON spec, never emits or parses Rust.
"""
import json


# ── loading / refs ────────────────────────────────────────────────────────────

def load_spec(path: str) -> dict:
    with open(path) as f:
        return json.load(f)


def resolve_ref(ref: str, spec: dict) -> dict:
    """Resolve a local `#/...` ref to its node."""
    node = spec
    for p in ref.lstrip("#/").split("/"):
        node = node[p]
    return node


# ── composition resolution (allOf / oneOf / anyOf) ────────────────────────────

def resolve_schema(schema: dict, spec: dict, _seen: set | None = None) -> tuple[dict, list[str]]:
    """Flatten a schema into one with merged `properties`/`required`.

    Resolves a top-level `$ref`, deep-merges every `allOf` member, and for
    `oneOf`/`anyOf` best-effort-merges the first branch while recording a note
    that the field set may be incomplete. Returns `(merged_schema, notes)`.
    A plain schema (no composition) round-trips unchanged, so callers that used
    `schema.get("properties", {})` keep working and gain allOf for free.
    """
    notes: list[str] = []
    if not isinstance(schema, dict):
        return {}, notes
    _seen = _seen or set()

    if "$ref" in schema:
        ref = schema["$ref"]
        if ref in _seen:                       # guard self-referential allOf chains
            return {}, notes
        merged, n = resolve_schema(resolve_ref(ref, spec), spec, _seen | {ref})
        return merged, notes + n

    props: dict = dict(schema.get("properties", {}))
    required: set = set(schema.get("required", []) or [])

    for member in schema.get("allOf", []) or []:
        m, n = resolve_schema(member, spec, _seen)
        props.update(m.get("properties", {}))
        required.update(m.get("required", []))
        notes += n

    for kw in ("oneOf", "anyOf"):
        branches = schema.get(kw) or []
        if branches:
            notes.append(
                f"{kw}: {len(branches)} branches — fields may be incomplete; cross-check the raw spec"
            )
            m, _ = resolve_schema(branches[0], spec, _seen)
            for k, v in m.get("properties", {}).items():
                props.setdefault(k, v)

    out: dict = {"properties": props, "required": sorted(required)}
    if "enum" in schema:
        out["enum"] = schema["enum"]
    if "type" in schema:
        out["type"] = schema["type"]
    return out, notes


def schema_properties(schema: dict, spec: dict) -> tuple[dict, list[str]]:
    """`(properties, notes)` after `$ref`/`allOf`/`oneOf` resolution."""
    merged, notes = resolve_schema(schema, spec)
    return merged.get("properties", {}), notes


# ── type description ──────────────────────────────────────────────────────────

def describe_type(prop: dict, spec: dict, nullable_suffix: bool = False) -> str:
    """Human type string for a property. `nullable_suffix` appends `?` for
    nullable scalars (callers that surface nullability separately leave it off)."""
    if "$ref" in prop:
        ref = prop["$ref"]
        name = ref.split("/")[-1]
        resolved = resolve_ref(ref, spec)
        if "enum" in resolved:
            return f"{name} (enum: {resolved['enum']})"
        return name
    t = prop.get("type", "")
    if t == "array":
        items = prop.get("items", {})
        if "$ref" in items:
            return f"array<{items['$ref'].split('/')[-1]}>"
        return f"array<{items.get('type', '?')}>"
    if t == "number":
        fmt = prop.get("format", "")
        base = f"number({fmt})" if fmt else "number"
    else:
        base = t or "?"
    if nullable_suffix and prop.get("nullable"):
        return f"{base}?"
    return base


# ── schema ⇄ path classification ──────────────────────────────────────────────

def is_enum(schema: dict) -> bool:
    return "enum" in schema


def is_provider(schema: dict) -> bool:
    """*arr provider shape: a dynamic `fields[]` blob behind impl + contract."""
    props = schema.get("properties", {})
    return all(k in props for k in ("fields", "implementation", "configContract"))


def build_schema_path_index(spec: dict) -> dict[str, set[str]]:
    """Map schema `$ref` → every HTTP method on a path where that schema appears.

    DELETE carries no body schema, so it's attributed via co-located GET/PUT on
    the same path (that's what makes a CRUD resource distinguishable).
    """
    path_data: dict[str, tuple[set[str], set[str]]] = {}
    for path, item in spec.get("paths", {}).items():
        refs: set[str] = set()
        methods: set[str] = set()
        for method, op in item.items():
            if not isinstance(op, dict):
                continue
            methods.add(method.upper())
            for s in _operation_body_schemas(op):
                if "$ref" in s:
                    refs.add(s["$ref"])
                elif "items" in s and "$ref" in s.get("items", {}):
                    refs.add(s["items"]["$ref"])
        path_data[path] = (refs, methods)

    index: dict[str, set[str]] = {}
    for refs, methods in path_data.values():
        for ref in refs:
            index.setdefault(ref, set()).update(methods)
    return index


def is_singleton(ref: str, index: dict[str, set[str]]) -> bool:
    m = index.get(ref, set())
    return "GET" in m and "PUT" in m and "POST" not in m and "DELETE" not in m


def is_crud(ref: str, index: dict[str, set[str]]) -> bool:
    m = index.get(ref, set())
    return "POST" in m and "PUT" in m and "DELETE" in m


def classify(ref: str, schema: dict, index: dict[str, set[str]]) -> str:
    """One label: enum | provider | singleton | crud | embedded | mixed."""
    if is_enum(schema):
        return "enum"
    if is_provider(schema):
        return "provider"
    if is_singleton(ref, index):
        return "singleton"
    if is_crud(ref, index):
        return "crud"
    if ref not in index:
        return "embedded"
    return "mixed"


def find_path_usages(schema_ref: str, spec: dict) -> list[tuple[str, str, str]]:
    """`(path, METHOD, role)` for every operation referencing `schema_ref`."""
    usages = []
    for path, item in spec.get("paths", {}).items():
        for method, op in item.items():
            if not isinstance(op, dict):
                continue
            role = None
            rb = op.get("requestBody", {})
            for ct in rb.get("content", {}).values():
                if _refs_schema(ct.get("schema", {}), schema_ref):
                    role = "request body"
            for status, resp in op.get("responses", {}).items():
                for ct in resp.get("content", {}).values():
                    if _refs_schema(ct.get("schema", {}), schema_ref) and role is None:
                        role = f"response {status}"
            if role:
                usages.append((path, method.upper(), role))
    return usages


# ── inline (path-defined) schemas — for specs that don't name component schemas ─

def component_schemas(spec: dict) -> dict:
    return spec.get("components", {}).get("schemas", {})


def inline_path_schemas(spec: dict) -> list[tuple[str, dict]]:
    """`(label, schema)` for request/response bodies defined inline (no `$ref`).

    Some non-*arr specs (e.g. bazarr) inline their bodies instead of naming
    `*Resource` component schemas; without this they're invisible to the tools.
    """
    out: list[tuple[str, dict]] = []
    for path, item in spec.get("paths", {}).items():
        for method, op in item.items():
            if not isinstance(op, dict):
                continue
            M = method.upper()
            rb = op.get("requestBody", {})
            for ct in rb.get("content", {}).values():
                if _is_inline_object(ct.get("schema", {})):
                    out.append((f"{path} {M} request", ct["schema"]))
            for status, resp in op.get("responses", {}).items():
                for ct in resp.get("content", {}).values():
                    if _is_inline_object(ct.get("schema", {})):
                        out.append((f"{path} {M} response {status}", ct["schema"]))
    return out


# ── internals ─────────────────────────────────────────────────────────────────

def _operation_body_schemas(op: dict):
    """Yield every request/response content schema dict on an operation."""
    for ct in op.get("requestBody", {}).get("content", {}).values():
        yield ct.get("schema", {})
    for resp in op.get("responses", {}).values():
        for ct in resp.get("content", {}).values():
            yield ct.get("schema", {})


def _refs_schema(s: dict, schema_ref: str) -> bool:
    return s.get("$ref") == schema_ref or s.get("items", {}).get("$ref") == schema_ref


def _is_inline_object(s: dict) -> bool:
    if not isinstance(s, dict) or "$ref" in s:
        return False
    if s.get("type") == "array":
        return _is_inline_object(s.get("items", {}))
    return s.get("type") == "object" or "properties" in s
