// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::path_exclude::PathExclude;
use crate::models::scope_exclude::ScopeExclude;
use crate::models::{Model, ValidationError};

/// Defines which parts of a repository should be excluded.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Excludes {
    /// Path excludes.
    #[serde(default)]
    pub paths: Vec<PathExclude>,
    /// Scopes that will be excluded from all projects.
    #[serde(default)]
    pub scopes: Vec<ScopeExclude>,
}

impl fmt::Display for Excludes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Excludes({} paths, {} scopes)",
            self.paths.len(),
            self.scopes.len()
        )
    }
}

impl Model for Excludes {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let excludes = Excludes::default();
        assert!(excludes.paths.is_empty());
        assert!(excludes.scopes.is_empty());
    }
}
