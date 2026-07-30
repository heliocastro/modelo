// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

/// A base trait for models that require common field-level validation logic.
pub trait BaseModel: crate::models::Model {
    /// Returns a map of field names to their current values (for debugging/inspection).
    /// This is a simplification for the base model.
    fn get_fields(&self) -> HashMap<String, String>;
}
