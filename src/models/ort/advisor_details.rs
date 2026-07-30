// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::advisor_capability::AdvisorCapability;
use crate::models::{Model, ValidationError};

/// Details about the used provider of vulnerability information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdvisorDetails {
    /// The name of the used advisor.
    pub name: String,
    /// The capabilities of the used advisor. Deprecated in python-ort; kept for compatibility.
    #[serde(default)]
    pub capabilities: Option<HashSet<AdvisorCapability>>,
}

impl fmt::Display for AdvisorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Model for AdvisorDetails {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.name.is_empty() {
            return Err(ValidationError::MissingField {
                field: "name".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_name_fails_validation() {
        let details = AdvisorDetails {
            name: String::new(),
            capabilities: None,
        };
        assert!(details.validate().is_err());
    }
}
