---
name: refactor-runtime-layout
description: >
  Refactor an existing SE/AE/VR-divergent RE type into shared raw runtime
  blocks, honest runtime-specific accessors, and an ergonomic runtime view
  without regressing ABI or layout honesty.
---

Use this skill when a runtime-divergent RE type already exists in Rust, but its
runtime-tail API has become hard to use or maintain.

This is not the "first translation" skill. Use it after the type already has a
real translation and now needs a structural runtime-layout cleanup.

## Use Cases

- a type already exposes runtime data, but only through scattered field-level
  pointer arithmetic
- consumers would otherwise repeat `is_se() / is_ae() / is_vr()` branching in
  many places
- a type has some honest common runtime overlap beyond the initial common
  prefix, and that overlap should become explicit shared raw blocks
- naming has drifted, for example a `_flat` accessor sounds universal or a
  version-specific accessor sounds common
- the file needs a `runtime_view()` / `runtime_view_mut()` facade to centralize
  branching while preserving honest raw access

## Workflow

1. Run `python scripts/audit_translation.py <TypeName>`.
2. Read the existing Rust file.
3. Read the matching CommonLib header and `.cpp`.
4. Inventory the current raw runtime access surface:
   - whole-tail accessors
   - field-level accessors
   - runtime guards
   - existing view/helper layers
5. Identify meaningful common raw blocks across runtimes.
6. Extract those common blocks behind shared runtime-data accessors.
7. Keep runtime-specific raw entrypoints for genuinely runtime-specific layout.
8. Add or repair a high-level `runtime_view()` / `runtime_view_mut()` layer.
9. Replace repeated local runtime branching with owner-side helper methods where
   the layout contract stays honest.
10. Run the standard checks.

## Refactor Rules

- Keep ABI and layout honesty first. This is a refactor, not an excuse to fake
  one universal tail layout.
- Prefer shared raw sub-structures when the overlap is meaningful and likely to
  be reused.
- Do not force tiny one-off structs for only one or two overlapping fields.
  In those cases, prefer direct runtime-aware getter/setter helpers on the
  owner type.
- The ergonomic facade should centralize branching, but the raw API should stay
  available and truthfully named.
- If a whole-tail accessor is only valid on `SE + AE`, name it `_flat`.
- Reserve bare `*_runtime_data()` names for truly common raw layouts.
- Use `_se`, `_ae`, `_vr` for runtime-specific raw layouts.
- Use `*_view()` / `*_view_mut()` for ergonomic branching facades.
- If multiple existing local accessors duplicate the same runtime-specific
  pointer arithmetic, replace them with shared runtime-data accessors or shared
  owner-side helpers.
- Prefer shared runtime guard helpers such as `require_se`, `require_ae`,
  `require_flat`, and `require_vr` instead of repeating ad-hoc panic text.
- If the current runtime accessor macros cannot express the needed pattern,
  extend `libskyrim/src/runtime.rs` instead of leaving one-off helpers in the
  RE file.

## Common Outcomes

Typical outputs of this skill include:

- a new shared raw block such as `*_WORLD_DATA`, `*_INTERACTION_DATA`, or other
  source-backed common runtime sub-structures
- renamed raw accessors such as `*_flat`, `*_se`, `*_ae`, `*_vr`
- `runtime_view()` / `runtime_view_mut()` facades
- direct runtime-aware getters/setters for tiny divergent tails
- cleanup of older local access patterns that no longer match the preferred
  runtime-layout style

## Validation

Run:

- `cargo fmt`
- `python scripts/check_generated_staleness.py`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`
