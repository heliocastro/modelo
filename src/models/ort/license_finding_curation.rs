// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::license_finding_curation_reason::LicenseFindingCurationReason;
use crate::models::{Model, ValidationError};

/// A curation for a license finding, correcting it or adding a license that was not detected.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseFindingCuration {
    /// A glob to match the file path of a license finding.
    pub path: String,
    /// A comma-separated list of start lines to match; matches any if empty or absent.
    #[serde(
        default,
        deserialize_with = "crate::models::ort::coerce::optional_string_from_scalar"
    )]
    pub start_lines: Option<String>,
    /// A matcher for the line count of a license finding; matches any if absent.
    #[serde(default)]
    pub line_count: Option<i64>,
    /// The detected license as an SPDX expression, or `None` for no license.
    #[serde(default)]
    pub detected_license: Option<String>,
    /// The concluded license as an SPDX expression, or `None` for no license.
    pub concluded_license: String,
    /// The reason why the curation was made.
    pub reason: LicenseFindingCurationReason,
    /// A comment explaining this curation.
    #[serde(default)]
    pub comment: Option<String>,
}

impl fmt::Display for LicenseFindingCuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.path, self.concluded_license)
    }
}

impl Model for LicenseFindingCuration {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.path.is_empty() {
            return Err(ValidationError::MissingField {
                field: "path".to_string(),
            });
        }
        if let Some(start_lines) = &self.start_lines {
            for part in start_lines.split(',') {
                if !part.trim().chars().all(|c| c.is_ascii_digit()) || part.trim().is_empty() {
                    return Err(ValidationError::InvalidField {
                        field: "start_lines".to_string(),
                        message: format!("contains non-numeric value: '{}'", part.trim()),
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

    fn make() -> LicenseFindingCuration {
        LicenseFindingCuration {
            path: "src/**".to_string(),
            start_lines: None,
            line_count: None,
            detected_license: None,
            concluded_license: "MIT".to_string(),
            reason: LicenseFindingCurationReason::Incorrect,
            comment: None,
        }
    }

    #[test]
    fn non_numeric_start_lines_fails_validation() {
        let mut curation = make();
        curation.start_lines = Some("1,two".to_string());
        assert!(curation.validate().is_err());
    }

    #[test]
    fn numeric_start_lines_passes_validation() {
        let mut curation = make();
        curation.start_lines = Some("1,2,3".to_string());
        assert!(curation.validate().is_ok());
    }
}
