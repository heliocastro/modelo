// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::vcs_type::VcsType;
use crate::models::{Model, ValidationError};

/// Curation data for [`crate::models::vcs_info::VcsInfo`]: an overlay of fields to apply on
/// top of the original VCS metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VcsInfoCurationData {
    /// The type of the VCS, for example Git, GitRepo, Mercurial, etc.
    #[serde(default, rename = "type")]
    pub vcs_type: Option<VcsType>,
    /// The URL to the VCS repository.
    #[serde(default)]
    pub url: Option<String>,
    /// The VCS-specific revision (tag, branch, SHA1) that the version of the package maps to.
    #[serde(default)]
    pub revision: Option<String>,
    /// The path inside the VCS to take into account.
    #[serde(default)]
    pub path: Option<String>,
}

impl fmt::Display for VcsInfoCurationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url.as_deref().unwrap_or(""))
    }
}

impl Model for VcsInfoCurationData {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_none() {
        let data = VcsInfoCurationData::default();
        assert!(data.url.is_none());
        assert!(data.vcs_type.is_none());
    }
}
