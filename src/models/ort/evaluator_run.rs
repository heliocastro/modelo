// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::base_run::BaseRun;
use crate::models::ort::rule_violation::RuleViolation;
use crate::models::{Model, ValidationError};

/// The summary of a single run of the evaluator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatorRun {
    /// The base run information (start/end time, environment).
    #[serde(flatten)]
    pub base: BaseRun,
    /// The rule violations found by the evaluator.
    #[serde(default)]
    pub violations: Vec<RuleViolation>,
}

impl fmt::Display for EvaluatorRun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl Model for EvaluatorRun {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ort::environment::Environment;
    use std::collections::HashMap;

    #[test]
    fn defaults_have_no_violations() {
        let run = EvaluatorRun {
            base: BaseRun {
                start_time: "2026-01-01T00:00:00Z".to_string(),
                end_time: "2026-01-01T00:01:00Z".to_string(),
                environment: Environment {
                    ort_version: "1.0".to_string(),
                    build_jdk: "17".to_string(),
                    java_version: "17".to_string(),
                    os: "Linux".to_string(),
                    processors: 8,
                    max_memory: 1024,
                    variables: HashMap::new(),
                },
            },
            violations: Vec::new(),
        };
        assert!(run.violations.is_empty());
    }
}
