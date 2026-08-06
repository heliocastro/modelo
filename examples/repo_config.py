# SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
# SPDX-License-Identifier: MIT
"""Validate a repository configuration (`.ort.yml`) with modelo and summarize it.

Equivalent of python-ort's `examples/repo_config.py`.

    python examples/repo_config.py tests/data/repo_config/curations.yml
"""

import argparse
import sys

from modelo.ort import RepositoryConfiguration


def count(value) -> int:
    return len(value) if value else 0


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("datafile")
    args = parser.parse_args()

    try:
        config = RepositoryConfiguration.from_yaml_file(args.datafile)
    except (ValueError, OSError) as e:
        print(f"invalid repository configuration: {e}", file=sys.stderr)
        sys.exit(1)

    excludes = config.excludes
    curations = config.curations
    print(f"path excludes:      {count(excludes and excludes.paths)}")
    print(f"scope excludes:     {count(excludes and excludes.scopes)}")
    print(f"package curations:  {count(curations and curations.packages)}")
    print(f"license findings:   {count(curations and curations.license_findings)}")
    print(f"snippet choices:    {count(config.snippet_choices)}")


if __name__ == "__main__":
    main()
