from __future__ import annotations

import argparse
import sys

from _common import (
    extract_direct_bases,
    extract_nested_types,
    find_mixin_owners,
    has_generated_rtti,
    has_generated_vtable,
    read_text,
    repo_relative,
    resolve_type_paths,
    type_has_runtime_layout_signals,
)


def choose_primary_skill(rust_exists: bool, has_runtime_layout: bool, placeholder_todo: bool) -> str:
    if not rust_exists:
        return "translate-runtime-layout" if has_runtime_layout else "translate-new"
    if has_runtime_layout:
        return "translate-runtime-layout"
    if placeholder_todo:
        return "translate-partial"
    return "verify-translation"


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Collect the main artifacts and workflow hints before translating or verifying a type."
    )
    parser.add_argument("target", help="Type name or libskyrim/src/re/*.rs path")
    args = parser.parse_args()

    paths = resolve_type_paths(args.target)
    header_text = read_text(paths.header)
    cpp_text = read_text(paths.cpp)
    rust_text = read_text(paths.rust)

    if not paths.header and not paths.rust:
        print(f"error: could not resolve {paths.type_name} to a CommonLib header or Rust file")
        return 1

    direct_bases = extract_direct_bases(header_text, paths.type_name)
    nested_types = extract_nested_types(header_text, paths.type_name)
    runtime_layout = type_has_runtime_layout_signals(header_text, cpp_text)
    placeholder_todo = "replace with full translation when layout is needed" in rust_text
    primary_skill = choose_primary_skill(paths.rust is not None, runtime_layout, placeholder_todo)
    mixin_owners = find_mixin_owners(paths.type_name)

    print(f"type: {paths.type_name}")
    print(f"header: {repo_relative(paths.header)}")
    print(f"cpp: {repo_relative(paths.cpp)}")
    print(f"rust: {repo_relative(paths.rust)}")
    print(f"direct_bases: {', '.join(direct_bases) if direct_bases else '<none>'}")
    print(f"nested_types: {', '.join(nested_types) if nested_types else '<none>'}")
    print(f"generated_rtti: {'yes' if has_generated_rtti(paths.type_name) else 'no'}")
    print(f"generated_vtable: {'yes' if has_generated_vtable(paths.type_name) else 'no'}")
    print(f"runtime_layout_signals: {'yes' if runtime_layout else 'no'}")
    print()
    print("recommended_pipeline:")
    print(f"  - primary skill: {primary_skill}")
    print("  - secondary skill: translation-auditor")
    if runtime_layout:
        print("  - follow-up skill: verify-translation")
    elif paths.rust is not None:
        print("  - follow-up skill: verify-translation")
    if mixin_owners:
        print("  - optional skill: add-extension-trait")
    print("  - script: python scripts/check_generated_staleness.py")
    print("  - validation: cargo fmt")
    print("  - validation: cargo check -p libskyrim")
    print("  - validation: cargo check -p libskyrim --tests")
    return 0


if __name__ == "__main__":
    sys.exit(main())
