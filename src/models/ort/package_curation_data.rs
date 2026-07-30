// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::ort::remote_artifact::RemoteArtifact;
use crate::models::ort::source_code_origin::SourceCodeOrigin;
use crate::models::ort::vcsinfo_curation_data::VcsInfoCurationData;
use crate::models::{Model, ValidationError};

/// Curation data for a package: an overlay of fields to apply on top of the original metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageCurationData {
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub purl: Option<String>,
    #[serde(default)]
    pub cpe: Option<String>,
    #[serde(default)]
    pub authors: Option<Vec<String>>,
    #[serde(default)]
    pub concluded_license: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub homepage_url: Option<String>,
    #[serde(default)]
    pub binary_artifact: Option<RemoteArtifact>,
    #[serde(default)]
    pub source_artifact: Option<RemoteArtifact>,
    #[serde(default)]
    pub vcs: Option<VcsInfoCurationData>,
    #[serde(default)]
    pub is_metadata_only: Option<bool>,
    #[serde(default)]
    pub is_modified: Option<bool>,
    #[serde(default)]
    pub declared_license_mapping: HashMap<String, Value>,
    #[serde(default)]
    pub source_code_origins: Option<Vec<SourceCodeOrigin>>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
}

impl fmt::Display for PackageCurationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.comment.as_deref().unwrap_or("<curation>"))
    }
}

impl Model for PackageCurationData {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let data = PackageCurationData::default();
        assert!(data.labels.is_empty());
        assert!(data.declared_license_mapping.is_empty());
    }
}
