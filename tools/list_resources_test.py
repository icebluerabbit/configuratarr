import sys
import unittest
from io import StringIO
from unittest.mock import patch

sys.path.insert(0, ".")
from fixture import SPEC
from list_resources import (
    is_enum,
    is_provider,
    is_singleton,
    is_crud,
    build_schema_path_index,
    main,
)


class TestIsEnum(unittest.TestCase):
    def test_enum_schema(self):
        self.assertTrue(is_enum(SPEC["components"]["schemas"]["ColonReplacementFormat"]))

    def test_struct_schema_not_enum(self):
        self.assertFalse(is_enum(SPEC["components"]["schemas"]["NamingResource"]))


class TestIsProvider(unittest.TestCase):
    def test_indexer_is_provider(self):
        # IndexerResource has fields + implementation + configContract
        self.assertTrue(is_provider(SPEC["components"]["schemas"]["IndexerResource"]))

    def test_naming_not_provider(self):
        self.assertFalse(is_provider(SPEC["components"]["schemas"]["NamingResource"]))


class TestBuildSchemaPathIndex(unittest.TestCase):
    def setUp(self):
        self.index = build_schema_path_index(SPEC)

    def test_naming_has_get_and_put(self):
        ref = "#/components/schemas/NamingResource"
        methods = self.index.get(ref, set())
        self.assertIn("GET", methods)
        self.assertIn("PUT", methods)

    def test_indexer_has_post_put_delete(self):
        ref = "#/components/schemas/IndexerResource"
        methods = self.index.get(ref, set())
        self.assertIn("POST", methods)
        self.assertIn("PUT", methods)
        self.assertIn("DELETE", methods)

    def test_field_not_directly_referenced(self):
        ref = "#/components/schemas/Field"
        self.assertNotIn(ref, self.index)


class TestIsSingleton(unittest.TestCase):
    def setUp(self):
        self.index = build_schema_path_index(SPEC)

    def test_naming_is_singleton(self):
        self.assertTrue(is_singleton("#/components/schemas/NamingResource", self.index))

    def test_indexer_not_singleton(self):
        self.assertFalse(is_singleton("#/components/schemas/IndexerResource", self.index))


class TestIsCrud(unittest.TestCase):
    def setUp(self):
        self.index = build_schema_path_index(SPEC)

    def test_indexer_is_crud(self):
        self.assertTrue(is_crud("#/components/schemas/IndexerResource", self.index))

    def test_naming_not_crud(self):
        self.assertFalse(is_crud("#/components/schemas/NamingResource", self.index))


class TestMainOutput(unittest.TestCase):
    def _run(self, *args):
        import json, tempfile, os
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as f:
            json.dump(SPEC, f)
            path = f.name
        try:
            with patch("sys.argv", ["list_resources.py", path, *args]):
                with patch("sys.stdout", new_callable=StringIO) as mock_out:
                    main()
                    return mock_out.getvalue()
        finally:
            os.unlink(path)

    def test_no_flags_lists_all(self):
        out = self._run()
        self.assertIn("NamingResource", out)
        self.assertIn("IndexerResource", out)
        self.assertIn("ColonReplacementFormat", out)

    def test_no_enums_excludes_enums(self):
        out = self._run("--no-enums")
        self.assertNotIn("ColonReplacementFormat", out)
        self.assertIn("NamingResource", out)

    def test_enums_only(self):
        out = self._run("--enums")
        self.assertIn("ColonReplacementFormat", out)
        self.assertNotIn("NamingResource", out)

    def test_singleton_filter(self):
        out = self._run("--singleton")
        self.assertIn("NamingResource", out)
        self.assertNotIn("IndexerResource", out)

    def test_crud_filter(self):
        out = self._run("--crud")
        self.assertIn("IndexerResource", out)
        self.assertNotIn("NamingResource", out)

    def test_provider_filter(self):
        out = self._run("--provider")
        self.assertIn("IndexerResource", out)
        self.assertNotIn("NamingResource", out)

    def test_tags_shown_in_output(self):
        out = self._run()
        self.assertIn("[singleton]", out)
        # IndexerResource is provider (provider check runs first)
        self.assertIn("[provider]", out)
        self.assertIn("[enum]", out)


if __name__ == "__main__":
    unittest.main()
