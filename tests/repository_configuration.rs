// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_repository_configuration.py`.

mod common;

use vale::models::includes::Includes;
use vale::models::path_include::PathInclude;
use vale::models::path_include_reason::PathIncludeReason;
use vale::models::repository_configuration::RepositoryConfiguration;

#[test]
fn only_include_valid() {
    let value = common::load_yaml("repo_config/only_include.yml");
    let includes = value.get("includes").expect("missing 'includes'");
    let paths = includes.get("paths").expect("missing 'paths' in includes");
    let path_cfg = &paths[0];
    assert_eq!(
        path_cfg.get("pattern").and_then(|v| v.as_str()),
        Some("test/**")
    );
    assert_eq!(
        path_cfg.get("reason").and_then(|v| v.as_str()),
        Some("SOURCE_OF")
    );
    assert_eq!(
        path_cfg.get("comment").and_then(|v| v.as_str()),
        Some("Included for test")
    );

    let repo_config = RepositoryConfiguration {
        includes: Some(Includes {
            paths: vec![PathInclude {
                pattern: "test/**".to_string(),
                reason: PathIncludeReason::SourceOf,
                comment: "Included for test".to_string(),
            }],
        }),
        ..Default::default()
    };

    let includes = repo_config.includes.expect("no path includes are provided");
    assert_eq!(includes.paths.len(), 1);
    assert_eq!(includes.paths[0].pattern, "test/**");
    assert_eq!(includes.paths[0].reason, PathIncludeReason::SourceOf);
    assert_eq!(includes.paths[0].comment, "Included for test");
}

#[test]
fn only_include_reason_fail() {
    let value = common::load_yaml("repo_config/only_include_reason_fail.yml");
    let includes = value.get("includes").expect("missing 'includes'");
    let paths = includes.get("paths").expect("missing 'paths' in includes");
    let path_cfg = &paths[0];
    assert_eq!(
        path_cfg.get("pattern").and_then(|v| v.as_str()),
        Some("test/**")
    );
    assert_eq!(
        path_cfg.get("reason").and_then(|v| v.as_str()),
        Some("BINARY_OF")
    );
    assert_eq!(
        path_cfg.get("comment").and_then(|v| v.as_str()),
        Some("Included for test")
    );

    // "BINARY_OF" is not a valid PathIncludeReason (only SOURCE_OF and OTHER exist), so building
    // a PathInclude from the raw YAML fragment must fail to deserialize.
    let result: Result<PathInclude, _> = serde_yaml::from_value(path_cfg.clone());
    assert!(result.is_err());
}
