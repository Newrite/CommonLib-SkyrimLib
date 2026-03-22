---
name: dependency-resolver
description: Load when processing #include dependencies in C++ headers,
  checking if Rust translations exist, creating stubs, or updating mod.rs.
---

## Decision Tree for Each `#include "RE/.../Type.h"`

1. Does `libskyrim/src/re/type_name.rs` exist?
   - YES → proceed, type is available
   - NO → go to step 2

2. Is the type used as parent class OR critical struct (layout needed)?
   - YES → PAUSE current translation, run full translate-new for dependency FIRST (Scenario A)
   - NO (only pointer/reference) → create stub (Scenario B)

3. After Scenario A or B completes → RESUME original translation

## Scenario A: Full Dependency Translation
- Run the complete translate-new workflow for the dependency
- This includes reading its .h AND .cpp, resolving its own dependencies recursively
- Only return to the original class after the dependency file is committed to re/ and mod.rs

## Scenario B: Stub Creation
File: `libskyrim/src/re/type_name.rs`

    // AUTO-STUB: Full translation pending
    // TODO: VERIFY — replace with full translation when layout is needed
    core_util::abstract_type! { pub type TypeName; }

Stub MUST also be registered in mod.rs immediately (see below).

## Singleton / Accessor Check
Before creating a stub, check if the type's .cpp contains `GetSingleton()` or
a global accessor pattern:
- YES → do NOT create abstract_type! stub
- Instead → translate at minimum: the struct declaration + the singleton accessor:

      relocation_variable! {
          pub fn get_singleton() -> *mut TypeName => VariantID::new(se, ae, vr), is_ptr
      }

## mod.rs Update (MANDATORY after every new file — stub or full)

Add to `libskyrim/src/re/mod.rs`:

    pub mod type_name;
    pub use type_name::*;

Rules:
- Apply after EVERY new .rs file — stub or full translation, no exceptions
- NEVER reference a type that has no corresponding entry in mod.rs
- mod.rs entries must be in alphabetical order
