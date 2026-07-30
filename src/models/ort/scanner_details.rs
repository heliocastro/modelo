// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// Details about the used source code scanner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScannerDetails {
    /// The name of the scanner.
    pub name: String,
    /// The version of the scanner.
    pub version: String,
    /// Configuration that ensures reproducible results.
    pub configuration: String,
}

impl fmt::Display for ScannerDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.version)
    }
}

impl Model for ScannerDetails {
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
        let details = ScannerDetails {
            name: String::new(),
            version: "1.0".to_string(),
            configuration: String::new(),
        };
        assert!(details.validate().is_err());
    }
}
