// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// A description of the environment that `modelo` (or the original ORT run) was executed in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Environment {
    /// The version of the OSS Review Toolkit as a string.
    pub ort_version: String,
    /// The version of Java used to build ORT.
    pub build_jdk: String,
    /// The version of Java used to run ORT.
    pub java_version: String,
    /// Name of the operating system.
    pub os: String,
    /// The number of logical processors available.
    pub processors: u32,
    /// The maximum amount of memory available.
    pub max_memory: u64,
    /// Selected environment variables that might be relevant for debugging.
    #[serde(default)]
    pub variables: HashMap<String, String>,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.ort_version, self.os)
    }
}

impl Model for Environment {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_variables_to_empty() {
        let env = Environment {
            ort_version: "1.0".to_string(),
            build_jdk: "17".to_string(),
            java_version: "17".to_string(),
            os: "Linux".to_string(),
            processors: 8,
            max_memory: 1024,
            variables: HashMap::new(),
        };
        assert!(env.variables.is_empty());
    }
}
