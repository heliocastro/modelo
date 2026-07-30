// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_package_curation.py`.
//!
//! python-ort's `Curations(packages=config_data)` builds a `Curations` directly from a bare list
//! of package curations. This port's `Curations::packages` is `Vec<PackageCuration>` (no separate
//! wrapping needed), so the fixtures (themselves bare lists) are deserialized straight into that.

mod common;

use vale::models::ort::package_curation::PackageCuration;

#[test]
fn ort_docs_simple_curation_example() {
    let value = common::load_yaml("example_simple_curation.yml");
    let packages: Vec<PackageCuration> =
        serde_yaml::from_value(value).expect("failed to deserialize example_simple_curation.yml");
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].id, "Maven:com.example.app:example:0.0.1");
}

#[test]
fn ort_docs_curation_example() {
    let value = common::load_yaml("example_curations.yml");
    let packages: Vec<PackageCuration> =
        serde_yaml::from_value(value).expect("failed to deserialize example_curations.yml");
    assert_eq!(packages.len(), 10);
}
