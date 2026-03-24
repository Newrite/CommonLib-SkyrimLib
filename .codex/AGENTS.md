# libskyrim Codex Guide

This file is the source of truth for Codex inside this repository.

## Scope

Use only these repository-local instruction sources:

- `.codex/AGENTS.md`
- `.codex/skills/*`

Ignore these paths unless the user explicitly asks for them:

- `.agents/**`
- `.qwen/**`
- agent files inside vendored dependencies such as `CommonLibVR/**`

## Project

`libskyrim` is a `#![no_std]` Rust translation layer for reverse-engineered
Skyrim runtime types and helpers. ABI compatibility with MSVC layouts is the
primary constraint. Wrong field types, offsets, vtable indices, or runtime
relocations can crash the game.

Key source locations:

- C++ headers: `CommonLibVR/include/RE/<Letter>/<Name>.h`
- C++ REX headers: `CommonLibVR/include/REX/**`
- C++ sources: `CommonLibVR/src/RE/<Letter>/<Name>.cpp`
- Rust RE output: `libskyrim/src/re/<snake_case>.rs`
- Rust REX support: `libskyrim/src/rex/*.rs`
- RE module index: `libskyrim/src/re/mod.rs`
- RTTI constants: `libskyrim/src/offsets/offsets_rtti.rs`
- VTABLE constants: `libskyrim/src/offsets/offsets_vtable.rs`
- relocation/runtime layer: `libskyrim/src/relocation.rs`, `libskyrim/src/runtime.rs`
- helper scripts: `scripts/generate_re_mod.py`, `scripts/generate_offsets.py`
  `scripts/audit_translation.py`, `scripts/bootstrap_translation.py`,
  `scripts/find_runtime_layout_candidates.py`,
  `scripts/find_extension_trait_candidates.py`,
  `scripts/check_generated_staleness.py`

## Hard Rules

- Read the matching `.cpp` whenever it exists. It is not optional.
- Do not collapse named bases or mixins into `[u8; N]`.
- Do not use `VariantID::new(se, ae, 0)` for `RELOCATION_ID`.
- Do not translate `ENABLE_SKYRIM_VR` literally into Rust `#[cfg]` in the
  cross-runtime RE layer unless the API really must disappear outside VR.
- Do not hand-maintain `libskyrim/src/re/mod.rs` when regeneration is intended.
- Do not hand-maintain generated offsets files when regeneration is intended.
- Do not add `todo!()` or `unimplemented!()` in translated RE code.
- Do not use `std`, `static mut`, or ad-hoc unwind assumptions.
- Do not derive `Debug` for raw RE structs unless manual debugging value is clear.

## Relocation Model

Use relocation types according to their real CommonLib semantics:

- `ID::new(id)`
  For real CommonLib `REL::ID(id)` selectors.

- `Offset::new(offset)`
  For real CommonLib `REL::Offset(offset)` selectors.

- `RelocationID::new(se, ae)`
  For `RELOCATION_ID(se, ae)` and the common Address Library case where VR reuses
  the SE ID.

- `RelocationID::with_vr(se, ae, vr)`
  Only when C++ truly provides a distinct VR ID.

- `VariantID::new(se, ae, vr_offset)`
  For real CommonLib `VariantID` semantics: SE and AE use Address Library IDs,
  VR uses a raw module-relative offset. Typical examples: RTTI and VTABLE
  constants.

- `VariantOffset::new(se, ae, vr)` or `VariantOffset::new_se_ae(se_ae, vr)`
  For runtime-varying offsets or vtable indices.

- `Relocation<T>`
  Low-level typed relocation wrapper. Prefer to consume it through
  `relocation_func!`, `relocation_variable!`, `define_vtable_hook!`, and
  `define_call_hook!`. Do not force explicit `Relocation<T>` into normal RE
  files when the macro layer already expresses the intent.

- `relocate(se_and_vr, ae)` and `relocate_vr(se, ae, vr)`
  Runtime value selectors for hook offsets or similar non-address values.

## Cross-Runtime Branching Model

CommonLib cross-VR code uses several different branching mechanisms. Translate
them by category instead of copying the preprocessor shape:

- layout or field-offset divergence
  Keep the common prefix in the main `repr(C)` type and move divergent tails
  behind runtime-data accessors.

- runtime-varying scalar values or indices
  Use `VariantOffset`, `relocate`, or `relocate_vr`.

- raw relocated addresses in `.cpp`
  Use `ID`, `Offset`, `RelocationID`, or `VariantID` according to the real
  CommonLib selector semantics.

- CommonLib wrapper methods implemented with `REL::RelocateVirtual(...)`
  Use `relocated_virtual_method!` for pure forwarding wrappers or
  `relocate_virtual!` inside a handwritten Rust method when only part of the
  method forwards to a vtable slot.

