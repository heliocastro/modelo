// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// Configuration for using local files as a storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalFileStorageConfiguration {
    /// The directory to use as a storage root.
    pub directory: String,
    /// Whether to use compression for storing files.
    #[serde(default = "default_compression")]
    pub compression: bool,
}

fn default_compression() -> bool {
    true
}

impl fmt::Display for LocalFileStorageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.directory)
    }
}

impl Model for LocalFileStorageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.directory.is_empty() {
            return Err(ValidationError::MissingField {
                field: "directory".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_directory_fails_validation() {
        let config = LocalFileStorageConfiguration {
            directory: String::new(),
            compression: true,
        };
        assert!(config.validate().is_err());
    }
}
