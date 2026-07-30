// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::provenance::Provenance;
use crate::models::text_location::TextLocation;
use crate::models::{Model, ValidationError};

/// A code snippet detected by a snippet scanner, with its provenance, license and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snippet {
    /// The matching score between the scanned code and the snippet. Scanner-specific.
    pub score: f64,
    /// The text location in the snippet that has matched.
    pub location: TextLocation,
    /// The provenance of the snippet, either an artifact or a repository.
    pub provenance: Provenance,
    /// The purl representing the author/vendor, artifact, and version of the snippet.
    pub purl: String,
    /// The SPDX license expression of the component the snippet is coming from.
    pub license: String,
    /// Scanner-specific snippet data that cannot be mapped into a generalized property.
    #[serde(default)]
    pub additional_data: HashMap<String, String>,
}

impl StdHash for Snippet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.purl.hash(state);
    }
}

impl PartialEq for Snippet {
    fn eq(&self, other: &Self) -> bool {
        self.purl == other.purl
    }
}

impl Eq for Snippet {}

impl fmt::Display for Snippet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.purl)
    }
}

impl Model for Snippet {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.purl.is_empty() {
            return Err(ValidationError::MissingField {
                field: "purl".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(purl: &str) -> Snippet {
        Snippet {
            score: 100.0,
            location: TextLocation {
                path: "a.rs".to_string(),
                start_line: 1,
                end_line: 1,
            },
            provenance: Provenance::Unknown,
            purl: purl.to_string(),
            license: "MIT".to_string(),
            additional_data: HashMap::new(),
        }
    }

    #[test]
    fn equality_is_by_purl() {
        assert_eq!(make("pkg:npm/foo@1.0"), make("pkg:npm/foo@1.0"));
    }

    #[test]
    fn empty_purl_fails_validation() {
        assert!(make("").validate().is_err());
    }
}
