// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Python bindings for the ORT model.
//!
//! The three entry points (`LicenseClassifications`, `RepositoryConfiguration`, `OrtResult`) parse
//! and validate YAML, and expose the parsed tree as ordinary Python objects: every nested Rust
//! struct becomes an instance of a class named after it, reachable with attribute access, the way
//! python-ort's pydantic models behave.
//!
//! ```python
//! result = OrtResult.from_yaml_file("evaluation-result.yml")
//! pprint(result.analyzer)
//! result.analyzer.result.projects[0].id.name
//! ```
//!
//! The object tree is produced by [`serializer::to_pyobject`], driven by the same `Serialize`
//! implementations as `to_json`/`to_yaml`, so the two views never drift apart and no per-type binding code
//! has to be maintained.
//!
//! # Exported classes
//!
//! `modelo.ort` exports every one of the model structs below (registered eagerly at import time
//! by the internal `register_known_classes` helper -- see `KNOWN_ORT_MODELS` in this file), plus
//! `Object`, the common base class ([`object::ModeloObject`]) all of them inherit from. All are
//! importable as soon as `modelo` is, regardless of whether a file has been parsed yet:
//!
//! ```python
//! from modelo.ort import AdvisorResult, Identifier, Vulnerability
//! ```
//!
//! ## Entry points
//!
//! Parsed and validated directly from YAML; the only classes with `from_yaml_str` /
//! `from_yaml_file`. Every model, including nested ones, has `to_json` / `to_yaml` (serializing
//! just the subtree rooted at that object).
//!
//! - `OrtResult` ([`modelo::models::ort::ort_result::OrtResult`]) -- a full ORT result file.
//! - `RepositoryConfiguration` ([`modelo::models::ort::repository_configuration::RepositoryConfiguration`])
//!   -- a `.ort.yml`.
//! - `LicenseClassifications` ([`modelo::models::ort::license_classifications::LicenseClassifications`])
//!   -- a `license-classifications.yml`.
//!
//! ```python
//! from modelo.ort import LicenseClassifications, OrtResult, RepositoryConfiguration
//!
//! result = OrtResult.from_yaml_file("evaluation-result.yml")
//! repo_config = RepositoryConfiguration.from_yaml_file(".ort.yml")
//! classifications = LicenseClassifications.from_yaml_file("license-classifications.yml")
//! ```
//!
//! ```rust,ignore
//! // Equivalent, using the `modelo` crate directly (no Python involved).
//! use modelo::models::Model;
//! use modelo::models::ort::ort_result::OrtResult;
//!
//! let yaml = std::fs::read_to_string("evaluation-result.yml")?;
//! let result: OrtResult = serde_yaml::from_str(&yaml)?;
//! result.validate()?;
//! ```
//!
//! ## Analyzer
//!
//! - `AnalyzerRun` ([`modelo::models::ort::analyzer_run::AnalyzerRun`])
//! - `AnalyzerResult` ([`modelo::models::ort::analyzer_result::AnalyzerResult`])
//! - `AnalyzerConfiguration` ([`modelo::models::ort::analyzer_configuration::AnalyzerConfiguration`])
//! - `DependencyGraph` ([`modelo::models::ort::dependency_graph::DependencyGraph`])
//! - `DependencyGraphNode` ([`modelo::models::ort::dependency_graph_node::DependencyGraphNode`])
//! - `DependencyGraphEdge` ([`modelo::models::ort::dependency_graph_edge::DependencyGraphEdge`])
//! - `DependencyReference` ([`modelo::models::ort::dependency_reference::DependencyReference`])
//! - `RootDependencyIndex` ([`modelo::models::ort::root_dependency_index::RootDependencyIndex`])
//! - `Package` ([`modelo::models::ort::package::Package`])
//! - `Project` ([`modelo::models::ort::project::Project`])
//! - `PackageReference` ([`modelo::models::ort::package_reference::PackageReference`])
//! - `Scope` ([`modelo::models::ort::scope::Scope`])
//! - `Identifier` ([`modelo::models::ort::identifier::Identifier`])
//! - `RemoteArtifact` ([`modelo::models::ort::remote_artifact::RemoteArtifact`])
//! - `Hash` ([`modelo::models::ort::hash::Hash`])
//! - `ProcessedDeclaredLicense` ([`modelo::models::ort::processed_declared_license::ProcessedDeclaredLicense`])
//! - `Environment` ([`modelo::models::ort::environment::Environment`])
//! - `BaseRun` ([`modelo::models::ort::base_run::BaseRun`])
//!
//! ```python
//! for project in result.analyzer.result.projects:
//!     print(project.id.name, project.id.version)
//! for pkg in result.analyzer.result.packages:
//!     print(pkg.id, pkg.declared_licenses_processed.spdx_expression)
//! ```
//!
//! ```rust,ignore
//! use modelo::models::ort::project::Project;
//! use modelo::models::ort::identifier::Identifier;
//!
//! fn describe(project: &Project) -> String {
//!     let Identifier { name, version, .. } = &project.id;
//!     format!("{name}@{version}")
//! }
//! ```
//!
//! ## Advisor
//!
//! - `AdvisorRun` ([`modelo::models::ort::advisor_run::AdvisorRun`])
//! - `AdvisorResult` ([`modelo::models::ort::advisor_result::AdvisorResult`])
//! - `AdvisorDetails` ([`modelo::models::ort::advisor_details::AdvisorDetails`])
//! - `AdvisorSummary` ([`modelo::models::ort::advisor_summary::AdvisorSummary`])
//! - `AdvisorConfiguration` ([`modelo::models::ort::advisor_configuration::AdvisorConfiguration`])
//! - `Defect` ([`modelo::models::ort::defect::Defect`])
//! - `Vulnerability` ([`modelo::models::ort::vulnerability::Vulnerability`])
//! - `VulnerabilityReference` ([`modelo::models::ort::vulnerability_reference::VulnerabilityReference`])
//! - `Issue` ([`modelo::models::ort::issue::Issue`])
//!
//! ```python
//! for package_id, results in result.advisor.results.items():
//!     for advisor_result in results:
//!         print(advisor_result.advisor.name)
//!         for vulnerability in advisor_result.vulnerabilities:
//!             print(" ", vulnerability.id, [r.severity for r in vulnerability.references])
//! ```
//!
//! ```rust,ignore
//! use modelo::models::ort::advisor_result::AdvisorResult;
//!
//! fn highest_score(result: &AdvisorResult) -> Option<f64> {
//!     result
//!         .vulnerabilities
//!         .iter()
//!         .flat_map(|v| v.references.iter())
//!         .filter_map(|r| r.score)
//!         .fold(None, |max, score| Some(max.map_or(score, |m: f64| m.max(score))))
//! }
//! ```
//!
//! ## Scanner
//!
//! - `ScannerRun` ([`modelo::models::ort::scanner_run::ScannerRun`])
//! - `ScanResult` ([`modelo::models::ort::scan_result::ScanResult`])
//! - `ScanSummary` ([`modelo::models::ort::scan_summary::ScanSummary`])
//! - `ScannerConfiguration` ([`modelo::models::ort::scanner_configuration::ScannerConfiguration`])
//! - `ScannerDetails` ([`modelo::models::ort::scanner_details::ScannerDetails`])
//! - `LicenseFinding` ([`modelo::models::ort::license_finding::LicenseFinding`])
//! - `LicenseFindingCuration` ([`modelo::models::ort::license_finding_curation::LicenseFindingCuration`])
//! - `CopyrightFinding` ([`modelo::models::ort::copyright_finding::CopyrightFinding`])
//! - `TextLocation` ([`modelo::models::ort::text_location::TextLocation`])
//! - `FileList` ([`modelo::models::ort::file_list::FileList`])
//! - `FileListEntry` ([`modelo::models::ort::file_list::FileListEntry`])
//! - `Snippet` ([`modelo::models::ort::snippet::Snippet`])
//! - `SnippetFinding` ([`modelo::models::ort::snippet_finding::SnippetFinding`])
//! - `SnippetChoice` ([`modelo::models::ort::snippet_choice::SnippetChoice`])
//! - `SnippetChoiceGiven` ([`modelo::models::ort::snippet_choice::SnippetChoiceGiven`])
//! - `SnippetChoiceCriteria` ([`modelo::models::ort::snippet_choice::SnippetChoiceCriteria`])
//! - `SnippetChoices` ([`modelo::models::ort::snippet_choices::SnippetChoices`])
//! - `SnippetProvenance` ([`modelo::models::ort::snippet_provenance::SnippetProvenance`])
//!
//! ```python
//! for scan_result in result.scanner.scan_results:
//!     for finding in scan_result.summary.license_findings:
//!         print(finding.license, finding.location.path, finding.score)
//! ```
//!
//! ```rust,ignore
//! use modelo::models::ort::scan_summary::ScanSummary;
//!
//! fn licenses(summary: &ScanSummary) -> Vec<String> {
//!     summary
//!         .license_findings
//!         .iter()
//!         .map(|f| f.license.clone())
//!         .collect()
//! }
//! ```
//!
//! ## Provenance & VCS
//!
//! - `ArtifactProvenance` ([`modelo::models::ort::provenance::ArtifactProvenance`])
//! - `RepositoryProvenance` ([`modelo::models::ort::provenance::RepositoryProvenance`])
//! - `ProvenanceResolutionResult` ([`modelo::models::ort::provenance_resolution_result::ProvenanceResolutionResult`])
//! - `VcsInfo` ([`modelo::models::ort::vcs_info::VcsInfo`])
//! - `VcsInfoCurationData` ([`modelo::models::ort::vcsinfo_curation_data::VcsInfoCurationData`])
//! - `VcsType` ([`modelo::models::ort::vcs_type::VcsType`])
//! - `VcsMatcher` ([`modelo::models::ort::vcsmatcher::VcsMatcher`])
//!
//! ```python
//! vcs = result.repository.vcs_processed
//! print(vcs.type, vcs.url, vcs.revision)
//! ```
//!
//! ```rust,ignore
//! use modelo::models::ort::vcs_info::VcsInfo;
//!
//! fn clone_url(vcs: &VcsInfo) -> &str {
//!     &vcs.url
//! }
//! ```
//!
//! ## Repository configuration (`.ort.yml`)
//!
//! - `Repository` ([`modelo::models::ort::repository::Repository`])
//! - `RepositoryAnalyzerConfiguration` ([`modelo::models::ort::repository_analyzer_configuration::RepositoryAnalyzerConfiguration`])
//! - `Curations` ([`modelo::models::ort::curations::Curations`])
//! - `PackageCuration` ([`modelo::models::ort::package_curation::PackageCuration`])
//! - `PackageCurationData` ([`modelo::models::ort::package_curation_data::PackageCurationData`])
//! - `PackageConfiguration` ([`modelo::models::ort::package_configuration::PackageConfiguration`])
//! - `PackageManagerConfiguration` ([`modelo::models::ort::package_manager_configuration::PackageManagerConfiguration`])
//! - `Excludes` ([`modelo::models::ort::excludes::Excludes`])
//! - `Includes` ([`modelo::models::ort::includes::Includes`])
//! - `PathExclude` ([`modelo::models::ort::path_exclude::PathExclude`])
//! - `PathInclude` ([`modelo::models::ort::path_include::PathInclude`])
//! - `ScopeExclude` ([`modelo::models::ort::scope_exclude::ScopeExclude`])
//! - `Resolutions` ([`modelo::models::ort::resolutions::Resolutions`])
//! - `IssueResolution` ([`modelo::models::ort::issue_resolution::IssueResolution`])
//! - `RuleViolationResolution` ([`modelo::models::ort::rule_violation_resolution::RuleViolationResolution`])
//! - `VulnerabilityResolution` ([`modelo::models::ort::vulnerability_resolution::VulnerabilityResolution`])
//! - `LicenseChoices` ([`modelo::models::ort::license_choices::LicenseChoices`])
//! - `PackageLicenseChoice` ([`modelo::models::ort::license_choices::PackageLicenseChoice`])
//! - `SpdxLicenseChoice` ([`modelo::models::ort::license_choices::SpdxLicenseChoice`])
//! - `LicenseCategorization` ([`modelo::models::ort::license_categorization::LicenseCategorization`])
//! - `LicenseCategory` ([`modelo::models::ort::license_category::LicenseCategory`])
//! - `RuleViolation` ([`modelo::models::ort::rule_violation::RuleViolation`])
//!
//! ```python
//! for curation in repo_config.curations.packages:
//!     print(curation.id, curation.curations.comment)
//! ```
//!
//! ```rust,ignore
//! use modelo::models::ort::repository_configuration::RepositoryConfiguration;
//!
//! fn excluded_paths(config: &RepositoryConfiguration) -> Vec<&str> {
//!     config
//!         .excludes
//!         .as_ref()
//!         .map(|e| e.paths.iter().map(|p| p.pattern.as_str()).collect())
//!         .unwrap_or_default()
//! }
//! ```
//!
//! ## Storage configuration
//!
//! - `FileArchiverConfiguration` ([`modelo::models::ort::file_archiver_configuration::FileArchiverConfiguration`])
//! - `FileStorageConfiguration` ([`modelo::models::ort::file_storage_configuration::FileStorageConfiguration`])
//! - `LocalFileStorageConfiguration` ([`modelo::models::ort::local_file_storage_configuration::LocalFileStorageConfiguration`])
//! - `HttpFileStorageConfiguration` ([`modelo::models::ort::http_file_storage_configuration::HttpFileStorageConfiguration`])
//! - `S3FileStorageConfiguration` ([`modelo::models::ort::s3_file_storage_configuration::S3FileStorageConfiguration`])
//! - `FileListStorageConfiguration` ([`modelo::models::ort::file_list_storage_configuration::FileListStorageConfiguration`])
//! - `ScanStorageConfiguration` ([`modelo::models::ort::scan_storage_configuration::ScanStorageConfiguration`])
//! - `ClearlyDefinedStorageConfiguration` ([`modelo::models::ort::scan_storage_configuration::ClearlyDefinedStorageConfiguration`])
//! - `FileBasedStorageConfiguration` ([`modelo::models::ort::scan_storage_configuration::FileBasedStorageConfiguration`])
//! - `PostgresStorageConfiguration` ([`modelo::models::ort::scan_storage_configuration::PostgresStorageConfiguration`])
//! - `PostgresConnection` ([`modelo::models::ort::postgres_connection::PostgresConnection`])
//! - `ProvenanceStorageConfiguration` ([`modelo::models::ort::provenance_storage_configuration::ProvenanceStorageConfiguration`])
//!
//! ```python
//! print(result.scanner.config.archive.file_storage.local_file_storage.directory)
//! ```
//!
//! ```rust,ignore
//! use modelo::models::ort::local_file_storage_configuration::LocalFileStorageConfiguration;
//!
//! fn storage_dir(cfg: &LocalFileStorageConfiguration) -> &str {
//!     &cfg.directory
//! }
//! ```
//!
//! ## Evaluator & resolutions
//!
//! - `EvaluatorRun` ([`modelo::models::ort::evaluator_run::EvaluatorRun`])
//! - `OrtResolutions` ([`modelo::models::ort::ort_resolutions::OrtResolutions`])
//! - `OrtResolutionIssue` ([`modelo::models::ort::ort_resolutions::OrtResolutionIssue`])
//! - `OrtResolutionRuleViolation` ([`modelo::models::ort::ort_resolutions::OrtResolutionRuleViolation`])
//! - `OrtResolutionVulnerability` ([`modelo::models::ort::ort_resolutions::OrtResolutionVulnerability`])
//!
//! ```python
//! for violation in result.evaluator.violations:
//!     print(violation.rule, violation.severity)
//! ```
//!
//! ```rust,ignore
//! use modelo::models::ort::evaluator_run::EvaluatorRun;
//!
//! fn violation_count(run: &EvaluatorRun) -> usize {
//!     run.violations.len()
//! }
//! ```

