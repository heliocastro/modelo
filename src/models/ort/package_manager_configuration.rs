// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// The configuration model for a single package manager.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageManagerConfiguration {
    /// The names of package managers that must run before this one, if any.
    #[serde(default)]
    pub must_run_after: Option<Vec<String>>,
    /// Custom configuration options for the package manager.
    #[serde(
        default,
        deserialize_with = "crate::models::ort::coerce::optional_string_map_from_scalars"
    )]
    pub options: Option<HashMap<String, String>>,
}

impl fmt::Display for PackageManagerConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PackageManagerConfiguration")
    }
}

impl Model for PackageManagerConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_none() {
        let config = PackageManagerConfiguration::default();
        assert!(config.must_run_after.is_none());
        assert!(config.options.is_none());
    }
}
