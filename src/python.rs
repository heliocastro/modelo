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

use crate::models::license_classifications::LicenseClassifications as RustLicenseClassifications;
use crate::models::ort_result::OrtResult as RustOrtResult;
use crate::models::repository_configuration::RepositoryConfiguration as RustRepositoryConfiguration;
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
        #[pyclass]
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

/// Python module `vale`, matching the `python-ort` import path (`from vale import OrtResult`).
#[pymodule]
fn vale(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LicenseClassifications>()?;
    m.add_class::<RepositoryConfiguration>()?;
    m.add_class::<OrtResult>()?;
    Ok(())
}
