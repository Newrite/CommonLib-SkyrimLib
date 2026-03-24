---
name: translation-auditor
description: >
  Audit an existing Rust translation against CommonLibVR without forcing a full
  rewrite. Use for gap analysis, field/vtable verification, relocation review,
  or a read-first pass before heavier translation work.
---

Audit the requested translation and compare it to CommonLibVR.

## Use This Skill When

- the user wants a gap report first
- the file may already be mostly correct
- you need to confirm layout, vtable, or relocation choices before editing
- you want a lighter pass than `verify-translation`

## Audit Checklist

- header vs Rust field layout
- nested structs
- named nested enums, index layers, totals/default constants, and other
  source-backed helper layers from the matching same-name header/source file
- mixin and parent representation
- source-backed `REX/**` dependencies that should live in `libskyrim/src/rex/*.rs`
  instead of as local RE-file definitions
- vtable slot ownership
- `ENABLE_SKYRIM_VR` / `REL::Module::IsVR()` branch category
- `RelocateVirtual(...)` wrappers vs true virtual-slot ownership
- visitor/functor category: closure wrapper, raw ABI visitor, or concrete
  stateful visitor type
- callback descendant category: abstract ABI callback base plus concrete nested
  callback type, as in
  `MagicCaster::PostCreationCallback : MagicTarget::IPostCreationModification`
- event category: fixed event base, runtime-varying event base, or raw sink
  pointer-only API
- override comment coverage
- `.cpp` relocated methods
- `RelocationID` vs `VariantID` usage
- `ID` / `Offset` usage
- runtime-data accessor opportunities
- `BSContainerForEachResult` unification opportunities for Rust-side sync traversal
- extension-trait needs for reusable mixins
- stale TODOs, placeholders, or compatibility hacks

## Helper Scripts

- Run `python scripts/audit_translation.py <TypeName>` for the quick local
  artifact picture before reading large source trees.
- If the type smells runtime-divergent, run
  `python scripts/find_runtime_layout_candidates.py --query <TypeName>` to
  confirm whether `translate-runtime-layout` should be part of the follow-up.

## Reporting

Prefer a concise finding list ordered by severity:

- wrong layout or wrong relocation semantics first
- missing methods or traits next
- style or cleanup items last

If the user asked for fixes, patch the confirmed issues after the audit.
Otherwise keep the pass read-oriented.
