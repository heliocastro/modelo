// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::repository_configuration::RepositoryConfiguration;
use crate::models::vcs_info::VcsInfo;
use crate::models::{Model, ValidationError};

/// A description of the source code repository that was used as input for `vale`.
// ponytail: dropped `PartialEq` here since `RepositoryConfiguration` now nests many
// config/ types that don't derive it; add back if a caller needs repository equality.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repository {
    /// Original VCS-related information from the working tree containing the analyzer root.
    pub vcs: VcsInfo,
    /// Processed VCS-related information, with common mistakes corrected.
    pub vcs_processed: VcsInfo,
    /// A map of nested repositories (e.g. Git submodules), keyed by relative path.
    #[serde(default)]
    pub nested_repositories: HashMap<String, VcsInfo>,
    /// The configuration of the repository, parsed from `.ort.yml`.
    pub config: RepositoryConfiguration,
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.vcs)
    }
}

impl Model for Repository {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_have_no_nested_repositories() {
        assert!(Repository::default().nested_repositories.is_empty());
    }
}
