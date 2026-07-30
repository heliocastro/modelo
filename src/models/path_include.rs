// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::path_include_reason::PathIncludeReason;
use crate::models::{Model, ValidationError};

/// Defines a glob-matched path that should be included in a repository's analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathInclude {
    /// A glob to match the path of the project definition file, relative to the repository root.
    pub pattern: String,
    /// The reason why the project is included.
    pub reason: PathIncludeReason,
    /// A comment further explaining why the reason is applicable here.
    #[serde(default)]
    pub comment: String,
}

impl fmt::Display for PathInclude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.pattern, self.reason)
    }
}

impl Model for PathInclude {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.pattern.is_empty() {
            return Err(ValidationError::MissingField {
                field: "pattern".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pattern_fails_validation() {
        let include = PathInclude {
            pattern: String::new(),
            reason: PathIncludeReason::SourceOf,
            comment: String::new(),
        };
        assert!(include.validate().is_err());
    }
}
