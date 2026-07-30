// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::vcs_type::VcsType;
use crate::models::{Model, ValidationError};

/// Bundles general Version Control System information.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct VcsInfo {
    /// The type of the VCS, for example Git, GitRepo, Mercurial, etc.
    #[serde(rename = "type", default)]
    pub vcs_type: VcsType,
    /// The URL to the VCS repository.
    #[serde(default)]
    pub url: String,
    /// The VCS-specific revision (tag, branch, SHA1) that the version of the package maps to.
    pub revision: String,
    /// The path inside the VCS to take into account.
    #[serde(default)]
    pub path: String,
}

impl fmt::Display for VcsInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}@{}", self.vcs_type, self.url, self.revision)
    }
}

impl Model for VcsInfo {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let info = VcsInfo {
            revision: "main".to_string(),
            ..Default::default()
        };
        assert_eq!(info.path, "");
        assert_eq!(info.vcs_type.name, "");
    }
}
