<!--
SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
SPDX-License-Identifier: MIT
-->

# modelo-cli

The `modelo` command-line binary: validates python-ort compatible YAML model files and provides
an interactive TUI, built on top of the [`modelo`](https://crates.io/crates/modelo) library.

## Usage

```sh
cargo install modelo-cli
modelo license-classifications path/to/file.yml
modelo repository-configuration path/to/.ort.yml
modelo ort-result path/to/result.yml
modelo tui [path/to/file.yml]
```

Run `modelo --help` for the full command reference.
