// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use crate::models::{Model, ValidationError};

/// A CVSS version 2 rating, see <https://nvd.nist.gov/vuln-metrics/cvss>.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cvss2Rating {
    Low,
    Medium,
    High,
}

impl Cvss2Rating {
    /// The upper bound score (inclusive) for this rating.
    pub fn upper_bound(self) -> f64 {
        match self {
            Cvss2Rating::Low => 4.0,
            Cvss2Rating::Medium => 7.0,
            Cvss2Rating::High => 10.0,
        }
    }

    /// The prefixes that refer to the CVSS version 2 scoring system.
    pub fn prefixes() -> &'static [&'static str] {
        &["CVSS2", "CVSSV2", "CVSS_V2", "CVSS:2"]
    }

    /// Gets the rating from a score, or `None` if the score is out of range.
    pub fn from_score(score: f64) -> Option<Self> {
        if !(0.0..=Cvss2Rating::High.upper_bound()).contains(&score) {
            return None;
        }
        if score < Cvss2Rating::Low.upper_bound() {
            Some(Cvss2Rating::Low)
        } else if score < Cvss2Rating::Medium.upper_bound() {
            Some(Cvss2Rating::Medium)
        } else {
            Some(Cvss2Rating::High)
        }
    }
}

impl fmt::Display for Cvss2Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cvss2Rating::Low => write!(f, "LOW"),
            Cvss2Rating::Medium => write!(f, "MEDIUM"),
            Cvss2Rating::High => write!(f, "HIGH"),
        }
    }
}

impl Model for Cvss2Rating {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_score_to_rating() {
        assert_eq!(Cvss2Rating::from_score(2.0), Some(Cvss2Rating::Low));
        assert_eq!(Cvss2Rating::from_score(10.0), Some(Cvss2Rating::High));
    }

    #[test]
    fn out_of_range_score_is_none() {
        assert_eq!(Cvss2Rating::from_score(-1.0), None);
        assert_eq!(Cvss2Rating::from_score(11.0), None);
    }
}
