// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// The URL of the repository provenance a snippet choice applies to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnippetProvenance {
    /// The URL of the repository provenance the snippet choice applies to.
    pub url: String,
}

impl fmt::Display for SnippetProvenance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl Model for SnippetProvenance {
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
        let provenance = SnippetProvenance { url: String::new() };
        assert!(provenance.validate().is_err());
    }
}
