// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_advisor_details.py`.
//!
//! python-ort's `AdvisorDetails` sets `extra="forbid"`, so an unknown field raises a
//! `ValidationError`; this port gets the same behaviour from `#[serde(deny_unknown_fields)]`.

use vale::models::ort::advisor_details::AdvisorDetails;

#[test]
fn unknown_field_is_rejected() {
    let yaml = "name: TestAdvisor\nunknown_field: value\n";
    assert!(serde_yaml::from_str::<AdvisorDetails>(yaml).is_err());

    let details: AdvisorDetails = serde_yaml::from_str("name: TestAdvisor\n").unwrap();
    assert_eq!(details.name, "TestAdvisor");
}
