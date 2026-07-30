// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::file_storage_configuration::FileStorageConfiguration;
use crate::models::scan_storage_configuration::PostgresStorageConfiguration;
use crate::models::{Model, ValidationError};

/// The configuration model for a `FileArchiver`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileArchiverConfiguration {
    /// Toggle to enable or disable the file archiver functionality altogether.
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Configuration of the file storage used for archiving the files.
    #[serde(default)]
    pub file_storage: Option<FileStorageConfiguration>,
    /// Configuration of the Postgres-based storage used for archiving the files.
    #[serde(default)]
    pub postgres_storage: Option<PostgresStorageConfiguration>,
}

fn default_enabled() -> bool {
    true
}

impl fmt::Display for FileArchiverConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileArchiverConfiguration(enabled={})", self.enabled)
    }
}

impl Model for FileArchiverConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_enabled() {
        let config = FileArchiverConfiguration {
            enabled: default_enabled(),
            file_storage: None,
            postgres_storage: None,
        };
        assert!(config.enabled);
    }
}
