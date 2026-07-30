// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::ort::file_storage_configuration::FileStorageConfiguration;
use crate::models::ort::postgres_connection::PostgresConnection;
use crate::models::{Model, ValidationError};

/// The way that scan results are stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StorageType {
    /// A storage that stores scan results by package.
    PackageBased = 1,
    /// A storage that stores scan results by provenance.
    ProvenanceBased = 2,
}

crate::models::ort::validated_int_enum::validated_int_enum!(StorageType {
    PackageBased = 1 => "PACKAGE_BASED",
    ProvenanceBased = 2 => "PROVENANCE_BASED",
});

fn default_storage_type() -> StorageType {
    StorageType::ProvenanceBased
}

// ponytail: python-ort's base `ScanStorageConfiguration` allows arbitrary extra fields
// (`extra="allow"`) and is used generically as the value type of
// `ScannerConfiguration.storages`. Kept as an opaque JSON passthrough here; the three
// concrete storage kinds below are exposed as their own typed structs for direct use.
/// Root of a hierarchy of scan storage configurations. Opaque passthrough for storage kinds
/// not modeled by [`ClearlyDefinedStorageConfiguration`], [`FileBasedStorageConfiguration`],
/// or [`PostgresStorageConfiguration`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ScanStorageConfiguration {
    #[serde(flatten)]
    pub raw: Value,
}

/// The configuration model of a storage based on ClearlyDefined.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClearlyDefinedStorageConfiguration {
    /// The URL of the ClearlyDefined server.
    pub server_url: String,
}

/// The configuration model of a file based scan result storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileBasedStorageConfiguration {
    /// The configuration of the file storage used to store the files.
    pub backend: FileStorageConfiguration,
    /// The way that scan results are stored.
    #[serde(default = "default_storage_type", rename = "type")]
    pub ort_type: StorageType,
}

/// The configuration for using Postgres as a scan result storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostgresStorageConfiguration {
    /// The configuration of the PostgreSQL database.
    pub connection: PostgresConnection,
    /// The way that scan results are stored.
    #[serde(default = "default_storage_type", rename = "type")]
    pub ort_type: StorageType,
}

impl fmt::Display for ScanStorageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ScanStorageConfiguration")
    }
}

impl Model for ScanStorageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl fmt::Display for ClearlyDefinedStorageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.server_url)
    }
}

impl Model for ClearlyDefinedStorageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.server_url.is_empty() {
            return Err(ValidationError::MissingField {
                field: "server_url".to_string(),
            });
        }
        Ok(())
    }
}

impl fmt::Display for FileBasedStorageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileBasedStorageConfiguration({})", self.ort_type)
    }
}

impl Model for FileBasedStorageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl fmt::Display for PostgresStorageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PostgresStorageConfiguration({})", self.ort_type)
    }
}

impl Model for PostgresStorageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_server_url_fails_validation() {
        let config = ClearlyDefinedStorageConfiguration {
            server_url: String::new(),
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn opaque_config_defaults_to_null() {
        assert_eq!(ScanStorageConfiguration::default().raw, Value::Null);
    }
}
