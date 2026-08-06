# Plan: Port `python-ort` to `modelo` (Rust)

## References
- python-ort - https://github.com/heliocastro/python-ort
- ort original models - https://github.com/oss-review-toolkit/ort/tree/main/model/src/main/kotlin

## Project Overview

- **Name**: `modelo`
- **Description**: Validation Extended - Modular dataclass model validator in Rust.
- **Goal**: A high-performance Rust implementation of the `python-ort` logic, providing modular model validation.
- **Success Criteria**:
  - High idiomatic Rust code (standards-compliant).
  - Matching functionality to `python-ort`.
  - CLI interface using `ratatui` and `clap`.
  - Python bindings that allow seamless replacement of `python-ort`.

## Implementation Strategy

### Phase 1: Foundation & Scaffolding

- [x] Initialize Cargo project named `modelo`.
- [x] Add dependencies: `clap`, `ratatui`, `serde`, `pyo3` (for bindings), `thiserror`, `anyhow`.
- [x] Set up project structure:
  - `src/lib.rs` (Core traits and engine).
  - `src/models/` (Step-by-step implementation of model classes).
  - `src/cli.rs` (Ratatui/Clap interface).
  - `tests/` (Integration tests).
- [x] Add MIT License.
- [x] Add copyrights for Helio Chissini de Castro <dev@heliocastro.info> with this year

### Phase 2: Incremental Model Implementation

*Each class from the original ORT/python-ort will be implemented one by one. No tests or compilation of the full suite until the core models are drafted to avoid premature failure cycles.*

- [x] **Step 1**: Core Validation Traits (The "engine" logic).
- [x] **Step 2**: Base Model Classes (Common properties).
- [x] Port the python-ort models to equivalent rust (~80 classes total across `models/`, `models/config/`, `models/licenses/`, `models/vulnerabilities/`, `models/config/snippet/`)
  - [x] identifier, hash, issue, severity, text_location, vcs_info, vcs_type (initial batch)
  - [x] remote_artifact, package_linkage, package_reference, scope, source_code_origin, processed_declared_license, package, project, repository (batch 2)
  - [x] remaining core `models/` classes: advisor_*, analyzer_*, base_run, copyright_finding, defect, dependency_graph*, evaluator_run, file_list, license_finding, license_source, ort_result, ort_resolutions, package_curation*, provenance*, rule_violation, scan_result, scan_summary, scanner_*, snippet*, vcsinfo_curation_data
  - [x] `models/config/` subpackage (~30 classes, including `repository_configuration` now field-by-field ported and `models/config/snippet/` subpackage)
  - [x] `models/licenses/` subpackage (3 classes)
  - [x] `models/vulnerabilities/` subpackage (5 classes)
- [x] Create a binary for https://github.com/heliocastro/python-ort/blob/main/src/tools/ort_validate.py using ratatui and clap

### Phase 3: Implement the auxiliary tools
- [x] Implement the https://github.com/heliocastro/python-ort/tree/main/src/ort/utils if not done yet
  - [x] `environment.py` -> `src/models/environment.rs`, now the typed field on `BaseRun` (was a `serde_json::Value` stub)
  - [x] `spdx/spdx_expression.py`, `spdx/spdx_license_choice.py` -> `src/models/spdx_expression.rs`, `src/models/spdx_license_choice.rs`
  - [x] `processed_declared_license.py` already ported in batch 2 (`src/models/processed_declared_license.rs`)
  - [x] `validated_enum.py` -> `src/models/validated_int_enum.rs`: the `validated_int_enum!` macro gives every int-backed enum python-ort's semantics (accepts the numeric value or the case-sensitive member name, serializes as the member name), replacing the earlier per-type hand-written impls
  - [ ] `yaml_loader.py`: custom-tag-tolerant YAML loader, belongs with the CLI's file-loading code (Phase 4) rather than `models/`, not ported yet
