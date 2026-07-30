// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::issue_resolution::IssueResolution;
use crate::models::ort::rule_violation_resolution::RuleViolationResolution;
use crate::models::ort::vulnerability_resolution::VulnerabilityResolution;
use crate::models::{Model, ValidationError};

/// Resolutions for issues, rule violations and vulnerabilities of a repository.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Resolutions {
    /// Resolutions for issues with the analysis or scan of the projects in this repository.
    #[serde(default)]
    pub issues: Vec<IssueResolution>,
    /// Resolutions for license policy violations.
    #[serde(default)]
    pub rule_violations: Vec<RuleViolationResolution>,
    /// Resolutions for vulnerabilities provided by the advisor.
    #[serde(default)]
    pub vulnerabilities: Vec<VulnerabilityResolution>,
}

impl fmt::Display for Resolutions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Resolutions({} issues, {} rule_violations, {} vulnerabilities)",
            self.issues.len(),
            self.rule_violations.len(),
            self.vulnerabilities.len()
        )
    }
}

impl Model for Resolutions {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let resolutions = Resolutions::default();
        assert!(resolutions.issues.is_empty());
        assert!(resolutions.rule_violations.is_empty());
        assert!(resolutions.vulnerabilities.is_empty());
    }
}
