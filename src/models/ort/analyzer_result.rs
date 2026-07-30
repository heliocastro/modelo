// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::dependency_graph::DependencyGraph;
use crate::models::ort::identifier::Identifier;
use crate::models::ort::issue::Issue;
use crate::models::ort::package::Package;
use crate::models::ort::project::Project;
use crate::models::{Model, ValidationError};

/// Merges all information from individual per-definition-file analyzer results.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalyzerResult {
    /// The projects, as they appear in the individual analyzer results.
    pub projects: HashSet<Project>,
    /// The set of identified packages for all projects.
    pub packages: HashSet<Package>,
    /// Issues that occurred within the analyzed projects themselves.
    #[serde(default)]
    pub issues: HashMap<Identifier, Vec<Issue>>,
    /// Dependency graphs keyed by the name of the package manager that created them.
    #[serde(default)]
    pub dependency_graphs: HashMap<String, DependencyGraph>,
}

impl fmt::Display for AnalyzerResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AnalyzerResult({} projects, {} packages)",
            self.projects.len(),
            self.packages.len()
        )
    }
}

impl Model for AnalyzerResult {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let result = AnalyzerResult::default();
        assert!(result.projects.is_empty());
        assert!(result.packages.is_empty());
    }
}
