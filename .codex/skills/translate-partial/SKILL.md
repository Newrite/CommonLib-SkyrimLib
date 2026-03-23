---
name: translate-partial
description: >
  Explicit command: /translate-partial <TypeName>.
  Audit an existing Rust translation against CommonLibVR and fill the missing or
  incorrect pieces without rewriting correct code.
---

Audit and complete the requested Rust translation.

## Workflow

1. Run `python scripts/audit_translation.py <TypeName>` for a fast local gap
   snapshot.
2. Read the existing Rust file.
3. Read the matching C++ header.
4. Read the matching `.cpp` if it exists.
5. Resolve every base and mixin type before touching fields.
6. Produce a concrete gap list.
7. Patch only the missing or incorrect pieces.
8. Run the standard checks.

## Required Gap Classes

Check all of these:

- missing fields or wrong field types
- missing nested structs
- missing or wrong size / offset asserts
- missing `inherit!` declarations
- missing RTTI / VTABLE constants
- missing `virtual_method!` declarations for fresh virtual slots
- missing override comment blocks
- missing `relocation_func!` or `relocation_variable!`
- wrong relocation type choice (`RelocationID` vs `VariantID`)
- missing extension trait for reusable virtual mixins
- stale `todo!()` or placeholder comments in translated logic

## Relocation Rules

- C++ `RELOCATION_ID(...)` methods must use `RelocationID`, not `VariantID`
- RTTI / VTABLE constants stay on `VariantID`
- runtime-varying indices or offsets use `VariantOffset`
- normal RE files should prefer macro-based relocation instead of explicit
  `Relocation<T>` unless a macro cannot express the case cleanly

## Layout Rules

- Never replace a named parent or mixin with `[u8; N]`
- If a dependency is required for layout, translate it first
- If a field exists only on some runtimes, use runtime-data accessor macros
  instead of inventing one fake universal layout

## Editing Rule

Preserve correct existing code. Patch narrowly. If a larger refactor is required
to make the file honest, do it, but keep the resulting structure idiomatic for
the current `libskyrim` architecture.

If the task adds or removes `libskyrim/src/re/*.rs` files as part of the fix,
regenerate `libskyrim/src/re/mod.rs` with `python scripts/generate_re_mod.py`.

## Validation

Run:

- `python scripts/generate_re_mod.py` when the RE file set changed
- `python scripts/check_generated_staleness.py`
- `cargo fmt`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`
