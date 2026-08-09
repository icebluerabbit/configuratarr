import json
import os
import sys
import tempfile
import unittest
from io import StringIO
from unittest.mock import patch

sys.path.insert(0, ".")
from fixture import SPEC
from get_path import extract_schema_ref, main


class TestExtractSchemaRef(unittest.TestCase):
    def test_direct_ref(self):
        schema = {"$ref": "#/components/schemas/NamingResource"}
        self.assertEqual(extract_schema_ref(schema), "#/components/schemas/NamingResource")

    def test_array_ref(self):
        schema = {"type": "array", "items": {"$ref": "#/components/schemas/IndexerResource"}}
        self.assertEqual(
            extract_schema_ref(schema),
            "array<#/components/schemas/IndexerResource>",
        )

    def test_primitive_returns_none(self):
        self.assertIsNone(extract_schema_ref({"type": "string"}))

    def test_empty_returns_none(self):
        self.assertIsNone(extract_schema_ref({}))


class TestMainOutput(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False)
        json.dump(SPEC, self.tmp)
        self.tmp.close()

    def tearDown(self):
        os.unlink(self.tmp.name)

    def _run(self, path):
        with patch("sys.argv", ["get_path.py", self.tmp.name, path]):
            with patch("sys.stdout", new_callable=StringIO) as out:
                main()
                return out.getvalue()

    def test_naming_id_shows_get_and_put(self):
        out = self._run("/api/v3/naming/{id}")
        self.assertIn("GET", out)
        self.assertIn("PUT", out)

    def test_naming_id_request_body_ref(self):
        out = self._run("/api/v3/naming/{id}")
        self.assertIn("NamingResource", out)
        self.assertIn("request body", out)

    def test_naming_id_response_ref(self):
        out = self._run("/api/v3/naming/{id}")
        self.assertIn("response 200", out)

    def test_indexer_shows_delete(self):
        out = self._run("/api/v3/indexer/{id}")
        self.assertIn("DELETE", out)

    def test_indexer_list_shows_array_response(self):
        out = self._run("/api/v3/indexer")
        self.assertIn("IndexerResource", out)

    def test_path_header_shown(self):
        out = self._run("/api/v3/naming/{id}")
        self.assertIn("Path:", out)
        self.assertIn("/api/v3/naming/{id}", out)

    def test_unknown_path_exits_nonzero(self):
        with patch("sys.argv", ["get_path.py", self.tmp.name, "/api/v3/doesnotexist"]):
            with self.assertRaises(SystemExit) as cm:
                main()
            self.assertNotEqual(cm.exception.code, 0)


if __name__ == "__main__":
    unittest.main()
