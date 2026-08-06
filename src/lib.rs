// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Core validation engine for modelo.
//!
//! This crate provides the base traits and orchestration for the
//! modular model validation.

pub mod cli;
pub mod models;
#[cfg(feature = "python")]
pub mod python;
pub mod tui;

pub fn run() -> anyhow::Result<()> {
    crate::cli::run()
}
