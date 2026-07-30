// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::ort::provenance::Provenance;
use crate::models::ort::scan_summary::ScanSummary;
use crate::models::ort::scanner_details::ScannerDetails;
use crate::models::{Model, ValidationError};

/// The result of a single scan of a single package.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanResult {
    /// Provenance information about the scanned source code.
    pub provenance: Provenance,
    /// Details about the used scanner.
    pub scanner: ScannerDetails,
    /// A summary of the scan results.
    pub summary: ScanSummary,
    /// Scanner-specific data that cannot be mapped into a generalized property.
    #[serde(default)]
    pub additional_data: HashMap<String, String>,
}

impl StdHash for ScanResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.provenance.hash(state);
    }
}

impl PartialEq for ScanResult {
    fn eq(&self, other: &Self) -> bool {
        self.provenance == other.provenance
    }
}

impl Eq for ScanResult {}

impl fmt::Display for ScanResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.provenance, self.scanner)
    }
}

impl Model for ScanResult {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_is_by_provenance() {
        let scanner = ScannerDetails {
            name: "ScanCode".to_string(),
            version: "1.0".to_string(),
            configuration: String::new(),
        };
        let summary = ScanSummary {
            start_time: "2026-01-01T00:00:00Z".to_string(),
            end_time: "2026-01-01T00:01:00Z".to_string(),
            license_findings: std::collections::HashSet::new(),
            copyright_findings: std::collections::HashSet::new(),
            snippet_findings: std::collections::HashSet::new(),
            issues: Vec::new(),
        };
        let a = ScanResult {
            provenance: Provenance::Unknown,
            scanner: scanner.clone(),
            summary: summary.clone(),
            additional_data: HashMap::new(),
        };
        let b = ScanResult {
            provenance: Provenance::Unknown,
            scanner,
            summary,
            additional_data: HashMap::new(),
        };
        assert_eq!(a, b);
    }
}
