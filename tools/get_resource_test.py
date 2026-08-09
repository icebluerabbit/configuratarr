import json
import os
import sys
import tempfile
import unittest
from io import StringIO
from unittest.mock import patch

sys.path.insert(0, ".")
from fixture import SPEC
from get_resource import describe_type, find_path_usages, resolve_ref, main


class TestResolveRef(unittest.TestCase):
    def test_resolves_enum(self):
        s = resolve_ref("#/components/schemas/ColonReplacementFormat", SPEC)
        self.assertIn("enum", s)
        self.assertIn("delete", s["enum"])

    def test_resolves_struct(self):
        s = resolve_ref("#/components/schemas/NamingResource", SPEC)
        self.assertIn("properties", s)


class TestDescribeType(unittest.TestCase):
    def test_string_nullable(self):
        prop = {"type": "string", "nullable": True}
        self.assertIn("string", describe_type(prop, SPEC))

    def test_integer(self):
        prop = {"type": "integer"}
        self.assertEqual(describe_type(prop, SPEC), "integer")

    def test_ref_enum_shows_values(self):
        prop = {"$ref": "#/components/schemas/ColonReplacementFormat"}
        result = describe_type(prop, SPEC)
        self.assertIn("ColonReplacementFormat", result)
        self.assertIn("enum", result)
        self.assertIn("delete", result)

    def test_ref_struct(self):
        prop = {"$ref": "#/components/schemas/Field"}
        result = describe_type(prop, SPEC)
        self.assertEqual(result, "Field")

    def test_array_of_ref(self):
        prop = {"type": "array", "items": {"$ref": "#/components/schemas/Field"}}
        result = describe_type(prop, SPEC)
        self.assertEqual(result, "array<Field>")

    def test_array_of_primitive(self):
        prop = {"type": "array", "items": {"type": "integer"}}
        result = describe_type(prop, SPEC)
        self.assertEqual(result, "array<integer>")

    def test_number_with_format(self):
        prop = {"type": "number", "format": "float"}
        result = describe_type(prop, SPEC)
        self.assertIn("number", result)
        self.assertIn("float", result)


class TestFindPathUsages(unittest.TestCase):
    def test_naming_in_get_and_put(self):
        usages = find_path_usages("#/components/schemas/NamingResource", SPEC)
        methods = {m for _, m, _ in usages}
        self.assertIn("GET", methods)
        self.assertIn("PUT", methods)

    def test_naming_not_in_post(self):
        usages = find_path_usages("#/components/schemas/NamingResource", SPEC)
        methods = {m for _, m, _ in usages}
        self.assertNotIn("POST", methods)

    def test_indexer_has_request_body_role(self):
        usages = find_path_usages("#/components/schemas/IndexerResource", SPEC)
        roles = {r for _, _, r in usages}
        self.assertTrue(any("request" in r for r in roles))

    def test_unknown_ref_returns_empty(self):
        usages = find_path_usages("#/components/schemas/DoesNotExist", SPEC)
        self.assertEqual(usages, [])


class TestMainOutput(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False)
        json.dump(SPEC, self.tmp)
        self.tmp.close()

    def tearDown(self):
        os.unlink(self.tmp.name)

    def _run(self, ref):
        with patch("sys.argv", ["get_resource.py", self.tmp.name, ref]):
            with patch("sys.stdout", new_callable=StringIO) as out:
                main()
                return out.getvalue()

    def test_naming_output_contains_fields(self):
        out = self._run("#/components/schemas/NamingResource")
        self.assertIn("renameMovies", out)
        self.assertIn("standardMovieFormat", out)

    def test_naming_classified_singleton(self):
        out = self._run("#/components/schemas/NamingResource")
        self.assertIn("singleton", out)

    def test_indexer_classified_correctly(self):
        out = self._run("#/components/schemas/IndexerResource")
        # has POST request body + PUT + DELETE in fixture paths
        self.assertIn("CRUD resource", out)

    def test_enum_schema_shows_values(self):
        out = self._run("#/components/schemas/ColonReplacementFormat")
        self.assertIn("enum", out)
        self.assertIn("delete", out)

    def test_readonly_flagged(self):
        out = self._run("#/components/schemas/IndexerResource")
        self.assertIn("readOnly", out)

    def test_nullable_flagged(self):
        out = self._run("#/components/schemas/NamingResource")
        self.assertIn("nullable", out)

    def test_path_usages_shown(self):
        out = self._run("#/components/schemas/NamingResource")
        self.assertIn("/api/v3/naming", out)

    def test_no_duplicate_paths(self):
        out = self._run("#/components/schemas/NamingResource")
        lines = [l for l in out.splitlines() if "/api/v3/naming/{id}" in l and "PUT" in l]
        self.assertEqual(len(lines), 1)


if __name__ == "__main__":
    unittest.main()
