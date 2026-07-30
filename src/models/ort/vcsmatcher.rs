// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::vcs_type::VcsType;
use crate::models::{Model, ValidationError};

/// A matcher which matches its properties against a repository provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VcsMatcher {
    /// The type to match for equality against `VcsInfo`'s type.
    #[serde(rename = "type")]
    pub orttype: VcsType,
    /// The URL to match for equality against `VcsInfo`'s URL.
    pub url: String,
    /// The revision to match for equality, or `None` to match any revision.
    #[serde(default)]
    pub revision: Option<String>,
}

impl fmt::Display for VcsMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.orttype, self.url)
    }
}

impl Model for VcsMatcher {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.url.is_empty() {
            return Err(ValidationError::MissingField {
                field: "url".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_url_fails_validation() {
        let matcher = VcsMatcher {
            orttype: VcsType::new("Git"),
            url: String::new(),
            revision: None,
        };
        assert!(matcher.validate().is_err());
    }
}
