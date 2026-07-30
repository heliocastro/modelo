// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_repo_config_curations.py`.

mod common;

use vale::models::repository_configuration::RepositoryConfiguration;

fn load_repo_config() -> RepositoryConfiguration {
    let value = common::load_yaml("repo_config/curations.yml");
    serde_yaml::from_value(value).expect("curations.yml raised a deserialize error")
}

#[test]
fn curations_yml_loads_without_error() {
    load_repo_config();
}

#[test]
fn curations_yml_analyzer_section() {
    let repo_config = load_repo_config();
    let analyzer = repo_config.analyzer.expect("analyzer section is missing");
    assert_eq!(analyzer.skip_excluded, Some(true));
    assert_eq!(
        analyzer.enabled_package_managers,
        Some(vec!["Conan".to_string()])
    );
}

#[test]
fn curations_yml_excludes_paths() {
    let repo_config = load_repo_config();
    let excludes = repo_config.excludes.expect("excludes section is missing");
    assert_eq!(excludes.paths.len(), 7);

    assert_eq!(excludes.paths[0].pattern, "buildfiles/**");
    assert_eq!(excludes.paths[0].reason.to_string(), "BUILD_TOOL_OF");

    assert_eq!(excludes.paths[1].pattern, "doc/**");
    assert_eq!(excludes.paths[1].reason.to_string(), "DOCUMENTATION_OF");
}

#[test]
fn curations_yml_excludes_scopes() {
    let repo_config = load_repo_config();
    let excludes = repo_config.excludes.expect("excludes section is missing");
    assert_eq!(excludes.scopes.len(), 2);

    assert_eq!(excludes.scopes[0].pattern, "androidJacocoAnt");
    assert_eq!(excludes.scopes[0].reason.to_string(), "TEST_DEPENDENCY_OF");

    assert_eq!(
        excludes.scopes[1].pattern,
        "debugAndroidTestCompileClasspath"
    );
    assert_eq!(excludes.scopes[1].reason.to_string(), "TEST_DEPENDENCY_OF");
}

#[test]
fn curations_yml_snippet_choices() {
    let repo_config = load_repo_config();
    assert_eq!(repo_config.snippet_choices.len(), 4);
    assert_eq!(
        repo_config.snippet_choices[0].provenance.url,
        "https://github.com/Kitware/iMSTK.git"
    );
    assert_eq!(
        repo_config.snippet_choices[3].provenance.url,
        "https://github.com/jason-zhj/commstf.git"
    );
}

#[test]
fn curations_yml_package_curations() {
    let repo_config = load_repo_config();
    let curations = repo_config.curations.expect("curations section is missing");
    let packages = curations.packages;
    assert_eq!(packages.len(), 4);

    assert_eq!(packages[0].id, "Conan::cppcodec:0.2.0");
    let vcs0 = packages[0]
        .curations
        .vcs
        .as_ref()
        .expect("missing VCS info for packages[0]");
    assert_eq!(
        vcs0.url.as_deref(),
        Some("https://some.repository.com/bitbucket/cppcodec.git")
    );
    assert_eq!(vcs0.revision.as_deref(), Some("v0.2"));

    assert_eq!(packages[1].id, "Conan::GeographicLib:1.52.0");
    let vcs1 = packages[1]
        .curations
        .vcs
        .as_ref()
        .expect("missing VCS info for packages[1]");
    assert_eq!(vcs1.revision.as_deref(), Some("r1.52"));

    assert_eq!(packages[2].id, "Conan::PsdInterface:7.7.0");
    let vcs2 = packages[2]
        .curations
        .vcs
        .as_ref()
        .expect("missing VCS info for packages[2]");
    assert_eq!(
        vcs2.url.as_deref(),
        Some("https://some.repository.com/bitbucket/psd-interface.git")
    );
}
