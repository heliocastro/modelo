// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// Possible reasons for resolving a rule violation using a
/// [`crate::models::ort::rule_violation_resolution::RuleViolationResolution`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RuleViolationResolutionReason {
    /// The rule violation cannot be fixed and is acceptable in this case.
    CantFixException = 1,
    /// Acceptable given that the dependency is dynamically linked.
    DynamicLinkageException = 2,
    /// Due to an inclusion of example code into a file, acceptable in this case.
    ExampleOfException = 3,
    /// Acceptable because the license for the respective package has been acquired.
    LicenseAcquiredException = 4,
    /// Acceptable given that the code it relates to has not been modified.
    NotModifiedException = 5,
    /// The implied patent grant is acceptable in this case.
    PatentGrantException = 6,
}

crate::models::ort::validated_int_enum::validated_int_enum!(RuleViolationResolutionReason {
    CantFixException = 1 => "CANT_FIX_EXCEPTION",
    DynamicLinkageException = 2 => "DYNAMIC_LINKAGE_EXCEPTION",
    ExampleOfException = 3 => "EXAMPLE_OF_EXCEPTION",
    LicenseAcquiredException = 4 => "LICENSE_ACQUIRED_EXCEPTION",
    NotModifiedException = 5 => "NOT_MODIFIED_EXCEPTION",
    PatentGrantException = 6 => "PATENT_GRANT_EXCEPTION",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(
            RuleViolationResolutionReason::CantFixException.to_string(),
            "CANT_FIX_EXCEPTION"
        );
    }
}
