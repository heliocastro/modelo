// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::path_include::PathInclude;
use crate::models::{Model, ValidationError};

/// Defines which parts of a repository should be included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Includes {
    /// Path includes.
    #[serde(default)]
    pub paths: Vec<PathInclude>,
}

impl fmt::Display for Includes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Includes({} paths)", self.paths.len())
    }
}

impl Model for Includes {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        assert!(Includes::default().paths.is_empty());
    }
}
