// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_advisor_result.py`.

mod common;

use vale::models::ort::advisor_details::AdvisorDetails;
use vale::models::ort::advisor_result::AdvisorResult;
use vale::models::ort::advisor_summary::AdvisorSummary;

#[test]
fn advisor_result_from_yaml() {
    let value = common::load_yaml("advisor/advisor_result.yml");
    let result: AdvisorResult =
        serde_yaml::from_value(value).expect("failed to deserialize AdvisorResult");

    assert_eq!(result.advisor.name, "VulnerableCode");
    assert_eq!(result.vulnerabilities.len(), 2);
    assert_eq!(result.vulnerabilities[0].id, "CVE-2024-1234");
    assert_eq!(result.vulnerabilities[0].references.len(), 2);
    assert_eq!(
        result.vulnerabilities[0].references[0]
            .scoring_system
            .as_deref(),
        Some("CVSS:3.1")
    );
    assert_eq!(result.vulnerabilities[0].references[0].score, Some(8.5));
}

#[test]
fn advisor_result_minimal() {
    let result = AdvisorResult {
        advisor: AdvisorDetails {
            name: "TestAdvisor".to_string(),
            capabilities: None,
        },
        summary: AdvisorSummary {
            start_time: "2024-01-01T00:00:00Z".to_string(),
            end_time: "2024-01-01T00:01:00Z".to_string(),
            issues: Vec::new(),
        },
        vulnerabilities: Vec::new(),
        defects: Vec::new(),
    };
    assert_eq!(result.advisor.name, "TestAdvisor");
    assert!(result.vulnerabilities.is_empty());
}

// python-ort's test_advisor_result_missing_advisor / test_advisor_result_missing_summary check
// that pydantic rejects a missing required field at construction time. `AdvisorResult` in this
// port is a plain Rust struct: omitting a required field is a compile error, not a runtime
// `ValidationError`, so there is nothing to assert at test-run time; the compiler already
// enforces it.

#[test]
fn advisor_summary_timestamps() {
    let summary = AdvisorSummary {
        start_time: "2024-06-01T10:00:00Z".to_string(),
        end_time: "2024-06-01T10:05:00Z".to_string(),
        issues: Vec::new(),
    };
    assert!(summary.start_time.starts_with("2024-06-01T10:00:00"));
    assert!(summary.end_time.starts_with("2024-06-01T10:05:00"));
    assert!(summary.issues.is_empty());
}
