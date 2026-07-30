// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// A generic severity, e.g. of issues, sorted from least severe to most severe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Severity {
    /// A hint is something that is provided for information only.
    Hint = 1,
    /// A warning is something that should be addressed.
    Warning = 2,
    /// An error is something that has to be addressed.
    Error = 3,
}

crate::models::ort::validated_int_enum::validated_int_enum!(Severity {
    Hint = 1 => "HINT",
    Warning = 2 => "WARNING",
    Error = 3 => "ERROR",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_by_severity() {
        assert!(Severity::Hint < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
    }

    #[test]
    fn displays_name() {
        assert_eq!(Severity::Error.to_string(), "ERROR");
    }
}
