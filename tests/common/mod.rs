// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Shared helpers for the integration test suite ported from python-ort's `tests/`.
//!
//! Each `tests/*.rs` file is its own binary and pulls this module in via `mod common;`, but not
//! every test file uses every helper/table here -- `dead_code` is expected per-binary.
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

use serde_yaml::Value;

/// Resolves a path under `tests/data/`, relative to the crate root, regardless of the
/// directory `cargo test` is invoked from.
pub fn fixture_path(rel: &str) -> PathBuf {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data")).join(rel)
}

/// Loads and parses a fixture file from `tests/data/` as a raw [`serde_yaml::Value`].
pub fn load_yaml(rel: &str) -> Value {
    let text = fs::read_to_string(fixture_path(rel))
        .unwrap_or_else(|e| panic!("failed to load fixture {rel}: {e}"));
    serde_yaml::from_str(&text).unwrap_or_else(|e| panic!("failed to parse fixture {rel}: {e}"))
}
