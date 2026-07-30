// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::hash::Hash;
use crate::models::{Model, ValidationError};

/// Bundles information about a remote artifact.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RemoteArtifact {
    /// The URL of the remote artifact.
    #[serde(default)]
    pub url: String,
    /// The hash of the remote artifact.
    #[serde(default)]
    pub hash: Option<Hash>,
}

impl fmt::Display for RemoteArtifact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl Model for RemoteArtifact {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let artifact = RemoteArtifact::default();
        assert_eq!(artifact.url, "");
        assert!(artifact.hash.is_none());
    }
}
