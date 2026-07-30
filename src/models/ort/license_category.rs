// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// A category that licenses can be assigned to, via a [`crate::models::ort::license_categorization::LicenseCategorization`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseCategory {
    /// The name of this category. Must be unique over all categories.
    pub name: String,
    /// A description for this category.
    #[serde(default)]
    pub description: String,
}

impl fmt::Display for LicenseCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Model for LicenseCategory {
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
        let category = LicenseCategory {
            name: String::new(),
            description: String::new(),
        };
        assert!(category.validate().is_err());
    }
}
