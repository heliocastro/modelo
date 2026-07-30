// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

/// References text located in a file, by line range.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextLocation {
    /// The path (with invariant separators) of the file that contains the text.
    pub path: String,
    /// The line the text is starting at.
    pub start_line: u32,
    /// The line the text is ending at.
    pub end_line: u32,
}

impl fmt::Display for TextLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}-{}", self.path, self.start_line, self.end_line)
    }
}

impl Model for TextLocation {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.end_line < self.start_line {
            return Err(ValidationError::OutOfRange {
                field: "end_line".to_string(),
                message: "must not be before start_line".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_inverted_range() {
        let loc = TextLocation {
            path: "a.rs".to_string(),
            start_line: 10,
            end_line: 5,
        };
        assert!(loc.validate().is_err());
    }
}
