// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::license_categorization::LicenseCategorization;
use crate::models::ort::license_category::LicenseCategory;
use crate::models::{Model, ValidationError};

/// Classifications for licenses, assigning metadata to licenses via customizable categories.
///
/// The available license categories need to be declared explicitly; [`Model::validate`] checks
/// that all references from [`Self::categorizations`] point to an existing category in
/// [`Self::categories`].
// ponytail: skipped porting `merge`/`licenses_by_category`/etc. helper methods from python-ort,
// they are query conveniences, not part of the data model. Add if a consumer needs them.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseClassifications {
    /// Defines metadata for the license categories.
    #[serde(default)]
    pub categories: Vec<LicenseCategory>,
    /// Defines metadata for licenses.
    #[serde(default)]
    pub categorizations: Vec<LicenseCategorization>,
}

impl fmt::Display for LicenseClassifications {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LicenseClassifications({} categories, {} categorizations)",
            self.categories.len(),
            self.categorizations.len()
        )
    }
}

impl Model for LicenseClassifications {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut seen_categories = HashSet::new();
        for category in &self.categories {
            if !seen_categories.insert(&category.name) {
                return Err(ValidationError::InvalidField {
                    field: "categories".to_string(),
                    message: format!("duplicate license category name: {}", category.name),
                });
            }
        }

        let mut seen_ids = HashSet::new();
        for categorization in &self.categorizations {
            if !seen_ids.insert(&categorization.id) {
                return Err(ValidationError::InvalidField {
                    field: "categorizations".to_string(),
                    message: format!("duplicate license categorization id: {}", categorization.id),
                });
            }
        }

        for categorization in &self.categorizations {
            for category in &categorization.categories {
                if !seen_categories.contains(category) {
                    return Err(ValidationError::InvalidField {
                        field: "categorizations".to_string(),
                        message: format!(
                            "categorization '{}' references unknown category '{category}'",
                            categorization.id
                        ),
                    });
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_reference_passes() {
        let classifications = LicenseClassifications {
            categories: vec![LicenseCategory {
                name: "permissive".to_string(),
                description: String::new(),
            }],
            categorizations: vec![LicenseCategorization {
                id: "MIT".to_string(),
                categories: HashSet::from(["permissive".to_string()]),
            }],
        };
        assert!(classifications.validate().is_ok());
    }

    #[test]
    fn unknown_category_reference_fails_validation() {
        let classifications = LicenseClassifications {
            categories: Vec::new(),
            categorizations: vec![LicenseCategorization {
                id: "MIT".to_string(),
                categories: HashSet::from(["permissive".to_string()]),
            }],
        };
        assert!(classifications.validate().is_err());
    }
}
