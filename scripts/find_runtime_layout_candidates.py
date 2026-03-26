from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

from _common import COMMONLIB_INCLUDE_RE, COMMONLIB_SRC_RE, read_text, repo_relative


MARKERS = {
    "RuntimeDataAccessors include": ("RuntimeDataAccessors.h", 6),
    "RelocateMemberIfNewer": ("RelocateMemberIfNewer(", 5),
    "RelocateMember": ("RelocateMember(", 3),
    "RUNTIME_DATA struct": ("RUNTIME_DATA", 3),
    "VR_RUNTIME_DATA struct": ("VR_RUNTIME_DATA", 3),
    "STATIC_ASSERT_SIZE macro": ("STATIC_ASSERT_SIZE(", 3),
    "RUNTIME_CAST_ACCESSOR_VERSIONED": ("RUNTIME_CAST_ACCESSOR_VERSIONED(", 3),
    "SE_ONLY_POINTER_ACCESSOR": ("SE_ONLY_POINTER_ACCESSOR(", 2),
    "VR_ONLY_POINTER_ACCESSOR": ("VR_ONLY_POINTER_ACCESSOR(", 2),
    "EXCLUSIVE_SKYRIM macro": ("EXCLUSIVE_SKYRIM_", 3),
    "ENABLE_SKYRIM_AE macro": ("ENABLE_SKYRIM_AE", 2),
}


def distinct_size_assert_count(text: str, type_name: str) -> int:
    sizes = set(
        re.findall(
            rf"static_assert\(sizeof\(\s*{re.escape(type_name)}\s*\)\s*==\s*([^)]+)\)",
            text,
        )
    )
    return len(sizes)


def matching_cpp(header: Path) -> Path | None:
    return COMMONLIB_SRC_RE / header.relative_to(COMMONLIB_INCLUDE_RE).with_suffix(".cpp")


def score_file(header: Path, query: str | None) -> tuple[int, list[str]]:
    header_text = read_text(header)
    cpp_path = matching_cpp(header)
    cpp_text = read_text(cpp_path)
    if query:
        haystack = "\n".join((header.as_posix(), header_text, cpp_text)).lower()
        if query.lower() not in haystack:
            return (0, [])

    reasons: list[str] = []
    score = 0
    combined = "\n".join((header_text, cpp_text))
    for label, (marker, weight) in MARKERS.items():
        if marker in combined:
            reasons.append(label)
            score += weight

    distinct_sizes = distinct_size_assert_count(combined, header.stem)
    if distinct_sizes > 1:
        reasons.append(f"{distinct_sizes} distinct sizeof asserts for {header.stem}")
        score += 4

    return (score, reasons)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Find CommonLib types that likely need runtime-data accessor translation."
    )
    parser.add_argument("--query", help="Optional type-name/path filter", default=None)
    parser.add_argument("--limit", type=int, default=25)
    args = parser.parse_args()

    candidates: list[tuple[int, Path, list[str]]] = []
    for header in sorted(COMMONLIB_INCLUDE_RE.rglob("*.h")):
        score, reasons = score_file(header, args.query)
        if score > 0:
            candidates.append((score, header, reasons))

    candidates.sort(key=lambda item: (-item[0], repo_relative(item[1])))
    for score, header, reasons in candidates[: args.limit]:
        print(f"{score:>2}  {repo_relative(header)}")
        print(f"    reasons: {', '.join(reasons)}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
