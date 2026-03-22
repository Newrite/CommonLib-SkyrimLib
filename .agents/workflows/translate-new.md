---
name: translate-new
description: Full translation of a C++ header that has no Rust equivalent yet.
---

Perform a COMPLETE translation of the C++ header: {{args}}

## Pre-flight Checklist (do not skip any step)

1. Confirm `libskyrim/src/re/` does NOT contain this type yet
2. Read `CommonLibVR/include/RE/` path for the header fully
3. Immediately check `CommonLibVR/src/RE/` for matching .cpp — read it fully.
   Extract ALL of the following:
   - All `RELOCATION_ID(SE, AE)` pairs → `relocation_func!` with comment
   - Static methods (no `this`) → `pub fn` without `&self`
   - Private helpers not in .h → `fn` (not `pub fn`)
   - `RelocateVirtual(SE_idx, AE_idx, ...)` → record both vtable indices as comment
   - Global accessors / singleton patterns → `relocation_variable!`
4. Run dependency resolution via `dependency-resolver` skill for every `#include`:
   - Missing base class → translate it first (full translate-new), then return here
   - Missing pointer-only type → create `abstract_type!` stub + register in mod.rs
5. Run `inheritance-mapper` skill: build full inheritance memory map and print
   the chain summary BEFORE writing any code

## Output Structure (follow this order exactly)

```rust
// 1. use imports (offsets_rtti, offsets_vtable, parent types)

// 2. #[repr(C)] struct with ALL fields and // 0xOFFSET comments

// 3. size_of assertion (mandatory)
// 4. offset_of assertions for every mixin field (mandatory)

// 5. impl RttiType (if class has RTTI)
// 6. inherit! macro(s) — primary base, then each mixin

// 7. impl ClassName {
//      pub const RTTI / VTABLE
//      // override (ParentName) comments for every overridden virtual
//      virtual_method! blocks
//      relocation_func! blocks with // RELOCATION_ID SE: X, AE: Y comments
//      private fn helpers (no pub)
// }

// 8. impl FormCastable (if inherits TESForm)
// 9. impl BSTHash / PartialEq (if C++ has BSCRC32_ / operator==)
// 10. Extension Traits for mixin methods (if any mixins)
```

## Completeness Rules
- ALL virtual functions must be translated — pure virtuals included
- ALL fields must have `// 0xOFFSET` comments copied from C++
- ALL RELOCATION_IDs must become `relocation_func!` — no `todo!()`
- ALL static methods must be present as associated functions
- ALL private helpers from .cpp must be present (no `pub`)
- Override comments `// override (Parent)` required for every vtable override

## Final Step
Add to `libskyrim/src/re/mod.rs` (in alphabetical order):

    pub mod type_name;
    pub use type_name::*;
