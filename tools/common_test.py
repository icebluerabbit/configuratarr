import sys
import unittest

sys.path.insert(0, ".")
from fixture import SPEC
from common import (
    resolve_ref,
    resolve_schema,
    schema_properties,
    describe_type,
    is_provider,
    is_singleton,
    is_crud,
    classify,
    build_schema_path_index,
    find_path_usages,
    inline_path_schemas,
)

ALLOF_SPEC = {
    "components": {
        "schemas": {
            "Base": {"properties": {"id": {"type": "integer"}}, "required": ["id"]},
            "Derived": {
                "allOf": [
                    {"$ref": "#/components/schemas/Base"},
                    {"properties": {"name": {"type": "string", "nullable": True}}, "required": ["name"]},
                ]
            },
            "OneOfThing": {
                "oneOf": [
                    {"properties": {"a": {"type": "string"}}},
                    {"properties": {"b": {"type": "integer"}}},
                ]
            },
            "SelfRef": {"allOf": [{"$ref": "#/components/schemas/SelfRef"},
                                  {"properties": {"x": {"type": "string"}}}]},
        }
    },
    "paths": {},
}

INLINE_SPEC = {
    "components": {"schemas": {}},
    "paths": {
        "/api/items": {
            "post": {
                "requestBody": {"content": {"application/json": {
                    "schema": {"type": "object", "properties": {
                        "title": {"type": "string"}, "count": {"type": "integer"}}}
                }}},
                "responses": {"200": {"content": {"application/json": {
                    "schema": {"type": "array", "items": {
                        "type": "object", "properties": {"id": {"type": "integer"}}}}
                }}}},
            }
        }
    },
}


class TestResolveSchema(unittest.TestCase):
    def test_allof_merges_properties(self):
        merged, notes = resolve_schema(ALLOF_SPEC["components"]["schemas"]["Derived"], ALLOF_SPEC)
        self.assertEqual(set(merged["properties"]), {"id", "name"})
        self.assertEqual(set(merged["required"]), {"id", "name"})
        self.assertEqual(notes, [])

    def test_oneof_flags_and_merges_first_branch(self):
        merged, notes = resolve_schema(ALLOF_SPEC["components"]["schemas"]["OneOfThing"], ALLOF_SPEC)
        self.assertIn("a", merged["properties"])
        self.assertTrue(any("oneOf" in n for n in notes))

    def test_selfref_allof_terminates(self):
        merged, _ = resolve_schema(ALLOF_SPEC["components"]["schemas"]["SelfRef"], ALLOF_SPEC)
        self.assertIn("x", merged["properties"])

    def test_plain_schema_roundtrips(self):
        props, notes = schema_properties(SPEC["components"]["schemas"]["NamingResource"], SPEC)
        self.assertIn("renameMovies", props)
        self.assertEqual(notes, [])


class TestDescribeType(unittest.TestCase):
    def test_nullable_suffix_opt_in(self):
        p = {"type": "string", "nullable": True}
        self.assertEqual(describe_type(p, SPEC), "string")
        self.assertEqual(describe_type(p, SPEC, nullable_suffix=True), "string?")

    def test_enum_ref(self):
        out = describe_type({"$ref": "#/components/schemas/ColonReplacementFormat"}, SPEC)
        self.assertIn("ColonReplacementFormat", out)
        self.assertIn("enum", out)


class TestClassify(unittest.TestCase):
    def setUp(self):
        self.index = build_schema_path_index(SPEC)

    def test_provider_precedence_in_single_label(self):
        # IndexerResource is a provider (fields+implementation+configContract).
        s = SPEC["components"]["schemas"]["IndexerResource"]
        self.assertEqual(classify("#/components/schemas/IndexerResource", s, self.index), "provider")
        # ...but is still CRUD on the method axis.
        self.assertTrue(is_crud("#/components/schemas/IndexerResource", self.index))

    def test_singleton(self):
        self.assertTrue(is_singleton("#/components/schemas/NamingResource", self.index))
        s = SPEC["components"]["schemas"]["NamingResource"]
        self.assertEqual(classify("#/components/schemas/NamingResource", s, self.index), "singleton")

    def test_embedded(self):
        s = SPEC["components"]["schemas"]["Field"]
        self.assertEqual(classify("#/components/schemas/Field", s, self.index), "embedded")


class TestFindPathUsages(unittest.TestCase):
    def test_indexer_usages(self):
        u = find_path_usages("#/components/schemas/IndexerResource", SPEC)
        methods = {m for _, m, _ in u}
        self.assertIn("POST", methods)
        self.assertIn("PUT", methods)


class TestInlinePathSchemas(unittest.TestCase):
    def test_finds_inline_request_and_response(self):
        found = inline_path_schemas(INLINE_SPEC)
        labels = [l for l, _ in found]
        self.assertTrue(any("request" in l for l in labels))
        self.assertTrue(any("response" in l for l in labels))

    def test_none_when_all_refs(self):
        self.assertEqual(inline_path_schemas(SPEC), [])


if __name__ == "__main__":
    unittest.main()
