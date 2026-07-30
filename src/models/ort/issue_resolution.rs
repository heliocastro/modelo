// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::issue_resolution_reason::IssueResolutionReason;
use crate::models::{Model, ValidationError};

/// Resolves an issue, e.g. to silence a false positive or an issue identified as not relevant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IssueResolution {
    /// A regular expression matching the messages of issues to resolve.
    pub message: String,
    /// The reason why the issue is resolved.
    pub reason: IssueResolutionReason,
    /// A comment further explaining why the reason is applicable here.
    pub comment: String,
}

impl fmt::Display for IssueResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.reason)
    }
}

impl Model for IssueResolution {
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
        let resolution = IssueResolution {
            message: String::new(),
            reason: IssueResolutionReason::CantFixIssue,
            comment: String::new(),
        };
        assert!(resolution.validate().is_err());
    }
}
