// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// A rating of how critical a health metric value is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Criticality {
    Low,
    Medium,
    High,
    Critical,
}

/// A health metric of a package, as retrieved by an advisor implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct HealthMetric {
    /// The unique, human-readable name of the health metric.
    pub name: String,
    /// The raw metric score as returned by the provider.
    pub value: f64,
    /// A rating how critical the value is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criticality: Option<Criticality>,
    /// The reason for the score.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Details about the findings.
    #[serde(default)]
    pub details: Vec<String>,
    /// Summary of documentation about the metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    /// Link to the documentation about the metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation_link: Option<String>,
}

impl fmt::Display for HealthMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Model for HealthMetric {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.name.is_empty() {
            return Err(ValidationError::MissingField {
                field: "name".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_minimal_and_criticality() {
        let m: HealthMetric =
            serde_json::from_str(r#"{"name":"score","value":4.5,"criticality":"HIGH"}"#).unwrap();
        assert_eq!(m.criticality, Some(Criticality::High));
        assert_eq!(m.to_string(), "score");
        assert!(m.validate().is_ok());
    }

    #[test]
    fn empty_name_fails_validation() {
        let m: HealthMetric = serde_json::from_str(r#"{"name":"","value":1.0}"#).unwrap();
        assert!(m.validate().is_err());
    }
}
