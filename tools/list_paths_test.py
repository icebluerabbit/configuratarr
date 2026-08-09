import json
import os
import sys
import tempfile
import unittest
from io import StringIO
from unittest.mock import patch

sys.path.insert(0, ".")
from fixture import SPEC
from list_paths import main


class TestListPaths(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False)
        json.dump(SPEC, self.tmp)
        self.tmp.close()

    def tearDown(self):
        os.unlink(self.tmp.name)

    def _run(self, *args):
        with patch("sys.argv", ["list_paths.py", self.tmp.name, *args]):
            with patch("sys.stdout", new_callable=StringIO) as out:
                main()
                return out.getvalue()

    def test_lists_all_paths(self):
        out = self._run()
        self.assertIn("/api/v3/naming", out)
        self.assertIn("/api/v3/indexer", out)
        self.assertIn("/api/v3/indexer/{id}", out)

    def test_methods_shown(self):
        out = self._run()
        self.assertIn("GET", out)
        self.assertIn("PUT", out)
        self.assertIn("POST", out)
        self.assertIn("DELETE", out)

    def test_prefix_filter(self):
        out = self._run("/api/v3/indexer")
        self.assertIn("/api/v3/indexer", out)
        self.assertNotIn("/api/v3/naming", out)

    def test_prefix_filter_exact_subpath(self):
        out = self._run("/api/v3/naming")
        self.assertIn("/api/v3/naming", out)
        self.assertNotIn("/api/v3/indexer", out)

    def test_naming_id_has_get_and_put(self):
        out = self._run()
        naming_id_line = next(
            (l for l in out.splitlines() if "/api/v3/naming/{id}" in l), None
        )
        self.assertIsNotNone(naming_id_line)
        self.assertIn("GET", naming_id_line)
        self.assertIn("PUT", naming_id_line)

    def test_indexer_id_has_delete(self):
        out = self._run()
        indexer_id_line = next(
            (l for l in out.splitlines() if "/api/v3/indexer/{id}" in l), None
        )
        self.assertIsNotNone(indexer_id_line)
        self.assertIn("DELETE", indexer_id_line)

    def test_paths_sorted(self):
        out = self._run()
        paths = [l.split()[-1] for l in out.splitlines() if l.strip()]
        self.assertEqual(paths, sorted(paths))


if __name__ == "__main__":
    unittest.main()
