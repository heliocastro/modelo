# SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
# SPDX-License-Identifier: MIT
"""Validate an ORT result YAML file with vale and print a section of it.

Equivalent of python-ort's `examples/ort_result.py`.

    python examples/ort_result.py tests/data/evaluation-result.yml --analyzer
"""

import argparse
import json
import sys

from vale.ort import OrtResult


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("datafile")
    parser.add_argument("-a", "--analyzer", action="store_true", help="dump the analyzer section")
    parser.add_argument("-v", "--advisor", action="store_true", help="dump the advisor section")
    args = parser.parse_args()

    try:
        parsed = OrtResult.from_yaml_file(args.datafile)
    except (ValueError, OSError) as e:
        print(f"invalid ORT result: {e}", file=sys.stderr)
        sys.exit(1)

    result = json.loads(parsed.to_json())
    if args.analyzer:
        result = result.get("analyzer")
    elif args.advisor:
        result = result.get("advisor")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
