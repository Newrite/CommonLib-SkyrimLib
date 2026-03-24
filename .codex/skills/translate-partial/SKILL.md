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
- missing `relocated_virtual_method!` or handwritten runtime wrappers for
  CommonLib `.cpp` methods built on `RelocateVirtual(...)`
- missing override comment blocks
- missing `relocation_func!` or `relocation_variable!`
- wrong relocation type choice (`RelocationID` vs `VariantID`)
- wrong handling of `ENABLE_SKYRIM_VR` / `REL::Module::IsVR()` branches
- wrong visitor strategy: closure wrapper vs raw ABI visitor vs safe sync adapter
- wrong event strategy: fixed event base vs runtime cast accessor vs raw sink
  pointer-only API
- duplicated local stop/continue enums that should use `BSContainerForEachResult`
- missing extension trait for reusable virtual mixins
- stale `todo!()` or placeholder comments in translated logic

## Relocation Rules

- `REL::ID(id)` maps to `ID::new(id)`
- `REL::Offset(offset)` maps to `Offset::new(offset)`
- C++ `RELOCATION_ID(...)` methods must use `RelocationID`, not `VariantID`
- RTTI / VTABLE constants stay on `VariantID`
- runtime-varying indices or offsets use `VariantOffset`
- normal RE files should prefer macro-based relocation instead of explicit
  `Relocation<T>` unless a macro cannot express the case cleanly
- pure `.cpp` wrappers around `REL::RelocateVirtual(...)` should prefer
  `relocated_virtual_method!`; mixed shim methods should keep a handwritten body
  and call `relocate_virtual!` only in the forwarding branch

## Layout Rules

- Never replace a named parent or mixin with `[u8; N]`
- If a dependency is required for layout, translate it first
- If an RE file depends on source-backed types from `CommonLibVR/include/REX/**`,
  translate those support types into `libskyrim/src/rex/*.rs` and import them
  from `crate::rex` instead of keeping local stand-in definitions
- If a field exists only on some runtimes, use runtime-data accessor macros
  instead of inventing one fake universal layout
- If a branch only changes method implementation and not memory layout, keep one
  Rust type and translate the method as a runtime-aware wrapper instead of
  splitting the struct

## Visitor Rules

- Preserve source-backed visitor lifetime semantics. Do not invent a safe
  adapter if the engine may retain the callback object.
- If the C++ surface is already closure-style (`std::function` wrapper), prefer
  a Rust closure helper over a bespoke ABI visitor API.
- Use `BSContainerForEachResult` for Rust-side synchronous traversal helpers
  instead of local stop/continue enums.
- If the file contains a concrete callback descendant such as
  `MagicCaster::PostCreationCallback : MagicTarget::IPostCreationModification`,
  keep it in the normal RE-type bucket: verify the abstract callback base as
  ABI, then translate the concrete descendant with honest layout and
  inheritance instead of trying to fold it into a generic adapter.

## Event Rules

- Reuse the shared `BSTEventSource<T>` / `BSTEventSink<T>` layer instead of
  per-file event stand-ins.
- Fixed-offset event owners may keep thin forwarding wrappers.
- Runtime-varying event bases should use cast/runtime-data accessors rather than
  fake always-present fields.
- Do not wrap low-level `BSTEventSource` methods in safe owner helpers unless
  source proves the sink lifetime / ownership contract.

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