- methods where flat runtimes call a vtable slot but VR uses a custom shim
  Keep one Rust method and branch at runtime inside the method body.

- compile-time-only public surface that should genuinely not exist outside VR
  Only here should `#[cfg]` be considered.

## Runtime Layout Model

Rust type size is compile-time fixed. Do not try to encode SE/AE/VR size
differences into one `repr(C)` struct by hand. Use:

- a common-prefix struct
- runtime-data accessors from `libskyrim::runtime`
- runtime layout asserts for fields that differ by runtime

## Inheritance Model

- Primary base at offset `0x00`: field `base`, then `inherit!(Child : Parent)`
- Non-zero base/mixin: named field at exact offset, then
  `inherit!(Child => Mixin, field_name)`
- If a mixin with virtual methods is reused by children, add an extension trait
  in the mixin file.

## Virtual Model

- Parent classes define their virtual methods with `virtual_method!`.
- CommonLib `.cpp` wrapper methods backed by `REL::RelocateVirtual(...)` use
  `relocated_virtual_method!` when they are pure forwarding wrappers.
- If a CommonLib method mixes runtime branching with only a partial
  `RelocateVirtual(...)` call, write a normal Rust method and use
  `relocate_virtual!` inside the relevant branch.
- Child classes document inherited overrides with `// override (Parent)` comments.
- Do not duplicate inherited parent virtuals in the child unless the class
  really introduces a fresh virtual slot.

## Visitor Model

CommonLib uses several different visitor and functor shapes. Do not force them
into one abstraction. Translate by category:

- `std::function` convenience wrappers
  Prefer normal Rust closure helpers such as `for_each_*` or `visit_*_fn`.
  They do not need an ABI visitor layer.

- simple synchronous ABI visitors
  These are small base interfaces with one callback surface and source-backed
  synchronous call sites. Translate the ABI type faithfully and, when useful,
  add:
  - a raw ABI entrypoint
  - a typed Rust `*_with(&mut visitor)` helper
  - a closure sugar `*_fn(...)` helper

- unclear lifetime or retained callback cases
  If source does not prove the engine uses the visitor synchronously and does
  not retain it, expose only the raw ABI entrypoint. Do not promise a safe
  adapter.

- concrete stateful visitor classes
  Types like `ArmorRatingVisitorBase`, `MagicItemDataCollector`, or other
  visitor/functor descendants with their own fields and helper virtuals are
  ordinary RE types. Translate them directly; they are not generic adapters.

- concrete callback descendants of small ABI interfaces
  Types like `MagicCaster::PostCreationCallback : MagicTarget::IPostCreationModification`
  are also ordinary RE types. Translate the abstract callback base as an ABI
  interface and the concrete descendant directly as a nested RE type with real
  layout and inheritance. Do not add a safe Rust closure adapter unless
  source-backed code proves the callback is synchronous and non-retained.

- shared control enums
  Use `BSContainerForEachResult` for Rust-side synchronous traversal helpers
  instead of inventing per-file stop/continue enums.

## Event Model

`BSTEventSource<T>` and `BSTEventSink<T>` form a reusable low-level mixin layer.
Translate them once and reuse them by category:

- fixed-offset event bases
  Represent them as normal named mixin fields / inheritance edges and translate
  owner helpers such as `GetEventSource`, `AddEventSink`, `RemoveEventSink`, or
  `SendEvent` as thin forwarding wrappers.

- runtime-varying or runtime-exclusive event bases
  Do not fake one always-present base field. Use runtime cast / runtime-data
  accessors just like other non-zero-offset mixins that move between SE, AE,
  and VR.

- raw sink pointer APIs
  If a type only accepts `BSTEventSink<T>*` parameters and does not own an
  event base in translated Rust yet, keep the raw pointer API. Do not invent an
  owner layer just to look symmetrical.

- safety boundary
  The low-level `BSTEventSource` methods are intentionally `unsafe`: they store
  raw sink pointers and dispatch through foreign vtables. Only add safe owner-
  side wrappers when source proves the sink lifetime / ownership contract.

## Required Macro Layer

Prefer the provided macros instead of manual address math:

- `virtual_method!`
- `relocated_virtual_method!`
- `relocate_virtual!`
- `relocation_func!`
- `relocation_variable!`
- `define_vtable_hook!`
- `define_call_hook!`
- `runtime_data_accessor!`
- `runtime_pointer_accessor!`
- mutable and optional runtime-data accessors when the field is runtime-specific

## Generated Files

