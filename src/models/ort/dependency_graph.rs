// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::dependency_graph_edge::DependencyGraphEdge;
use crate::models::ort::dependency_graph_node::DependencyGraphNode;
use crate::models::ort::dependency_reference::DependencyReference;
use crate::models::ort::identifier::Identifier;
use crate::models::ort::root_dependency_index::RootDependencyIndex;
use crate::models::{Model, ValidationError};

/// The graph of dependencies of a project, shared across scopes to minimize duplication.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyGraph {
    /// The identifiers of the packages that appear in the dependency graph.
    #[serde(default)]
    pub packages: Vec<Identifier>,
    /// The dependency graph as a list of root nodes, kept for backwards compatibility.
    #[serde(default)]
    pub scope_roots: HashSet<DependencyReference>,
    /// A mapping from scope names to the direct dependencies of the scopes.
    #[serde(default)]
    pub scopes: HashMap<String, Vec<RootDependencyIndex>>,
    /// The nodes of this dependency graph.
    #[serde(default)]
    pub nodes: Vec<DependencyGraphNode>,
    /// The edges of this dependency graph.
    #[serde(default)]
    pub edges: HashSet<DependencyGraphEdge>,
}

impl fmt::Display for DependencyGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DependencyGraph({} packages, {} nodes, {} edges)",
            self.packages.len(),
            self.nodes.len(),
            self.edges.len()
        )
    }
}

impl Model for DependencyGraph {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let graph = DependencyGraph::default();
        assert!(graph.packages.is_empty());
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
    }
}
