// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;
use std::str::FromStr;

use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::{Model, ValidationError};

/// A unique identifier for a software component, e.g. `Maven:org.example:artifact:1.0`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    /// The type of component this identifier describes (e.g. the package manager or ecosystem).
    pub orttype: String,
    /// The namespace of the component, e.g. the group for Maven or the scope for NPM.
    pub namespace: String,
    /// The name of the component.
    pub name: String,
    /// The version of the component.
    pub version: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}",
            self.orttype, self.namespace, self.name, self.version
        )
    }
}

impl FromStr for Identifier {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = value.split(':').collect();
        let [orttype, namespace, name, version] = parts.as_slice() else {
            return Err(ValidationError::InvalidField {
                field: "identifier".to_string(),
                message: "must be in the format 'type:namespace:name:version'".to_string(),
            });
        };
        Ok(Self {
            orttype: orttype.to_string(),
            namespace: namespace.to_string(),
            name: name.to_string(),
            version: version.to_string(),
        })
    }
}

impl Serialize for Identifier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Identifier::from_str(&raw).map_err(DeError::custom)
    }
}

impl Model for Identifier {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_string() {
        let id: Identifier = "Maven:org.example:artifact:1.0".parse().unwrap();
        assert_eq!(id.orttype, "Maven");
        assert_eq!(id.version, "1.0");
    }

    #[test]
    fn roundtrips_display() {
        let id: Identifier = "Maven:org.example:artifact:1.0".parse().unwrap();
        assert_eq!(id.to_string(), "Maven:org.example:artifact:1.0");
    }

    #[test]
    fn rejects_wrong_part_count() {
        assert!("Maven:org.example".parse::<Identifier>().is_err());
    }
}