pub mod object;
pub mod serializer;

use std::fs;
use std::sync::OnceLock;

use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Serialize, de::DeserializeOwned};

use modelo::models::Model;
use modelo::models::ort::license_classifications::LicenseClassifications as RustLicenseClassifications;
use modelo::models::ort::ort_result::OrtResult as RustOrtResult;
use modelo::models::ort::repository_configuration::RepositoryConfiguration as RustRepositoryConfiguration;

use crate::object::ModeloObject;
use crate::serializer::to_pyobject;

/// Parses `s` as YAML into `T` and validates it, mapping any failure to a `PyValueError`
/// (mirroring python-ort's pattern of surfacing pydantic `ValidationError`s to callers).
fn parse_and_validate<T: Model + DeserializeOwned>(s: &str) -> PyResult<T> {
    let value: T = serde_yaml::from_str(s).map_err(|e| PyValueError::new_err(format!("{e}")))?;
    value
        .validate()
        .map_err(|e| PyValueError::new_err(format!("{e}")))?;
    Ok(value)
}

/// Reads `path` and delegates to [`parse_and_validate`], mapping file I/O failure to `PyOSError`.
fn read_and_parse<T: Model + DeserializeOwned>(path: &str) -> PyResult<T> {
    let contents = fs::read_to_string(path).map_err(|e| PyOSError::new_err(format!("{e}")))?;
    parse_and_validate(&contents)
}

