// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::issue::Issue;
use crate::models::{Model, ValidationError};

/// A short summary of an advisor result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdvisorSummary {
    /// The time the advisor started, in RFC 3339 format.
    pub start_time: String,
    /// The time the advisor finished, in RFC 3339 format.
    pub end_time: String,
    /// The issues that occurred during the advisor run.
    #[serde(default)]
    pub issues: Vec<Issue>,
}

impl fmt::Display for AdvisorSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.start_time, self.end_time)
    }
}

impl Model for AdvisorSummary {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_time_range() {
        let summary = AdvisorSummary {
            start_time: "2026-01-01T00:00:00Z".to_string(),
            end_time: "2026-01-01T00:01:00Z".to_string(),
            issues: Vec::new(),
        };
        assert_eq!(
            summary.to_string(),
            "2026-01-01T00:00:00Z - 2026-01-01T00:01:00Z"
        );
    }
}
