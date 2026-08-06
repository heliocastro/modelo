// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use modelo::models::ort::hash::Hash;
use modelo::models::ort::identifier::Identifier;
use modelo::models::Model;

#[test]
fn identifier_and_hash_round_trip_via_serde() {
    let id: Identifier = "Maven:org.example:artifact:1.0".parse().unwrap();
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "\"Maven:org.example:artifact:1.0\"");

    let hash = Hash::new("deadbeef", "SHA-256");
    hash.validate().expect("known algorithm must validate");
}
