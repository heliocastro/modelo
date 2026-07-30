<!--
SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
SPDX-License-Identifier: MIT
-->

# vale

VALE is an acronym for **Val**idator **E**xtended, a Rust multi-model validation library
and a Python bindings extension intended as a drop-in replacement for python-ort.

## Build

```sh
cargo build --release              # library + `vale` binary
cargo test                         # unit + integration tests (203 tests)
cargo clippy --all-targets         # lints
cargo doc --open                   # API documentation
```

The Python extension module is behind a feature flag, so the binary never links against
libpython:

```sh
cargo build --release --features python
```

## Validating files: the `vale` CLI

```sh
vale license-classifications  tests/data/license-classifications.yml
vale repository-configuration tests/data/repo_config/curations.yml
vale ort-result               tests/data/evaluation-result.yml
```

Each subcommand prints `valid <kind> file.` and exits 0, or reports the first parse/validation
error and exits non-zero. `--debug` additionally pretty-prints the parsed model. This is the
equivalent of python-ort's `ort-validate` (`src/tools/ort_validate.py`).

### Interactive TUI

`vale tui [FILE]` (or `vale` with no arguments) opens a `ratatui` interface: pick the model kind,
type a file path, see the validation result. `↑`/`↓` (or `k`/`j`) move and scroll, `Enter`
confirms, `b` goes back from the result screen, `q`/`Esc` quits.

## Using it from Python

```sh
pip install vale
```

or, from a checkout:

```sh
pip install maturin
maturin develop --features python     # into the active virtualenv
# or: pip install .                   # maturin is the build backend
```

The module exposes the three top-level models, each with the same four entry points:

```python
from vale import LicenseClassifications, OrtResult, RepositoryConfiguration

result = OrtResult.from_yaml_file("tests/data/evaluation-result.yml")
config = RepositoryConfiguration.from_yaml_str(open(".ort.yml").read())

import json
analyzer = json.loads(result.to_json())["analyzer"]     # full parsed document as JSON
print(repr(config))                                      # short summary line
```

Invalid input raises `ValueError` (with the failing field path, as pydantic's `ValidationError`
does); an unreadable path raises `OSError`.

Nested models are reached through `json.loads(obj.to_json())` rather than attribute access —
see the note at the top of `src/python.rs` for why, and how to expand a class when direct
attribute access is needed.

### Examples

Runnable counterparts of python-ort's `examples/`, using only the standard library:

```sh
python examples/license_classifications.py tests/data/license-classifications.yml
python examples/repo_config.py             tests/data/repo_config/curations.yml
python examples/ort_result.py              tests/data/evaluation-result.yml --analyzer
```

### Checking parity against python-ort

`examples/parity_check.py` parses every fixture in `tests/data/` with both libraries and diffs the
serialized result, ignoring set-backed field ordering (arbitrary in both):

```sh
pip install . python-ort
python examples/parity_check.py
```

All 9 fixtures agree; `analyzer-result.yml` is rejected by both libraries for the same unknown
field. The behaviours this covers:

- Int-backed enums (`Severity`, `*Reason`, …) accept the numeric value *or* the case-sensitive
  member name, and serialize as the member name — python-ort's `ValidatedIntEnum`.
- Unknown fields are rejected wherever python-ort sets `extra="forbid"`, and accepted where it
  sets `extra="allow"`/`"ignore"`.
- Loose scalars are coerced as python-ort's `mode="before"` validators do (an integer
  `start_lines`, non-string `options` values).

## Continuous integration

`.github/workflows/ci.yml` runs on every push and pull request: `format` (rustfmt), `lint`
(clippy, warnings denied), `test` (Linux/macOS/Windows), `python` (bindings built and smoke-tested
on 3.10 and 3.13) and `parity` (the diff above against python-ort).

Every third-party action is pinned to a full commit SHA with the released tag in a trailing comment
(`uses: actions/checkout@3d3c42e... # v7.0.1`), so a moving tag cannot change what runs. To bump
one, resolve the new tag's SHA (`gh api repos/<owner>/<repo>/commits/<tag> --jq .sha`) and update
both the SHA and the comment. `actionlint .github/workflows/*.yml` validates the files.

## Deployment

Both registries publish from a `v*` tag (`git tag -s v0.1.0 && git push origin v0.1.0`) or a
manual **Run workflow** dispatch. Bump the version in `Cargo.toml` *and* `pyproject.toml` first —
they are separate files and must agree.

### PyPI — `.github/workflows/release-pypi.yml`

Builds wheels with `maturin-action` for Linux (x86_64, aarch64), macOS (x86_64, aarch64) and
Windows (x64), plus an sdist, then uploads everything in one `publish` job via
`pypa/gh-action-pypi-publish` (`maturin upload` only authenticates with an API token, so it cannot
do the OIDC exchange trusted publishing needs).

One-time setup: on PyPI, add a [trusted
publisher](https://docs.pypi.org/trusted-publishers/) for the `vale` project pointing at this
repository and the workflow file `release-pypi.yml` with environment `pypi`; then create a GitHub
environment named `pypi`. No API token is stored — the job authenticates with the OIDC token from
`id-token: write`.

### crates.io — `.github/workflows/release-crates.yml`

Runs the test suite and `cargo package` first, then `cargo publish`.

One-time setup: create a crates.io API token scoped to `publish-update` for this crate, and store
it as the secret `CARGO_REGISTRY_TOKEN` in a GitHub environment named `crates-io`.

### Local dry runs

```sh
cargo publish --dry-run
maturin build --release --features python   # wheel lands in target/wheels/
```

## License

MIT — see [LICENSE](LICENSE). Copyright 2026 Helio Chissini de Castro <dev@heliocastro.info>.
