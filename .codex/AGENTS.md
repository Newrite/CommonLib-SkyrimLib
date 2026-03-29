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
- FFI bridge layer: `libskyrim/src/ffi.rs`, `libskyrim/cpp/src/bridge.cpp`
- helper scripts: `scripts/generate_re_mod.py`, `scripts/generate_offsets.py`
  `scripts/audit_translation.py`, `scripts/bootstrap_translation.py`,
  `scripts/find_runtime_layout_candidates.py`,
  `scripts/find_extension_trait_candidates.py`,
  `scripts/check_generated_staleness.py`

## Hard Rules

- Read the matching `.cpp` whenever it exists. It is not optional.
- Do not worsen the Rust-facing API only to mirror a C++ signature literally
  when that literal shape carries no real behavioral, layout, ownership, or ABI
  difference.
- When a Rust RE translation corresponds to a same-name CommonLib header/source
  file pair, carry over the full source-backed data surface from that file
  pair, not only the layout-critical fields and methods. This includes nested
  enums, named index layers for arrays, totals/default constants, nested helper
  types, and private/source-only helpers that belong to the same translated
  type.
- Do not collapse named bases or mixins into `[u8; N]`.
- Do not use `VariantID::new(se, ae, 0)` for `RELOCATION_ID`.
- Do not translate `ENABLE_SKYRIM_VR` literally into Rust `#[cfg]` in the
  cross-runtime RE layer unless the API really must disappear outside VR.
- Do not hand-maintain `libskyrim/src/re/mod.rs` when regeneration is intended.
- Do not hand-maintain generated offsets files when regeneration is intended.
- Do not add `todo!()` or `unimplemented!()` in translated RE code.
- Do leave a source-backed `// TODO:` comment at the exact compromise site when
  you intentionally keep a raw-pointer stand-in, opaque stub, missing helper, or
  other honest gap because a required dependency, bridge, allocator/factory
  path, destructor/delete path, or ownership contract is not translated yet.
- Do not leave vague TODOs. State the current compromise, the missing
  prerequisite, and the intended end state.
- Do not use `std`, `static mut`, or ad-hoc unwind assumptions.
- Do not derive `Debug` for raw RE structs unless manual debugging value is clear.

## Rust API Surface Policy

Preserve source-backed fidelity where it matters:

- memory layout
- field types and offsets
- inheritance and vtable slot ownership
- relocation semantics
- pointer ownership and lifetime contracts
- any wrapper behavior that meaningfully changes what the engine call can do

Do not preserve C++ surface details mechanically when they only make the Rust
API worse and do not carry a real behavioral difference.

Typical examples:

- a getter or name helper that is semantically read-only may use `&self` in
  Rust even if the CommonLib wrapper is not `const`
- a raw `*const c_char` / `const char*` getter may keep the low-level method
  but should usually also expose an ergonomic `*_as_str()` helper
- a low-level raw pointer entrypoint may stay available while higher-level
  wrappers use `Option<&T>`, `GameRef`, or similar Rust-facing helpers when the
  contract is clearer that way

When relaxing a literal C++ signature:

- keep the source-backed low-level semantics honest
- do not hide mutation, ownership transfer, retained callbacks, or other real
  contracts
- prefer adding or repairing an ergonomic wrapper/helper over inventing new
  engine behavior
- if there is any real uncertainty, keep the raw/strict method and add the more
  ergonomic helper alongside it instead of narrowing the contract silently

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

When the type already exists and the problem is not "translate the missing tail"
but "turn a large, already-usable runtime object into a maintainable API",
treat that as a runtime-layout refactor, not a plain translation pass. In that
mode:

- keep the low-level raw runtime surface honest
- extract shared raw sub-structures for meaningful common blocks, not only for
  the initial common prefix
- prefer one owner-side `runtime_view()` / `runtime_view_mut()` facade to hide
  repeated `is_se() / is_ae() / is_vr()` branching from consumers
- if the remaining common overlap is only a few fields, add direct runtime-aware
  getter/setter helpers instead of inventing tiny one-off structs just to avoid
  branches
- keep runtime-specific raw entrypoints available when the underlying layout is
  genuinely runtime-specific

For runtime accessor naming:

- bare `*_runtime_data()` is reserved for a truly common raw layout
- use `*_flat` for `SE + AE` raw layouts that intentionally exclude `VR`
- use `*_se`, `*_ae`, and `*_vr` for runtime-specific raw layouts
- use `*_view()` / `*_view_mut()` for the ergonomic facade that centralizes
  runtime branching without pretending the raw layout is universal

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

## Smart Pointer and Construction Model

- Reuse the shared smart-pointer layers instead of collapsing source-backed
  `NiPointer<T>`, `BSTSmartPointer<T>`, or `hkRefPtr<T>` fields to raw pointers
  once the pointee family is supported honestly in Rust.
- When CommonLib constructs a smart pointer on the C++ side, such as
  `make_hkref<T>()`, `make_nismart<T>()`, `make_smart<T>()`, or a helper that
  fills a smart-pointer out-param, prefer the ABI-safe out-param bridge pattern:
  - C++ bridge entrypoint in `libskyrim/cpp/src/bridge.cpp`
  - matching FFI declaration in `libskyrim/src/ffi.rs`
  - Rust-side construction through `ffi::try_construct_out_param(...)`
  - typed helper on `hkRefPtr::try_construct_with(...)`,
    `NiPointer::try_construct_with(...)`, or
    `BSTSmartPointer::try_construct_with(...)`
