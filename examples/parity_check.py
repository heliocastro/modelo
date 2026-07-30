# SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
# SPDX-License-Identifier: MIT
"""Check that vale and python-ort produce the same parse of the same YAML files.

Requires both packages importable, e.g.:

    maturin develop --features python      # installs `vale`
    pip install python-ort                 # or: PYTHONPATH=../python-ort/src
    python examples/parity_check.py

Exits non-zero if any fixture parses differently. Fixtures both libraries reject for the same
reason count as agreement -- rejecting invalid input is the behaviour under test.
"""

import json
import sys
from pathlib import Path

import vale as rust
from ort import OrtResult, RepositoryConfiguration, ort_yaml_load
from ort.models import LicenseClassifications

DATA = Path(__file__).resolve().parent.parent / "tests" / "data"

CASES = [
    ("license-classifications.yml", LicenseClassifications, rust.LicenseClassifications),
    ("repo_config/curations.yml", RepositoryConfiguration, rust.RepositoryConfiguration),
    ("repo_config/license_choices.yml", RepositoryConfiguration, rust.RepositoryConfiguration),
    ("repo_config/only_include.yml", RepositoryConfiguration, rust.RepositoryConfiguration),
    ("repo_config/str_boolean.ort.yml", RepositoryConfiguration, rust.RepositoryConfiguration),
    (
        "repo_config/example_simple_package_config.yml",
        RepositoryConfiguration,
        rust.RepositoryConfiguration,
    ),
    # Contains a package with an unknown `scopes` field, which both libraries reject.
    ("analyzer-result.yml", OrtResult, rust.OrtResult),
    ("evaluation-result.yml", OrtResult, rust.OrtResult),
    ("scanoss_snippets.yml", OrtResult, rust.OrtResult),
]


def prune(value):
    """Drop null/empty entries so "field omitted" compares equal to "field null"."""
    if isinstance(value, dict):
        return {k: prune(v) for k, v in value.items() if v is not None and v != [] and v != {}}
    if isinstance(value, list):
        return [prune(v) for v in value]
    return value


def canon(value):
    """Sort every list, so fields backed by a Python `set` / Rust `HashSet` (whose dump order is
    arbitrary on both sides) still compare equal."""
    if isinstance(value, dict):
        return {k: canon(v) for k, v in value.items()}
    if isinstance(value, list):
        return sorted((canon(v) for v in value), key=lambda v: json.dumps(v, sort_keys=True))
    return value


def diff(a, b, path=""):
    out = []
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a:
                out.append(f"{path}.{k}: only in vale = {json.dumps(b[k])[:80]}")
            elif k not in b:
                out.append(f"{path}.{k}: only in python-ort = {json.dumps(a[k])[:80]}")
            else:
                out += diff(a[k], b[k], f"{path}.{k}")
    elif isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            out.append(f"{path}: length {len(a)} (python-ort) vs {len(b)} (vale)")
        for i, (x, y) in enumerate(zip(a, b)):
            out += diff(x, y, f"{path}[{i}]")
    elif a != b:
        out.append(f"{path}: {json.dumps(a)[:60]} (python-ort) vs {json.dumps(b)[:60]} (vale)")
    return out


def parse_python(path, model):
    with path.open() as fd:
        # `by_alias=True` so keyword-clashing fields dump under their YAML name (`from_` -> `from`).
        return prune(json.loads(model(**ort_yaml_load(fd)).model_dump_json(by_alias=True)))


def main() -> None:
    mismatches = 0
    for name, py_model, rs_model in CASES:
        path = DATA / name
        py_json = py_error = rs_json = rs_error = None
        try:
            py_json = parse_python(path, py_model)
        except Exception as e:  # noqa: BLE001 - any parse failure is a result, not a crash
            py_error = f"{type(e).__name__}: {e}"
        try:
            rs_json = prune(json.loads(rs_model.from_yaml_file(str(path)).to_json()))
        except Exception as e:  # noqa: BLE001
            rs_error = f"{type(e).__name__}: {e}"

        if py_error and rs_error:
            print(f"[{name}] both reject the file (agreement)")
            continue
        if py_error or rs_error:
            mismatches += 1
            which = "python-ort" if py_error else "vale"
            print(f"[{name}] only {which} rejects it: {(py_error or rs_error)[:200]}")
            continue

        diffs = diff(canon(py_json), canon(rs_json))
        if diffs:
            mismatches += 1
            print(f"[{name}] {len(diffs)} difference(s):")
            for d in diffs[:15]:
                print("   ", d)
        else:
            order_only = bool(diff(py_json, rs_json))
            print(f"[{name}] identical" + (" (modulo set ordering)" if order_only else ""))

    print(f"\nmismatching fixtures: {mismatches} / {len(CASES)}")
    sys.exit(1 if mismatches else 0)


if __name__ == "__main__":
    main()
