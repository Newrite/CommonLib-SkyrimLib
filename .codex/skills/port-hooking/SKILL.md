---
name: port-hooking
description: >
  Port CommonLib-style C++ hook code to Rust using libskyrim's current
  relocation, hook, and runtime-selection layers.
---

Use this skill when converting plugin-side hooking code from CommonLib C++ into
Rust.

## Input Pattern

Typical source material includes:

- `REL::Relocation<decltype(fn)> original;`
- `RELOCATION_ID(...)`
- `REL::ID(...)` / `REL::Offset(...)`
- `REL::VariantOffset` / `REL::Relocate(...)`
- `write_vfunc(...)`
- trampoline call / branch hooks
- VR-specific alternate vtable indices

## Mapping Rules

- `REL::ID(id)` -> `ID::new(id)`
- `REL::Offset(offset)` -> `Offset::new(offset)`
- `RELOCATION_ID(se, ae)` -> `RelocationID::new(se, ae)`
- true `REL::VariantID` -> `VariantID::new(se, ae, vr_offset)`
- `REL::VariantOffset` -> `VariantOffset::new(...)` or `new_se_ae(...)`
- CommonLib `REL::Relocate(se_and_vr, ae)` -> `relocate(se_and_vr, ae)`
- CommonLib `REL::Relocate(se, ae, vr)` -> `relocate_vr(se, ae, vr)`
- CommonLib `REL::RelocateVirtual(...)` inside translated RE wrappers ->
  `relocated_virtual_method!` for pure forwarding wrappers or
  `relocate_virtual!` in a handwritten method body when only part of the logic
  forwards to the vtable
- typed original function storage should prefer the hook macros, which now keep
  `ORIGINAL` as `Relocation<Signature>`

## Preferred Rust Layer

Use:

- `define_vtable_hook!` for vtable interception
- `define_call_hook!` for relocated call hooks
- `Relocation<T>` directly only when the macros cannot express the case cleanly

## Workflow

1. Identify whether the hook is vtable-based or call-branch based.
2. Translate address selectors to `RelocationID`, `VariantID`, or `VariantOffset`
   using the correct semantics.
3. Replace manual runtime branching with `VariantOffset`, `relocate`, or
   `relocate_vr` where possible.
4. Port the hook body and preserve nullability / engine-contract comments.
5. Keep the original function invocation typed and source-backed.

If the hook targets an RE type that is not already familiar, run
`python scripts/bootstrap_translation.py <TypeName>` first so the hook port
uses the right header / cpp / Rust file set and the current relocation model.

## Validation

Run:

- `cargo fmt`
- `python scripts/check_generated_staleness.py`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`
