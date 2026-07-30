// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::identifier::Identifier;
use crate::models::ort::license_source::LicenseSource;
use crate::models::ort::severity::Severity;
use crate::models::{Model, ValidationError};

/// A violation of a rule found during evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleViolation {
    /// The identifier of the rule that found this violation.
    pub rule: String,
    /// The identifier of the package that caused this rule violation, if any.
    #[serde(default)]
    pub pkg: Option<Identifier>,
    /// The name of the license that caused this rule violation, if any.
    #[serde(default)]
    pub license: Option<String>,
    /// The sources of the license. Can be empty if the rule does not work on licenses.
    #[serde(default)]
    pub license_sources: HashSet<LicenseSource>,
    /// The severity of the rule violation.
    pub severity: Severity,
    /// A message explaining the rule violation.
    pub message: String,
    /// Text explaining how the rule violation can be fixed. May contain Markdown.
    pub how_to_fix: String,
}

impl fmt::Display for RuleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.rule, self.message)
    }
}

impl Model for RuleViolation {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.rule.is_empty() {
            return Err(ValidationError::MissingField {
                field: "rule".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> RuleViolation {
        RuleViolation {
            rule: "NO_GPL".to_string(),
            pkg: None,
            license: None,
            license_sources: HashSet::new(),
            severity: Severity::Error,
            message: "GPL license found".to_string(),
            how_to_fix: String::new(),
        }
    }

    #[test]
    fn empty_rule_fails_validation() {
        let mut violation = make();
        violation.rule = String::new();
        assert!(violation.validate().is_err());
    }
}
