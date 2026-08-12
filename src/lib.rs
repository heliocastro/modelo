// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Core validation engine for modelo.
//!
//! This crate provides the base traits and models for the modular model
//! validation. It intentionally has no CLI/TUI dependencies (clap, ratatui); the
//! `modelo-cli` crate provides the `modelo` binary built on top of this library.

pub mod models;
#[cfg(feature = "python")]
pub mod python;
