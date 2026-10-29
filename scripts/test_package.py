import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
import zipfile

spec = importlib.util.spec_from_file_location('plugin', Path(__file__).with_name('plugin.py'))
plugin = importlib.util.module_from_spec(spec)
spec.loader.exec_module(plugin)


class PackageTests(unittest.TestCase):
    def test_runtime_only_package_is_repeatable_and_checks_tag(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / 'plugin.wasm').write_bytes(b'\0asm\x01\0\0\0')
            (root / 'private-fixture.json').write_text('must not ship')
            manifest = {'version': '1.2.3', 'runtime': 'wasm-component', 'entrypoint': 'plugin.wasm'}
            archive = plugin.package(root, manifest, 'v1.2.3')
            original = archive.read_bytes()
            with zipfile.ZipFile(archive) as zipped:
                self.assertEqual(set(zipped.namelist()), {'manifest.json', 'plugin.wasm'})
                self.assertEqual(json.loads(zipped.read('manifest.json')), manifest)
            self.assertEqual(plugin.package(root, manifest).read_bytes(), original)
            with self.assertRaises(ValueError):
                plugin.package(root, manifest, 'v2.0.0')

    def test_traversal_and_symlinks_are_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / 'file').write_text('test')
            (root / 'link').symlink_to(root / 'file')
            for name in ('../file', '/etc/passwd', 'a\\b', 'link', 'missing'):
                with self.subTest(name=name), self.assertRaises(ValueError):
                    plugin.relative_file(root, name)


if __name__ == '__main__':
    unittest.main()
