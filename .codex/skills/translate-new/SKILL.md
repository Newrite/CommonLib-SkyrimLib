---
name: translate-new
description: >
  Explicit command: /translate-new <TypeName>.
  Translate a new CommonLibVR RE type into libskyrim with correct layout,
  inheritance, relocation mapping, and module registration.
---

Translate the requested type from CommonLibVR into `libskyrim/src/re/`.

## Workflow

1. Confirm the Rust file does not already exist.
2. Run `python scripts/bootstrap_translation.py <TypeName>` to gather the main
   source paths and likely workflow.
3. Read the full C++ header.
4. Read the matching `.cpp` if it exists.
5. Resolve the inheritance chain before writing fields.
6. Translate nested structs before the parent type.
7. Write the Rust file with layout asserts, inheritance metadata, and required
   traits.
8. Regenerate `libskyrim/src/re/mod.rs` with `python scripts/generate_re_mod.py`.
9. Run `python scripts/check_generated_staleness.py`.
10. Run the standard checks.

## Mandatory Source Checks

The `.cpp` is required whenever it exists. Extract all of the following:

- `RELOCATION_ID(...)` methods
- static methods not visible in the header
- private helpers defined only in the `.cpp`
- singleton/global accessors
- `RelocateVirtual(...)` comments or vtable index clues

## Dependency Rules

For each parent or embedded field type:

- If the Rust type already exists, use it.
- If the layout is required and the Rust type does not exist, stop and translate
  that dependency first.
- If the type is pointer-only or reference-only, an `abstract_type!` stub is
  acceptable.

Never replace a named parent or mixin with `[u8; N]`.

## Output Rules

Write files in this order:

1. imports
2. nested structs
3. main `#[repr(C)]` struct with offset comments
4. `size_of` assert
5. `offset_of` asserts for mixins
6. RTTI / VTABLE constants and `RttiType`
7. `inherit!` declarations
8. `impl` blocks with:
   - `virtual_method!` for fresh virtual slots
   - `relocation_func!` for `.cpp` relocated methods
   - `relocation_variable!` for globals/singletons
   - non-public helpers from `.cpp`
9. extension trait if the type is a reusable virtual mixin

Do not hand-maintain `libskyrim/src/re/mod.rs` if the generator script is
available. Regenerate it.

## Relocation Rules

- `RELOCATION_ID(se, ae)` -> `RelocationID::new(se, ae)`
- `RELOCATION_ID(se, ae, vr)` -> `RelocationID::with_vr(se, ae, vr)` only if the
  VR ID is real and source-backed
- true CommonLib `VariantID` -> `VariantID::new(se, ae, vr_offset)`
- runtime-varying non-address values -> `VariantOffset`

Do not use `VariantID::new(se, ae, 0)` for relocated methods.

## Runtime Layout Rules

If SE/AE/VR layout differs:

- keep the common struct prefix in the main type
- move divergent tails behind runtime-data accessor macros
- do not fake multiple runtime sizes in one Rust type

## Extension Trait Rule

If the translated type is used as a non-zero-offset mixin and exposes virtual or
important public helper methods, add `<TypeName>Ext` in the same file.

## Validation

Run:

- `python scripts/generate_re_mod.py`
- `python scripts/check_generated_staleness.py`
- `cargo fmt`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`
