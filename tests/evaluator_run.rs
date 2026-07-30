// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_evaluator_run.py`.
//!
//! `EvaluatorRun.start_time`/`end_time` are plain `String`s here (RFC 3339 text), not parsed
//! `datetime`s, so the python test's type-mismatch/missing-required-field cases (which rely on
//! pydantic's runtime validation) either don't apply or are compile-time guaranteed instead; see
//! the per-test comments below.

mod common;

use std::collections::HashMap;

use std::str::FromStr;
use vale::models::ort::base_run::BaseRun;
use vale::models::ort::environment::Environment;
use vale::models::ort::evaluator_run::EvaluatorRun;
use vale::models::ort::identifier::Identifier;
use vale::models::ort::rule_violation::RuleViolation;
use vale::models::ort::severity::Severity;

fn make_environment() -> Environment {
    Environment {
        ort_version: "81.1.0".to_string(),
        build_jdk: "21.0.10".to_string(),
        java_version: "21.0.10".to_string(),
        os: "Mac OS X".to_string(),
        processors: 12,
        max_memory: 6_442_450_944,
        variables: HashMap::new(),
    }
}

fn make_violation() -> RuleViolation {
    RuleViolation {
        rule: "NO_LICENSE_IN_DEPENDENCY".to_string(),
        pkg: Some(Identifier::from_str("PyPI::packaging:26.0").unwrap()),
        license: None,
        license_sources: Default::default(),
        severity: Severity::Error,
        message: "No license information is available for dependency 'PyPI::packaging:26.0'."
            .to_string(),
        how_to_fix: "Please conclude the appropriate license with a package curation.".to_string(),
    }
}

#[test]
fn valid_minimal() {
    let run = EvaluatorRun {
        base: BaseRun {
            start_time: "2026-03-12T15:08:05Z".to_string(),
            end_time: "2026-03-12T15:08:08Z".to_string(),
            environment: make_environment(),
        },
        violations: Vec::new(),
    };
    assert!(run.violations.is_empty());
}

#[test]
fn valid_with_violations() {
    let run = EvaluatorRun {
        base: BaseRun {
            start_time: "2026-03-12T15:08:05Z".to_string(),
            end_time: "2026-03-12T15:08:08Z".to_string(),
            environment: make_environment(),
        },
        violations: vec![make_violation()],
    };
    assert_eq!(run.violations.len(), 1);
    assert_eq!(run.violations[0].rule, "NO_LICENSE_IN_DEPENDENCY");
    assert_eq!(run.violations[0].severity, Severity::Error);
}

#[test]
fn valid_with_multiple_violations() {
    let violations = vec![
        make_violation(),
        RuleViolation {
            rule: "SOME_WARNING_RULE".to_string(),
            pkg: None,
            license: None,
            license_sources: Default::default(),
            severity: Severity::Warning,
            message: "A warning was raised.".to_string(),
            how_to_fix: "Review the warning.".to_string(),
        },
    ];
    let run = EvaluatorRun {
        base: BaseRun {
            start_time: "2026-01-01T00:00:00Z".to_string(),
            end_time: "2026-01-01T00:01:00Z".to_string(),
            environment: make_environment(),
        },
        violations,
    };
    assert_eq!(run.violations.len(), 2);
    assert_eq!(run.violations[1].severity, Severity::Warning);
    assert!(run.violations[1].pkg.is_none());
}

// python-ort's missing-start_time / missing-end_time / missing-environment / missing-extra-field
// cases rely on pydantic's `extra="forbid"` and required-field runtime validation. `EvaluatorRun`
// here is a plain Rust struct with no `deny_unknown_fields`: a missing field is a compile error
// (already enforced), and an extra field during deserialization is silently ignored rather than
// rejected -- so there is nothing to assert at test-run time for either case.

#[test]
fn invalid_violations_type_rejected_by_deserializer() {
    let yaml = r#"
start_time: "2026-03-12T15:08:05Z"
end_time: "2026-03-12T15:08:08Z"
environment:
  ort_version: "1.0"
  build_jdk: "17"
  java_version: "17"
  os: "Linux"
  processors: 8
  max_memory: 1024
violations: "not_a_list"
"#;
    assert!(serde_yaml::from_str::<EvaluatorRun>(yaml).is_err());
}

#[test]
fn invalid_violation_entry_rejected_by_deserializer() {
    let yaml = r#"
start_time: "2026-03-12T15:08:05Z"
end_time: "2026-03-12T15:08:08Z"
environment:
  ort_version: "1.0"
  build_jdk: "17"
  java_version: "17"
  os: "Linux"
  processors: 8
  max_memory: 1024
violations:
  - rule: "MISSING_REQUIRED_FIELDS"
"#;
    assert!(serde_yaml::from_str::<EvaluatorRun>(yaml).is_err());
}

#[test]
fn evaluator_run_from_yaml() {
    let evaluator = common::load_yaml("evaluation-result.yml")
        .get("evaluator")
        .cloned()
        .expect("expected 'evaluator' key in YAML data");

    let run: EvaluatorRun =
        serde_yaml::from_value(evaluator).expect("failed to deserialize EvaluatorRun from YAML");

    assert_eq!(run.base.environment.ort_version, "81.1.0-008.sha.6d867a5");
    assert_eq!(run.base.environment.os, "Mac OS X");
    assert_eq!(run.base.environment.processors, 12);
    assert!(!run.violations.is_empty());
    assert_eq!(run.violations[0].rule, "NO_LICENSE_IN_DEPENDENCY");
    assert_eq!(run.violations[0].severity, Severity::Error);
    assert!(run.violations[0].license.is_none());
    assert!(run.violations[0].license_sources.is_empty());
    assert_eq!(
        run.violations[0].pkg.as_ref().map(|p| p.to_string()),
        Some("PyPI::packaging:26.0".to_string())
    );
}
