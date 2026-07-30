// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::issue::Issue;
use crate::models::package_linkage::PackageLinkage;
use crate::models::{Model, ValidationError};

fn default_linkage() -> PackageLinkage {
    PackageLinkage::Dynamic
}

/// A tree-like structure representing the dependencies of a project, used in the legacy
/// (pre-`DependencyGraph`) analyzer result format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyReference {
    /// The numeric index of the package dependency referenced by this object.
    pub pkg: i64,
    /// The index of the fragment in the dependency graph this reference is contained in.
    #[serde(default)]
    pub fragment: i64,
    /// The dependencies of this dependency, forming a tree-like structure.
    #[serde(default)]
    pub dependencies: HashSet<DependencyReference>,
    /// The type of linkage used for the referred package from its dependent package.
    #[serde(default = "default_linkage")]
    pub linkage: PackageLinkage,
    /// Issues that occurred handling this dependency.
    pub issues: Vec<Issue>,
}

// Matches `PackageReference`'s precedent: identity is (pkg, fragment) only, not the full
// recursive `dependencies` set.
impl StdHash for DependencyReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pkg.hash(state);
        self.fragment.hash(state);
    }
}

impl PartialEq for DependencyReference {
    fn eq(&self, other: &Self) -> bool {
        self.pkg == other.pkg && self.fragment == other.fragment
    }
}

impl Eq for DependencyReference {}

impl fmt::Display for DependencyReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pkg[{}]@{}", self.pkg, self.fragment)
    }
}

impl Model for DependencyReference {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(pkg: i64, fragment: i64) -> DependencyReference {
        DependencyReference {
            pkg,
            fragment,
            dependencies: HashSet::new(),
            linkage: PackageLinkage::Dynamic,
            issues: Vec::new(),
        }
    }

    #[test]
    fn equality_is_by_pkg_and_fragment() {
        assert_eq!(make(1, 0), make(1, 0));
        assert_ne!(make(1, 0), make(1, 1));
    }
}
