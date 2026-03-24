from __future__ import annotations

import argparse
import re
import sys

from _common import (
    extract_direct_bases,
    extract_event_bases,
    find_mixin_owners,
    has_generated_rtti,
    has_generated_vtable,
    read_text,
    repo_relative,
    resolve_type_paths,
    type_has_event_signals,
    type_has_runtime_layout_signals,
)


def bool_label(value: bool) -> str:
    return "yes" if value else "no"


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Audit one translated CommonLib type against its local Rust/C++ artifacts."
    )
    parser.add_argument("target", help="Type name or libskyrim/src/re/*.rs path")
    args = parser.parse_args()

    paths = resolve_type_paths(args.target)
    rust_text = read_text(paths.rust)
    header_text = read_text(paths.header)
    cpp_text = read_text(paths.cpp)

    if not paths.rust and not paths.header and not paths.cpp:
        print(f"error: could not resolve any source artifacts for {paths.type_name}")
        return 1

    placeholder_todo = "replace with full translation when layout is needed" in rust_text
    size_assert = f"size_of::<{paths.type_name}>()" in rust_text
    offset_asserts = len(
        re.findall(rf"offset_of!\(\s*{re.escape(paths.type_name)}\s*,", rust_text)
    )
    inherit_count = len(re.findall(rf"inherit!\(\s*{re.escape(paths.type_name)}\b", rust_text))
    extension_trait = f"trait {paths.type_name}Ext" in rust_text
    runtime_accessors = len(re.findall(r"runtime_[a-z_]*accessor!", rust_text))
    virtuals = len(re.findall(r"virtual_method!\s*\{", rust_text))
    relocation_funcs = len(re.findall(r"relocation_func!\s*\{", rust_text))
    relocation_vars = len(re.findall(r"relocation_variable!\s*\{", rust_text))
    direct_bases = extract_direct_bases(header_text, paths.type_name)
    event_bases = extract_event_bases(header_text, paths.type_name)
    mixin_owners = find_mixin_owners(paths.type_name)
    event_signals = type_has_event_signals(header_text, cpp_text)
    event_helpers = len(
        re.findall(
            r"\b(?:GetEventSource|AddEventSink|PrependEventSink|RemoveEventSink|SendEvent|RegisterFor\w+|UnregisterFor\w+)\b",
            header_text + "\n" + cpp_text,
        )
    )

    print(f"type: {paths.type_name}")
    print(f"rust: {repo_relative(paths.rust)}")
    print(f"header: {repo_relative(paths.header)}")
    print(f"cpp: {repo_relative(paths.cpp)}")
    print(f"direct_bases: {', '.join(direct_bases) if direct_bases else '<none>'}")
    print(f"event_bases: {', '.join(event_bases) if event_bases else '<none>'}")
    print(f"generated_rtti: {bool_label(has_generated_rtti(paths.type_name))}")
    print(f"generated_vtable: {bool_label(has_generated_vtable(paths.type_name))}")
    print(f"runtime_layout_signals: {bool_label(type_has_runtime_layout_signals(header_text, cpp_text))}")
    print(f"event_signals: {bool_label(event_signals)}")
    print(f"event_helper_mentions: {event_helpers}")
    print(f"placeholder_todo: {bool_label(placeholder_todo)}")
    print(f"size_assert: {bool_label(size_assert)}")
    print(f"offset_asserts: {offset_asserts}")
    print(f"inherit_macros: {inherit_count}")
    print(f"extension_trait: {bool_label(extension_trait)}")
    print(f"mixin_owners: {len(mixin_owners)}")
    print(f"runtime_accessors: {runtime_accessors}")
    print(f"virtual_methods: {virtuals}")
    print(f"relocation_funcs: {relocation_funcs}")
    print(f"relocation_variables: {relocation_vars}")

    suggestions = []
    if paths.rust is None:
        suggestions.append("skill: translate-new")
    elif placeholder_todo:
        suggestions.append("skill: translate-partial")
    else:
        suggestions.append("skill: verify-translation")

    if type_has_runtime_layout_signals(header_text, cpp_text):
        suggestions.append("skill: translate-runtime-layout")
    if event_signals:
        suggestions.append(
            "note: apply BSTEvent owner rules (fixed base vs runtime accessor vs raw sink pointer API)"
        )
    if not extension_trait and mixin_owners and (virtuals or relocation_funcs):
        suggestions.append("consider: add-extension-trait")

    if suggestions:
        print("suggestions:")
        for suggestion in suggestions:
            print(f"  - {suggestion}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
