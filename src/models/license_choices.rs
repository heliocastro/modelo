// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::identifier::Identifier;
use crate::models::{Model, ValidationError};

// ponytail: python-ort's `SpdxLicenseChoice` (src/ort/utils/spdx/spdx_license_choice.py) is a
// small util type outside the models port list; ported inline here since it is only used by
// license choice configuration.
/// An individual license choice: `given` is the (sub-)expression the `choice` is applied on,
/// or the whole package expression if `given` is absent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpdxLicenseChoice {
    /// The SPDX (sub-)expression the choice applies to, or `None` for the whole expression.
    #[serde(default)]
    pub given: Option<String>,
    /// The SPDX expression to choose.
    pub choice: String,
}

/// The `SpdxLicenseChoice`s defined for a specific package.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageLicenseChoice {
    /// The identifier of the package this choice applies to.
    pub package_id: Identifier,
    /// The license choices for the package.
    #[serde(default)]
    pub license_choices: Vec<SpdxLicenseChoice>,
}

/// The `SpdxLicenseChoice`s applied to all or specific packages in a repository.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseChoices {
    /// License choices applied to all packages in the repository.
    #[serde(default)]
    pub repository_license_choices: Vec<SpdxLicenseChoice>,
    /// License choices applied to specific packages.
    #[serde(default)]
    pub package_license_choices: Vec<PackageLicenseChoice>,
}

impl fmt::Display for LicenseChoices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LicenseChoices({} repository, {} package)",
            self.repository_license_choices.len(),
            self.package_license_choices.len()
        )
    }
}

impl Model for LicenseChoices {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let choices = LicenseChoices::default();
        assert!(choices.repository_license_choices.is_empty());
        assert!(choices.package_license_choices.is_empty());
    }
}
