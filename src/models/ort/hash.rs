// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

const ALGORITHMS: &[(&str, &[&str])] = &[
    ("MD5", &["MD5"]),
    ("SHA1", &["SHA-1", "SHA1"]),
    ("SHA256", &["SHA-256", "SHA256"]),
    ("SHA384", &["SHA-384", "SHA384"]),
    ("SHA512", &["SHA-512", "SHA512"]),
    ("SHA1GIT", &["SHA-1-GIT", "SHA1-GIT", "SHA1GIT", "SWHID"]),
];

/// Normalizes an alias (e.g. "SHA-256") to its canonical algorithm name (e.g. "SHA256").
fn canonical_algorithm(value: &str) -> String {
    for (canonical, aliases) in ALGORITHMS {
        if aliases.contains(&value) {
            return (*canonical).to_string();
        }
    }
    "UNKNOWN".to_string()
}

/// Bundles a hash algorithm with its hash value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash {
    /// The value calculated using the hash algorithm.
    pub value: String,
    /// The algorithm used to calculate the hash value.
    #[serde(deserialize_with = "deserialize_algorithm")]
    pub algorithm: String,
}

fn deserialize_algorithm<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(canonical_algorithm(&raw))
}

impl Hash {
    pub fn new(value: impl Into<String>, algorithm: impl AsRef<str>) -> Self {
        Self {
            value: value.into(),
            algorithm: canonical_algorithm(algorithm.as_ref()),
        }
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.value)
    }
}

impl Model for Hash {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.algorithm == "UNKNOWN" {
            return Err(ValidationError::InvalidField {
                field: "algorithm".to_string(),
                message: "unrecognized hash algorithm".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_known_alias() {
        assert_eq!(Hash::new("abc", "SHA-256").algorithm, "SHA256");
    }

    #[test]
    fn unknown_alias_fails_validation() {
        let hash = Hash::new("abc", "CRC32");
        assert!(hash.validate().is_err());
    }
}
