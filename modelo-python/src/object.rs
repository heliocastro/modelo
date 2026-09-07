// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! The generic model object exposed to Python.
//!
//! Every Rust model struct is materialised as an instance of a dynamically created subclass of
//! [`ModeloObject`] named after the struct, so `type(result.analyzer).__name__ == "AnalyzerRun"`
//! and fields are reached with plain attribute access, the way python-ort's pydantic models
//! behave. The subclasses are created once per interpreter and cached.

use pyo3::conversion::IntoPyObjectExt;
use pyo3::exceptions::{PyAttributeError, PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyDict, PyList, PyTuple, PyType};
use serde_json::Value as JsonValue;

/// A model instance: an ordered set of named fields plus the originating Rust type name.
#[pyclass(module = "modelo.ort", name = "Object", subclass)]
pub struct ModeloObject {
    type_name: String,
    keys: Vec<String>,
    values: Vec<Py<PyAny>>,
}

impl ModeloObject {
    fn get(&self, name: &str) -> Option<&Py<PyAny>> {
        self.keys
            .iter()
            .position(|k| k == name)
            .map(|i| &self.values[i])
    }

    fn to_json_value(&self, py: Python<'_>) -> PyResult<JsonValue> {
        let mut map = serde_json::Map::with_capacity(self.keys.len());
        for (key, value) in self.keys.iter().zip(&self.values) {
            map.insert(key.clone(), value_to_json(py, value.bind(py))?);
        }
        Ok(JsonValue::Object(map))
    }
}

#[pymethods]
impl ModeloObject {
    #[new]
    #[pyo3(signature = (type_name, fields = None))]
    fn new(type_name: String, fields: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        if let Some(fields) = fields {
            for (key, value) in fields.iter() {
                keys.push(key.extract::<String>()?);
                values.push(value.unbind());
            }
        }
        Ok(Self {
            type_name,
            keys,
            values,
        })
    }

    /// The name of the model this object was deserialized from.
    #[getter]
    pub fn __type__(&self) -> &str {
        &self.type_name
    }

    pub fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        match self.get(name) {
            Some(value) => Ok(value.clone_ref(py)),
            None => Err(PyAttributeError::new_err(format!(
                "'{}' object has no attribute '{name}'",
                self.type_name
            ))),
        }
    }

    pub fn __getitem__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        match self.get(name) {
            Some(value) => Ok(value.clone_ref(py)),
            None => Err(PyKeyError::new_err(name.to_string())),
        }
    }

    pub fn __contains__(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    pub fn __len__(&self) -> usize {
        self.keys.len()
    }

    /// Iterates the field names, like a `dict` — without this, Python falls back to the legacy
    /// sequence protocol and calls `__getitem__(0)`, which fails with a confusing `TypeError`.
    pub fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(PyList::new(py, &self.keys)?.as_any().try_iter()?.into())
    }

    pub fn __dir__(&self) -> Vec<String> {
        let mut names: Vec<String> = [
            "__type__", "keys", "values", "items", "to_dict", "to_json", "to_yaml",
        ]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
        names.extend(self.keys.iter().cloned());
        names
    }

    /// The field names, in the order they are declared on the model.
    pub fn keys(&self) -> Vec<String> {
        self.keys.clone()
    }

    pub fn values(&self, py: Python<'_>) -> Vec<Py<PyAny>> {
        self.values.iter().map(|v| v.clone_ref(py)).collect()
    }

    pub fn items<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let items = self
            .keys
            .iter()
            .zip(&self.values)
            .map(|(k, v)| -> PyResult<Bound<'py, PyTuple>> {
                PyTuple::new(py, [k.as_str().into_py_any(py)?, v.clone_ref(py)])
            })
            .collect::<PyResult<Vec<_>>>()?;
        PyList::new(py, items)
    }

    /// A plain, recursively converted `dict` — handy for `json.dumps` or `pprint`.
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (key, value) in self.keys.iter().zip(&self.values) {
            dict.set_item(key, to_plain(py, value.bind(py))?)?;
        }
        Ok(dict)
    }

    /// The subtree rooted at this object, serialized as pretty-printed JSON.
    pub fn to_json(&self, py: Python<'_>) -> PyResult<String> {
        serde_json::to_string_pretty(&self.to_json_value(py)?)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// The subtree rooted at this object, serialized as YAML.
    pub fn to_yaml(&self, py: Python<'_>) -> PyResult<String> {
        serde_yaml::to_string(&self.to_json_value(py)?)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn __eq__(&self, other: &Bound<'_, PyAny>) -> PyResult<bool> {
        let Ok(other) = other.cast::<Self>() else {
            return Ok(false);
        };
        let other = other.borrow();
        if self.type_name != other.type_name || self.keys != other.keys {
            return Ok(false);
        }
        let py = other.py();
        for (a, b) in self.values.iter().zip(&other.values) {
            if !a.bind(py).eq(b.bind(py))? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let mut out = String::from(&self.type_name);
        out.push('(');
        for (i, (key, value)) in self.keys.iter().zip(&self.values).enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(key);
            out.push('=');
            out.push_str(&value.bind(py).repr()?.to_string_lossy());
        }
        out.push(')');
        Ok(out)
    }
}

/// Recursively replaces [`ModeloObject`]s inside `value` with dicts, leaving everything else as is.
fn to_plain<'py>(py: Python<'py>, value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    if let Ok(obj) = value.cast::<ModeloObject>() {
        return Ok(obj.borrow().to_dict(py)?.into_any());
    }
    if let Ok(list) = value.cast::<PyList>() {
        let items = list
            .iter()
            .map(|item| to_plain(py, &item))
            .collect::<PyResult<Vec<_>>>()?;
        return Ok(PyList::new(py, items)?.into_any());
    }
    if let Ok(dict) = value.cast::<PyDict>() {
        let out = PyDict::new(py);
        for (key, item) in dict.iter() {
            out.set_item(key, to_plain(py, &item)?)?;
        }
        return Ok(out.into_any());
    }
    Ok(value.clone())
}

