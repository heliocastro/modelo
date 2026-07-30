// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use vale::models::hash::Hash;
use vale::models::identifier::Identifier;
use vale::models::Model;

#[test]
fn identifier_and_hash_round_trip_via_serde() {
    let id: Identifier = "Maven:org.example:artifact:1.0".parse().unwrap();
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "\"Maven:org.example:artifact:1.0\"");

    let hash = Hash::new("deadbeef", "SHA-256");
    hash.validate().expect("known algorithm must validate");
}
