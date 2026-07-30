// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::package_manager_configuration::PackageManagerConfiguration;
use crate::models::{Model, ValidationError};

/// Repository-level overrides for the analyzer configuration, set via `.ort.yml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryAnalyzerConfiguration {
    /// Enables analysis of projects that use version ranges to declare their dependencies.
    #[serde(default)]
    pub allow_dynamic_versions: Option<bool>,
    /// The case-insensitive names of package managers that are enabled.
    #[serde(default)]
    pub enabled_package_managers: Option<Vec<String>>,
    /// The case-insensitive names of package managers that are disabled.
    #[serde(default)]
    pub disabled_package_managers: Option<Vec<String>>,
    /// Per-package-manager configuration, keyed by the (case-insensitive) package manager name.
    #[serde(default)]
    pub package_managers: Option<HashMap<String, PackageManagerConfiguration>>,
    /// Whether excluded scopes and paths should be skipped during the analysis.
    #[serde(default)]
    pub skip_excluded: Option<bool>,
}

impl fmt::Display for RepositoryAnalyzerConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RepositoryAnalyzerConfiguration")
    }
}

impl Model for RepositoryAnalyzerConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_none() {
        let config = RepositoryAnalyzerConfiguration::default();
        assert!(config.allow_dynamic_versions.is_none());
        assert!(config.skip_excluded.is_none());
    }
}