/// Recursively converts `value` into a [`JsonValue`], following the same shape [`to_plain`]
/// produces for `to_dict` — [`ModeloObject`]s become objects, lists stay lists, and scalars are
/// converted with their nearest JSON equivalent.
fn value_to_json(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<JsonValue> {
    if value.is_none() {
        return Ok(JsonValue::Null);
    }
    if let Ok(obj) = value.cast::<ModeloObject>() {
        return obj.borrow().to_json_value(py);
    }
    if let Ok(list) = value.cast::<PyList>() {
        let items = list
            .iter()
            .map(|item| value_to_json(py, &item))
            .collect::<PyResult<Vec<_>>>()?;
        return Ok(JsonValue::Array(items));
    }
    if let Ok(dict) = value.cast::<PyDict>() {
        let mut map = serde_json::Map::with_capacity(dict.len());
        for (key, item) in dict.iter() {
            let key: String = key.extract()?;
            map.insert(key, value_to_json(py, &item)?);
        }
        return Ok(JsonValue::Object(map));
    }
    if let Ok(b) = value.extract::<bool>() {
        return Ok(JsonValue::Bool(b));
    }
    if let Ok(i) = value.extract::<i64>() {
        return Ok(JsonValue::Number(i.into()));
    }
    if let Ok(f) = value.extract::<f64>() {
        return Ok(serde_json::Number::from_f64(f).map_or(JsonValue::Null, JsonValue::Number));
    }
    if let Ok(s) = value.extract::<String>() {
        return Ok(JsonValue::String(s));
    }
    // Fallback for any type not otherwise recognised (e.g. raw bytes): its `repr`.
    Ok(JsonValue::String(
        value.repr()?.to_string_lossy().into_owned(),
    ))
}

static CLASSES: PyOnceLock<Py<PyDict>> = PyOnceLock::new();

/// Returns (creating it on first use) the `ModeloObject` subclass named `name`.
fn class_for<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyType>> {
    let cache = CLASSES
        .get_or_try_init(py, || PyResult::Ok(PyDict::new(py).unbind()))?
        .bind(py);
    if let Some(class) = cache.get_item(name)? {
        return Ok(class.cast_into::<PyType>()?);
    }

    let namespace = PyDict::new(py);
    namespace.set_item("__module__", "modelo.ort")?;
    let bases = PyTuple::new(py, [py.get_type::<ModeloObject>()])?;
    let class = py
        .get_type::<PyType>()
        .call1((name, bases, namespace))?
        .cast_into::<PyType>()?;
    cache.set_item(name, &class)?;
    // The class claims to live in `modelo.ort`; publishing it there makes that true, so
    // `from modelo.ort import AnalyzerRun` and `pickle` resolve it like any other class.
    if let Ok(module) = py.import("modelo.ort") {
        module.setattr(name, &class)?;
    }
    Ok(class)
}

/// Builds an instance of the model class `name` holding `fields`.
pub(crate) fn make_object(
    py: Python<'_>,
    name: &str,
    fields: &Bound<'_, PyDict>,
) -> PyResult<Py<PyAny>> {
    Ok(class_for(py, name)?.call1((name, fields))?.unbind())
}

/// Eagerly creates and publishes the `ModeloObject` subclass for each name in `names`, so that
/// `from modelo.ort import <Model>` works for every model reachable from the object tree, not
/// just the ones that happen to have been produced by a prior parse. Also set directly on
/// `ort_module` (the Rust `modelo._modelo.ort` submodule), because `class_for` only publishes to
/// the higher-level `modelo.ort` Python module: `from modelo._modelo.ort import <Model>`, which
/// `modelo/ort.py` relies on, resolves attributes on `ort_module` itself.
pub(crate) fn register_known_classes(
    py: Python<'_>,
    ort_module: &Bound<'_, PyModule>,
    names: &[&str],
) -> PyResult<()> {
    for name in names {
        let class = class_for(py, name)?;
        ort_module.setattr(*name, class)?;
    }
    Ok(())
}
