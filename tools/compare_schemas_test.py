import json
import os
import sys
import tempfile
import unittest
from io import StringIO
from unittest.mock import patch

sys.path.insert(0, ".")
from fixture import SPEC, SPEC2
from compare_schemas import get_fields, main


class TestGetFields(unittest.TestCase):
    def test_naming_fields(self):
        fields = get_fields("#/components/schemas/NamingResource", SPEC)
        self.assertIn("renameMovies", fields)
        self.assertIn("standardMovieFormat", fields)
        self.assertIn("id", fields)

    def test_enum_ref_resolved_in_type(self):
        fields = get_fields("#/components/schemas/NamingResource", SPEC)
        self.assertIn("colonReplacementFormat", fields)
        t = fields["colonReplacementFormat"]
        self.assertIn("ColonReplacementFormat", t)
        self.assertIn("enum", t)

    def test_nullable_shown_in_type(self):
        fields = get_fields("#/components/schemas/NamingResource", SPEC)
        # standardMovieFormat is nullable
        self.assertIn("?", fields["standardMovieFormat"])

    def test_array_type(self):
        fields = get_fields("#/components/schemas/IndexerResource", SPEC)
        self.assertIn("array", fields["fields"])

    def test_missing_schema_returns_empty(self):
        fields = get_fields("#/components/schemas/DoesNotExist", SPEC)
        self.assertEqual(fields, {})


class TestMainOutput(unittest.TestCase):
    def setUp(self):
        self.f1 = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False)
        self.f2 = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False)
        json.dump(SPEC, self.f1)
        json.dump(SPEC2, self.f2)
        self.f1.close()
        self.f2.close()

    def tearDown(self):
        os.unlink(self.f1.name)
        os.unlink(self.f2.name)

    def _run(self, *extra):
        with patch("sys.argv", [
            "compare_schemas.py",
            self.f1.name, "#/components/schemas/NamingResource",
            self.f2.name, "#/components/schemas/NamingResource",
            *extra,
        ]):
            with patch("sys.stdout", new_callable=StringIO) as out:
                main()
                return out.getvalue()

    def test_only_in_a_shown(self):
        out = self._run()
        self.assertIn("Only in A", out)
        # renameMovies is in SPEC but not SPEC2
        self.assertIn("renameMovies", out)

    def test_only_in_b_shown(self):
        out = self._run()
        self.assertIn("Only in B", out)
        # renameEpisodes is in SPEC2 but not SPEC
        self.assertIn("renameEpisodes", out)

    def test_identical_fields_count_shown(self):
        out = self._run()
        self.assertIn("Identical", out)

    def test_all_flag_shows_identical_fields(self):
        out = self._run("--all")
        # id and movieFolderFormat are in both
        self.assertIn("id", out)
        self.assertIn("movieFolderFormat", out)

    def test_headers_show_spec_paths(self):
        out = self._run()
        self.assertIn("NamingResource", out)

    def test_missing_schema_exits_nonzero(self):
        with patch("sys.argv", [
            "compare_schemas.py",
            self.f1.name, "#/components/schemas/DoesNotExist",
            self.f2.name, "#/components/schemas/NamingResource",
        ]):
            with self.assertRaises(SystemExit) as cm:
                main()
            self.assertNotEqual(cm.exception.code, 0)


if __name__ == "__main__":
    unittest.main()
