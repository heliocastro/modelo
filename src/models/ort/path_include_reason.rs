// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// Possible reasons for including a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PathIncludeReason {
    /// Contains source code used to build distributed build artifacts.
    SourceOf = 1,
    /// A fallback reason when none of the other reasons apply.
    Other = 2,
}

crate::models::ort::validated_int_enum::validated_int_enum!(PathIncludeReason {
    SourceOf = 1 => "SOURCE_OF",
    Other = 2 => "OTHER",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(PathIncludeReason::SourceOf.to_string(), "SOURCE_OF");
    }
}
