from __future__ import annotations

import argparse
import re
import sys
from collections import defaultdict
from pathlib import Path

from _common import COMMONLIB_INCLUDE_RE, LIBSKYRIM_RE, find_rust_file, read_text, repo_relative


RUST_PRIMARY_INHERIT_RE = re.compile(r"inherit!\(\s*(\w+)\s*:\s*(\w+)\s*\)")
RUST_MIXIN_INHERIT_RE = re.compile(r"inherit!\(\s*(\w+)\s*=>\s*(\w+)\s*,\s*(\w+)\s*\)")
COMMONLIB_TYPE_RE = re.compile(
    r"\b(?:class|struct)\s+([A-Za-z_]\w*)\s*(?::\s*([^{;]+))?\s*\{",
    re.MULTILINE,
)


def normalize_base_name(raw_base: str) -> str:
    base = re.sub(r"\b(public|protected|private|virtual)\b", "", raw_base)
    base = re.sub(r"\s+", " ", base).strip()
    if not base:
        return ""
    if "::" in base:
        base = base.split("::")[-1]
    if "<" in base:
        return ""
    if not base[:1].isupper():
        return ""
    return base


def collect_rust_inheritance(
    file_texts: dict[Path, str],
) -> tuple[dict[str, set[str]], dict[str, set[str]]]:
    primary_owners: dict[str, set[str]] = defaultdict(set)
    mixin_owners: dict[str, set[str]] = defaultdict(set)

    for path, text in file_texts.items():
        owner = repo_relative(path)

        for _child, base in RUST_PRIMARY_INHERIT_RE.findall(text):
            primary_owners[base].add(owner)

        for _child, mixin, _field in RUST_MIXIN_INHERIT_RE.findall(text):
            mixin_owners[mixin].add(owner)

    return primary_owners, mixin_owners


def collect_commonlib_inheritance() -> dict[str, set[str]]:
    children_by_base: dict[str, set[str]] = defaultdict(set)

    for header_path in sorted(COMMONLIB_INCLUDE_RE.rglob("*.h")):
        header_text = read_text(header_path)
        if not header_text:
            continue

        for child, clause in COMMONLIB_TYPE_RE.findall(header_text):
            if not clause:
                continue
            if not child[:1].isupper():
                continue

            clause = re.sub(r"//.*", "", clause)
            clause = "\n".join(
                line for line in clause.splitlines() if not line.strip().startswith("#")
            )

            for raw_base in clause.split(","):
                base = normalize_base_name(raw_base)
                if base:
                    children_by_base[base].add(child)

    return children_by_base


def extension_trait_status(type_name: str, rust_text: str) -> str:
    trait_name = f"{type_name}Ext"
    trait_match = re.search(rf"\btrait\s+{re.escape(trait_name)}\b", rust_text)
    if not trait_match:
        return "missing_trait"

    blanket_impl_match = re.search(
        rf"impl\s*<(?P<bounds>.*?)>\s*{re.escape(trait_name)}\s+for\s+T\s*\{{",
        rust_text,
        re.DOTALL,
    )
    if not blanket_impl_match:
        return "incomplete_trait"

    bounds = blanket_impl_match.group("bounds")
    has_as_ref = re.search(rf"\bAsRef<\s*{re.escape(type_name)}\s*>", bounds) is not None
    has_as_mut = re.search(rf"\bAsMut<\s*{re.escape(type_name)}\s*>", bounds) is not None

    trait_body_match = re.search(
        rf"\btrait\s+{re.escape(trait_name)}\b[^\{{]*\{{(?P<body>.*?)\n\}}",
        rust_text,
        re.DOTALL,
    )
    trait_body = trait_body_match.group("body") if trait_body_match else ""

    needs_as_mut = "&mut self" in trait_body
    if not has_as_ref:
        return "incomplete_trait"
    if needs_as_mut and not has_as_mut:
        return "incomplete_trait"
    return "complete_trait"


def method_surface_counts(rust_text: str) -> tuple[int, int, int]:
    virtuals = rust_text.count("virtual_method!")
    relocs = rust_text.count("relocation_func!")
    pub_helpers = len(re.findall(r"\bpub(?:\s+unsafe)?\s+fn\b", rust_text))
    return virtuals, relocs, pub_helpers


def main() -> int:
    parser = argparse.ArgumentParser(
        description=(
            "Find reusable RE base or mixin types that look like they should expose "
            "an extension trait, using both current Rust inheritance and the "
            "source-backed CommonLibVR inheritance surface."
        )
    )
    parser.add_argument("--limit", type=int, default=50)
    args = parser.parse_args()

    rust_files = sorted(path for path in LIBSKYRIM_RE.glob("*.rs") if path.name != "mod.rs")
    file_texts = {path: read_text(path) for path in rust_files}

    rust_primary_owners, rust_mixin_owners = collect_rust_inheritance(file_texts)
    commonlib_children_by_base = collect_commonlib_inheritance()

    candidate_names = (
        set(rust_primary_owners)
        | set(rust_mixin_owners)
        | {base for base in commonlib_children_by_base if find_rust_file(base) is not None}
    )

    candidates = []
    for type_name in sorted(candidate_names):
        rust_path = find_rust_file(type_name)
        if rust_path is None:
            continue

        rust_text = file_texts.get(rust_path, read_text(rust_path))
        if not rust_text:
            continue

        trait_status = extension_trait_status(type_name, rust_text)
        if trait_status == "complete_trait":
            continue

        virtuals, relocs, pub_helpers = method_surface_counts(rust_text)
        if virtuals + relocs + pub_helpers == 0:
            continue

        rust_primary = sorted(rust_primary_owners.get(type_name, set()))
        rust_mixins = sorted(rust_mixin_owners.get(type_name, set()))
        commonlib_children = sorted(commonlib_children_by_base.get(type_name, set()))
        future_children = sorted(
            child for child in commonlib_children if find_rust_file(child) is None
        )

        if not rust_primary and not rust_mixins and not future_children:
            continue

        candidates.append(
            (
                -(len(rust_primary) + len(rust_mixins)),
                -len(future_children),
                type_name,
                rust_path,
                trait_status,
                virtuals,
                relocs,
                pub_helpers,
                rust_primary,
                rust_mixins,
                commonlib_children,
                future_children,
            )
        )

    candidates.sort()
    for (
        _rust_score,
        _future_score,
        type_name,
        rust_path,
        trait_status,
        virtuals,
        relocs,
        pub_helpers,
        rust_primary,
        rust_mixins,
        commonlib_children,
        future_children,
    ) in candidates[: args.limit]:
        rust_owner_count = len(rust_primary) + len(rust_mixins)
        print(
            f"{type_name}  {repo_relative(rust_path)}  "
            f"status={trait_status}  rust_owners={rust_owner_count}  "
            f"future_children={len(future_children)}  "
            f"surface=virtual:{virtuals},reloc:{relocs},pub:{pub_helpers}"
        )
        if rust_primary:
            print(f"    rust primary-base owners: {', '.join(rust_primary)}")
        if rust_mixins:
            print(f"    rust nonzero mixin owners: {', '.join(rust_mixins)}")
        if future_children:
            print(f"    CommonLibVR-only children: {', '.join(future_children)}")
        elif commonlib_children:
            print(f"    CommonLibVR children: {', '.join(commonlib_children)}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
