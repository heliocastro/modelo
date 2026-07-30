// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;
use std::hash::{Hash as StdHash, Hasher};

use serde::{Deserialize, Serialize};

use crate::models::text_location::TextLocation;
use crate::models::{Model, ValidationError};

/// A single copyright finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CopyrightFinding {
    /// The copyright statement.
    pub statement: String,
    /// The text location where the copyright statement was found.
    pub location: TextLocation,
}

impl StdHash for CopyrightFinding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.statement.hash(state);
    }
}

impl PartialEq for CopyrightFinding {
    fn eq(&self, other: &Self) -> bool {
        self.statement == other.statement
    }
}

impl Eq for CopyrightFinding {}

impl fmt::Display for CopyrightFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.statement)
    }
}

impl Model for CopyrightFinding {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.statement.is_empty() {
            return Err(ValidationError::MissingField {
                field: "statement".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(statement: &str) -> CopyrightFinding {
        CopyrightFinding {
            statement: statement.to_string(),
            location: TextLocation {
                path: "NOTICE".to_string(),
                start_line: 1,
                end_line: 1,
            },
        }
    }

    #[test]
    fn equality_is_by_statement() {
        assert_eq!(make("Copyright 2026 Foo"), make("Copyright 2026 Foo"));
    }

    #[test]
    fn empty_statement_fails_validation() {
        assert!(make("").validate().is_err());
    }
}
