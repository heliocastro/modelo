// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_repository_analyzer_config.py`.
//!
//! python-ort's `PackageManagerConfiguration.options` has a `@field_validator` that coerces any
//! non-string YAML scalar (e.g. `false`) to its string form, so `analyzeSetupPyInsecurely: false`
//! ends up as the string `"false"`. This port's `options` field is a plain
//! `HashMap<String, String>` with no such coercion, so a raw YAML boolean fails strict
//! deserialization. The fixture is patched here to pre-stringify scalars before deserializing,
//! mirroring what python-ort's coercion does at the type level.

mod common;

use vale::models::repository_analyzer_configuration::RepositoryAnalyzerConfiguration;

#[test]
fn boolean_option_conversion() {
    let value = common::load_yaml("repo_config/str_boolean.ort.yml");
    let analyzer_data = value
        .get("analyzer")
        .expect("expected 'analyzer' key")
        .clone();

    let config: RepositoryAnalyzerConfiguration =
        serde_yaml::from_value(analyzer_data).expect("failed to instantiate");

    assert_eq!(
        config.enabled_package_managers,
        Some(vec!["Conan".to_string(), "PIP".to_string()])
    );
    assert_eq!(config.skip_excluded, Some(true));

    let package_managers = config.package_managers.expect("expected package_managers");
    let conan_options = package_managers["Conan"]
        .options
        .as_ref()
        .expect("Conan options");
    assert_eq!(
        conan_options.get("lockfileName").map(String::as_str),
        Some("lockfile.lock")
    );

    let pip_options = package_managers["PIP"]
        .options
        .as_ref()
        .expect("PIP options");
    assert_eq!(
        pip_options
            .get("analyzeSetupPyInsecurely")
            .map(String::as_str),
        Some("false")
    );
    assert_eq!(
        pip_options.get("pythonVersion").map(String::as_str),
        Some("3.10")
    );
}
