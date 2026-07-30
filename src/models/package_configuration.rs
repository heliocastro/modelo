// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::identifier::Identifier;
use crate::models::license_finding_curation::LicenseFindingCuration;
use crate::models::path_exclude::PathExclude;
use crate::models::source_code_origin::SourceCodeOrigin;
use crate::models::vcsmatcher::VcsMatcher;
use crate::models::{Model, ValidationError};

/// Configures path excludes and license finding curations for a specific package, identified
/// by its [`Identifier`] and (optionally) its provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageConfiguration {
    /// The identifier which must match the package's identifier for this configuration to apply.
    pub id: Identifier,
    /// The source artifact this configuration applies to.
    #[serde(default)]
    pub source_artifact_url: Option<String>,
    /// The VCS and revision this configuration applies to.
    #[serde(default)]
    pub vcs: Option<VcsMatcher>,
    /// The source code origin this configuration applies to.
    #[serde(default)]
    pub source_code_origin: Option<SourceCodeOrigin>,
    /// Path excludes.
    #[serde(default)]
    pub path_excludes: Vec<PathExclude>,
    /// License finding curations.
    #[serde(default)]
    pub license_finding_curations: Vec<LicenseFindingCuration>,
}

impl fmt::Display for PackageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Model for PackageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn defaults_have_no_excludes() {
        let config = PackageConfiguration {
            id: Identifier::from_str("Maven:org.example:artifact:1.0").unwrap(),
            source_artifact_url: None,
            vcs: None,
            source_code_origin: None,
            path_excludes: Vec::new(),
            license_finding_curations: Vec::new(),
        };
        assert!(config.path_excludes.is_empty());
    }
}
