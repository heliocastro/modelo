// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};
use serde::de::DeserializeOwned;

use crate::models::ort::license_classifications::LicenseClassifications;
use crate::models::ort::ort_result::OrtResult;
use crate::models::ort::repository_configuration::RepositoryConfiguration;
use crate::models::Model;

#[derive(Parser, Debug)]
#[command(author, version, about = "Validate python-ort YAML data files")]
pub struct Args {
    /// Enable debug logging (pretty-prints the parsed model on success).
    #[arg(short, long, global = true)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Validate a license-classifications YAML file.
    LicenseClassifications { datafile: PathBuf },
    /// Validate a repository-configuration (.ort.yml) YAML file.
    RepositoryConfiguration { datafile: PathBuf },
    /// Validate an ort-result YAML file.
    OrtResult { datafile: PathBuf },
    /// Launch the interactive TUI for model inspection/validation.
    Tui {
        /// Optional file path to pre-load when the TUI starts.
        datafile: Option<PathBuf>,
    },
}

pub fn run() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Some(Command::LicenseClassifications { datafile }) => {
            validate_file::<LicenseClassifications>(
                "license-classifications",
                &datafile,
                args.debug,
            )
        }
        Some(Command::RepositoryConfiguration { datafile }) => {
            validate_file::<RepositoryConfiguration>(
                "repository-configuration",
                &datafile,
                args.debug,
            )
        }
        Some(Command::OrtResult { datafile }) => {
            validate_file::<OrtResult>("ort-result", &datafile, args.debug)
        }
        Some(Command::Tui { datafile }) => crate::tui::run(datafile),
        None => crate::tui::run(None),
    }
}

/// Reads `path` and delegates to [`validate_str`].
fn validate_file<T>(kind: &str, path: &Path, debug: bool) -> anyhow::Result<()>
where
    T: Model + DeserializeOwned,
{
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    validate_str::<T>(kind, &contents, debug)
        .with_context(|| format!("{} is not a valid {kind} file", path.display()))
}

/// Deserializes `contents` as YAML into `T` and validates it, printing a success message
/// (with the parsed value pretty-printed when `debug` is set). Returns `Err` on parse or
/// validation failure, which the caller surfaces as a non-zero exit code.
fn validate_str<T>(kind: &str, contents: &str, debug: bool) -> anyhow::Result<()>
where
    T: Model + DeserializeOwned,
{
    let value: T =
        serde_yaml::from_str(contents).with_context(|| format!("failed to parse as {kind}"))?;
    value.validate()?;
    println!("valid {kind} file.");
    if debug {
        println!("{value:#?}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_license_classifications_yaml_passes() {
        let yaml = r#"
categories:
  - name: permissive
categorizations:
  - id: MIT
    categories: [permissive]
"#;
        assert!(
            validate_str::<LicenseClassifications>("license-classifications", yaml, false).is_ok()
        );
    }

    #[test]
    fn duplicate_category_name_fails_validation() {
        let yaml = r#"
categories:
  - name: permissive
  - name: permissive
categorizations: []
"#;
        assert!(
            validate_str::<LicenseClassifications>("license-classifications", yaml, false).is_err()
        );
    }

    #[test]
    fn malformed_yaml_fails_to_parse() {
        let yaml = "not: [valid: yaml";
        assert!(
            validate_str::<LicenseClassifications>("license-classifications", yaml, false).is_err()
        );
    }
}
