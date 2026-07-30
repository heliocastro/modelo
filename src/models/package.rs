// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use purl::Purl;
use serde::{Deserialize, Serialize};

use crate::models::identifier::Identifier;
use crate::models::processed_declared_license::ProcessedDeclaredLicense;
use crate::models::remote_artifact::RemoteArtifact;
use crate::models::source_code_origin::SourceCodeOrigin;
use crate::models::vcs_info::VcsInfo;
use crate::models::{Model, ValidationError};

/// A generic descriptor for a software package: its identity, licensing, and
/// where to retrieve it and its source code. Does not describe dependencies;
/// see [`crate::models::package_reference::PackageReference`] for that.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Package {
    /// The unique identifier of this package.
    pub id: Identifier,
    /// An additional identifier in package URL syntax (https://github.com/package-url/purl-spec).
    pub purl: Purl,
    /// An optional additional identifier in CPE syntax.
    #[serde(default)]
    pub cpe: Option<String>,
    /// The set of authors declared for this package.
    #[serde(default)]
    pub authors: HashSet<String>,
    /// The set of licenses declared for this package.
    pub declared_licenses: HashSet<String>,
    /// The declared licenses processed into an SPDX expression.
    pub declared_licenses_processed: ProcessedDeclaredLicense,
    /// The concluded license as an SPDX expression, if curated.
    #[serde(default)]
    pub concluded_license: Option<String>,
    /// The description of the package, as provided by the package manager.
    pub description: String,
    /// The homepage of the package.
    pub homepage_url: String,
    /// The remote artifact where the binary package can be downloaded.
    pub binary_artifact: RemoteArtifact,
    /// The remote artifact where the source package can be downloaded.
    pub source_artifact: RemoteArtifact,
    /// Original VCS-related information as defined in the package's metadata.
    pub vcs: VcsInfo,
    /// Processed VCS-related information, normalized and possibly curated.
    pub vcs_processed: VcsInfo,
    /// Whether the package is just metadata, e.g. a Maven BOM artifact.
    #[serde(default)]
    pub is_metadata_only: bool,
    /// Whether the source code has been modified compared to the original.
    #[serde(default)]
    pub is_modified: bool,
    /// The considered source code origins, in priority order.
    #[serde(default)]
    pub source_code_origins: Option<Vec<SourceCodeOrigin>>,
    /// User defined labels associated with this package.
    #[serde(default)]
    pub labels: HashMap<String, String>,
}

impl StdHash for Package {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Package {}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Model for Package {
    fn validate(&self) -> Result<(), ValidationError> {
        if let Some(origins) = &self.source_code_origins {
            if origins.is_empty() {
                return Err(ValidationError::InvalidField {
                    field: "source_code_origins".to_string(),
                    message: "must not be empty when present".to_string(),
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn make() -> Package {
        Package {
            id: Identifier::from_str("Maven:org.example:artifact:1.0").unwrap(),
            purl: Purl::from_str("pkg:maven/org.example/artifact@1.0").unwrap(),
            cpe: None,
            authors: HashSet::new(),
            declared_licenses: HashSet::new(),
            declared_licenses_processed: ProcessedDeclaredLicense::default(),
            concluded_license: None,
            description: String::new(),
            homepage_url: String::new(),
            binary_artifact: RemoteArtifact::default(),
            source_artifact: RemoteArtifact::default(),
            vcs: VcsInfo::default(),
            vcs_processed: VcsInfo::default(),
            is_metadata_only: false,
            is_modified: false,
            source_code_origins: None,
            labels: HashMap::new(),
        }
    }

    #[test]
    fn empty_source_code_origins_fails_validation() {
        let mut pkg = make();
        pkg.source_code_origins = Some(Vec::new());
        assert!(pkg.validate().is_err());
    }
}
