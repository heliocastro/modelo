# SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
# SPDX-License-Identifier: MIT
"""Validate an ORT result YAML file with vale and print a section of it.

Equivalent of python-ort's `examples/ort_result.py`.

    python examples/ort_result.py tests/data/evaluation-result.yml --analyzer
"""

import argparse
import sys
from pprint import pprint

from vale.ort import OrtResult


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("datafile")
    parser.add_argument("-a", "--analyzer", action="store_true", help="dump the analyzer section")
    parser.add_argument("-v", "--advisor", action="store_true", help="dump the advisor section")
    args = parser.parse_args()

    try:
        result = OrtResult.from_yaml_file(args.datafile)
    except (ValueError, OSError) as e:
        print(f"invalid ORT result: {e}", file=sys.stderr)
        sys.exit(1)

    if args.analyzer:
        pprint(result.analyzer)
    elif args.advisor:
        pprint(result.advisor)
    else:
        print(f"repository:  {result.repository.vcs.url}")
        print(f"revision:    {result.repository.vcs.revision}")
        for section in ("analyzer", "scanner", "advisor", "evaluator"):
            print(f"{section + ':':13}{'present' if result[section] else 'not run'}")
        if result.analyzer and result.analyzer.result:
            for project in result.analyzer.result.projects[:5]:
                print(f"  project {project.id} ({project.definition_file_path})")


if __name__ == "__main__":
    main()
