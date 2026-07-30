// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_scan_result.py`.
//!
//! `ScanResult`, `ScannerDetails`, `ScanSummary`, `Vulnerability` etc. are plain Rust structs
//! here, not pydantic models, so python-ort's `extra="forbid"` / missing-required-field /
//! wrong-type ValidationError cases are compile-time guaranteed (or simply don't apply) rather
//! than runtime checks; only the cases with a genuine Rust-level equivalent (deserializer
//! rejecting a malformed YAML document) are ported as such.

mod common;

use std::collections::{HashMap, HashSet};

use vale::models::ort::provenance::{Provenance, RepositoryProvenance};
use vale::models::ort::scan_result::ScanResult;
use vale::models::ort::scan_summary::ScanSummary;
use vale::models::ort::scanner_details::ScannerDetails;
use vale::models::ort::scanner_run::ScannerRun;
use vale::models::ort::snippet::Snippet;
use vale::models::ort::snippet_finding::SnippetFinding;
use vale::models::ort::text_location::TextLocation;
use vale::models::ort::vcs_info::VcsInfo;
use vale::models::ort::vcs_type::VcsType;

fn make_scanner_details() -> ScannerDetails {
    ScannerDetails {
        name: "SCANOSS".to_string(),
        version: "0.12.1".to_string(),
        configuration: String::new(),
    }
}

fn make_provenance() -> Provenance {
    Provenance::Repository(RepositoryProvenance {
        vcs_info: VcsInfo {
            vcs_type: VcsType::new("Git"),
            url: "https://github.com/heliocastro/python-ort.git".to_string(),
            revision: "15544ad032100f4f6bda18c9db6be0f489c50070".to_string(),
            path: String::new(),
        },
        resolved_revision: "15544ad032100f4f6bda18c9db6be0f489c50070".to_string(),
    })
}

fn make_summary() -> ScanSummary {
    ScanSummary {
        start_time: "2026-03-04T17:47:21Z".to_string(),
        end_time: "2026-03-04T17:47:23Z".to_string(),
        license_findings: HashSet::new(),
        copyright_findings: HashSet::new(),
        snippet_findings: HashSet::new(),
        issues: Vec::new(),
    }
}

fn make_snippet() -> Snippet {
    Snippet {
        score: 60.0,
        location: TextLocation {
            path: "model/src/main/kotlin/AnalyzerResult.kt".to_string(),
            start_line: 31,
            end_line: 56,
        },
        provenance: Provenance::Repository(RepositoryProvenance {
            vcs_info: VcsInfo {
                vcs_type: VcsType::new("Git"),
                url: "https://github.com/oss-review-toolkit/ort.git".to_string(),
                revision: String::new(),
                path: String::new(),
            },
            resolved_revision: ".".to_string(),
        }),
        purl: "pkg:github/oss-review-toolkit/ort".to_string(),
        license: "Apache-2.0".to_string(),
        additional_data: HashMap::from([(
            "file_hash".to_string(),
            "86eb0bcdef039e1cde377c92f5b7c44c".to_string(),
        )]),
    }
}

fn make_snippet_finding() -> SnippetFinding {
    SnippetFinding {
        source_location: TextLocation {
            path: "src/ort/models/analyzer_result.py".to_string(),
            start_line: 16,
            end_line: 41,
        },
        snippets: HashSet::from([make_snippet()]),
    }
}

#[test]
fn scan_result_valid_minimal() {
    let result = ScanResult {
        provenance: make_provenance(),
        scanner: make_scanner_details(),
        summary: make_summary(),
        additional_data: HashMap::new(),
    };
    assert_eq!(result.scanner.name, "SCANOSS");
    assert_eq!(result.scanner.version, "0.12.1");
    assert!(result.additional_data.is_empty());
}

#[test]
fn scan_result_with_additional_data() {
    let result = ScanResult {
        provenance: make_provenance(),
        scanner: make_scanner_details(),
        summary: make_summary(),
        additional_data: HashMap::from([("key".to_string(), "value".to_string())]),
    };
    assert_eq!(
        result.additional_data.get("key").map(String::as_str),
        Some("value")
    );
}

#[test]
fn scan_result_with_snippet_findings() {
    let finding = make_snippet_finding();
    let summary = ScanSummary {
        start_time: "2026-03-04T17:47:21Z".to_string(),
        end_time: "2026-03-04T17:47:23Z".to_string(),
        license_findings: HashSet::new(),
        copyright_findings: HashSet::new(),
        snippet_findings: HashSet::from([finding]),
        issues: Vec::new(),
    };
    let result = ScanResult {
        provenance: make_provenance(),
        scanner: make_scanner_details(),
        summary,
        additional_data: HashMap::new(),
    };
    assert_eq!(result.summary.snippet_findings.len(), 1);
    let snippet_finding = result.summary.snippet_findings.iter().next().unwrap();
    assert_eq!(
        snippet_finding.source_location.path,
        "src/ort/models/analyzer_result.py"
    );
    let snippet = snippet_finding.snippets.iter().next().unwrap();
    assert_eq!(snippet.score, 60.0);
    assert_eq!(snippet.license, "Apache-2.0");
}

