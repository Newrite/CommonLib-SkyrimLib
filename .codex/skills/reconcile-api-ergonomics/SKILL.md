---
name: reconcile-api-ergonomics
description: >
  Audit and repair an existing Rust RE/SDK surface when a mechanically literal
  CommonLib C++ signature worsens ergonomics without preserving a meaningful
  behavioral, ownership, layout, or ABI difference. Use for fixes such as
  semantically read-only getters that still require `&mut self`, raw C-string
  helpers that need `*_as_str()` sugar, or public wrappers that should stay
  source-backed but become more idiomatic for Rust consumers.
---

Audit the requested Rust-facing API and reconcile source-backed fidelity with
idiomatic Rust ergonomics.

## Use This Skill When

- the user wants to improve an existing public RE or SDK API without changing
  real engine behavior
- a translated getter or helper feels more literal to C++ than semantically
  necessary
- a method currently exposes only a low-level raw form but obviously needs an
  ergonomic helper alongside it
- a consumer such as `sdk`, `hooks`, `events`, or `RustSKSETemplate` has to
  fight the API rather than use it naturally
- the low-level contract has already been established by translation,
  `translation-auditor`, or `verify-translation`

## Do Not Use This Skill When

- the main problem is missing fields, wrong layout, bad offsets, or wrong
  relocation semantics
- the C++ signature difference reflects a real behavioral contract such as
  mutation, ownership transfer, retained callbacks, or aliasing risk
- the task is really a new translation or a broad verification pass; use the
  translation skills first

## Workflow

1. Read the existing Rust API surface that users call today.
2. Read the matching CommonLib header.
3. Read the matching `.cpp` when it exists.
4. Classify each candidate API detail:
   - layout / ABI / ownership critical
   - behaviorally meaningful wrapper contract
   - ergonomics-only surface choice
5. Patch only the ergonomics-only or clearly safe wrapper layer.
6. Keep or preserve a low-level/raw entrypoint when that helps maintain honesty.
7. Run validation.

## Typical Fixes

- relax `&mut self` to `&self` for semantically read-only getters
- add `*_as_str()` helpers over read-only `*const c_char` / `const char*`
  getters
- add small Rust-facing helpers that return `Option<&T>`, `GameRef`, or similar
  when the contract becomes clearer without hiding real edge cases
- repair extension traits so child types inherit the improved mixin API

## Guardrails

- do not change struct layout, offsets, inheritance, vtable ownership, or
  relocation semantics
- do not hide real mutation, ownership transfer, or retained-callback behavior
- do not delete the raw method just because an ergonomic helper was added
- if uncertainty remains, keep the raw/strict method and add the ergonomic
  helper alongside it
- keep the change narrow and source-backed; this is a refinement pass, not a
  rewrite

## Validation

Run:

- `cargo fmt`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`

If the change affects a public plugin-facing API and
`S:/Programming/RustSKSETemplate` is available, prefer a quick consumer smoke
check there as well.
