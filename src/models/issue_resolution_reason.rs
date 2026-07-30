// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// Possible reasons for resolving an issue using an [`crate::models::issue_resolution::IssueResolution`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IssueResolutionReason {
    /// The issue originates from the build tool used by the project.
    BuildToolIssue = 1,
    /// The issue can not be fixed.
    CantFixIssue = 2,
    /// The issue is due to an irrelevant scanner issue.
    ScannerIssue = 3,
}

crate::models::validated_int_enum::validated_int_enum!(IssueResolutionReason {
    BuildToolIssue = 1 => "BUILD_TOOL_ISSUE",
    CantFixIssue = 2 => "CANT_FIX_ISSUE",
    ScannerIssue = 3 => "SCANNER_ISSUE",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(
            IssueResolutionReason::CantFixIssue.to_string(),
            "CANT_FIX_ISSUE"
        );
    }
}