#[test]
fn scan_result_provenance_with_vcs_info() {
    let result = ScanResult {
        provenance: make_provenance(),
        scanner: make_scanner_details(),
        summary: make_summary(),
        additional_data: HashMap::new(),
    };
    let Provenance::Repository(repo) = &result.provenance else {
        panic!("expected RepositoryProvenance");
    };
    assert_eq!(
        repo.vcs_info.url,
        "https://github.com/heliocastro/python-ort.git"
    );
    assert_eq!(
        repo.resolved_revision,
        "15544ad032100f4f6bda18c9db6be0f489c50070"
    );
}

#[test]
fn scan_result_hash_and_equality() {
    let result1 = ScanResult {
        provenance: make_provenance(),
        scanner: make_scanner_details(),
        summary: make_summary(),
        additional_data: HashMap::new(),
    };
    let result2 = ScanResult {
        provenance: make_provenance(),
        scanner: ScannerDetails {
            name: "ScanCode".to_string(),
            version: "1.0".to_string(),
            configuration: String::new(),
        },
        summary: make_summary(),
        additional_data: HashMap::new(),
    };
    assert_eq!(result1, result2);
    let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
    let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
    use std::hash::{Hash, Hasher};
    result1.hash(&mut hasher1);
    result2.hash(&mut hasher2);
    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn scan_result_in_set() {
    let result1 = ScanResult {
        provenance: make_provenance(),
        scanner: make_scanner_details(),
        summary: make_summary(),
        additional_data: HashMap::new(),
    };
    let result2 = ScanResult {
        provenance: make_provenance(),
        scanner: make_scanner_details(),
        summary: make_summary(),
        additional_data: HashMap::new(),
    };
    let results: HashSet<ScanResult> = HashSet::from([result1, result2]);
    assert_eq!(results.len(), 1);
}

#[test]
fn snippet_finding_hash_and_equality() {
    let finding1 = make_snippet_finding();
    let finding2 = make_snippet_finding();
    assert_eq!(finding1, finding2);
    let findings: HashSet<SnippetFinding> = HashSet::from([finding1, finding2]);
    assert_eq!(findings.len(), 1);
}

#[test]
fn snippet_hash_and_equality() {
    let snippet1 = make_snippet();
    let snippet2 = make_snippet();
    assert_eq!(snippet1, snippet2);
    let snippets: HashSet<Snippet> = HashSet::from([snippet1, snippet2]);
    assert_eq!(snippets.len(), 1);
}

// python-ort's test_scan_result_missing_* / test_scan_result_invalid_extra_field /
// test_scan_result_invalid_*_type and the ScannerDetails/ScanSummary missing-field cases all rely
// on pydantic's runtime `ValidationError`. `ScanResult`, `ScannerDetails` and `ScanSummary` here
// are plain Rust structs with no `deny_unknown_fields`: a missing field is a compile error
// (already enforced by the type system) and there is no dynamic "wrong type" case to construct in
// safe Rust -- so these have no runtime equivalent to assert.

#[test]
fn scan_result_from_yaml() {
    let scanner_data = common::load_yaml("evaluation-result.yml")
        .get("scanner")
        .cloned()
        .expect("expected 'scanner' key in YAML data");

    let run: ScannerRun =
        serde_yaml::from_value(scanner_data).expect("failed to deserialize ScannerRun from YAML");

    assert_eq!(run.base.environment.ort_version, "80.0.0");
    assert_eq!(run.base.environment.os, "Mac OS X");
    assert_eq!(run.base.environment.processors, 12);

    assert!(run.config.skip_concluded);
    assert!(run.config.skip_excluded);
    let scanners = run
        .config
        .scanners
        .expect("expected scanners config to be present");
    assert!(scanners.contains_key("ScanCode"));
    assert!(scanners.contains_key("SCANOSS"));

    let scan_results = run
        .scan_results
        .expect("expected scan_results to be present");
    assert_eq!(scan_results.len(), 1);

    let scan_result = scan_results.iter().next().unwrap();
    assert_eq!(scan_result.scanner.name, "SCANOSS");
    assert_eq!(scan_result.scanner.version, "0.12.1");

    assert!(!scan_result.summary.snippet_findings.is_empty());
    let first_finding = scan_result.summary.snippet_findings.iter().next().unwrap();
    assert!(!first_finding.source_location.path.is_empty());
    assert!(!first_finding.snippets.is_empty());
    let first_snippet = first_finding.snippets.iter().next().unwrap();
    assert!(!first_snippet.license.is_empty());
    assert!(!first_snippet.purl.is_empty());

    assert_eq!(run.scanners.len(), 1);

    assert_eq!(run.files.len(), 1);
    let file_list = run.files.iter().next().unwrap();
    assert!(!file_list.files.is_empty());

    let storages = run
        .config
        .storages
        .expect("expected storages config to be present");
    assert!(storages.contains_key("postgres"));
}
