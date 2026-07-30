// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::analyzer_configuration::AnalyzerConfiguration;
use crate::models::analyzer_result::AnalyzerResult;
use crate::models::base_run::BaseRun;
use crate::models::{Model, ValidationError};

/// The summary of a single run of the analyzer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerRun {
    /// The base run information (start/end time, environment).
    #[serde(flatten)]
    pub base: BaseRun,
    /// The analyzer configuration used for this run.
    pub config: AnalyzerConfiguration,
    /// The result of this run.
    #[serde(default)]
    pub result: Option<AnalyzerResult>,
}

impl fmt::Display for AnalyzerRun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl Model for AnalyzerRun {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::environment::Environment;
    use std::collections::HashMap;

    #[test]
    fn defaults_have_no_result() {
        let run = AnalyzerRun {
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
            config: AnalyzerConfiguration::default(),
            result: None,
        };
        assert!(run.result.is_none());
    }
}
