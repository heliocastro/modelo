<!--
SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
SPDX-License-Identifier: MIT
-->

# modelo

modelo is a Rust multi-model validation library and a Python bindings extension intended as a
drop-in replacement for python-ort.

## Namespaces

Each model family lives in its own namespace, so families can be added without disturbing the
others. Currently there is one: **ort**, the
[OSS Review Toolkit](https://github.com/oss-review-toolkit/ort) model.

| | ORT model | Shared validation engine |
| --- | --- | --- |
| Rust | `modelo::models::ort::*` (`src/models/ort/`) | `modelo::models::{Model, ValidationError}` |
| Python | `modelo.ort` | — |

Adding a family means a new `src/models/<name>/` directory whose types implement
`models::Model`, plus a `modelo.<name>` submodule in `src/python/mod.rs`. The engine, the CLI and the
bindings' plumbing stay as they are.

## Build

```sh
cargo build --release              # library + `modelo` binary
cargo test                         # unit + integration tests (203 tests)
cargo clippy --all-targets         # lints
cargo doc --open                   # API documentation
```

The Python extension module is behind a feature flag, so the binary never links against
libpython:

```sh
cargo build --release --features python
```

## Validating files: the `modelo` CLI

```sh
modelo license-classifications  tests/data/license-classifications.yml
modelo repository-configuration tests/data/repo_config/curations.yml
modelo ort-result               tests/data/evaluation-result.yml
```

The subcommands are still flat rather than grouped per family (`modelo ort ort-result ...`); that
regrouping is worth doing when a second family lands, not before.

Each subcommand prints `valid <kind> file.` and exits 0, or reports the first parse/validation
error and exits non-zero. `--debug` additionally pretty-prints the parsed model. This is the
equivalent of python-ort's `ort-validate` (`src/tools/ort_validate.py`).

### Interactive TUI

`modelo tui [FILE]` (or `modelo` with no arguments) opens a `ratatui` interface: pick the model kind,
type a file path, see the validation result. `↑`/`↓` (or `k`/`j`) move and scroll, `Enter`
confirms, `b` goes back from the result screen, `q`/`Esc` quits.

## Using it from Python

```sh
pip install modelo
```

or, from a checkout:

```sh
pip install maturin
maturin develop --features python     # into the active virtualenv
# or: pip install .                   # maturin is the build backend
```

The `modelo.ort` submodule exposes the three top-level ORT models, each parsed from YAML and
navigated with plain attribute access, the way python-ort's pydantic models are:

```python
from pprint import pprint

from modelo.ort import LicenseClassifications, OrtResult, RepositoryConfiguration

result = OrtResult.from_yaml_file("tests/data/evaluation-result.yml")
config = RepositoryConfiguration.from_yaml_str(open(".ort.yml").read())

pprint(result.analyzer)                                 # AnalyzerRun(start_time=..., ...)
result.analyzer.environment.ort_version
result.analyzer.result.projects[0].id
result.repository.vcs.url
```

Invalid input raises `ValueError` (with the failing field path, as pydantic's `ValidationError`
does); an unreadable path raises `OSError`.

Every nested model is an instance of a class named after it (`type(result.analyzer).__name__ ==
"AnalyzerRun"`), all of them subclasses of `modelo.ort.Object`. Besides attribute access they
support `keys()`, `values()`, `items()`, `obj["field"]`, `"field" in obj`, `len(obj)`, `==` and
`to_dict()`:

```python
import json

result.to_dict()                                        # nested plain dicts and lists
json.loads(result.to_json()) == result.to_dict()        # True — one source of truth
```

Fields that ORT itself serializes as scalars stay scalars: `Identifier` is a string
(`"PIP::requirements.txt:1.0"`), enums are their member names, and free-form maps such as
`labels` are dicts.

### Type checking

The wheel is a PEP 561 typed package: `modelo/py.typed` ships alongside `modelo/ort.pyi`, so `mypy`,
`pyright` and `ty` resolve `import modelo.ort` and the model classes without falling back to `Any`.
The compiled extension lives at `modelo._modelo`; `modelo.ort` is a thin Python re-export of it, which is
what makes the module statically resolvable.

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
publisher](https://docs.pypi.org/trusted-publishers/) for the `modelo` project pointing at this
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
