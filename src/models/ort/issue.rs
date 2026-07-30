// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::severity::Severity;
use crate::models::{Model, ValidationError};

/// An issue that occurred while executing a scan or analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Issue {
    /// The timestamp of the issue, in RFC 3339 format.
    pub timestamp: String,
    /// A description of the issue source, e.g. the tool that caused the issue.
    pub source: String,
    /// The issue's message.
    pub message: String,
    /// The issue's severity.
    pub severity: Severity,
    /// The affected file or directory the issue is limited to, if any.
    #[serde(default)]
    pub affected_path: Option<String>,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.source, self.message)
    }
}

impl Model for Issue {
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
        let issue = Issue {
            timestamp: "2026-01-01T00:00:00Z".to_string(),
            source: "scanner".to_string(),
            message: String::new(),
            severity: Severity::Warning,
            affected_path: None,
        };
        assert!(issue.validate().is_err());
    }
}
