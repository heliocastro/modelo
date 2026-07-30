// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::scope_exclude_reason::ScopeExcludeReason;
use crate::models::{Model, ValidationError};

/// Defines a scope that should be excluded from all projects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeExclude {
    /// A regular expression to match the names of scopes to exclude.
    pub pattern: String,
    /// The reason for excluding the scope.
    pub reason: ScopeExcludeReason,
    /// A comment further explaining why the reason is applicable here.
    #[serde(default)]
    pub comment: String,
}

impl fmt::Display for ScopeExclude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.pattern, self.reason)
    }
}

impl Model for ScopeExclude {
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
        let exclude = ScopeExclude {
            pattern: String::new(),
            reason: ScopeExcludeReason::TestToolOf,
            comment: String::new(),
        };
        assert!(exclude.validate().is_err());
    }
}
