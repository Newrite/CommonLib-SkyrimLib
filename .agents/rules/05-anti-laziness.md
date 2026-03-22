---
trigger: always_on
description: Completeness enforcement. Zero tolerance for omissions.
---

## ABSOLUTE PROHIBITIONS
- Comments like `// ... (many more)` → FORBIDDEN
- Skipping virtual functions → FORBIDDEN
- Truncating enum constants → FORBIDDEN
- Assuming .cpp doesn't exist without checking → FORBIDDEN
- Skipping static methods because they have no `this` → FORBIDDEN
- Translating only one overload of a const/non-const pair → FORBIDDEN

## The .cpp Rule (CRITICAL)
For EVERY header `include/RE/.../Name.h`:
1. IMMEDIATELY check `src/RE/.../Name.cpp`
2. Read it fully if it exists
3. Extract ALL of the following — missing any one is a bug:
   - All `RELOCATION_ID(SE, AE)` method implementations
   - Static methods (`RetType ClassName::Method(Args)` with no `this`)
   - Private helper methods present in .cpp but NOT declared in .h
   - `RelocateVirtual(SE_idx, AE_idx, ...)` calls — record both indices
   - Global accessors / singleton patterns
4. Do NOT finalize translation before this step

## Static Methods (CRITICAL)
`RetType ClassName::Method(Args)` in .cpp with no `this` = C++ static method.
Translate as associated function with no `&self`:

    impl ClassName {
        pub fn method(args) -> RetType { ... }
    }

These exist ONLY in .cpp — they will never appear in .h declarations.

## Const / Non-Const Overload Pairs
If .h or .cpp declares BOTH `T* GetFoo()` and `const T* GetFoo() const`:
- Translate BOTH — never drop either variant
- Non-const: `fn get_foo(&mut self) -> *mut T`
- Const:     `fn get_foo_ref(&self) -> *const T`
- Private helper (e.g. `GetFooImpl`): `fn get_foo_impl(&self) -> *const T` (no `pub`)

## TODO Policy
Uncertain about an offset, ABI, or behavior? Write:
`// TODO: VERIFY — [what needs verification and why]`
This is REQUIRED. Silent omission is a bug, not a shortcut.

## Large Classes
Use as many agent turns as needed. Output EVERY method.
If class has 100+ virtual functions — translate all 100+.
Never summarize, never truncate, never defer to "remaining methods follow the same pattern".
