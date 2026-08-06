// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_package_configuration.py`.

mod common;

use modelo::models::ort::package_configuration::PackageConfiguration;

#[test]
fn ort_docs_simple_package_configuration() {
    let value = common::load_yaml("repo_config/example_simple_package_config.yml");
    let configs = value
        .get("package_configurations")
        .expect("expected 'package_configurations' key")
        .clone();
    let configs: Vec<PackageConfiguration> =
        serde_yaml::from_value(configs).expect("failed to deserialize PackageConfiguration list");
    assert_eq!(configs.len(), 1);
    assert_eq!(configs[0].id.to_string(), "Maven:com.example:package:1.2.3");
    assert_eq!(configs[0].license_finding_curations.len(), 1);
}
