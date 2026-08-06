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
//! implementations as `to_json`, so the two views never drift apart and no per-type binding code
//! has to be maintained.

pub mod object;
pub mod serializer;

use std::fs;
use std::sync::OnceLock;

use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Serialize, de::DeserializeOwned};

use crate::models::Model;
use crate::models::ort::license_classifications::LicenseClassifications as RustLicenseClassifications;
use crate::models::ort::ort_result::OrtResult as RustOrtResult;
use crate::models::ort::repository_configuration::RepositoryConfiguration as RustRepositoryConfiguration;
use crate::python::object::ModeloObject;
use crate::python::serializer::to_pyobject;

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
    Ok(())
}
