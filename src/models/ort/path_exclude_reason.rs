// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// Possible reasons for excluding a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PathExcludeReason {
    /// Contains only build tools not included in distributed build artifacts.
    BuildToolOf = 1,
    /// Contains only data files such as fonts or images.
    DataFileOf = 2,
    /// Contains only documentation.
    DocumentationOf = 3,
    /// Contains only source code examples.
    ExampleOf = 4,
    /// Contains only optional components for the code that is built.
    OptionalComponentOf = 5,
    /// Any other reason.
    Other = 6,
    /// Contains only packages that must be provided by the user.
    ProvidedBy = 7,
    /// Contains only files used for testing.
    TestOf = 8,
    /// Contains only tools used for testing.
    TestToolOf = 9,
}

crate::models::ort::validated_int_enum::validated_int_enum!(PathExcludeReason {
    BuildToolOf = 1 => "BUILD_TOOL_OF",
    DataFileOf = 2 => "DATA_FILE_OF",
    DocumentationOf = 3 => "DOCUMENTATION_OF",
    ExampleOf = 4 => "EXAMPLE_OF",
    OptionalComponentOf = 5 => "OPTIONAL_COMPONENT_OF",
    Other = 6 => "OTHER",
    ProvidedBy = 7 => "PROVIDED_BY",
    TestOf = 8 => "TEST_OF",
    TestToolOf = 9 => "TEST_TOOL_OF",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(PathExcludeReason::TestToolOf.to_string(), "TEST_TOOL_OF");
    }
}
