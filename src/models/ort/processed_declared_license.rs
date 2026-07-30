// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// The result of mapping a package's or project's declared licenses to SPDX.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessedDeclaredLicense {
    /// The resulting SPDX expression, or `None` if no license could be mapped.
    #[serde(default)]
    pub spdx_expression: Option<String>,
    /// A map from the original declared license strings to the SPDX expressions
    /// they were mapped to.
    #[serde(default)]
    pub mapped: HashMap<String, String>,
    /// Declared licenses that could not be mapped to an SPDX expression.
    #[serde(default)]
    pub unmapped: HashSet<String>,
}

impl fmt::Display for ProcessedDeclaredLicense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.spdx_expression.as_deref().unwrap_or(""))
    }
}

impl Model for ProcessedDeclaredLicense {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let processed = ProcessedDeclaredLicense::default();
        assert!(processed.spdx_expression.is_none());
        assert!(processed.mapped.is_empty());
    }
}
