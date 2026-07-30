// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::base_run::BaseRun;
use crate::models::file_list::FileList;
use crate::models::identifier::Identifier;
use crate::models::issue::Issue;
use crate::models::provenance_resolution_result::ProvenanceResolutionResult;
use crate::models::scan_result::ScanResult;
use crate::models::scanner_configuration::ScannerConfiguration;
use crate::models::{Model, ValidationError};

// ponytail: python-ort's `issues` field is `dict[Identifier, set[Issue]]`; `Issue` doesn't
// derive `Hash`/`Eq` in this port, so a `Vec` is used per identifier instead of a set.
/// The summary of a single run of the scanner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerRun {
    /// The base run information (start/end time, environment).
    #[serde(flatten)]
    pub base: BaseRun,
    /// The scanner configuration used for this run.
    pub config: ScannerConfiguration,
    /// The results of provenance resolution for all projects and packages.
    pub provenances: HashSet<ProvenanceResolutionResult>,
    /// The scan results of this run.
    #[serde(default)]
    pub scan_results: Option<HashSet<ScanResult>>,
    /// Issues that occurred during a scan besides the ones created by scanners themselves.
    #[serde(default)]
    pub issues: HashMap<Identifier, Vec<Issue>>,
    /// The project / package identifiers that have been scanned, with the scanner names used.
    pub scanners: HashMap<Identifier, HashSet<String>>,
    /// The list of files for each resolved provenance.
    pub files: HashSet<FileList>,
}

impl fmt::Display for ScannerRun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl Model for ScannerRun {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::environment::Environment;

    #[test]
    fn displays_base_run() {
        let run = ScannerRun {
            base: BaseRun {
                start_time: "2026-01-01T00:00:00Z".to_string(),
                end_time: "2026-01-01T00:01:00Z".to_string(),
                environment: Environment {
                    ort_version: "1.0".to_string(),
                    build_jdk: "17".to_string(),
                    java_version: "17".to_string(),
                    os: "Linux".to_string(),
                    processors: 8,
                    max_memory: 1024,
                    variables: HashMap::new(),
                },
            },
            config: ScannerConfiguration::default(),
            provenances: HashSet::new(),
            scan_results: None,
            issues: HashMap::new(),
            scanners: HashMap::new(),
            files: HashSet::new(),
        };
        assert_eq!(
            run.to_string(),
            "2026-01-01T00:00:00Z - 2026-01-01T00:01:00Z"
        );
    }
}
