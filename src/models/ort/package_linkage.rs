// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// The linkage type between a package and the dependent package that refers to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PackageLinkage {
    /// A dynamically linked package retrieved as an external artifact.
    Dynamic = 1,
    /// A statically linked package retrieved as an external artifact.
    Static = 2,
    /// A dynamically linked package whose source is part of the project itself.
    ProjectDynamic = 3,
    /// A statically linked package whose source is part of the project itself.
    ProjectStatic = 4,
}

crate::models::ort::validated_int_enum::validated_int_enum!(PackageLinkage {
    Dynamic = 1 => "DYNAMIC",
    Static = 2 => "STATIC",
    ProjectDynamic = 3 => "PROJECT_DYNAMIC",
    ProjectStatic = 4 => "PROJECT_STATIC",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(PackageLinkage::ProjectStatic.to_string(), "PROJECT_STATIC");
    }
}
