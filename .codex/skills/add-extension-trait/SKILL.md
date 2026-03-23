---
name: add-extension-trait
description: >
  Explicit command: /add-extension-trait <path/to/file.rs>.
  Add or repair an extension trait for a reusable RE mixin so child types expose
  the mixin API ergonomically through `AsRef` / `AsMut`.
---

Add or repair an extension trait for the requested file.

## Eligibility

Proceed only if all are true:

- the file defines a reusable RE type
- the type is used as a base or mixin from another `libskyrim/src/re/*.rs` file
- the type exposes virtual methods or important public helper methods worth
  re-exporting through children
- the trait does not already exist or is incomplete

Use `python scripts/find_extension_trait_candidates.py` when you want a quick
candidate list before opening many mixin files by hand.

## Trait Pattern

- Trait name: `<TypeName>Ext`
- Define it in the same file as the mixin type
- Include all reusable virtual methods
- Include important ergonomic wrappers and helper methods when they are part of
  the intended child-facing API
- Use a blanket impl over `AsRef<TypeName>` and `AsMut<TypeName>` as needed

## Rules

- Keep raw virtual methods on the mixin impl itself
- Do not move logic from the mixin impl into the trait
- Do not redefine mixin methods on child types
- If a safe wrapper is useful, add it to the mixin impl and expose it through the
  trait as well
- Ensure `mod.rs` re-exports the module so the trait is reachable

## Validation

Run:

- `cargo fmt`
- `cargo check -p libskyrim`
