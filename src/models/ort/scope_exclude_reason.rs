// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// Possible reasons for excluding a scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ScopeExcludeReason {
    /// Contains only packages used for building source code.
    BuildToolOf = 1,
    /// Contains only packages used for building source code, not in distributed build.
    BuildDependencyOf = 2,
    /// Contains only packages used for development.
    DevDependencyOf = 3,
    /// Contains only packages used for building the documentation.
    DocumentationDependencyOf = 4,
    /// Contains only packages that have to be provided by the user.
    ProvidedBy = 5,
    /// Contains only packages that have to be provided by the user of distributed artifacts.
    ProvidedDependencyOf = 6,
    /// Contains only packages used for testing source code.
    TestToolOf = 7,
    /// Contains only packages used for testing, not in distributed build.
    TestDependencyOf = 8,
    /// Contains only packages provided by the user at runtime, not in distributed artifacts.
    RuntimeDependencyOf = 9,
}

crate::models::ort::validated_int_enum::validated_int_enum!(ScopeExcludeReason {
    BuildToolOf = 1 => "BUILD_TOOL_OF",
    BuildDependencyOf = 2 => "BUILD_DEPENDENCY_OF",
    DevDependencyOf = 3 => "DEV_DEPENDENCY_OF",
    DocumentationDependencyOf = 4 => "DOCUMENTATION_DEPENDENCY_OF",
    ProvidedBy = 5 => "PROVIDED_BY",
    ProvidedDependencyOf = 6 => "PROVIDED_DEPENDENCY_OF",
    TestToolOf = 7 => "TEST_TOOL_OF",
    TestDependencyOf = 8 => "TEST_DEPENDENCY_OF",
    RuntimeDependencyOf = 9 => "RUNTIME_DEPENDENCY_OF",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(
            ScopeExcludeReason::RuntimeDependencyOf.to_string(),
            "RUNTIME_DEPENDENCY_OF"
        );
    }
}
