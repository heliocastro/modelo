// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::environment::Environment;
use crate::models::{Model, ValidationError};

/// The summary of a single run of an ORT tool (analyzer, scanner, advisor, evaluator).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BaseRun {
    /// The time the tool was started, in RFC 3339 format.
    pub start_time: String,
    /// The time the tool finished, in RFC 3339 format.
    pub end_time: String,
    /// The environment in which the tool was executed.
    pub environment: Environment,
}

impl fmt::Display for BaseRun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.start_time, self.end_time)
    }
}

impl Model for BaseRun {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_time_range() {
        let run = BaseRun {
            start_time: "2026-01-01T00:00:00Z".to_string(),
            end_time: "2026-01-01T00:01:00Z".to_string(),
            environment: Environment {
                ort_version: "1.0".to_string(),
                build_jdk: "17".to_string(),
                java_version: "17".to_string(),
                os: "Linux".to_string(),
                processors: 8,
                max_memory: 1024,
                variables: std::collections::HashMap::new(),
            },
        };
        assert_eq!(
            run.to_string(),
            "2026-01-01T00:00:00Z - 2026-01-01T00:01:00Z"
        );
    }
}
