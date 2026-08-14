// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Core validation engine for modelo.
//!
//! This crate provides the base traits and models for the modular model
//! validation. It intentionally has no CLI/TUI dependencies (clap, ratatui); the
//! `modelo-cli` crate provides the `modelo` binary built on top of this library.
//!
//! Every model implements [`Model`](crate::models::Model), whose
//! [`validate`](crate::models::Model::validate) method performs semantic validation beyond
//! what `serde` deserialization already enforces, plus
//! `serde::{Serialize, Deserialize}` and `std::fmt::Display`. Models are grouped by the standard
//! they describe; the only one currently implemented is [`ort`](crate::models::ort), the
//! [OSS Review Toolkit (ORT)](https://github.com/oss-review-toolkit/ort) result model.
//!
#![doc = include_str!("../docs/examples.md")]

pub mod models;
