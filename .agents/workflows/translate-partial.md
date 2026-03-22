---
name: translate-partial
description: Audit and complete a partially translated Rust file against its C++ source.
---

Audit and complete the partial translation for: {{args}}

## Steps

1. Read the existing Rust file in `libskyrim/src/re/`
2. Read the corresponding C++ header in `CommonLibVR/include/RE/`
3. Read the corresponding `.cpp` file — MANDATORY even if it seems complete.
   Extract and verify:
   - All `RELOCATION_ID(SE, AE)` pairs — are they all in `relocation_func!`?
   - Static methods (no `this`) — are they all present as associated functions?
   - Private helpers not in .h — are they present as non-pub `fn`?
   - `RelocateVirtual(SE, AE)` calls — are both vtable indices recorded?
   - Singleton / global accessor patterns — translated via `relocation_variable!`?

4. Produce a full gap analysis — list EVERYTHING missing:
   - Virtual functions present in C++ but absent in Rust
   - Member fields missing or with wrong types
   - Fields missing `// 0xOFFSET` comments
   - Missing `size_of` assertion
   - Missing `offset_of` assertions for mixin fields
   - Missing RTTI / VTable constants
   - Missing `inherit!` macro calls (primary or mixin)
   - Missing trait implementations (`FormCastable`, `RttiType`, `BSTHash`, `PartialEq`)
   - Missing `// override (ParentName)` comments for vtable overrides
   - `RELOCATION_ID` methods present in .cpp but missing `relocation_func!` in Rust
   - Static methods present in .cpp but absent in Rust
   - Private helper methods present in .cpp but absent in Rust
   - `relocation_func!` entries missing `// RELOCATION_ID SE: X, AE: Y` comment
   - Methods using `todo!()` instead of real `relocation_func!`

5. For each gap: add the missing piece with a comment `// ADDED: gap-fill`

6. Do NOT rewrite existing correct code — only append/patch

7. After all gaps are filled, run `translation-auditor` skill as final verification.
   Fix any remaining ❌ items before finishing.
