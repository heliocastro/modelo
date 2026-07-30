// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::ort::identifier::Identifier;
use crate::models::ort::processed_declared_license::ProcessedDeclaredLicense;
use crate::models::ort::scope::Scope;
use crate::models::ort::vcs_info::VcsInfo;
use crate::models::{Model, ValidationError};

/// A software project: similar to a [`crate::models::ort::package::Package`], but
/// carries the dependency scopes that refer to the actual packages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    /// The unique identifier of this project.
    pub id: Identifier,
    /// An optional additional identifier in CPE syntax.
    #[serde(default)]
    pub cpe: Option<String>,
    /// The path to the definition file, relative to the repository root.
    pub definition_file_path: String,
    /// The set of authors declared for this project.
    #[serde(default)]
    pub authors: HashSet<String>,
    /// The set of licenses declared for this project.
    pub declared_licenses: HashSet<String>,
    /// The declared licenses processed into an SPDX expression.
    pub declared_licenses_processed: ProcessedDeclaredLicense,
    /// Original VCS-related information as defined in the project's metadata.
    pub vcs: VcsInfo,
    /// Processed VCS-related information, with common mistakes corrected.
    pub vcs_processed: VcsInfo,
    /// The description of the project.
    #[serde(default)]
    pub description: String,
    /// The URL to the project's homepage.
    pub homepage_url: String,
    /// The scopes and their dependencies, when no shared dependency graph is used.
    #[serde(default, rename = "scopes")]
    pub scope_dependencies: HashSet<Scope>,
    /// The scope names, when dependencies are held in a shared dependency graph.
    #[serde(default)]
    pub scope_names: Option<HashSet<String>>,
}

impl StdHash for Project {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Project {}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Model for Project {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.definition_file_path.is_empty() {
            return Err(ValidationError::MissingField {
                field: "definition_file_path".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn make() -> Project {
        Project {
            id: Identifier::from_str("Gradle:org.example:project:1.0").unwrap(),
            cpe: None,
            definition_file_path: "build.gradle".to_string(),
            authors: HashSet::new(),
            declared_licenses: HashSet::new(),
            declared_licenses_processed: ProcessedDeclaredLicense::default(),
            vcs: VcsInfo::default(),
            vcs_processed: VcsInfo::default(),
            description: String::new(),
            homepage_url: String::new(),
            scope_dependencies: HashSet::new(),
            scope_names: None,
        }
    }

    #[test]
    fn empty_definition_file_path_fails_validation() {
        let mut project = make();
        project.definition_file_path = String::new();
        assert!(project.validate().is_err());
    }
}
