// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::{Model, ValidationError};

const KNOWN_TYPES: &[&str] = &[
    "Git",
    "GitHub",
    "GitLab",
    "GitRepo",
    "git-repo",
    "repo",
    "Mercurial",
    "hg",
    "Subversion",
    "svn",
];

/// A Version Control System type, e.g. `Git`, `Mercurial`, or `Subversion`.
///
/// Unlike an enum, an unknown type is accepted and normalized to an empty name,
/// matching `python-ort`'s permissive parsing behavior.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VcsType {
    pub name: String,
}

impl VcsType {
    pub fn new(name: impl AsRef<str>) -> Self {
        let name = name.as_ref();
        if KNOWN_TYPES
            .iter()
            .any(|known| known.eq_ignore_ascii_case(name))
        {
            Self {
                name: name.to_string(),
            }
        } else {
            Self::default()
        }
    }
}

impl fmt::Display for VcsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Serialize for VcsType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.name)
    }
}

impl<'de> Deserialize<'de> for VcsType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Ok(VcsType::new(raw))
    }
}

impl Model for VcsType {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_known_alias_case_insensitive() {
        assert_eq!(VcsType::new("gitlab").name, "gitlab");
    }

    #[test]
    fn unknown_type_becomes_empty() {
        assert_eq!(VcsType::new("Perforce").name, "");
    }
}
