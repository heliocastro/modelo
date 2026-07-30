// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// Metadata for a specific license, assigning it to zero or more [`crate::models::ort::license_category::LicenseCategory`] ids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseCategorization {
    /// The SPDX license expression of this categorization.
    pub id: String,
    /// The identifiers of the license categories this license is assigned to.
    #[serde(default)]
    pub categories: HashSet<String>,
}

impl fmt::Display for LicenseCategorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Model for LicenseCategorization {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::MissingField {
                field: "id".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_id_fails_validation() {
        let categorization = LicenseCategorization {
            id: String::new(),
            categories: HashSet::new(),
        };
        assert!(categorization.validate().is_err());
    }
}