- [x] Implement the examples folder — `examples/license_classifications.py`, `examples/ort_result.py`, `examples/repo_config.py` (counterparts of python-ort's, standard library only, no `click`/`rich`), plus `examples/parity_check.py`

### Phase 4: Testing & Verification

- [x] Unit tests for individual models.
- [x] Integration tests in `tests/` directory.
  - [x] Replicated python-ort's full `tests/` suite as Rust integration tests: all 17 YAML
    fixtures copied verbatim into `tests/data/` (preserving `advisor/` and `repo_config/`
    subdirectories), and all `test_*.py` files ported to `tests/*.rs` (`advisor_details`,
    `advisor_result`, `cvss_ratings`, `evaluator_run`, `license_classifications`,
    `package_configuration`, `package_curation`, `repo_config_curations`, `repo_config_files`,
    `repo_config_license_choices`, `repository_analyzer_config`, `repository_configuration`,
    `scan_result`, `vulnerability_reference`), plus a small shared `tests/common/mod.rs` helper.
  - [x] Skipped `test_validated_int_enum.py`: it tests python-ort's `ValidatedIntEnum` Pydantic
    mechanism, which has no equivalent construct in this port (each int-backed enum here has its
    own hand-written `Serialize`/`Deserialize`, already covered by that enum's own unit tests).
- [ ] Performance benchmarking (optional but recommended).

### Phase 5: CLI & User Interface

- [x] Implement `clap` argument parsing.
- [x] Implement `ratatui` TUI for interactive model inspection/validation.

### Phase 6: Python Bindings (The "Drop-in" Replacement)

- [x] Implement `pyo3` modules (`src/python.rs`, feature-gated behind `python`; see `Cargo.toml`'s `[features]` and the `pyo3 = { optional = true }` dependency to keep the `modelo` binary target from linking against libpython).
- [x] Match `python-ort` API exactly (Class names, method signatures) — the 3 top-level models (`LicenseClassifications`, `RepositoryConfiguration`, `OrtResult`) expose `from_yaml_str`/`from_yaml_file`/`to_json`/`to_dict`/`__repr__`, and the whole parsed tree is reachable with attribute access: every nested model is an instance of a `modelo.ort.Object` subclass named after it. The object tree is built by a `serde::Serializer` (`src/python/serializer.rs`) driven by the same `Serialize` impls as `to_json`, so the two views cannot drift and no per-class binding code is maintained.
- [x] Package with `maturin` for easy installation (`pyproject.toml` at repo root; `maturin` itself was not installed in the build environment, so `maturin build` was not exercised — `cargo build --features python` was verified instead).

### Phase 7: Rewrite the commits
- [x] Check all commits, add --signoff to every single one, rewrite if needed. All 9 pre-existing commits (merge
    commits included) were rewritten with `git filter-branch --msg-filter` to carry
    `Signed-off-by: Helio Chissini de Castro <dev@heliocastro.info>`; commit hashes changed, and the repository has no
    remote, so nothing needed a force-push.

### Phase 8: Check parity and readme
- [x] Check parity of library results against /Users/helio/code/helio/python-ort library — `examples/parity_check.py`
    parses all 9 fixtures in `tests/data/` with both libraries and diffs the serialized documents (set-backed fields
    are order-normalized, since their dump order is arbitrary on both sides). Result: 0 mismatches;
    `analyzer-result.yml` is rejected by both for the same unknown `scopes` field. Three real gaps were found and
    fixed to get there:
  - [x] int-backed enums: name-or-number on input and name on output (`src/models/validated_int_enum.rs`), matching
        python-ort's `ValidatedIntEnum` — previously numeric-only in both directions
  - [x] `#[serde(deny_unknown_fields)]` on the 76 structs whose python-ort counterpart sets `extra="forbid"`; the
        classes using `extra="allow"`/`"ignore"` are deliberately excluded, as are the four `*Run` structs, where
        serde forbids combining `deny_unknown_fields` with `flatten`
  - [x] `src/models/coerce.rs` for python-ort's `mode="before"` scalar coercions
        (`LicenseFindingCuration.start_lines` from an int, `PackageManagerConfiguration.options` values from any
        scalar)
  - [x] all fixture patching in `tests/common/mod.rs` deleted as a result: the fixtures now deserialize verbatim, and
        the three tests that documented the lenient behaviour assert python-ort's rejection instead
- [x] Update the README.md on how to build, how to run the examples, how to run the ort_validator, how to use it in
    python.
- [x] Write some examples in python using the this rust library (see Phase 3)

### Phase 9: Prepare CI
- [x] Create Github CI build and test workflows, in multiple jobs — `.github/workflows/ci.yml`: `format` (rustfmt),
    `lint` (clippy, warnings denied), `test` (Linux/macOS/Windows matrix), `python` (bindings built and smoke-tested on
    3.10 and 3.13), `parity` (the python-ort diff above)
- [x] Create a workflow to deploy the library in the pypi registry ( like pydantic ), but do no execute any command or
    live tests — `.github/workflows/release-pypi.yml`: per-platform `maturin-action` wheel jobs + sdist, then one
    `publish` job using PyPI trusted publishing. Tag/dispatch-triggered only; never run against the live registry
    from here.
- [x] Create a workflow to deploy thr Rust library in Cargo registry, but do not execut any command or live tests —
    `.github/workflows/release-crates.yml`: `cargo test` + `cargo package`, then `cargo publish` with
    `CARGO_REGISTRY_TOKEN`. `cargo package --locked` was verified locally; nothing was published.
- [x] Update the README with the information on deployment and how to deploy on both cases

### Phase 10: Multi-model namespaces
- [x] Move the ORT model into its own namespace so further model families can be added beside it:
    `src/models/*.rs` -> `src/models/ort/*.rs`, i.e. `modelo::models::ort::<module>`. The validation
    engine (`Model`, `ValidationError`) stays family-agnostic at `modelo::models`, which is why the
    ~90 model files' `use crate::models::{Model, ValidationError}` lines are unchanged.
- [x] Python bindings: the three ORT classes moved from the top-level `modelo` module into a `modelo.ort`
    submodule (registered in `sys.modules` so `import modelo.ort` and `from modelo.ort import X` both
    work, with `__name__` and `#[pyclass(module = "modelo.ort")]` set so tracebacks and pickling
    report the dotted name). No top-level aliases were kept -- nothing is released yet.
- [x] Examples updated to `from modelo.ort import ...`; the parity check still reports 0 mismatches.
- [ ] CLI subcommands are still flat (`modelo ort-result`) rather than grouped per family
    (`modelo ort ort-result`); deferred until a second family exists to group against.

## Development Rules

- **Commit Style**: Conventional Commits (`feat:`, `fix:`, `docs:`, etc.) with `-s` (Signed-off-by).
- **Branch Names**: `feat/<name>` for features, `fix/<name>` for fixes; `<name>` in
  `lower_snake_case`, no ticket-number-only names (e.g. `feat/ort_namespace`).
- **Documentation**: All public items must have doc comments.
- **Headers**: All source files must include `Reuse` headers and MIT license notice.
- **Laziness/Efficiency**: Use standard library where possible. Use `ponytail` principles if ambiguity arises.
