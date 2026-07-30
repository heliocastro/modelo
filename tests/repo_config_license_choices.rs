// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_repo_config_license_choices.py`.

mod common;

use vale::models::ort::repository_configuration::RepositoryConfiguration;

fn load_repo_config(name: &str) -> RepositoryConfiguration {
    let value = common::load_yaml(&format!("repo_config/{name}"));
    serde_yaml::from_value(value)
        .unwrap_or_else(|e| panic!("{name} raised a deserialize error: {e}"))
}

#[test]
fn license_choices_yml_loads_without_error() {
    load_repo_config("license_choices.yml");
}

#[test]
fn license_choices_yml_excludes_scopes() {
    let repo_config = load_repo_config("license_choices.yml");
    let excludes = repo_config.excludes.expect("excludes section is missing");
    assert_eq!(excludes.scopes.len(), 1);
    assert_eq!(excludes.scopes[0].pattern, "devDependencies");
    assert_eq!(excludes.scopes[0].reason.to_string(), "DEV_DEPENDENCY_OF");
    assert_eq!(excludes.scopes[0].comment, "Packages for development only.");
}

#[test]
fn license_choices_yml_package_license_choices() {
    let repo_config = load_repo_config("license_choices.yml");
    let license_choices = repo_config
        .license_choices
        .expect("license_choices section is missing");
    assert_eq!(license_choices.package_license_choices.len(), 1);

    let package_choice = &license_choices.package_license_choices[0];
    assert_eq!(
        package_choice.package_id.to_string(),
        "NPM::promised-io:0.3.6"
    );

    assert_eq!(package_choice.license_choices.len(), 1);
    assert_eq!(
        package_choice.license_choices[0].given.as_deref(),
        Some("AFL-2.1 OR BSD-3-Clause")
    );
    assert_eq!(package_choice.license_choices[0].choice, "BSD-3-Clause");
}

#[test]
fn bad_license_choices_yml_is_rejected() {
    // The misspelled `package_license_choice` key (singular) is rejected, as in python-ort.
    let value = common::load_yaml("repo_config/bad_license_choices.yml");
    assert!(serde_yaml::from_value::<RepositoryConfiguration>(value).is_err());
}