fn to_pretty_json<T: Serialize>(value: &T) -> PyResult<String> {
    serde_json::to_string_pretty(value).map_err(|e| PyValueError::new_err(format!("{e}")))
}

fn to_yaml<T: Serialize>(value: &T) -> PyResult<String> {
    serde_yaml::to_string(value).map_err(|e| PyValueError::new_err(format!("{e}")))
}

macro_rules! pymodel {
    ($py_name:ident, $rust_ty:ty) => {
        #[pyclass(module = "modelo.ort")]
        pub struct $py_name {
            pub inner: $rust_ty,
            // Built on first attribute access rather than at parse time, so callers that only
            // want `to_json` never pay for materialising the whole tree.
            node: OnceLock<Py<PyAny>>,
        }

        impl $py_name {
            fn new(inner: $rust_ty) -> Self {
                Self {
                    inner,
                    node: OnceLock::new(),
                }
            }

            /// The parsed model as a tree of Python objects, built once and cached.
            fn node<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, ModeloObject>> {
                let node = match self.node.get() {
                    Some(node) => node,
                    None => {
                        let node = to_pyobject(py, &self.inner)?;
                        self.node.get_or_init(|| node)
                    }
                };
                Ok(node.bind(py).cast::<ModeloObject>()?.clone())
            }
        }

        #[pymethods]
        impl $py_name {
            #[staticmethod]
            fn from_yaml_str(s: &str) -> PyResult<Self> {
                Ok(Self::new(parse_and_validate(s)?))
            }

            #[staticmethod]
            fn from_yaml_file(path: &str) -> PyResult<Self> {
                Ok(Self::new(read_and_parse(path)?))
            }

            fn to_json(&self) -> PyResult<String> {
                to_pretty_json(&self.inner)
            }

            fn to_yaml(&self) -> PyResult<String> {
                to_yaml(&self.inner)
            }

            /// The model as nested plain `dict`s and lists.
            fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
                self.node(py)?.borrow().to_dict(py)
            }

            fn keys(&self, py: Python<'_>) -> PyResult<Vec<String>> {
                Ok(self.node(py)?.borrow().keys())
            }

            fn items(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
                Ok(self.node(py)?.borrow().items(py)?.into())
            }

            fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
                self.node(py)?.borrow().__getattr__(py, name)
            }

            fn __getitem__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
                self.node(py)?.borrow().__getitem__(py, name)
            }

            fn __contains__(&self, py: Python<'_>, name: &str) -> PyResult<bool> {
                Ok(self.node(py)?.borrow().__contains__(name))
            }

            fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
                Ok(self.node(py)?.borrow().__len__())
            }

            fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
                self.node(py)?.borrow().__iter__(py)
            }

            fn __dir__(&self, py: Python<'_>) -> PyResult<Vec<String>> {
                let mut names: Vec<String> = [
                    "from_yaml_str",
                    "from_yaml_file",
                    "to_json",
                    "to_yaml",
                    "to_dict",
                    "keys",
                    "items",
                ]
                .iter()
                .map(|s| (*s).to_string())
                .collect();
                names.extend(self.node(py)?.borrow().keys());
                Ok(names)
            }

            fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
                self.node(py)?.borrow().__repr__(py)
            }
        }
    };
}

