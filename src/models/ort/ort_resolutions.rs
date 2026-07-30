// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::issue_resolution_reason::IssueResolutionReason;
use crate::models::ort::rule_violation_reason::RuleViolationResolutionReason;
use crate::models::ort::vulnerability_resolution_reason::VulnerabilityResolutionReason;
use crate::models::{Model, ValidationError};

/// An issue resolution entry in a standalone `resolutions.yml` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrtResolutionIssue {
    pub message: String,
    pub reason: IssueResolutionReason,
    #[serde(default)]
    pub comment: Option<String>,
}

/// A rule violation resolution entry in a standalone `resolutions.yml` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrtResolutionRuleViolation {
    pub message: String,
    pub reason: RuleViolationResolutionReason,
    #[serde(default)]
    pub comment: Option<String>,
}

/// A vulnerability resolution entry in a standalone `resolutions.yml` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrtResolutionVulnerability {
    pub id: String,
    pub reason: VulnerabilityResolutionReason,
    #[serde(default)]
    pub comment: Option<String>,
}

// ponytail: python-ort models this as three pydantic subclasses forming a "one of the three
// lists is required" union (`OrtResolutions1/2/3`). A single struct with a validate() check is
// simpler and has the same effect for a Rust consumer.
/// The top-level format of a standalone ORT `resolutions.yml` file: resolves issues, rule
/// violations and/or security vulnerabilities. At least one of the three lists must be present.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrtResolutions {
    #[serde(default)]
    pub issues: Option<Vec<OrtResolutionIssue>>,
    #[serde(default)]
    pub rule_violations: Option<Vec<OrtResolutionRuleViolation>>,
    #[serde(default)]
    pub vulnerabilities: Option<Vec<OrtResolutionVulnerability>>,
}

impl fmt::Display for OrtResolutions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OrtResolutions")
    }
}

impl Model for OrtResolutions {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.issues.is_none() && self.rule_violations.is_none() && self.vulnerabilities.is_none()
        {
            return Err(ValidationError::Generic(
                "at least one of issues, rule_violations, or vulnerabilities must be present"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_resolutions_fail_validation() {
        assert!(OrtResolutions::default().validate().is_err());
    }

    #[test]
    fn one_populated_list_passes_validation() {
        let resolutions = OrtResolutions {
            issues: Some(Vec::new()),
            rule_violations: None,
            vulnerabilities: None,
        };
        assert!(resolutions.validate().is_ok());
    }
}
