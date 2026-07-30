// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::package_curation_data::PackageCurationData;
use crate::models::{Model, ValidationError};

/// A curation for a package, identified by its (possibly Ivy-style) identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageCuration {
    /// The identifier of the package this curation applies to.
    pub id: String,
    /// The curation data to overlay on top of the package's original metadata.
    pub curations: PackageCurationData,
}

impl fmt::Display for PackageCuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Model for PackageCuration {
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
        let curation = PackageCuration {
            id: String::new(),
            curations: PackageCurationData::default(),
        };
        assert!(curation.validate().is_err());
    }
}
