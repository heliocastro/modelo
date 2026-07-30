// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// Configuration for HTTP-based file storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpFileStorageConfiguration {
    /// The URL of the HTTP server, e.g. `https://example.com/storage`.
    pub url: String,
    /// A query string appended to the URL and path. May contain auth data.
    #[serde(default)]
    pub query: String,
    /// Custom headers added to all HTTP requests. Values may contain credentials.
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

impl fmt::Display for HttpFileStorageConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl Model for HttpFileStorageConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.url.is_empty() {
            return Err(ValidationError::MissingField {
                field: "url".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_url_fails_validation() {
        let config = HttpFileStorageConfiguration {
            url: String::new(),
            query: String::new(),
            headers: HashMap::new(),
        };
        assert!(config.validate().is_err());
    }
}
