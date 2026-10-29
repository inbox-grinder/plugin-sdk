#!/usr/bin/env python3
"""Test, build and package a plugin from an independently checked-out repository."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import stat
import subprocess
import zipfile


def run(*args, **kwargs):
    return subprocess.run(args, check=True, **kwargs)


def relative_file(root, name):
    path = Path(name)
    if path.is_absolute() or not path.parts or '..' in path.parts or '\\' in name:
        raise ValueError(f'Invalid package path: {name}')
    source = root / path
    if not source.is_file() or source.is_symlink() or not source.resolve().is_relative_to(root.resolve()):
        raise ValueError(f'Unsafe or missing package file: {name}')
    return source


def package(root, manifest, tag=None):
    version = manifest['version']
    if not re.fullmatch(r'\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?', version):
        raise ValueError('Expected a SemVer manifest version')
    if tag is not None and tag not in (version, 'v' + version):
        raise ValueError('Release tag must match manifest.version')
    names = [manifest['entrypoint']]
    names += [item['path'] for item in manifest.get('templates', []) if 'path' in item]
    if len(names) != len(set(names)) or 'manifest.json' in names:
        raise ValueError('Duplicate package paths')
    files = [(name, relative_file(root, name)) for name in names]
    if manifest['runtime'] == 'wasm-component':
        module = relative_file(root, manifest['entrypoint']).read_bytes()
        if module[:4] != b'\0asm' or len(module) > 8 * 1024 * 1024:
            raise ValueError('Invalid or oversized WASM module')
    output = root / 'dist'
    output.mkdir(exist_ok=True)
    archive = output / f"{root.name}-{version}.zip"
    entries = [('manifest.json', (json.dumps(manifest, indent=2) + '\n').encode(), 0o644)]
    entries += [(name, source.read_bytes(), 0o755 if os.access(source, os.X_OK) else 0o644)
                for name, source in files]
    if sum(len(data) for _, data, _ in entries) > 128 * 1024 * 1024:
        raise ValueError('Expanded package exceeds catalogue limit')
    with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED) as target:
        for name, data, mode in sorted(entries):
            entry = zipfile.ZipInfo(name, date_time=(2026, 1, 1, 0, 0, 0))
            entry.compress_type = zipfile.ZIP_DEFLATED
            entry.create_system = 3
            entry.external_attr = (stat.S_IFREG | mode) << 16
            target.writestr(entry, data)
    if archive.stat().st_size > 16 * 1024 * 1024:
        archive.unlink()
        raise ValueError('Compressed package exceeds catalogue limit')
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    archive.with_suffix('.sha256').write_text(f'{digest}  {archive.name}\n')
    print(f'Packaged {archive.name} sha256={digest}')
    return archive


def fixture_check(command, root):
    fixture = root / 'fixtures/email-input.json'
    expected = json.loads((root / 'fixtures/extraction-result.json').read_text())
    actual = json.loads(run(*command, input=fixture.read_bytes(), capture_output=True).stdout)
    if actual != expected:
        raise ValueError('Native plugin output differs from expected fixture')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', type=Path, default=Path.cwd())
    parser.add_argument('--tag')
    args = parser.parse_args()
    root = args.root.resolve()
    os.chdir(root)
    config = json.loads((root / '.github/plugin.json').read_text())
    kind = config['kind']
    source_manifest = 'manifest-wasm.json' if kind in ('rust-template', 'go-template') else 'manifest.json'
    manifest = json.loads((root / source_manifest).read_text())
    from jsonschema import Draft202012Validator, FormatChecker
    schema = json.loads((Path(__file__).resolve().parents[1] / 'schemas/plugin-manifest.schema.json').read_text())
    Draft202012Validator(schema, format_checker=FormatChecker()).validate(manifest)
    if args.tag and args.tag not in (manifest['version'], 'v' + manifest['version']):
        raise ValueError('Release tag must match manifest.version')
    if kind in ('rust-wasm', 'rust-template'):
        run('cargo', 'fmt', '--check')
        run('cargo', 'test', '--locked')
        metadata = json.loads(run('cargo', 'metadata', '--no-deps', '--format-version', '1', capture_output=True).stdout)
        own = next(p for p in metadata['packages'] if Path(p['manifest_path']).parent == root)
        if own['version'] != manifest['version']:
            raise ValueError('Cargo and manifest versions differ')
        run('cargo', 'build', '--locked', '--release', '--target', 'wasm32-unknown-unknown', '--lib')
        artifact = Path(metadata['target_directory']) / 'wasm32-unknown-unknown/release' / (config['crate'] + '.wasm')
        shutil.copyfile(artifact, root / manifest['entrypoint'])
        if kind == 'rust-template':
            run('cargo', 'build', '--locked', '--release', '--bin', 'plugin')
            fixture_check([str(Path(metadata['target_directory']) / 'release/plugin')], root)
    elif kind == 'go-template':
        run('go', 'test', './...')
        run('go', 'build', '-o', 'plugin', '.')
        fixture_check([str(root / 'plugin')], root)
        run('tinygo', 'build', '-target=wasip1', '-gc=leaking', '-no-debug', '-o', manifest['entrypoint'], '.')
    elif kind == 'python':
        fixture_check(['python3', str(root / 'plugin.py')], root)
    else:
        raise ValueError(f'Unsupported build kind: {kind}')
    package(root, manifest, args.tag)


if __name__ == '__main__':
    main()