- Do not emulate a nontrivial C++ constructor with raw Rust allocation alone.
  Allocation helpers are not a substitute for a real engine constructor or
  factory path.
- If source-backed C++ shows that a `make_*` path is unavailable for the target
  type, for example because the constructor is deleted or the type only has
  argumentful factories, do not fake a zero-init construction path. Keep the
  current honest surface and leave a `// TODO:` comment describing the missing
  source-backed factory/constructor prerequisite.

## TODO Comment Policy

Use comment TODOs to preserve honesty when a translation cannot be completed
yet, while still forbidding executable placeholders such as `todo!()`.

- Put the TODO at the exact field, helper, or type stub where the compromise
  lives.
- Prefer one of these forms:
  - `// TODO: VERIFY - replace with full translation when layout or methods are needed.`
  - `// TODO: SOURCE - replace <current stand-in> with <target> after <missing prerequisite>; source: <Header.h/.cpp reason>.`
- Remove or rewrite stale TODOs when the blocker is resolved or when the reason
  changes.
- Mention every remaining code TODO in the final user report for that task.

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
- `runtime_cast_accessor!` / `runtime_cast_mut_accessor!` for moved mixin/base
  accessors, including cases that already have a named `VariantOffset` constant

Do not introduce local helper methods such as `moved_base_ref`, `moved_base_mut`,
or other ad-hoc pointer-arithmetic wrappers when the current runtime accessor
macros can express the same surface. If a macro cannot currently express the
source-backed pattern, extend the shared macro layer in `libskyrim/src/runtime.rs`
instead of copying a one-off helper into each RE file.

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
- Matching-name translations should still mirror the source-backed nested data
  surface even when a nested enum or helper only names array slots, presets, or
  other non-layout metadata. Do not reduce such layers to bare `*_TOTAL`
  constants when the C++ file provides the named layer.
- If an `libskyrim/src/re/*.rs` translation depends on source-backed helper
  structs, enums, aliases, or ABI types from `CommonLibVR/include/REX/**`, put
  those Rust translations in `libskyrim/src/rex/*.rs` and import them from
  `crate::rex`, not as ad-hoc local definitions inside the RE file.
- If a translated RE file needs a non-opaque dependency from another CommonLib
  RE header, do not keep that dependency as a local `*View`, `*Fields`, or
  similarly renamed stand-in inside the consumer file just because the proper
  Rust file does not exist yet.
- Instead, create or extend the matching Rust file for that dependency
  (`AIProcess` -> `libskyrim/src/re/ai_process.rs`, etc.) and place the minimal
  source-backed partial translation there.
- Such partial dependency translations should keep the real C++ type name when
  they represent that type. If only a subset of fields is currently needed,
  translate that subset in the dependency file as a partial but honestly named
  type rather than inventing a consumer-local `AIProcessView`.
- During verification or partial-translation passes, treat consumer-local
  layout views for real cross-file RE dependencies as mismatches to fix unless
  the helper is genuinely private to the same C++ file and does not correspond
  to a named external type.
- Apply the same rule to runtime-tail access: if consumer code needs fields from
  another named RE type's runtime data, prefer owner-side helper/accessor
  methods on that type backed by the shared runtime macros instead of copying
  foreign pointer arithmetic into the consumer file.
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
- `refactor-runtime-layout`
  Refactor an existing runtime-divergent type into shared raw blocks, honest
  runtime-specific accessors, and an ergonomic `runtime_view` surface.
- `port-hooking`
  Port plugin-side hooks from CommonLib C++ to Rust using the current
  relocation/hook macro layer.
- `reconcile-api-ergonomics`
  Repair overly literal Rust-facing RE/SDK APIs when a direct C++ mirror harms
  ergonomics without preserving a meaningful behavioral difference.

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
3. `reconcile-api-ergonomics` if the public Rust surface still feels more
   literal than semantically necessary
4. `add-extension-trait` if needed
5. `verify-translation`
6. `python scripts/check_generated_staleness.py`
7. standard validation commands

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

### Existing runtime-layout refactor

Use this when the type is already translated enough to be usable, but its
runtime-tail API has grown awkward, overly local, or too dependent on repeated
manual branching.

1. `python scripts/audit_translation.py <TypeName>`
2. `refactor-runtime-layout`
3. `verify-translation`
4. `python scripts/check_generated_staleness.py`
5. standard validation commands

### Hook porting from CommonLib-style C++

1. `python scripts/bootstrap_translation.py <TypeName>` when the hook targets a translated RE type
2. `port-hooking`
3. `python scripts/check_generated_staleness.py`
4. standard validation commands

### Rust-facing API ergonomics reconciliation

Use once the low-level contract is already understood, either as a late
translation pass before final verification or as a post-verification cleanup
when the file or SDK surface is source-backed but unnecessarily awkward for
Rust consumers.

1. `translation-auditor` or `verify-translation` to confirm the low-level
   source-backed contract first
2. `reconcile-api-ergonomics`
3. `python scripts/check_generated_staleness.py` if generated files may have
   changed
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
