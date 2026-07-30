// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::ort::provenance::Provenance;
use crate::models::{Model, ValidationError};

/// A single file entry in a [`FileList`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileListEntry {
    /// The path of the file relative to the root of the provenance.
    pub path: String,
    /// The sha1 checksum of the file, as 40 lowercase hexadecimal digits.
    pub sha1: String,
}

impl StdHash for FileListEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl PartialEq for FileListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for FileListEntry {}

/// The file info for files contained in a given [`Provenance`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileList {
    /// The provenance this file list corresponds to.
    pub provenance: Provenance,
    /// The files contained in `provenance`, excluding directories.
    pub files: HashSet<FileListEntry>,
}

impl StdHash for FileList {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.provenance.hash(state);
    }
}

impl PartialEq for FileList {
    fn eq(&self, other: &Self) -> bool {
        self.provenance == other.provenance
    }
}

impl Eq for FileList {}

impl fmt::Display for FileList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} files)", self.provenance, self.files.len())
    }
}

impl Model for FileList {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_equality_is_by_path() {
        let a = FileListEntry {
            path: "src/main.rs".to_string(),
            sha1: "abc".to_string(),
        };
        let b = FileListEntry {
            path: "src/main.rs".to_string(),
            sha1: "def".to_string(),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn displays_provenance_and_file_count() {
        let list = FileList {
            provenance: Provenance::Unknown,
            files: HashSet::new(),
        };
        assert_eq!(list.to_string(), "<unknown provenance> (0 files)");
    }
}
