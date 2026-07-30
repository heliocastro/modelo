// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{Model, ValidationError};

/// The configuration model of the advisor.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdvisorConfiguration {
    /// Whether excluded scopes and paths should be skipped when giving the advice.
    #[serde(default)]
    pub skip_excluded: bool,
    /// Per-advisor plugin configuration, keyed by the plugin id.
    #[serde(default)]
    pub advisors: Option<HashMap<String, Value>>,
}

impl fmt::Display for AdvisorConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AdvisorConfiguration")
    }
}

impl Model for AdvisorConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_skip_excluded_to_false() {
        assert!(!AdvisorConfiguration::default().skip_excluded);
    }
}
