// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::ort::file_archiver_configuration::FileArchiverConfiguration;
use crate::models::ort::file_list_storage_configuration::FileListStorageConfiguration;
use crate::models::ort::provenance_storage_configuration::ProvenanceStorageConfiguration;
use crate::models::ort::scan_storage_configuration::ScanStorageConfiguration;
use crate::models::{Model, ValidationError};

/// The configuration model of the scanner.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScannerConfiguration {
    /// Skip packages with concluded license and authors, using only declared info.
    #[serde(default)]
    pub skip_concluded: bool,
    /// Whether excluded scopes and paths are skipped during the scan.
    #[serde(default)]
    pub skip_excluded: bool,
    /// Whether the scanner should add files without license to the scanner results.
    #[serde(default)]
    pub include_files_without_findings: bool,
    /// Configuration of a `FileArchiver` that archives selected scanned files in external storage.
    #[serde(default)]
    pub archive: Option<FileArchiverConfiguration>,
    /// Mappings from scanner-returned licenses to valid SPDX licenses; only applied to new scans.
    #[serde(default)]
    pub detected_license_mapping: HashMap<String, String>,
    /// The storage to store the file lists by provenance.
    #[serde(default)]
    pub file_list_storage: Option<FileListStorageConfiguration>,
    /// Scanner-specific configuration options, keyed by scanner class name.
    #[serde(default)]
    pub scanners: Option<HashMap<String, Value>>,
    /// The configurations of the scan result storages available, keyed by storage id.
    #[serde(default)]
    pub storages: Option<HashMap<String, ScanStorageConfiguration>>,
    /// The IDs of scan storages that are queried for existing scan results.
    #[serde(default)]
    pub storage_readers: Option<Vec<String>>,
    /// The IDs of scan storages that are called to persist scan results.
    #[serde(default)]
    pub storage_writers: Option<Vec<String>>,
    /// Glob expressions matching file paths to exclude from scan results.
    #[serde(default)]
    pub ignore_patterns: Vec<String>,
    /// Configuration of the storage for provenance information.
    #[serde(default)]
    pub provenance_storage: Option<ProvenanceStorageConfiguration>,
}

impl fmt::Display for ScannerConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ScannerConfiguration")
    }
}

impl Model for ScannerConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_have_no_ignore_patterns() {
        assert!(ScannerConfiguration::default().ignore_patterns.is_empty());
    }
}
