// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::package_reference::PackageReference;
use crate::models::{Model, ValidationError};

/// Puts package dependencies into context, e.g. a package manager's "compile" or
/// "test" scope.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    /// The package manager's native name for the scope, e.g. "compile" or "devDependencies".
    pub name: String,
    /// The set of references to packages in this scope.
    #[serde(default)]
    pub dependencies: HashSet<PackageReference>,
}

impl StdHash for Scope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Scope {}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Model for Scope {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.name.is_empty() {
            return Err(ValidationError::MissingField {
                field: "name".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_is_by_name() {
        let a = Scope {
            name: "compile".to_string(),
            dependencies: HashSet::new(),
        };
        let b = Scope {
            name: "compile".to_string(),
            dependencies: HashSet::new(),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn empty_name_fails_validation() {
        let scope = Scope {
            name: String::new(),
            dependencies: HashSet::new(),
        };
        assert!(scope.validate().is_err());
    }
}
