// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::rule_violation_reason::RuleViolationResolutionReason;
use crate::models::{Model, ValidationError};

/// Resolves a rule violation, silencing it as not relevant or acceptable / approved.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleViolationResolution {
    /// A regular expression matching the messages of rule violations to resolve.
    pub message: String,
    /// The reason why the rule violation is resolved.
    pub reason: RuleViolationResolutionReason,
    /// A comment further explaining why the reason is applicable here.
    pub comment: String,
}

impl fmt::Display for RuleViolationResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.reason)
    }
}

impl Model for RuleViolationResolution {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.message.is_empty() {
            return Err(ValidationError::MissingField {
                field: "message".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_message_fails_validation() {
        let resolution = RuleViolationResolution {
            message: String::new(),
            reason: RuleViolationResolutionReason::CantFixException,
            comment: String::new(),
        };
        assert!(resolution.validate().is_err());
    }
}
