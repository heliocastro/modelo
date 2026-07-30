// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::advisor_run::AdvisorRun;
use crate::models::analyzer_run::AnalyzerRun;
use crate::models::evaluator_run::EvaluatorRun;
use crate::models::repository::Repository;
use crate::models::scanner_run::ScannerRun;
use crate::models::{Model, ValidationError};

/// The common output format for the analyzer and scanner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrtResult {
    /// Information about the repository that was used as input.
    pub repository: Repository,
    /// Details about the analyzer run using `repository` as input, if it was run.
    #[serde(default)]
    pub analyzer: Option<AnalyzerRun>,
    /// Details about the scanner run using the analyzer result as input, if it was run.
    #[serde(default)]
    pub scanner: Option<ScannerRun>,
    /// Details about the advisor run using the analyzer result as input, if it was run.
    #[serde(default)]
    pub advisor: Option<AdvisorRun>,
    /// Details about the evaluator run, if it was run.
    #[serde(default)]
    pub evaluator: Option<EvaluatorRun>,
    /// User defined labels associated to this result.
    #[serde(default)]
    pub labels: HashMap<String, String>,
}

impl fmt::Display for OrtResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repository)
    }
}

impl Model for OrtResult {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_have_no_runs() {
        let result = OrtResult {
            repository: Repository::default(),
            analyzer: None,
            scanner: None,
            advisor: None,
            evaluator: None,
            labels: HashMap::new(),
        };
        assert!(result.analyzer.is_none());
    }
}
