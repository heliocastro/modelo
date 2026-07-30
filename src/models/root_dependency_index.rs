// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// The index of a root dependency of a scope in a [`crate::models::dependency_graph::DependencyGraph`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RootDependencyIndex {
    /// The index of the root dependency referenced by this object.
    pub root: i64,
    /// The index of the fragment of the dependency graph this reference points to.
    #[serde(default)]
    pub fragment: i64,
}

impl fmt::Display for RootDependencyIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.root, self.fragment)
    }
}

impl Model for RootDependencyIndex {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_root_and_fragment() {
        let index = RootDependencyIndex {
            root: 2,
            fragment: 0,
        };
        assert_eq!(index.to_string(), "2@0");
    }
}
