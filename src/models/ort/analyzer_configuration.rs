// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::package_manager_configuration::PackageManagerConfiguration;
use crate::models::{Model, ValidationError};

const DEFAULT_PACKAGE_MANAGERS: &[&str] = &[
    "Bazel",
    "Bower",
    "Bundler",
    "Cargo",
    "Carthage",
    "CocoaPods",
    "Composer",
    "Conan",
    "GoMod",
    "GradleInspector",
    "Maven",
    "NPM",
    "NuGet",
    "PIP",
    "Pipenv",
    "PNPM",
    "Poetry",
    "Pub",
    "SBT",
    "SpdxDocumentFile",
    "Stack",
    "SwiftPM",
    "Tycho",
    "Unmanaged",
    "Yarn",
    "Yarn2",
];

fn default_enabled_package_managers() -> Vec<String> {
    DEFAULT_PACKAGE_MANAGERS
        .iter()
        .map(|s| s.to_string())
        .collect()
}

/// The configuration model of the analyzer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalyzerConfiguration {
    /// Enables analysis of projects that use version ranges to declare their dependencies.
    #[serde(default)]
    pub allow_dynamic_versions: bool,
    /// The case-insensitive names of package managers that are enabled.
    #[serde(default = "default_enabled_package_managers")]
    pub enabled_package_managers: Vec<String>,
    /// The case-insensitive names of package managers that are disabled.
    #[serde(default)]
    pub disabled_package_managers: Option<Vec<String>>,
    /// Per-package-manager configuration, keyed by the (case-insensitive) package manager name.
    #[serde(default)]
    pub package_managers: Option<HashMap<String, PackageManagerConfiguration>>,
    /// Whether excluded scopes and paths should be skipped during the analysis.
    #[serde(default)]
    pub skip_excluded: bool,
}

impl Default for AnalyzerConfiguration {
    fn default() -> Self {
        Self {
            allow_dynamic_versions: false,
            enabled_package_managers: default_enabled_package_managers(),
            disabled_package_managers: None,
            package_managers: None,
            skip_excluded: false,
        }
    }
}

impl fmt::Display for AnalyzerConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AnalyzerConfiguration({} enabled package managers)",
            self.enabled_package_managers.len()
        )
    }
}

impl Model for AnalyzerConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_include_known_package_managers() {
        let config = AnalyzerConfiguration::default();
        assert!(config
            .enabled_package_managers
            .contains(&"Cargo".to_string()));
        assert!(!config.allow_dynamic_versions);
    }
}
