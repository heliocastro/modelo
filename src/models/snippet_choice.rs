// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::snippet_choice_reason::SnippetChoiceReason;
use crate::models::text_location::TextLocation;
use crate::models::{Model, ValidationError};

/// The source file criteria for which a snippet choice is made.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnippetChoiceGiven {
    /// The source file for which the snippet choice is made.
    pub source_location: TextLocation,
}

/// The snippet criteria that make up a snippet choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnippetChoiceCriteria {
    /// The purl of the snippet chosen by this choice. `None` if `reason` is
    /// [`SnippetChoiceReason::NoRelevantFinding`].
    #[serde(default)]
    pub purl: Option<String>,
    /// The reason why this snippet choice was made.
    pub reason: SnippetChoiceReason,
    /// An optional comment describing the snippet choice.
    #[serde(default)]
    pub comment: Option<String>,
}

/// A snippet choice for a given source file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnippetChoice {
    /// The source file criteria for which the snippet choice is made.
    pub given: SnippetChoiceGiven,
    /// The snippet criteria that make up the choice.
    pub choice: SnippetChoiceCriteria,
}

impl fmt::Display for SnippetChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.given.source_location, self.choice.reason)
    }
}

impl Model for SnippetChoice {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_source_location_and_reason() {
        let choice = SnippetChoice {
            given: SnippetChoiceGiven {
                source_location: TextLocation {
                    path: "src/main.rs".to_string(),
                    start_line: 1,
                    end_line: 10,
                },
            },
            choice: SnippetChoiceCriteria {
                purl: None,
                reason: SnippetChoiceReason::NoRelevantFinding,
                comment: None,
            },
        };
        assert_eq!(choice.to_string(), "src/main.rs:1-10 (NO_RELEVANT_FINDING)");
    }
}
