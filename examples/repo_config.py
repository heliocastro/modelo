# SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
# SPDX-License-Identifier: MIT
"""Validate a repository configuration (`.ort.yml`) with vale and summarize it.

Equivalent of python-ort's `examples/repo_config.py`.

    python examples/repo_config.py tests/data/repo_config/curations.yml
"""

import argparse
import json
import sys

from vale.ort import RepositoryConfiguration


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("datafile")
    args = parser.parse_args()

    try:
        parsed = RepositoryConfiguration.from_yaml_file(args.datafile)
    except (ValueError, OSError) as e:
        print(f"invalid repository configuration: {e}", file=sys.stderr)
        sys.exit(1)

    config = json.loads(parsed.to_json())
    excludes = config.get("excludes") or {}
    curations = config.get("curations") or {}
    print(f"path excludes:      {len(excludes.get('paths') or [])}")
    print(f"scope excludes:     {len(excludes.get('scopes') or [])}")
    print(f"package curations:  {len(curations.get('packages') or [])}")
    print(f"license findings:   {len(curations.get('license_findings') or [])}")
    print(f"snippet choices:    {len(config.get('snippet_choices') or [])}")


if __name__ == "__main__":
    main()
