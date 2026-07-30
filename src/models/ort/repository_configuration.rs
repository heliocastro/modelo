// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::curations::Curations;
use crate::models::ort::excludes::Excludes;
use crate::models::ort::includes::Includes;
use crate::models::ort::license_choices::LicenseChoices;
use crate::models::ort::package_configuration::PackageConfiguration;
use crate::models::ort::repository_analyzer_configuration::RepositoryAnalyzerConfiguration;
use crate::models::ort::resolutions::Resolutions;
use crate::models::ort::snippet_choices::SnippetChoices;
use crate::models::{Model, ValidationError};

/// The configuration of a repository, parsed from `.ort.yml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryConfiguration {
    /// Analyzer-specific options.
    #[serde(default)]
    pub analyzer: Option<RepositoryAnalyzerConfiguration>,
    /// Defines which parts of a repository should be included.
    #[serde(default)]
    pub includes: Option<Includes>,
    /// Defines which parts of a repository should be excluded.
    #[serde(default)]
    pub excludes: Option<Excludes>,
    /// Defines resolutions for issues with this repository.
    #[serde(default)]
    pub resolutions: Option<Resolutions>,
    /// Defines curations for packages or license findings in this repository.
    #[serde(default)]
    pub curations: Option<Curations>,
    /// Package and provenance specific configurations.
    #[serde(default)]
    pub package_configurations: Vec<PackageConfiguration>,
    /// A configuration to select a license from a multi-licensed package.
    #[serde(default)]
    pub license_choices: Option<LicenseChoices>,
    /// A configuration to select a snippet from a package with multiple snippet findings.
    #[serde(default)]
    pub snippet_choices: Vec<SnippetChoices>,
}

impl fmt::Display for RepositoryConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RepositoryConfiguration")
    }
}

impl Model for RepositoryConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let config = RepositoryConfiguration::default();
        assert!(config.analyzer.is_none());
        assert!(config.package_configurations.is_empty());
        assert!(config.snippet_choices.is_empty());
    }
}
