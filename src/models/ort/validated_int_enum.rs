// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! The Rust counterpart of python-ort's `ValidatedIntEnum` (`src/ort/utils/validated_enum.py`).
//!
//! python-ort's int-backed enums accept *either* the numeric value or the case-sensitive member
//! name when parsing, and always serialize back as the member **name**. The
//! `validated_int_enum!` macro generates `Serialize`, `Deserialize`, `Display` and
//! [`Model`](crate::models::Model) impls with exactly those semantics, so YAML/JSON written by
//! either implementation round-trips through the other.

/// Implements python-ort's `ValidatedIntEnum` semantics for an existing `#[repr(u8)]`-style enum.
///
/// ```ignore
/// validated_int_enum!(Severity { Hint = 1 => "HINT", Warning = 2 => "WARNING" });
/// ```
macro_rules! validated_int_enum {
    ($ty:ident { $($variant:ident = $num:expr => $name:literal),+ $(,)? }) => {
        impl serde::Serialize for $ty {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(match self {
                    $($ty::$variant => $name),+
                })
            }
        }

        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                match serde_yaml::Value::deserialize(deserializer)? {
                    serde_yaml::Value::String(s) => match s.as_str() {
                        $($name => Ok($ty::$variant),)+
                        other => Err(serde::de::Error::custom(format!(
                            concat!("invalid ", stringify!($ty), " value: {}"),
                            other
                        ))),
                    },
                    serde_yaml::Value::Number(n) => match n.as_u64() {
                        $(Some($num) => Ok($ty::$variant),)+
                        _ => Err(serde::de::Error::custom(format!(
                            concat!("invalid ", stringify!($ty), " value: {}"),
                            n
                        ))),
                    },
                    other => Err(serde::de::Error::custom(format!(
                        concat!("invalid ", stringify!($ty), " value: {:?}"),
                        other
                    ))),
                }
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    $($ty::$variant => $name),+
                })
            }
        }

        impl crate::models::Model for $ty {
            fn validate(&self) -> Result<(), crate::models::ValidationError> {
                Ok(())
            }
        }
    };
}

pub(crate) use validated_int_enum;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::models::ort::severity::Severity;

    #[derive(Serialize, Deserialize)]
    struct Holder {
        level: Severity,
    }

    #[test]
    fn accepts_member_name() {
        let h: Holder = serde_yaml::from_str("level: WARNING").unwrap();
        assert_eq!(h.level, Severity::Warning);
    }

    #[test]
    fn accepts_numeric_value() {
        let h: Holder = serde_yaml::from_str("level: 3").unwrap();
        assert_eq!(h.level, Severity::Error);
    }

    #[test]
    fn is_case_sensitive() {
        assert!(serde_yaml::from_str::<Holder>("level: warning").is_err());
    }

    #[test]
    fn rejects_unknown_name_and_value() {
        assert!(serde_yaml::from_str::<Holder>("level: NOPE").is_err());
        assert!(serde_yaml::from_str::<Holder>("level: 42").is_err());
    }

    #[test]
    fn serializes_as_member_name() {
        let json = serde_json::to_string(&Holder {
            level: Severity::Hint,
        })
        .unwrap();
        assert_eq!(json, r#"{"level":"HINT"}"#);
    }
}
