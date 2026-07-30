// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_cvss_ratings.py`.

use vale::models::ort::cvss2_rating::Cvss2Rating;
use vale::models::ort::cvss3_rating::Cvss3Rating;
use vale::models::ort::cvss4_rating::Cvss4Rating;

#[test]
fn cvss2_upper_bounds() {
    assert_eq!(Cvss2Rating::Low.upper_bound(), 4.0);
    assert_eq!(Cvss2Rating::Medium.upper_bound(), 7.0);
    assert_eq!(Cvss2Rating::High.upper_bound(), 10.0);
}

#[test]
fn cvss2_prefixes() {
    assert_eq!(
        Cvss2Rating::prefixes().to_vec(),
        vec!["CVSS2", "CVSSV2", "CVSS_V2", "CVSS:2"]
    );
}

#[test]
fn cvss2_from_score_valid() {
    assert_eq!(Cvss2Rating::from_score(0.0), Some(Cvss2Rating::Low));
    assert_eq!(Cvss2Rating::from_score(3.9), Some(Cvss2Rating::Low));
    assert_eq!(Cvss2Rating::from_score(4.0), Some(Cvss2Rating::Medium));
    assert_eq!(Cvss2Rating::from_score(6.9), Some(Cvss2Rating::Medium));
    assert_eq!(Cvss2Rating::from_score(7.0), Some(Cvss2Rating::High));
    assert_eq!(Cvss2Rating::from_score(10.0), Some(Cvss2Rating::High));
}

#[test]
fn cvss2_from_score_out_of_range() {
    for score in [-0.1, -1.0, 10.1, 100.0] {
        assert_eq!(Cvss2Rating::from_score(score), None, "score {score}");
    }
}

#[test]
fn cvss3_upper_bounds() {
    assert_eq!(Cvss3Rating::None.upper_bound(), 0.0);
    assert_eq!(Cvss3Rating::Low.upper_bound(), 4.0);
    assert_eq!(Cvss3Rating::Medium.upper_bound(), 7.0);
    assert_eq!(Cvss3Rating::High.upper_bound(), 9.0);
    assert_eq!(Cvss3Rating::Critical.upper_bound(), 10.0);
}

#[test]
fn cvss3_prefixes() {
    assert_eq!(
        Cvss3Rating::prefixes().to_vec(),
        vec!["CVSS3", "CVSSV3", "CVSS_V3", "CVSS:3"]
    );
}

#[test]
fn cvss3_from_score_valid() {
    // Note: python-ort's `from_score` never returns `NONE` for a real score (see the port's own
    // unit test in `src/models/cvss3_rating.rs`), so 0.0 maps to `Low` here too.
    assert_eq!(Cvss3Rating::from_score(0.0), Some(Cvss3Rating::Low));
    assert_eq!(Cvss3Rating::from_score(3.9), Some(Cvss3Rating::Low));
    assert_eq!(Cvss3Rating::from_score(4.0), Some(Cvss3Rating::Medium));
    assert_eq!(Cvss3Rating::from_score(6.9), Some(Cvss3Rating::Medium));
    assert_eq!(Cvss3Rating::from_score(7.0), Some(Cvss3Rating::High));
    assert_eq!(Cvss3Rating::from_score(8.9), Some(Cvss3Rating::High));
    assert_eq!(Cvss3Rating::from_score(9.0), Some(Cvss3Rating::Critical));
    assert_eq!(Cvss3Rating::from_score(10.0), Some(Cvss3Rating::Critical));
}

#[test]
fn cvss3_from_score_out_of_range() {
    for score in [-0.1, -1.0, 10.1, 100.0] {
        assert_eq!(Cvss3Rating::from_score(score), None, "score {score}");
    }
}

#[test]
fn cvss4_upper_bounds() {
    assert_eq!(Cvss4Rating::None.upper_bound(), 0.0);
    assert_eq!(Cvss4Rating::Low.upper_bound(), 4.0);
    assert_eq!(Cvss4Rating::Medium.upper_bound(), 7.0);
    assert_eq!(Cvss4Rating::High.upper_bound(), 9.0);
    assert_eq!(Cvss4Rating::Critical.upper_bound(), 10.0);
}

#[test]
fn cvss4_prefixes() {
    assert_eq!(
        Cvss4Rating::prefixes().to_vec(),
        vec!["CVSS4", "CVSSV4", "CVSS_V4", "CVSS:4"]
    );
}

#[test]
fn cvss4_from_score_valid() {
    assert_eq!(Cvss4Rating::from_score(0.0), Some(Cvss4Rating::Low));
    assert_eq!(Cvss4Rating::from_score(3.9), Some(Cvss4Rating::Low));
    assert_eq!(Cvss4Rating::from_score(4.0), Some(Cvss4Rating::Medium));
    assert_eq!(Cvss4Rating::from_score(6.9), Some(Cvss4Rating::Medium));
    assert_eq!(Cvss4Rating::from_score(7.0), Some(Cvss4Rating::High));
    assert_eq!(Cvss4Rating::from_score(8.9), Some(Cvss4Rating::High));
    assert_eq!(Cvss4Rating::from_score(9.0), Some(Cvss4Rating::Critical));
    assert_eq!(Cvss4Rating::from_score(10.0), Some(Cvss4Rating::Critical));
}

#[test]
fn cvss4_from_score_out_of_range() {
    for score in [-0.1, -1.0, 10.1, 100.0] {
        assert_eq!(Cvss4Rating::from_score(score), None, "score {score}");
    }
}
