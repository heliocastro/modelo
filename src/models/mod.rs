// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! The validation engine, and one module per model family.
//!
//! [`Model`] and [`ValidationError`] are family-agnostic and live here; the models themselves are
//! namespaced by the standard they describe, currently only [`ort`].
//!
//! ```
//! use modelo::models::Model;
//! use modelo::models::ort::identifier::Identifier;
//!
//! let id: Identifier = "Maven:org.example:artifact:1.0".parse().unwrap();
//! id.validate().expect("a fully-qualified identifier is valid");
//! ```

use std::fmt::{Debug, Display};
use thiserror::Error;

pub mod ort;

/// Errors that can occur during model validation.
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Field '{field}' is invalid: {message}")]
    InvalidField { field: String, message: String },

    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Value out of range for field '{field}': {message}")]
    OutOfRange { field: String, message: String },

    #[error("Generic validation error: {0}")]
    Generic(String),
}

/// The core trait that all models must implement.
///
/// A model represents a structure that can be validated.
pub trait Model: Debug + Display {
    /// Performs validation on the model instance.
    ///
    /// Returns `Ok(())` if validation passes, or a `ValidationError` if it fails.
    fn validate(&self) -> Result<(), ValidationError>;
}
