---
name: verify-translation
description: Deep verification of an existing Rust translation against the full C++ inheritance chain.
---

Perform deep verification of the Rust translation for: {{args}}

## Verification Protocol

1. Read `libskyrim/src/re/<type>.rs`
2. Read `CommonLibVR/include/RE/` header for the same type
3. Read `CommonLibVR/src/RE/` .cpp for the same type — MANDATORY
4. Recursively trace the FULL C++ inheritance chain to root.
   For each class in the chain: read its .h AND .cpp before verifying.
5. Run `translation-auditor` skill for detailed field/vtable comparison.
6. For each class in the chain, verify ALL of the following:

### Layout Checks
- Field order matches exactly (C++ offset comments vs Rust field positions)
- Every Rust field has `// 0xOFFSET` comment matching C++ source
- `size_of` assertion value matches C++ `sizeof` comment
- `offset_of` assertions exist for every mixin field and match C++ offsets
- No padding gaps — if present, `_pad: [u8; N]` field exists

### VTable Checks
- Virtual function count matches between C++ and Rust
- Every vtable index is correct (0-based, continuous, no gaps)
- `RelocateVirtual(SE, AE)` calls have both indices recorded as comments
- `// override (ParentName)` comments present for all overridden virtuals
- No `bytemuck::Zeroable` on any polymorphic type

### RELOCATION_ID Checks
- Every `RELOCATION_ID(se, ae)` in .cpp has a corresponding `relocation_func!`
- Every `relocation_func!` has `// RELOCATION_ID SE: X, AE: Y` comment above it
- No hardcoded raw IDs — all use `VariantID::new(se, ae, vr)`
- No `todo!()` standing in for a real `relocation_func!`
- VR ID = 0 entries have `// TODO: VERIFY — VR ID unknown` comment

### Static & Private Method Checks
- All static methods from .cpp present as associated functions (no `&self`)
- All private helpers from .cpp present as non-pub `fn`

### Trait & Macro Checks
- `RttiType` implemented if class has RTTI
- `FormCastable` implemented if class inherits `TESForm`
- `BSTHash` / `PartialEq` implemented if C++ has `BSCRC32_` / `operator==`
- `inherit!` macro present for every base class (primary and each mixin)
- All offset/RTTI/VTable constants imported from offsets modules, not hardcoded

## Report Format

For each check output one line:
✅ layout   — all fields match with correct offset comments
✅ static   — `GetSingleton` present as associated fn
✅ reloc    — SE: 37787 AE: 38736 matches relocation_func! with comment
❌ layout   — offset 0x10: missing `flags` field (C++ has `ObjRefFlags`)
❌ vtable   — virtual[42] missing, gap in index sequence
❌ reloc    — `AddSpell` has todo!() instead of relocation_func!
❌ override — `InitializeDataComponent` missing `// override (BaseFormComponent)`
❌ static   — `LookupByHandle` in .cpp not translated
⚠️  vtable  — index uncertain, no .cpp RELOCATION_ID found
⚠️  VR ID   — VariantID::new(X, Y, 0) — VR unknown, TODO comment missing

## After Report
- Apply fixes for ALL ❌ items automatically
- For ⚠️ items: add `// TODO: VERIFY — <reason>` comment and continue
- Re-run checks after fixes to confirm all ❌ resolved
