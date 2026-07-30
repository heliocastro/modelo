// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use crate::models::{Model, ValidationError};

/// A CVSS version 3 rating, see
/// <https://www.first.org/cvss/v3.1/specification-document#Qualitative-Severity-Rating-Scale>.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cvss3Rating {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl Cvss3Rating {
    /// The upper bound score (inclusive) for this rating.
    pub fn upper_bound(self) -> f64 {
        match self {
            Cvss3Rating::None => 0.0,
            Cvss3Rating::Low => 4.0,
            Cvss3Rating::Medium => 7.0,
            Cvss3Rating::High => 9.0,
            Cvss3Rating::Critical => 10.0,
        }
    }

    /// The prefixes that refer to the CVSS version 3 scoring system.
    pub fn prefixes() -> &'static [&'static str] {
        &["CVSS3", "CVSSV3", "CVSS_V3", "CVSS:3"]
    }

    /// Gets the rating from a score, or `None` if the score is out of range.
    pub fn from_score(score: f64) -> Option<Self> {
        if !(0.0..=Cvss3Rating::Critical.upper_bound()).contains(&score) {
            return None;
        }
        if score < Cvss3Rating::None.upper_bound() {
            Some(Cvss3Rating::None)
        } else if score < Cvss3Rating::Low.upper_bound() {
            Some(Cvss3Rating::Low)
        } else if score < Cvss3Rating::Medium.upper_bound() {
            Some(Cvss3Rating::Medium)
        } else if score < Cvss3Rating::High.upper_bound() {
            Some(Cvss3Rating::High)
        } else {
            Some(Cvss3Rating::Critical)
        }
    }
}

impl fmt::Display for Cvss3Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cvss3Rating::None => write!(f, "NONE"),
            Cvss3Rating::Low => write!(f, "LOW"),
            Cvss3Rating::Medium => write!(f, "MEDIUM"),
            Cvss3Rating::High => write!(f, "HIGH"),
            Cvss3Rating::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl Model for Cvss3Rating {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_score_to_rating() {
        // Note: a score of exactly 0.0 maps to `Low`, not `None`, matching python-ort's own
        // `from_score` logic (the `< upper_bound` comparison makes `NONE` unreachable at 0.0).
        assert_eq!(Cvss3Rating::from_score(0.0), Some(Cvss3Rating::Low));
        assert_eq!(Cvss3Rating::from_score(10.0), Some(Cvss3Rating::Critical));
    }

    #[test]
    fn out_of_range_score_is_none() {
        assert_eq!(Cvss3Rating::from_score(-1.0), None);
        assert_eq!(Cvss3Rating::from_score(11.0), None);
    }
}
