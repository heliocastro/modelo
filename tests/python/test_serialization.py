# SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
# SPDX-License-Identifier: MIT
"""Tests for the `to_json`/`to_yaml`/`to_dict` serialization methods shared by every
entry-point model class (`OrtResult`, `RepositoryConfiguration`, `LicenseClassifications`).

Run with:

    uv run pytest tests/python
"""

import json
from pathlib import Path

import pytest
import yaml

from modelo.ort import LicenseClassifications, OrtResult, RepositoryConfiguration

DATA_DIR = Path(__file__).parent.parent / "data"

ENTRY_POINTS = [
    (LicenseClassifications, DATA_DIR / "license-classifications.yml"),
    (RepositoryConfiguration, DATA_DIR / "repo_config" / "curations.yml"),
    (OrtResult, DATA_DIR / "evaluation-result.yml"),
]


@pytest.mark.parametrize(("model_cls", "path"), ENTRY_POINTS, ids=[cls.__name__ for cls, _ in ENTRY_POINTS])
def test_to_json_returns_valid_json_string(model_cls, path):
    """`to_json()` returns a string that `json.loads` can parse back into a dict."""
    parsed = model_cls.from_yaml_file(str(path))

    result = parsed.to_json()

    assert isinstance(result, str)
    decoded = json.loads(result)
    assert isinstance(decoded, dict)
    assert decoded


@pytest.mark.parametrize(("model_cls", "path"), ENTRY_POINTS, ids=[cls.__name__ for cls, _ in ENTRY_POINTS])
def test_to_yaml_returns_valid_yaml_string(model_cls, path):
    """`to_yaml()` returns a string that `yaml.safe_load` can parse back into a dict."""
    parsed = model_cls.from_yaml_file(str(path))

    result = parsed.to_yaml()

    assert isinstance(result, str)
    decoded = yaml.safe_load(result)
    assert isinstance(decoded, dict)
    assert decoded


@pytest.mark.parametrize(("model_cls", "path"), ENTRY_POINTS, ids=[cls.__name__ for cls, _ in ENTRY_POINTS])
def test_to_dict_returns_plain_dict(model_cls, path):
    """`to_dict()` returns a plain `dict`, recursively free of `Object` instances."""
    parsed = model_cls.from_yaml_file(str(path))

    result = parsed.to_dict()

    assert isinstance(result, dict)
    assert result
    _assert_no_modelo_objects(result)


@pytest.mark.parametrize(("model_cls", "path"), ENTRY_POINTS, ids=[cls.__name__ for cls, _ in ENTRY_POINTS])
def test_to_json_to_dict_and_to_yaml_agree(model_cls, path):
    """The three views of the same parsed model carry the same data.

    `yaml.safe_load` parses RFC 3339 timestamps into `datetime` objects, while `json.loads`
    and `to_dict()` keep them as plain strings, so timestamps are stringified before comparing.
    """
    parsed = model_cls.from_yaml_file(str(path))

    from_json = json.loads(parsed.to_json())
    from_yaml = json.loads(json.dumps(yaml.safe_load(parsed.to_yaml()), default=lambda o: o.isoformat().replace("+00:00", "Z")))
    from_dict = json.loads(json.dumps(parsed.to_dict()))

    assert from_json == from_yaml == from_dict


def _assert_no_modelo_objects(value: object) -> None:
    """Recursively asserts `value` contains no `modelo.ort.Object` instances (or subclasses)."""
    from modelo.ort import Object

    if isinstance(value, Object):
        raise AssertionError(f"found a live Object instance in to_dict() output: {value!r}")
    if isinstance(value, dict):
        for item in value.values():
            _assert_no_modelo_objects(item)
    elif isinstance(value, list):
        for item in value:
            _assert_no_modelo_objects(item)


# Nested models (reachable from an `OrtResult`, never parsed directly from YAML) also expose
# `to_json`/`to_yaml`, each serializing just the subtree rooted at that object.
_ORT_RESULT = OrtResult.from_yaml_file(str(DATA_DIR / "evaluation-result.yml"))

NESTED_MODELS = [
    _ORT_RESULT.scanner,
    _ORT_RESULT.analyzer.result.packages[0],
    _ORT_RESULT.analyzer.result.packages[0].vcs,
]


@pytest.mark.parametrize("nested", NESTED_MODELS, ids=[type(m).__name__ for m in NESTED_MODELS])
def test_nested_model_to_json_returns_valid_json_string(nested):
    """`to_json()` on a nested model returns a string that `json.loads` can parse back into a dict."""
    result = nested.to_json()

    assert isinstance(result, str)
    decoded = json.loads(result)
    assert isinstance(decoded, dict)
    assert decoded


@pytest.mark.parametrize("nested", NESTED_MODELS, ids=[type(m).__name__ for m in NESTED_MODELS])
def test_nested_model_to_yaml_returns_valid_yaml_string(nested):
    """`to_yaml()` on a nested model returns a string that `yaml.safe_load` can parse back into a dict."""
    result = nested.to_yaml()

    assert isinstance(result, str)
    decoded = yaml.safe_load(result)
    assert isinstance(decoded, dict)
    assert decoded


@pytest.mark.parametrize("nested", NESTED_MODELS, ids=[type(m).__name__ for m in NESTED_MODELS])
def test_nested_model_to_json_matches_to_dict(nested):
    """`to_json()` and `to_dict()` describe the same subtree."""
    from_json = json.loads(nested.to_json())
    from_dict = json.loads(json.dumps(nested.to_dict()))

    assert from_json == from_dict


def test_nested_model_to_json_is_subtree_of_parent():
    """A nested model's `to_json()` matches the corresponding branch of the parent's `to_dict()`."""
    scanner_from_parent = _ORT_RESULT.to_dict()["scanner"]

    assert json.loads(_ORT_RESULT.scanner.to_json()) == scanner_from_parent
