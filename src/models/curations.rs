// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::license_finding_curation::LicenseFindingCuration;
use crate::models::package_curation::PackageCuration;
use crate::models::{Model, ValidationError};

/// Curations for artifacts in a repository.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Curations {
    /// Curations for third-party packages.
    #[serde(default)]
    pub packages: Vec<PackageCuration>,
    /// Curations for license findings.
    #[serde(default)]
    pub license_findings: Vec<LicenseFindingCuration>,
}

impl fmt::Display for Curations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Curations({} packages, {} license_findings)",
            self.packages.len(),
            self.license_findings.len()
        )
    }
}

impl Model for Curations {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let curations = Curations::default();
        assert!(curations.packages.is_empty());
        assert!(curations.license_findings.is_empty());
    }
}
