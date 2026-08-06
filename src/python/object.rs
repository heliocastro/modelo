// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! The generic model object exposed to Python.
//!
//! Every Rust model struct is materialised as an instance of a dynamically created subclass of
//! [`ValeObject`] named after the struct, so `type(result.analyzer).__name__ == "AnalyzerRun"`
//! and fields are reached with plain attribute access, the way python-ort's pydantic models
//! behave. The subclasses are created once per interpreter and cached.

use pyo3::exceptions::{PyAttributeError, PyKeyError};
use pyo3::prelude::*;
use pyo3::sync::GILOnceCell;
use pyo3::types::{PyDict, PyList, PyTuple, PyType};

/// A model instance: an ordered set of named fields plus the originating Rust type name.
#[pyclass(module = "vale.ort", name = "Object", subclass)]
#[derive(Clone)]
pub struct ValeObject {
    type_name: String,
    keys: Vec<String>,
    values: Vec<PyObject>,
}

impl ValeObject {
    fn get(&self, name: &str) -> Option<&PyObject> {
        self.keys
            .iter()
            .position(|k| k == name)
            .map(|i| &self.values[i])
    }
}

#[pymethods]
impl ValeObject {
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

    pub fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<PyObject> {
        match self.get(name) {
            Some(value) => Ok(value.clone_ref(py)),
            None => Err(PyAttributeError::new_err(format!(
                "'{}' object has no attribute '{name}'",
                self.type_name
            ))),
        }
    }

    pub fn __getitem__(&self, py: Python<'_>, name: &str) -> PyResult<PyObject> {
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
    pub fn __iter__(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(PyList::new_bound(py, &self.keys).as_any().iter()?.into())
    }

    pub fn __dir__(&self) -> Vec<String> {
        let mut names: Vec<String> = ["__type__", "keys", "values", "items", "to_dict"]
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

    pub fn values(&self, py: Python<'_>) -> Vec<PyObject> {
        self.values.iter().map(|v| v.clone_ref(py)).collect()
    }

    pub fn items<'py>(&self, py: Python<'py>) -> Bound<'py, PyList> {
        let items: Vec<Bound<'py, PyTuple>> = self
            .keys
            .iter()
            .zip(&self.values)
            .map(|(k, v)| PyTuple::new_bound(py, [k.into_py(py), v.clone_ref(py)]))
            .collect();
        PyList::new_bound(py, items)
    }

    /// A plain, recursively converted `dict` — handy for `json.dumps` or `pprint`.
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        for (key, value) in self.keys.iter().zip(&self.values) {
            dict.set_item(key, to_plain(py, value.bind(py))?)?;
        }
        Ok(dict)
    }

    pub fn __eq__(&self, other: &Bound<'_, PyAny>) -> PyResult<bool> {
        let Ok(other) = other.downcast::<Self>() else {
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

/// Recursively replaces [`ValeObject`]s inside `value` with dicts, leaving everything else as is.
fn to_plain<'py>(py: Python<'py>, value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    if let Ok(obj) = value.downcast::<ValeObject>() {
        return Ok(obj.borrow().to_dict(py)?.into_any());
    }
    if let Ok(list) = value.downcast::<PyList>() {
        let items = list
            .iter()
            .map(|item| to_plain(py, &item))
            .collect::<PyResult<Vec<_>>>()?;
        return Ok(PyList::new_bound(py, items).into_any());
    }
    if let Ok(dict) = value.downcast::<PyDict>() {
        let out = PyDict::new_bound(py);
        for (key, item) in dict.iter() {
            out.set_item(key, to_plain(py, &item)?)?;
        }
        return Ok(out.into_any());
    }
    Ok(value.clone())
}

static CLASSES: GILOnceCell<Py<PyDict>> = GILOnceCell::new();

/// Returns (creating it on first use) the `ValeObject` subclass named `name`.
fn class_for<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyType>> {
    let cache = CLASSES
        .get_or_try_init(py, || PyResult::Ok(PyDict::new_bound(py).unbind()))?
        .bind(py);
    if let Some(class) = cache.get_item(name)? {
        return Ok(class.downcast_into::<PyType>()?);
    }

    let namespace = PyDict::new_bound(py);
    namespace.set_item("__module__", "vale.ort")?;
    let bases = PyTuple::new_bound(py, [py.get_type_bound::<ValeObject>()]);
    let class = py
        .get_type_bound::<PyType>()
        .call1((name, bases, namespace))?
        .downcast_into::<PyType>()?;
    cache.set_item(name, &class)?;
    // The class claims to live in `vale.ort`; publishing it there makes that true, so
    // `from vale.ort import AnalyzerRun` and `pickle` resolve it like any other class.
    if let Ok(module) = py.import_bound("vale.ort") {
        module.setattr(name, &class)?;
    }
    Ok(class)
}

/// Builds an instance of the model class `name` holding `fields`.
pub(crate) fn make_object(
    py: Python<'_>,
    name: &str,
    fields: &Bound<'_, PyDict>,
) -> PyResult<PyObject> {
    Ok(class_for(py, name)?.call1((name, fields))?.unbind())
}
