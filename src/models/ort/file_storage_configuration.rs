// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::ort::http_file_storage_configuration::HttpFileStorageConfiguration;
use crate::models::ort::local_file_storage_configuration::LocalFileStorageConfiguration;
use crate::models::ort::s3_file_storage_configuration::S3FileStorageConfiguration;
use crate::models::{Model, ValidationError};

/// The configuration model for a file storage. Only one of the storage options should be set.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileStorageConfiguration {
    /// The configuration of an HTTP-based file storage.
    #[serde(default)]
    pub http_file_storage: Option<HttpFileStorageConfiguration>,
    /// The configuration of a local file storage.
    #[serde(default)]
    pub local_file_storage: Option<LocalFileStorageConfiguration>,
    /// The configuration of an S3-based file storage.
    #[serde(default)]
    pub s3_file_storage: Option<S3FileStorageConfiguration>,
}

impl fmt::Display for FileStorageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileStorageConfiguration")
    }
}

impl Model for FileStorageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_none() {
        let config = FileStorageConfiguration::default();
        assert!(config.http_file_storage.is_none());
        assert!(config.local_file_storage.is_none());
        assert!(config.s3_file_storage.is_none());
    }
}
