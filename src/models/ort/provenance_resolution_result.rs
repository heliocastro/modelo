// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::ort::identifier::Identifier;
use crate::models::ort::issue::Issue;
use crate::models::ort::provenance::Provenance;
use crate::models::ort::vcs_info::VcsInfo;
use crate::models::{Model, ValidationError};

/// The results of provenance resolution for the package denoted by `id`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceResolutionResult {
    /// The identifier of the package.
    pub id: Identifier,
    /// The resolved provenance of the package. Can be `None` only if a resolution issue occurred.
    #[serde(default)]
    pub package_provenance: Option<Provenance>,
    /// The (recursive) sub-repositories of `package_provenance`.
    #[serde(default)]
    pub sub_repositories: HashMap<String, VcsInfo>,
    /// The issue that happened during package provenance resolution, if any.
    #[serde(default)]
    pub package_provenance_resolution_issue: Option<Issue>,
    /// The issue that happened during nested provenance resolution, if any.
    #[serde(default)]
    pub nested_provenance_resolution_issue: Option<Issue>,
}

impl StdHash for ProvenanceResolutionResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for ProvenanceResolutionResult {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ProvenanceResolutionResult {}

impl fmt::Display for ProvenanceResolutionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Model for ProvenanceResolutionResult {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn equality_is_by_id() {
        let make = || ProvenanceResolutionResult {
            id: Identifier::from_str("Maven:org.example:artifact:1.0").unwrap(),
            package_provenance: None,
            sub_repositories: HashMap::new(),
            package_provenance_resolution_issue: None,
            nested_provenance_resolution_issue: None,
        };
        assert_eq!(make(), make());
    }
}
