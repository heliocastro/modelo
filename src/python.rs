// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

// ponytail: data-interchange bindings for the 3 top-level models (LicenseClassifications,
// RepositoryConfiguration, OrtResult) only — from_yaml_str/from_yaml_file/to_json/__repr__.
// Full per-field attribute parity across the ~100 nested classes is explicitly out of scope
// here; that's a `ponytail` call, not an oversight. Expand incrementally, class-by-class,
// using this file's pattern, once a concrete consumer needs direct Python attribute access
// instead of `json.loads(obj.to_json())`.

use std::fs;

use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

use crate::models::ort::license_classifications::LicenseClassifications as RustLicenseClassifications;
use crate::models::ort::ort_result::OrtResult as RustOrtResult;
use crate::models::ort::repository_configuration::RepositoryConfiguration as RustRepositoryConfiguration;
use crate::models::Model;

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
        #[pyclass(module = "vale.ort")]
        #[derive(Clone)]
        pub struct $py_name(pub $rust_ty);

        #[pymethods]
        impl $py_name {
            #[staticmethod]
            fn from_yaml_str(s: &str) -> PyResult<Self> {
                Ok(Self(parse_and_validate(s)?))
            }

            #[staticmethod]
            fn from_yaml_file(path: &str) -> PyResult<Self> {
                Ok(Self(read_and_parse(path)?))
            }

            fn to_json(&self) -> PyResult<String> {
                to_pretty_json(&self.0)
            }

            fn __repr__(&self) -> String {
                format!("{}", self.0)
            }
        }
    };
}

pymodel!(LicenseClassifications, RustLicenseClassifications);
pymodel!(RepositoryConfiguration, RustRepositoryConfiguration);
pymodel!(OrtResult, RustOrtResult);

/// Python module `vale`. Each model family gets its own submodule, mirroring the Rust
/// `models::<family>` layout, so `from vale.ort import OrtResult` is the ORT import path.
#[pymodule]
fn vale(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let ort = PyModule::new_bound(py, "ort")?;
    ort.add_class::<LicenseClassifications>()?;
    ort.add_class::<RepositoryConfiguration>()?;
    ort.add_class::<OrtResult>()?;
    m.add_submodule(&ort)?;
    // `add_submodule` keys the parent attribute off the name given above, so the dotted name has
    // to be set afterwards for tracebacks and pickling to report `vale.ort`.
    ort.setattr("__name__", "vale.ort")?;

    // `add_submodule` only sets the attribute on the parent; without a `sys.modules` entry,
    // `import vale.ort` and `from vale.ort import X` fail for a submodule defined in Rust.
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("vale.ort", &ort)?;
    Ok(())
}
