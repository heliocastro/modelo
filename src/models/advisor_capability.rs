// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// The capabilities of a specific advisor implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AdvisorCapability {
    /// The advisor can report defects.
    Defects = 1,
    /// The advisor can report security vulnerabilities.
    Vulnerabilities = 2,
}

crate::models::validated_int_enum::validated_int_enum!(AdvisorCapability {
    Defects = 1 => "DEFECTS",
    Vulnerabilities = 2 => "VULNERABILITIES",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(AdvisorCapability::Defects.to_string(), "DEFECTS");
    }
}
