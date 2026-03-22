---
name: translation-auditor
description: Load when comparing existing Rust translation against C++ source,
  verifying field offsets, vtable indices, or producing gap analysis reports.
---

## Audit Methodology

### 0. Pre-Audit: Read Both Sources
Before any comparison:
1. Read the Rust file in `libskyrim/src/re/`
2. Read the C++ header `CommonLibVR/include/RE/.../Name.h`
3. Read the C++ source `CommonLibVR/src/RE/.../Name.cpp` — MANDATORY
4. Build the list of ALL expected items from C++ (fields, virtuals, statics, helpers)

### 1. Field-by-Field Comparison
For each C++ field comment `// 0x08 SomeType name`:
- Find corresponding Rust field
- Verify type mapping is correct
- Verify it will land at offset 0x08 given preceding fields
- Verify `// 0x08` comment is present on the Rust field

### 2. VTable Index Audit
List all `virtual` functions in C++ in order.
Compare with `virtual_method!` indices in Rust.
Any gap in numbering = missing override or wrong index.
Check that `// override (ParentName)` comments are present for all overrides.

### 3. Size Cross-check
C++ `// sizeof == 0x1B8` → Rust `size_of` assertion must be `0x1B8`.
If assertion is missing → flag as ❌ FAIL.
If mixin field present → `offset_of!` assertion must also exist.

### 4. Static Methods Audit
From .cpp, list all `RetType ClassName::Method(Args)` with no `this`.
Each must exist in Rust as `pub fn method(args)` with no `&self`.
Missing static method → ❌ FAIL.

### 5. RELOCATION_ID Audit
For every `RELOCATION_ID(se, ae)` found in .cpp:
- Corresponding `relocation_func!` must exist in Rust
- Comment `// RELOCATION_ID SE: X, AE: Y` must be present above it
- Both SE and AE values must match
Missing relocation or mismatched IDs → ❌ FAIL.

### 6. Private Helper Audit
For every method in .cpp NOT declared in .h:
- Must exist in Rust as `fn helper_name(...)` (no `pub`)
Missing private helper → ❌ FAIL.

### 7. Trait Implementation Audit
Check that all required traits are implemented:
- `RttiType` — if class has RTTI
- `FormCastable` — if class inherits TESForm
- `BSTHash` / `PartialEq` — if C++ has `BSCRC32_` or `operator==`
- `inherit!` macro — for every base class (primary and mixin)

### Report Format
For each check output one line:
✅ offset 0x08 — `formID: u32` matches `TESForm::formID`
✅ static — `LookupByHandle` present as associated fn
✅ RELOCATION_ID — SE: 37787 AE: 38736 matches relocation_func!
❌ offset 0x10 — missing `flags` field (C++ has `ObjRefFlags`)
❌ static — `GetSingleton` missing from .cpp, not translated
❌ RELOCATION_ID — `AddSpell` has no relocation_func!, only todo!()
❌ override — `InitializeDataComponent` override comment missing
⚠️  vtable[42] — index uncertain, no .cpp RELOCATION_ID found
⚠️  VR ID — VariantID::new(X, Y, 0) — VR unknown, TODO needed
