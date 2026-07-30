// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::ort::text_location::TextLocation;
use crate::models::{Model, ValidationError};

/// A license finding, pointing to a single license or a complex SPDX expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseFinding {
    /// The found license as an SPDX expression.
    pub license: String,
    /// The text location where the license was found.
    pub location: TextLocation,
    /// The scanner-specific confidence score of this finding, if any.
    #[serde(default)]
    pub score: Option<f64>,
}

impl StdHash for LicenseFinding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.license.hash(state);
    }
}

impl PartialEq for LicenseFinding {
    fn eq(&self, other: &Self) -> bool {
        self.license == other.license
    }
}

impl Eq for LicenseFinding {}

impl fmt::Display for LicenseFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.license)
    }
}

impl Model for LicenseFinding {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.license.is_empty() {
            return Err(ValidationError::MissingField {
                field: "license".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(license: &str) -> LicenseFinding {
        LicenseFinding {
            license: license.to_string(),
            location: TextLocation {
                path: "LICENSE".to_string(),
                start_line: 1,
                end_line: 1,
            },
            score: None,
        }
    }

    #[test]
    fn equality_is_by_license() {
        assert_eq!(make("MIT"), make("MIT"));
    }

    #[test]
    fn empty_license_fails_validation() {
        assert!(make("").validate().is_err());
    }
}
