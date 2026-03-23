---
name: verify-translation
description: >
  Explicit command: /verify-translation <TypeName>.
  Deep verification of an existing Rust translation against the full CommonLibVR
  inheritance chain. Fixes all source-backed mismatches it finds.
---

Verify the requested translation against the full inheritance chain.

## Verification Scope

Read and verify:

1. the Rust file
2. the matching C++ header
3. the matching `.cpp` if it exists
4. every parent header and parent `.cpp` needed to understand inherited layout,
   virtual slots, and helper methods

Before editing, `python scripts/audit_translation.py <TypeName>` is a good fast
preflight when the existing Rust file may already be mostly correct.

## Required Checks

### Layout

- field order
- field types
- offset comments
- nested structs
- `size_of` assert
- `offset_of` asserts for mixins
- no blob substitution for named parents or mixins

### Virtuals

- fresh virtual slots are represented with `virtual_method!`
- inherited overrides are documented with `// override (Parent)` comment blocks
- child files do not redundantly redeclare inherited parent virtuals
- vtable indices match source-backed order

### Relocations

- `.cpp` relocated methods use `relocation_func!`
- global or singleton access uses `relocation_variable!` when appropriate
- `RELOCATION_ID` maps to `RelocationID`
- true `VariantID` use is reserved for constants such as RTTI and VTABLE
- no legacy `VariantID::new(se, ae, 0)` for relocated functions

### Runtime layout

- runtime-varying fields use runtime-data accessor macros where needed
- no fake single-layout struct for truly divergent runtime tails

### Ergonomics and reuse

- extension traits exist for reusable virtual mixins
- trait blanket impls use `AsRef` / `AsMut` correctly

### Generated artifacts

- if verification required adding or removing RE files, regenerate
  `libskyrim/src/re/mod.rs` with `python scripts/generate_re_mod.py`
- if the issue is in generated offsets, fix the C++ source input and regenerate
  via `python scripts/generate_offsets.py` instead of patching generated Rust by hand

## Fix Policy

Fix all source-backed problems found during verification. Do not stop after
reporting. The goal of this skill is a corrected translation, not a read-only
review.

## Validation

Run:

- `python scripts/generate_re_mod.py` when the RE file set changed
- `python scripts/check_generated_staleness.py`
- `cargo fmt`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`