- `libskyrim/src/re/mod.rs`
  Generated by `scripts/generate_re_mod.py`. Prefer regenerating it after adding
  or removing RE files instead of hand-editing.

- `libskyrim/src/offsets/offsets_*.rs`
  Generated by `scripts/generate_offsets.py` from the C++ offsets headers.
  If offsets drift, regenerate from source instead of manually patching the Rust
  generated files.

## Helper Scripts

- `python scripts/bootstrap_translation.py <TypeName>`
  Gather the main header / cpp / Rust paths, direct bases, nested types, and a
  recommended skill pipeline before starting work.

- `python scripts/audit_translation.py <TypeName>`
  Fast artifact-level audit for one type. Use before editing when you want a
  quick gap picture without manually grepping the whole tree.

- `python scripts/find_runtime_layout_candidates.py`
  Find CommonLib types that likely need runtime-data accessors or split-layout
  handling.

- `python scripts/find_extension_trait_candidates.py`
  Find reusable mixins in `libskyrim/src/re/*.rs` that are inherited by other
  types and still lack an extension trait.

- `python scripts/check_generated_staleness.py`
  Check whether generated files such as `libskyrim/src/re/mod.rs` or
  `libskyrim/src/offsets/offsets_*.rs` are stale before or after a translation
  pass.

## RE Type Rules

- Every concrete layout type must have `#[repr(C)]` and a `size_of` assert.
- Every mixin field must have an `offset_of` assert.
- Every field must carry the C++ offset comment.
- Nested structs from C++ must be translated in the same Rust file before the
  parent struct.
- If an `libskyrim/src/re/*.rs` translation depends on source-backed helper
  structs, enums, aliases, or ABI types from `CommonLibVR/include/REX/**`, put
  those Rust translations in `libskyrim/src/rex/*.rs` and import them from
  `crate::rex`, not as ad-hoc local definitions inside the RE file.
- Treat `rex` as the home for shared RE extension support types rather than a
  mirror of the full Windows SDK. Keep it minimal and source-backed.
- `REX::Enum<E, U>` maps to `core_util::Enum<E, U>`.
- `REX::EnumSet<E, U>` and `stl::enumeration<E, U>` map to
  `core_util::EnumSet<E, U>`.
- Use `bitflags!` only for real bitmasks with power-of-two values.

## Workflow Map

Use these `.codex` skills for repository work:

- `translate-new`
  New type with no existing Rust file.
- `translate-partial`
  Existing file is incomplete and needs missing pieces added.
- `verify-translation`
  Full inheritance-chain verification with fixes.
- `translation-auditor`
  Read-only or light-touch audit / gap analysis.
- `add-extension-trait`
  Add or repair a mixin extension trait.
- `translate-runtime-layout`
  Runtime-split layout work for SE/AE/VR-divergent structs and accessors.
- `port-hooking`
  Port plugin-side hooks from CommonLib C++ to Rust using the current
  relocation/hook macro layer.

## Recommended Pipeline

### New ordinary RE type

1. `translation-auditor` only if the type looks suspicious or unusually complex
2. `python scripts/bootstrap_translation.py <TypeName>`
3. `translate-new`
4. `add-extension-trait` if the new type is a reusable virtual mixin
5. `verify-translation`
6. `python scripts/generate_re_mod.py`
7. `python scripts/check_generated_staleness.py`
8. standard validation commands

### Existing partial RE type

1. `python scripts/audit_translation.py <TypeName>`
2. `translate-partial`
3. `add-extension-trait` if needed
4. `verify-translation`
5. `python scripts/check_generated_staleness.py`
6. standard validation commands

### Runtime-divergent layout

1. `python scripts/find_runtime_layout_candidates.py --query <TypeName>`
2. `translation-auditor` or `verify-translation` to confirm divergence
3. `translate-runtime-layout`
   If the C++ branch is actually a method shim or `RelocateVirtual(...)`
   wrapper rather than a layout split, keep a normal RE translation and use the
   runtime branching rules above instead of forcing the type through split-tail
   layout work.
4. `verify-translation`
5. `python scripts/check_generated_staleness.py`
6. standard validation commands

### Hook porting from CommonLib-style C++

1. `python scripts/bootstrap_translation.py <TypeName>` when the hook targets a translated RE type
2. `port-hooking`
3. `python scripts/check_generated_staleness.py`
4. standard validation commands

### Offsets regeneration

1. update the C++ offsets source headers if needed
2. run `python scripts/generate_offsets.py`
3. run `python scripts/check_generated_staleness.py`
4. validate the resulting Rust build

## Validation

Default verification after edits:

- `cargo fmt`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`
- `python scripts/check_generated_staleness.py` when generated files may have changed

If a command cannot be run, say so explicitly.
