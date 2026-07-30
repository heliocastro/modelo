// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// Configuration for using an AWS S3 bucket as a storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S3FileStorageConfiguration {
    /// The AWS access key.
    #[serde(default)]
    pub access_key_id: Option<String>,
    /// The AWS region to be used.
    #[serde(default)]
    pub aws_region: Option<String>,
    /// The name of the S3 bucket used to store files in.
    pub bucket_name: String,
    /// Whether to use compression for storing files.
    #[serde(default)]
    pub compression: bool,
    /// A custom endpoint to perform AWS API requests against.
    #[serde(default)]
    pub custom_endpoint: Option<String>,
    /// Whether to enable path style access. Required for many non-AWS S3 providers.
    #[serde(default)]
    pub path_style_access: bool,
    /// The AWS secret for the access key.
    #[serde(default)]
    pub secret_access_key: Option<String>,
}

impl fmt::Display for S3FileStorageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bucket_name)
    }
}

impl Model for S3FileStorageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.bucket_name.is_empty() {
            return Err(ValidationError::MissingField {
                field: "bucket_name".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bucket_name_fails_validation() {
        let config = S3FileStorageConfiguration {
            access_key_id: None,
            aws_region: None,
            bucket_name: String::new(),
            compression: false,
            custom_endpoint: None,
            path_style_access: false,
            secret_access_key: None,
        };
        assert!(config.validate().is_err());
    }
}
