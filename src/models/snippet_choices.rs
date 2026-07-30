// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::snippet_choice::SnippetChoice;
use crate::models::snippet_provenance::SnippetProvenance;
use crate::models::{Model, ValidationError};

/// A collection of snippet choices for a given provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnippetChoices {
    /// The provenance the snippet choices apply to.
    pub provenance: SnippetProvenance,
    /// The snippet choices for the given source file.
    pub choices: Vec<SnippetChoice>,
}

impl fmt::Display for SnippetChoices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} choices)", self.provenance, self.choices.len())
    }
}

impl Model for SnippetChoices {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_provenance_and_choice_count() {
        let choices = SnippetChoices {
            provenance: SnippetProvenance {
                url: "https://example.com/repo.git".to_string(),
            },
            choices: Vec::new(),
        };
        assert_eq!(
            choices.to_string(),
            "https://example.com/repo.git (0 choices)"
        );
    }
}
