// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! `serde` helpers mirroring python-ort's `mode="before"` field validators, which coerce loosely
//! typed YAML scalars into the field's declared type instead of rejecting them.

use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde_yaml::Value;

/// Renders a YAML scalar the way python-ort's `str(v).lower()` coercion does, or returns `None`
/// for non-scalars.
fn scalar_to_string(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

/// Deserializes an optional string field that python-ort also accepts as an integer
/// (e.g. `LicenseFindingCuration.start_lines: 22`). An empty string becomes `None`,
/// matching python-ort's `parse_start_lines`.
pub fn optional_string_from_scalar<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    match Option::<Value>::deserialize(deserializer)? {
        None | Some(Value::Null) => Ok(None),
        Some(value) => match scalar_to_string(&value) {
            Some(s) if s.is_empty() => Ok(None),
            Some(s) => Ok(Some(s)),
            None => Err(serde::de::Error::custom(
                "expected a string or an integer scalar",
            )),
        },
    }
}

/// Deserializes an optional `string -> string` map whose values python-ort coerces from any
/// scalar via `str(v).lower()` (e.g. `PackageManagerConfiguration.options: {legacy: false}`).
pub fn optional_string_map_from_scalars<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<HashMap<String, String>>, D::Error> {
    let raw = match Option::<HashMap<String, Value>>::deserialize(deserializer)? {
        None => return Ok(None),
        Some(raw) => raw,
    };
    raw.into_iter()
        .map(|(k, v)| {
            scalar_to_string(&v)
                .map(|s| (k.clone(), s))
                .ok_or_else(|| serde::de::Error::custom(format!("option '{k}' is not a scalar")))
        })
        .collect::<Result<HashMap<_, _>, _>>()
        .map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct Holder {
        #[serde(default, deserialize_with = "optional_string_from_scalar")]
        start_lines: Option<String>,
        #[serde(default, deserialize_with = "optional_string_map_from_scalars")]
        options: Option<HashMap<String, String>>,
    }

    #[test]
    fn coerces_int_start_lines_and_scalar_options() {
        let h: Holder =
            serde_yaml::from_str("start_lines: 22\noptions:\n  legacy: false\n  depth: 3").unwrap();
        assert_eq!(h.start_lines.as_deref(), Some("22"));
        let options = h.options.unwrap();
        assert_eq!(options["legacy"], "false");
        assert_eq!(options["depth"], "3");
    }

    #[test]
    fn empty_and_absent_start_lines_are_none() {
        let h: Holder = serde_yaml::from_str("start_lines: ''").unwrap();
        assert!(h.start_lines.is_none());
        let h: Holder = serde_yaml::from_str("{}").unwrap();
        assert!(h.start_lines.is_none() && h.options.is_none());
    }

    #[test]
    fn rejects_non_scalar_values() {
        assert!(serde_yaml::from_str::<Holder>("start_lines: [1, 2]").is_err());
        assert!(serde_yaml::from_str::<Holder>("options:\n  a: [1]").is_err());
    }
}
