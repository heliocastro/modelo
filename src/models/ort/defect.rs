// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// A software defect, as retrieved by an advisor implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Defect {
    /// The (external) ID of this defect, e.g. a bug ID or ticket number.
    pub id: String,
    /// The URL pointing to the source of this defect.
    pub url: String,
    /// A short summary title for this defect, if available.
    #[serde(default)]
    pub title: Option<String>,
    /// The state of the associated defect if available (e.g. "OPEN", "BLOCKED").
    #[serde(default)]
    pub state: Option<String>,
    /// The severity assigned to the defect if available.
    #[serde(default)]
    pub severity: Option<String>,
    /// An optional description of this defect.
    #[serde(default)]
    pub description: Option<String>,
    /// The creation time of this defect, in RFC 3339 format, if available.
    #[serde(default)]
    pub creation_time: Option<String>,
    /// The last modification time of this defect, in RFC 3339 format, if available.
    #[serde(default)]
    pub modification_time: Option<String>,
    /// The time this defect was closed, in RFC 3339 format, if available.
    #[serde(default)]
    pub closing_time: Option<String>,
    /// The version of the release in which this defect was fixed, if available.
    #[serde(default)]
    pub fix_release_version: Option<String>,
    /// A URL pointing to the release in which this defect was fixed, if available.
    #[serde(default)]
    pub fix_release_url: Option<String>,
    /// Labels assigned to this defect by the source issue tracker system.
    #[serde(default)]
    pub labels: HashMap<String, String>,
}

impl fmt::Display for Defect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Model for Defect {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::MissingField {
                field: "id".to_string(),
            });
        }
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

    fn make() -> Defect {
        Defect {
            id: "BUG-1".to_string(),
            url: "https://example.com/BUG-1".to_string(),
            title: None,
            state: None,
            severity: None,
            description: None,
            creation_time: None,
            modification_time: None,
            closing_time: None,
            fix_release_version: None,
            fix_release_url: None,
            labels: HashMap::new(),
        }
    }

    #[test]
    fn empty_url_fails_validation() {
        let mut defect = make();
        defect.url = String::new();
        assert!(defect.validate().is_err());
    }
}
