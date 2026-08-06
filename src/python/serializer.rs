// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! A [`serde::Serializer`] that materialises Rust models directly as Python objects.
//!
//! Going through serde rather than hand-written per-type bindings means every one of the ~100
//! ORT model structs gets Python attribute access for free, and the shape always matches what
//! `to_json` produces, because both are driven by the same `Serialize` implementations. Struct
//! names are preserved (serde hands them to `serialize_struct`), so the resulting objects are
//! instances of properly named classes instead of anonymous dicts.

use std::fmt;

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};
use serde::{ser, Serialize};

use crate::python::object::make_object;

/// Wraps a [`PyErr`] so it can travel through serde's error channel.
#[derive(Debug)]
pub struct Error(PyErr);

impl From<Error> for PyErr {
    fn from(error: Error) -> Self {
        error.0
    }
}

impl From<PyErr> for Error {
    fn from(error: PyErr) -> Self {
        Self(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self(pyo3::exceptions::PyValueError::new_err(msg.to_string()))
    }
}

type Result<T> = std::result::Result<T, Error>;

/// Converts any [`Serialize`] model into a tree of Python objects, lists and scalars.
pub fn to_pyobject<T: Serialize>(py: Python<'_>, value: &T) -> PyResult<PyObject> {
    value
        .serialize(Serializer::for_type::<T>(py))
        .map_err(PyErr::from)
}

/// The class name to use for a value of type `T`, when it can be established from the Rust type.
///
/// `#[serde(flatten)]` makes a struct serialize as a map, so `serialize_struct` — and with it the
/// struct name serde would otherwise hand us — is never reached for the run types. The declared
/// Rust type of the field being serialized is still known at that point, so it is carried along
/// as a hint. Only types from this crate's model namespace qualify: that rules out generic
/// containers such as `HashMap<String, String>`, which must stay plain dicts.
fn model_name<T: ?Sized>() -> Option<&'static str> {
    let path = std::any::type_name::<T>();
    if !path.starts_with("modelo::models::") || path.contains('<') {
        return None;
    }
    path.rsplit("::").next()
}

pub struct Serializer<'py> {
    py: Python<'py>,
    /// Class name for a map produced by a flattened struct; see [`model_name`].
    hint: Option<&'static str>,
}

impl<'py> Serializer<'py> {
    fn for_type<T: ?Sized>(py: Python<'py>) -> Self {
        Self {
            py,
            hint: model_name::<T>(),
        }
    }
}

impl<'py> ser::Serializer for Serializer<'py> {
    type Ok = PyObject;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'py>;
    type SerializeTuple = SeqSerializer<'py>;
    type SerializeTupleStruct = SeqSerializer<'py>;
    type SerializeTupleVariant = VariantSeqSerializer<'py>;
    type SerializeMap = MapSerializer<'py>;
    type SerializeStruct = StructSerializer<'py>;
    type SerializeStructVariant = VariantStructSerializer<'py>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        Ok(v.to_string().into_py(self.py))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(v.into_py(self.py))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Ok(PyBytes::new_bound(self.py, v).into())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(self.py.None())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok> {
        value.serialize(Serializer::for_type::<T>(self.py))
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(self.py.None())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Ok(self.py.None())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(variant.into_py(self.py))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        value.serialize(Serializer::for_type::<T>(self.py))
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        let dict = PyDict::new_bound(self.py);
        dict.set_item(
            variant,
            value.serialize(Serializer::for_type::<T>(self.py))?,
        )
        .map_err(Error::from)?;
        Ok(dict.into())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer {
            py: self.py,
            items: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(VariantSeqSerializer {
            variant,
            inner: SeqSerializer {
                py: self.py,
                items: Vec::with_capacity(len),
            },
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapSerializer {
            py: self.py,
            // A flattened struct reaches us as a map; the hint restores its class name.
            name: self.hint,
            dict: PyDict::new_bound(self.py).unbind(),
            key: None,
        })
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(StructSerializer {
            py: self.py,
            name,
            fields: PyDict::new_bound(self.py).unbind(),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(VariantStructSerializer {
            variant,
            inner: StructSerializer {
                py: self.py,
                name: variant,
                fields: PyDict::new_bound(self.py).unbind(),
            },
        })
    }
}

pub struct SeqSerializer<'py> {
    py: Python<'py>,
    items: Vec<PyObject>,
}

impl<'py> SeqSerializer<'py> {
    fn push<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.items
            .push(value.serialize(Serializer::for_type::<T>(self.py))?);
        Ok(())
    }

    fn finish(self) -> PyObject {
        PyList::new_bound(self.py, self.items).into()
    }
}

impl ser::SerializeSeq for SeqSerializer<'_> {
    type Ok = PyObject;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.finish())
    }
}

impl ser::SerializeTuple for SeqSerializer<'_> {
    type Ok = PyObject;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.finish())
    }
}

impl ser::SerializeTupleStruct for SeqSerializer<'_> {
    type Ok = PyObject;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.finish())
    }
}

pub struct VariantSeqSerializer<'py> {
    variant: &'static str,
    inner: SeqSerializer<'py>,
}

impl ser::SerializeTupleVariant for VariantSeqSerializer<'_> {
    type Ok = PyObject;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.inner.push(value)
    }

    fn end(self) -> Result<Self::Ok> {
        let py = self.inner.py;
        let dict = PyDict::new_bound(py);
        dict.set_item(self.variant, self.inner.finish())
            .map_err(Error::from)?;
        Ok(dict.into())
    }
}

pub struct MapSerializer<'py> {
    py: Python<'py>,
    name: Option<&'static str>,
    dict: Py<PyDict>,
    key: Option<PyObject>,
}

impl ser::SerializeMap for MapSerializer<'_> {
    type Ok = PyObject;
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<()> {
        self.key = Some(key.serialize(Serializer::for_type::<T>(self.py))?);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        let key = self
            .key
            .take()
            .ok_or_else(|| <Error as ser::Error>::custom("map value serialized before its key"))?;
        let value = value.serialize(Serializer::for_type::<T>(self.py))?;
        self.dict
            .bind(self.py)
            .set_item(key, value)
            .map_err(Error::from)
    }

    fn end(self) -> Result<Self::Ok> {
        match self.name {
            Some(name) => make_object(self.py, name, self.dict.bind(self.py)).map_err(Error::from),
            None => Ok(self.dict.into_any()),
        }
    }
}

pub struct StructSerializer<'py> {
    py: Python<'py>,
    name: &'static str,
    fields: Py<PyDict>,
}

impl StructSerializer<'_> {
    fn insert<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<()> {
        let value = value.serialize(Serializer::for_type::<T>(self.py))?;
        self.fields
            .bind(self.py)
            .set_item(key, value)
            .map_err(Error::from)
    }

    fn finish(self) -> Result<PyObject> {
        make_object(self.py, self.name, self.fields.bind(self.py)).map_err(Error::from)
    }
}

impl ser::SerializeStruct for StructSerializer<'_> {
    type Ok = PyObject;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.insert(key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.finish()
    }
}

pub struct VariantStructSerializer<'py> {
    variant: &'static str,
    inner: StructSerializer<'py>,
}

impl ser::SerializeStructVariant for VariantStructSerializer<'_> {
    type Ok = PyObject;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.inner.insert(key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        let py = self.inner.py;
        let variant = self.variant;
        let dict = PyDict::new_bound(py);
        dict.set_item(variant, self.inner.finish()?)
            .map_err(Error::from)?;
        Ok(dict.into())
    }
}