pymodel!(LicenseClassifications, RustLicenseClassifications);
pymodel!(RepositoryConfiguration, RustRepositoryConfiguration);
pymodel!(OrtResult, RustOrtResult);

/// Every model struct reachable from the three entry points, other than the entry points
/// themselves (which are `pyclass`es registered explicitly below). Kept in sync manually with
/// `src/models/ort/*.rs`'s `pub struct` declarations (enums are excluded: they serialize as
/// plain strings or delegate to a struct variant, never producing their own `ModeloObject`
/// class).
///
/// Registering these eagerly (rather than the first time a parse happens to produce one) means
/// `from modelo.ort import AdvisorResult` and friends work right after `import modelo`, matching
/// python-ort's pydantic models, which are all importable regardless of whether they were ever
/// instantiated.
const KNOWN_ORT_MODELS: &[&str] = &[
    "AdvisorConfiguration",
    "AdvisorDetails",
    "AdvisorResult",
    "AdvisorRun",
    "AdvisorSummary",
    "AnalyzerConfiguration",
    "AnalyzerResult",
    "AnalyzerRun",
    "ArtifactProvenance",
    "BaseRun",
    "ClearlyDefinedStorageConfiguration",
    "CopyrightFinding",
    "Curations",
    "Defect",
    "DependencyGraph",
    "DependencyGraphEdge",
    "DependencyGraphNode",
    "DependencyReference",
    "Environment",
    "EvaluatorRun",
    "Excludes",
    "FileArchiverConfiguration",
    "FileBasedStorageConfiguration",
    "FileList",
    "FileListEntry",
    "FileListStorageConfiguration",
    "FileStorageConfiguration",
    "Hash",
    "HttpFileStorageConfiguration",
    "Identifier",
    "Includes",
    "Issue",
    "IssueResolution",
    "LicenseCategorization",
    "LicenseCategory",
    "LicenseChoices",
    "LicenseFinding",
    "LicenseFindingCuration",
    "LocalFileStorageConfiguration",
    "OrtResolutionIssue",
    "OrtResolutionRuleViolation",
    "OrtResolutions",
    "OrtResolutionVulnerability",
    "Package",
    "PackageConfiguration",
    "PackageCuration",
    "PackageCurationData",
    "PackageLicenseChoice",
    "PackageManagerConfiguration",
    "PackageReference",
    "PathExclude",
    "PathInclude",
    "PostgresConnection",
    "PostgresStorageConfiguration",
    "ProcessedDeclaredLicense",
    "Project",
    "ProvenanceResolutionResult",
    "ProvenanceStorageConfiguration",
    "RemoteArtifact",
    "Repository",
    "RepositoryAnalyzerConfiguration",
    "RepositoryProvenance",
    "Resolutions",
    "RootDependencyIndex",
    "RuleViolation",
    "RuleViolationResolution",
    "S3FileStorageConfiguration",
    "ScanResult",
    "ScanStorageConfiguration",
    "ScanSummary",
    "ScannerConfiguration",
    "ScannerDetails",
    "ScannerRun",
    "Scope",
    "ScopeExclude",
    "Snippet",
    "SnippetChoice",
    "SnippetChoiceCriteria",
    "SnippetChoiceGiven",
    "SnippetChoices",
    "SnippetFinding",
    "SnippetProvenance",
    "SpdxLicenseChoice",
    "TextLocation",
    "VcsInfo",
    "VcsInfoCurationData",
    "VcsMatcher",
    "VcsType",
    "Vulnerability",
    "VulnerabilityReference",
    "VulnerabilityResolution",
];

