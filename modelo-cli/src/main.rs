// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

mod cli;
mod tui;

fn main() -> anyhow::Result<()> {
    cli::run()
}
