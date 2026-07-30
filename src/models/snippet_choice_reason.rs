// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// The reason for which a snippet choice has been made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SnippetChoiceReason {
    /// No relevant finding has been found for the corresponding source file.
    NoRelevantFinding = 1,
    /// One snippet finding is relevant; all other snippets are ignored.
    OriginalFinding = 2,
    /// A fallback reason when none of the other reasons apply.
    Other = 3,
}

crate::models::validated_int_enum::validated_int_enum!(SnippetChoiceReason {
    NoRelevantFinding = 1 => "NO_RELEVANT_FINDING",
    OriginalFinding = 2 => "ORIGINAL_FINDING",
    Other = 3 => "OTHER",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(
            SnippetChoiceReason::OriginalFinding.to_string(),
            "ORIGINAL_FINDING"
        );
    }
}
