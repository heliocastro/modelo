// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::ort::issue::Issue;
use crate::models::ort::package_linkage::PackageLinkage;
use crate::models::{Model, ValidationError};

/// A human-readable reference to a software package, along with its own
/// transitive dependencies in the scope it was referenced from.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageReference {
    /// The identifier of the package.
    pub id: String,
    /// The type of linkage used for the referred package from its dependent package.
    pub linkage: PackageLinkage,
    /// The set of references to packages this package depends on.
    #[serde(default)]
    pub dependencies: HashSet<PackageReference>,
    /// A list of issues that occurred handling this `PackageReference`.
    #[serde(default)]
    pub issues: Vec<Issue>,
}

impl StdHash for PackageReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for PackageReference {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PackageReference {}

impl fmt::Display for PackageReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.id, self.linkage)
    }
}

impl Model for PackageReference {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::MissingField {
                field: "id".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(id: &str) -> PackageReference {
        PackageReference {
            id: id.to_string(),
            linkage: PackageLinkage::Dynamic,
            dependencies: HashSet::new(),
            issues: Vec::new(),
        }
    }

    #[test]
    fn equality_is_by_id() {
        assert_eq!(make("pkg:npm/foo@1.0"), make("pkg:npm/foo@1.0"));
    }

    #[test]
    fn empty_id_fails_validation() {
        assert!(make("").validate().is_err());
    }
}
