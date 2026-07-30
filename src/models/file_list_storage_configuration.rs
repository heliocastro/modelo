// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::file_storage_configuration::FileStorageConfiguration;
use crate::models::scan_storage_configuration::PostgresStorageConfiguration;
use crate::models::{Model, ValidationError};

/// Configuration for the storage backends used for persisting file lists.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileListStorageConfiguration {
    /// Configuration of the file storage used for storing the file lists.
    #[serde(default)]
    pub file_storage: Option<FileStorageConfiguration>,
    /// Configuration of the Postgres-based storage used for storing the file lists.
    #[serde(default)]
    pub postgres_storage: Option<PostgresStorageConfiguration>,
}

impl fmt::Display for FileListStorageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileListStorageConfiguration")
    }
}

impl Model for FileListStorageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_none() {
        let config = FileListStorageConfiguration::default();
        assert!(config.file_storage.is_none());
        assert!(config.postgres_storage.is_none());
    }
}
