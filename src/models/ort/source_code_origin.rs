// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

/// The origin a package's source code was retrieved from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SourceCodeOrigin {
    /// The source code was retrieved from a version control system.
    Vcs = 1,
    /// The source code was retrieved from a source artifact.
    Artifact = 2,
}

crate::models::ort::validated_int_enum::validated_int_enum!(SourceCodeOrigin {
    Vcs = 1 => "VCS",
    Artifact = 2 => "ARTIFACT",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_name() {
        assert_eq!(SourceCodeOrigin::Vcs.to_string(), "VCS");
    }
}
