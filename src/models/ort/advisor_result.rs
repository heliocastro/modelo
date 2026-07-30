// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::advisor_details::AdvisorDetails;
use crate::models::ort::advisor_summary::AdvisorSummary;
use crate::models::ort::defect::Defect;
use crate::models::ort::vulnerability::Vulnerability;
use crate::models::{Model, ValidationError};

/// The result of a specific advisor execution for a single package.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdvisorResult {
    /// Details about the used advisor.
    pub advisor: AdvisorDetails,
    /// A summary of the advisor results.
    pub summary: AdvisorSummary,
    /// The vulnerabilities found.
    #[serde(default)]
    pub vulnerabilities: Vec<Vulnerability>,
    /// The defects found. Deprecated in python-ort; kept for compatibility.
    #[serde(default)]
    pub defects: Vec<Defect>,
}

impl fmt::Display for AdvisorResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.advisor)
    }
}

impl Model for AdvisorResult {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_advisor_name() {
        let result = AdvisorResult {
            advisor: AdvisorDetails {
                name: "OSV".to_string(),
                capabilities: None,
            },
            summary: AdvisorSummary {
                start_time: "2026-01-01T00:00:00Z".to_string(),
                end_time: "2026-01-01T00:01:00Z".to_string(),
                issues: Vec::new(),
            },
            vulnerabilities: Vec::new(),
            defects: Vec::new(),
        };
        assert_eq!(result.to_string(), "OSV");
    }
}