/// Python extension module `modelo._modelo`. Each model family gets its own submodule, mirroring the
/// Rust `models::<family>` layout; the `modelo` Python package re-exports them, so
/// `from modelo.ort import OrtResult` is the ORT import path.
#[pymodule]
fn _modelo(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let ort = PyModule::new(py, "ort")?;
    ort.add_class::<ModeloObject>()?;
    ort.add_class::<LicenseClassifications>()?;
    ort.add_class::<RepositoryConfiguration>()?;
    ort.add_class::<OrtResult>()?;
    m.add_submodule(&ort)?;
    // `add_submodule` keys the parent attribute off the name given above, so the dotted name has
    // to be set afterwards for tracebacks and pickling to report `modelo.ort`, which is the module
    // the classes are re-exported from.
    ort.setattr("__name__", "modelo.ort")?;

    // `add_submodule` only sets the attribute on the parent; without a `sys.modules` entry,
    // `from modelo._modelo.ort import X` fails for a submodule defined in Rust.
    py.import("sys")?
        .getattr("modules")?
        .set_item("modelo._modelo.ort", &ort)?;

    // `class_for` (used by `register_known_classes`) publishes each class onto the "modelo.ort"
    // module looked up through `sys.modules`, so this only works once "modelo.ort" -- the Python
    // wrapper module, not this Rust submodule -- has itself started importing, which it has by
    // the time it reaches `from modelo._modelo.ort import ...`.
    object::register_known_classes(py, &ort, KNOWN_ORT_MODELS)?;
    Ok(())
}
