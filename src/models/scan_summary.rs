// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::copyright_finding::CopyrightFinding;
use crate::models::issue::Issue;
use crate::models::license_finding::LicenseFinding;
use crate::models::snippet_finding::SnippetFinding;
use crate::models::{Model, ValidationError};

/// Summary of a scan, including timings, findings and issues.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanSummary {
    /// The time the scan was started, in RFC 3339 format.
    pub start_time: String,
    /// The time the scan finished, in RFC 3339 format.
    pub end_time: String,
    /// The detected license findings.
    #[serde(default, rename = "licenses")]
    pub license_findings: HashSet<LicenseFinding>,
    /// The detected copyright findings.
    #[serde(default, rename = "copyrights")]
    pub copyright_findings: HashSet<CopyrightFinding>,
    /// The detected snippet findings.
    #[serde(default, rename = "snippets")]
    pub snippet_findings: HashSet<SnippetFinding>,
    /// The issues that occurred during the scan.
    #[serde(default)]
    pub issues: Vec<Issue>,
}

impl fmt::Display for ScanSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ScanSummary({} licenses, {} copyrights)",
            self.license_findings.len(),
            self.copyright_findings.len()
        )
    }
}

impl Model for ScanSummary {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let summary = ScanSummary {
            start_time: "2026-01-01T00:00:00Z".to_string(),
            end_time: "2026-01-01T00:01:00Z".to_string(),
            license_findings: HashSet::new(),
            copyright_findings: HashSet::new(),
            snippet_findings: HashSet::new(),
            issues: Vec::new(),
        };
        assert!(summary.license_findings.is_empty());
    }
}
