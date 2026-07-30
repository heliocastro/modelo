// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// The source where a license originates from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LicenseSource {
    /// Licenses which are part of the concluded license of a package.
    Concluded = 1,
    /// Licenses which are part of the (processed) declared licenses of a package.
    Declared = 2,
    /// Licenses which were detected by a license scanner.
    Detected = 3,
}

crate::models::validated_int_enum::validated_int_enum!(LicenseSource {
    Concluded = 1 => "CONCLUDED",
    Declared = 2 => "DECLARED",
    Detected = 3 => "DETECTED",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(LicenseSource::Detected.to_string(), "DETECTED");
    }
}
