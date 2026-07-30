// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::ort::snippet::Snippet;
use crate::models::ort::text_location::TextLocation;
use crate::models::{Model, ValidationError};

/// Snippet findings for a source file location.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnippetFinding {
    /// The text location in the scanned source file where the snippet matched.
    pub source_location: TextLocation,
    /// The corresponding snippets.
    pub snippets: HashSet<Snippet>,
}

impl StdHash for SnippetFinding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source_location.hash(state);
    }
}

impl PartialEq for SnippetFinding {
    fn eq(&self, other: &Self) -> bool {
        self.source_location == other.source_location
    }
}

impl Eq for SnippetFinding {}

impl fmt::Display for SnippetFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} snippets)",
            self.source_location,
            self.snippets.len()
        )
    }
}

impl Model for SnippetFinding {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_is_by_source_location() {
        let location = TextLocation {
            path: "a.rs".to_string(),
            start_line: 1,
            end_line: 1,
        };
        let a = SnippetFinding {
            source_location: location.clone(),
            snippets: HashSet::new(),
        };
        let b = SnippetFinding {
            source_location: location,
            snippets: HashSet::new(),
        };
        assert_eq!(a, b);
    }
}
