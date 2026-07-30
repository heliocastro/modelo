// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::{Model, ValidationError};

/// The level of strictness to apply when validating an SPDX license expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SpdxExpressionStrictness {
    /// Any license identifier string is leniently allowed.
    AllowAny = 0,
    /// All SPDX license identifiers, including deprecated ones, and `LicenseRef`s are allowed.
    AllowDeprecated = 1,
    /// Only current (non-deprecated) SPDX license identifiers and `LicenseRef`s are allowed.
    AllowCurrent = 2,
    /// Same as `AllowCurrent`, but also allows `LicenseRef`s containing "exception" after `WITH`.
    AllowLicenseRefExceptions = 3,
}

impl Serialize for SpdxExpressionStrictness {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for SpdxExpressionStrictness {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(SpdxExpressionStrictness::AllowAny),
            1 => Ok(SpdxExpressionStrictness::AllowDeprecated),
            2 => Ok(SpdxExpressionStrictness::AllowCurrent),
            3 => Ok(SpdxExpressionStrictness::AllowLicenseRefExceptions),
            other => Err(serde::de::Error::custom(format!(
                "invalid SpdxExpressionStrictness value: {other}"
            ))),
        }
    }
}

impl fmt::Display for SpdxExpressionStrictness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpdxExpressionStrictness::AllowAny => write!(f, "ALLOW_ANY"),
            SpdxExpressionStrictness::AllowDeprecated => write!(f, "ALLOW_DEPRECATED"),
            SpdxExpressionStrictness::AllowCurrent => write!(f, "ALLOW_CURRENT"),
            SpdxExpressionStrictness::AllowLicenseRefExceptions => {
                write!(f, "ALLOW_LICENSEREF_EXCEPTIONS")
            }
        }
    }
}

impl Model for SpdxExpressionStrictness {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(
            SpdxExpressionStrictness::AllowCurrent.to_string(),
            "ALLOW_CURRENT"
        );
    }
}
