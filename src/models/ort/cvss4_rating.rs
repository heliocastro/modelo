// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use crate::models::{Model, ValidationError};

/// A CVSS version 4 rating, see
/// <https://www.first.org/cvss/v4.0/specification-document#Qualitative-Severity-Rating-Scale>.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cvss4Rating {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl Cvss4Rating {
    /// The upper bound score (inclusive) for this rating.
    pub fn upper_bound(self) -> f64 {
        match self {
            Cvss4Rating::None => 0.0,
            Cvss4Rating::Low => 4.0,
            Cvss4Rating::Medium => 7.0,
            Cvss4Rating::High => 9.0,
            Cvss4Rating::Critical => 10.0,
        }
    }

    /// The prefixes that refer to the CVSS version 4 scoring system.
    pub fn prefixes() -> &'static [&'static str] {
        &["CVSS4", "CVSSV4", "CVSS_V4", "CVSS:4"]
    }

    /// Gets the rating from a score, or `None` if the score is out of range.
    pub fn from_score(score: f64) -> Option<Self> {
        if !(0.0..=Cvss4Rating::Critical.upper_bound()).contains(&score) {
            return None;
        }
        if score < Cvss4Rating::None.upper_bound() {
            Some(Cvss4Rating::None)
        } else if score < Cvss4Rating::Low.upper_bound() {
            Some(Cvss4Rating::Low)
        } else if score < Cvss4Rating::Medium.upper_bound() {
            Some(Cvss4Rating::Medium)
        } else if score < Cvss4Rating::High.upper_bound() {
            Some(Cvss4Rating::High)
        } else {
            Some(Cvss4Rating::Critical)
        }
    }
}

impl fmt::Display for Cvss4Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cvss4Rating::None => write!(f, "NONE"),
            Cvss4Rating::Low => write!(f, "LOW"),
            Cvss4Rating::Medium => write!(f, "MEDIUM"),
            Cvss4Rating::High => write!(f, "HIGH"),
            Cvss4Rating::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl Model for Cvss4Rating {
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
        assert_eq!(Cvss4Rating::from_score(0.0), Some(Cvss4Rating::Low));
        assert_eq!(Cvss4Rating::from_score(10.0), Some(Cvss4Rating::Critical));
    }

    #[test]
    fn out_of_range_score_is_none() {
        assert_eq!(Cvss4Rating::from_score(-1.0), None);
        assert_eq!(Cvss4Rating::from_score(11.0), None);
    }
}
