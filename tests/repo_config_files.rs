// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_repo_config_files.py`.
//!
//! python-ort parametrizes over every `*.yml` file under `tests/data/repo_config`, asserting
//! that all except two known-bad files load into `RepositoryConfiguration` without error, and
//! that those two do raise a `ValidationError`. Both cases hold here too: the models carry
//! `#[serde(deny_unknown_fields)]` wherever python-ort sets `extra="forbid"`.

mod common;

use vale::models::repository_configuration::RepositoryConfiguration;

fn load(name: &str) -> serde_yaml::Value {
    common::load_yaml(&format!("repo_config/{name}"))
}

fn assert_valid(value: serde_yaml::Value, name: &str) {
    let result: Result<RepositoryConfiguration, _> = serde_yaml::from_value(value);
    assert!(
        result.is_ok(),
        "{name} should load without error: {result:?}"
    );
}

#[test]
fn curations_yml_is_valid() {
    let value = load("curations.yml");
    let result: Result<RepositoryConfiguration, _> = serde_yaml::from_value(value);
    assert!(result.is_ok(), "curations.yml should load without error");
}

#[test]
fn example_simple_package_config_yml_is_valid() {
    assert_valid(
        load("example_simple_package_config.yml"),
        "example_simple_package_config.yml",
    );
}

#[test]
fn license_choices_yml_is_valid() {
    assert_valid(load("license_choices.yml"), "license_choices.yml");
}

#[test]
fn only_include_yml_is_valid() {
    assert_valid(load("only_include.yml"), "only_include.yml");
}

#[test]
fn str_boolean_ort_yml_is_valid() {
    assert_valid(load("str_boolean.ort.yml"), "str_boolean.ort.yml");
}

#[test]
fn only_include_reason_fail_yml_rejected_by_deserializer() {
    // `reason: BINARY_OF` isn't a valid PathIncludeReason at all (unlike the other "known
    // discrepancy" fixture below), so this still fails to deserialize even without patching --
    // matching python-ort's expectation.
    let value = load("only_include_reason_fail.yml");
    let result: Result<RepositoryConfiguration, _> = serde_yaml::from_value(value);
    assert!(result.is_err());
}

#[test]
fn bad_license_choices_yml_rejected_by_deserializer() {
    // Unknown/misspelled field name (`package_license_choice` instead of
    // `package_license_choices`), rejected here as by python-ort's `extra="forbid"`.
    let value = load("bad_license_choices.yml");
    let result: Result<RepositoryConfiguration, _> = serde_yaml::from_value(value);
    assert!(result.is_err());
}
