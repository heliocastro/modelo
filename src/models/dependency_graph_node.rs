// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::issue::Issue;
use crate::models::package_linkage::PackageLinkage;
use crate::models::{Model, ValidationError};

fn default_linkage() -> PackageLinkage {
    PackageLinkage::Dynamic
}

/// A node in a [`crate::models::dependency_graph::DependencyGraph`], corresponding to a package.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyGraphNode {
    /// The numeric index of the package referenced by this node, into the graph's package list.
    #[serde(default)]
    pub pkg: Option<i64>,
    /// The index of the fragment in the dependency graph this node is contained in.
    #[serde(default)]
    pub fragment: i64,
    /// The type of linkage used for the referred package from its dependent package.
    #[serde(default = "default_linkage")]
    pub linkage: PackageLinkage,
    /// Issues that occurred handling this dependency.
    #[serde(default)]
    pub issues: Vec<Issue>,
}

impl fmt::Display for DependencyGraphNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.pkg {
            Some(pkg) => write!(f, "pkg[{pkg}]@{}", self.fragment),
            None => write!(f, "<no package>@{}", self.fragment),
        }
    }
}

impl Model for DependencyGraphNode {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_package_index_and_fragment() {
        let node = DependencyGraphNode {
            pkg: Some(3),
            fragment: 1,
            linkage: PackageLinkage::Dynamic,
            issues: Vec::new(),
        };
        assert_eq!(node.to_string(), "pkg[3]@1");
    }
}
