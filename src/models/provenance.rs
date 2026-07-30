// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::remote_artifact::RemoteArtifact;
use crate::models::vcs_info::VcsInfo;
use crate::models::{Model, ValidationError};

/// Provenance information for a source artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactProvenance {
    /// The source artifact that was downloaded.
    pub source_artifact: RemoteArtifact,
}

impl StdHash for ArtifactProvenance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source_artifact.url.hash(state);
    }
}

impl PartialEq for ArtifactProvenance {
    fn eq(&self, other: &Self) -> bool {
        self.source_artifact.url == other.source_artifact.url
    }
}

impl Eq for ArtifactProvenance {}

/// Provenance information for a Version Control System location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryProvenance {
    /// The VCS info used to resolve the revision. May still contain a moving revision.
    pub vcs_info: VcsInfo,
    /// The resolved fixed VCS revision (e.g. a Git commit SHA1), not blank and not moving.
    pub resolved_revision: String,
}

impl StdHash for RepositoryProvenance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vcs_info.url.hash(state);
        self.resolved_revision.hash(state);
    }
}

impl PartialEq for RepositoryProvenance {
    fn eq(&self, other: &Self) -> bool {
        self.vcs_info.url == other.vcs_info.url && self.resolved_revision == other.resolved_revision
    }
}

impl Eq for RepositoryProvenance {}

/// Provenance information about the origin of source code: either unknown, an artifact
/// download, or a VCS checkout.
///
/// Serialized untagged: the variant is inferred from which fields are present, matching
/// python-ort's `_provenance_discriminator`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Provenance {
    Repository(RepositoryProvenance),
    Artifact(ArtifactProvenance),
    Unknown,
}

impl fmt::Display for Provenance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Provenance::Repository(repo) => {
                write!(f, "{}@{}", repo.vcs_info.url, repo.resolved_revision)
            }
            Provenance::Artifact(artifact) => write!(f, "{}", artifact.source_artifact.url),
            Provenance::Unknown => write!(f, "<unknown provenance>"),
        }
    }
}

impl Model for Provenance {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_equality_is_by_url() {
        let a = ArtifactProvenance {
            source_artifact: RemoteArtifact {
                url: "https://example.com/a.tar.gz".to_string(),
                hash: None,
            },
        };
        let b = ArtifactProvenance {
            source_artifact: RemoteArtifact {
                url: "https://example.com/a.tar.gz".to_string(),
                hash: None,
            },
        };
        assert_eq!(a, b);
    }

    #[test]
    fn unknown_displays_placeholder() {
        assert_eq!(Provenance::Unknown.to_string(), "<unknown provenance>");
    }
}
