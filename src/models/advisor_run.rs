// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::advisor_configuration::AdvisorConfiguration;
use crate::models::advisor_result::AdvisorResult;
use crate::models::base_run::BaseRun;
use crate::models::identifier::Identifier;
use crate::models::issue::Issue;
use crate::models::{Model, ValidationError};

// ponytail: `provider_issues` is `set[Issue]` in python-ort; `Issue` doesn't derive `Hash`/`Eq`
// in this port, so a `Vec` is used instead.
/// The summary of a single run of the advisor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvisorRun {
    /// The base run information (start/end time, environment).
    #[serde(flatten)]
    pub base: BaseRun,
    /// The advisor configuration used for this run.
    pub config: AdvisorConfiguration,
    /// Issues that occurred while preparing and querying advisor providers for this run.
    #[serde(default)]
    pub provider_issues: Vec<Issue>,
    /// The result of this run.
    #[serde(default)]
    pub results: HashMap<Identifier, Vec<AdvisorResult>>,
}

impl fmt::Display for AdvisorRun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl Model for AdvisorRun {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::environment::Environment;

    #[test]
    fn defaults_have_no_results() {
        let run = AdvisorRun {
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
            config: AdvisorConfiguration::default(),
            provider_issues: Vec::new(),
            results: HashMap::new(),
        };
        assert!(run.results.is_empty());
    }
}
