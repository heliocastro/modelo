// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::file_storage_configuration::FileStorageConfiguration;
use crate::models::scan_storage_configuration::PostgresStorageConfiguration;
use crate::models::{Model, ValidationError};

/// Configuration of the storage to use for provenance information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceStorageConfiguration {
    /// Configuration of a file storage.
    #[serde(default)]
    pub file_storage: Option<FileStorageConfiguration>,
    /// Configuration of a PostgreSQL storage.
    #[serde(default)]
    pub postgres_storage: Option<PostgresStorageConfiguration>,
}

impl fmt::Display for ProvenanceStorageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProvenanceStorageConfiguration")
    }
}

impl Model for ProvenanceStorageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_none() {
        let config = ProvenanceStorageConfiguration::default();
        assert!(config.file_storage.is_none());
        assert!(config.postgres_storage.is_none());
    }
}
