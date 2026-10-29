# Plugin SDK and release tooling

Rust wire types and the shared GitHub Actions workflow live here. Plugin repositories pin both to an exact commit.

Run `cargo test --locked` for SDK tests and `python3 -m unittest discover -s scripts -p "test_*.py"` for packaging tests.

The reusable workflow tests each plugin, builds its portable runtime, validates its manifest, and packages only runtime files. Pushing `v<manifest.version>` creates a GitHub release after all checks pass. Each release has one package ZIP and a SHA-256 file. Rust and Go templates publish their WASM variant. The Python template publishes its local-binary script and cannot be listed in the WASM-only catalogue.

Callers must pass a matching `tooling-ref` and use the same commit in `jobs.plugin.uses`. Update these pins explicitly when adopting tooling changes. Repositories do not currently declare an open-source license; public visibility does not grant a license.
