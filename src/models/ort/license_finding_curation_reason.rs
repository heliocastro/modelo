// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// Possible reasons for a license finding curation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LicenseFindingCurationReason {
    /// The finding occurs in source code, e.g. the name of a variable.
    Code = 1,
    /// The finding occurs in data, e.g. a JSON object defining all SPDX licenses.
    DataOf = 2,
    /// The finding occurs in documentation, e.g. code comments or README.md.
    DocumentationOf = 3,
    /// The detected license is not correct.
    Incorrect = 4,
    /// The applicable license was not detected by the scanner.
    NotDetected = 5,
    /// The finding references a file or URL, e.g. `SEE LICENSE IN LICENSE`.
    Reference = 6,
}

crate::models::ort::validated_int_enum::validated_int_enum!(LicenseFindingCurationReason {
    Code = 1 => "CODE",
    DataOf = 2 => "DATA_OF",
    DocumentationOf = 3 => "DOCUMENTATION_OF",
    Incorrect = 4 => "INCORRECT",
    NotDetected = 5 => "NOT_DETECTED",
    Reference = 6 => "REFERENCE",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(
            LicenseFindingCurationReason::NotDetected.to_string(),
            "NOT_DETECTED"
        );
    }
}
