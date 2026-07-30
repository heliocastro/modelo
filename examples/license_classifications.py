# SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
# SPDX-License-Identifier: MIT
"""Validate a license-classifications YAML file with vale.

Equivalent of python-ort's `examples/licenses_classification.py`.

    python examples/license_classifications.py tests/data/license-classifications.yml
"""

import argparse
import json
import sys

from vale.ort import LicenseClassifications


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("datafile")
    args = parser.parse_args()

    try:
        parsed = LicenseClassifications.from_yaml_file(args.datafile)
    except (ValueError, OSError) as e:
        print(f"invalid license-classifications file: {e}", file=sys.stderr)
        sys.exit(1)

    data = json.loads(parsed.to_json())
    print(f"{len(data['categories'])} categories, {len(data['categorizations'])} categorizations")
    for categorization in data["categorizations"][:5]:
        print(f"  {categorization['id']}: {', '.join(sorted(categorization['categories']))}")


if __name__ == "__main__":
    main()
