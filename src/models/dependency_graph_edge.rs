// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// A directed depends-on edge in a [`crate::models::dependency_graph::DependencyGraph`].
///
/// The endpoints are numeric indices into the graph's list of nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyGraphEdge {
    /// The index of the source node of this edge.
    #[serde(rename = "from")]
    pub from_: i64,
    /// The index of the destination node of this edge.
    #[serde(rename = "to")]
    pub to_: i64,
}

impl fmt::Display for DependencyGraphEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from_, self.to_)
    }
}

impl Model for DependencyGraphEdge {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_edge() {
        let edge = DependencyGraphEdge { from_: 0, to_: 1 };
        assert_eq!(edge.to_string(), "0 -> 1");
    }
}
