// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// An individual license choice: `choice` is applied to `given` (the complete license
/// expression, or a sub-expression of it). If `given` is absent, `choice` is applied to
/// the complete expression of the package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpdxLicenseChoice {
    /// The complete expression, or sub-expression, that `choice` is applied on.
    #[serde(default)]
    pub given: Option<String>,
    /// The license expression to apply.
    pub choice: String,
}

impl fmt::Display for SpdxLicenseChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.given {
            Some(given) => write!(f, "{given} -> {}", self.choice),
            None => write!(f, "{}", self.choice),
        }
    }
}

impl Model for SpdxLicenseChoice {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.choice.is_empty() {
            return Err(ValidationError::MissingField {
                field: "choice".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_choice_fails_validation() {
        let choice = SpdxLicenseChoice {
            given: None,
            choice: String::new(),
        };
        assert!(choice.validate().is_err());
    }
}
