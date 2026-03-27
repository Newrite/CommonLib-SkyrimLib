# `libskyrim::sdk`

`libskyrim::sdk` is the planned high-level Rust-first layer for plugin authors.
It sits above the source-backed ABI layers in `libskyrim::re` and
`libskyrim::skse`.

## Purpose

The low-level layers have a different job from the future SDK:

- `re`
  Faithful CommonLib/RE translations, runtime-aware layout handling, relocated
  wrappers, and ABI-correct ownership boundaries.
- `skse`
  Faithful SKSE interface layer and thin Rust facades over SKSE services.
- `sdk`
  Domain-oriented ergonomic APIs for common plugin work: lifecycle, Papyrus,
  forms, events, tasks, messaging, gameplay helpers, and selective hook sugar.

The SDK is intentionally not a mirror of CommonLib type names. It should group
behavior by plugin author intent rather than by reverse-engineered class name.

## Design Constraints

The SDK must remain stable while the translated `RE` layer continues to grow.
That means:

- new `re` files do not automatically imply new `sdk` wrappers
- the SDK should prefer operation-oriented APIs over field-oriented APIs
- the SDK should hide runtime divergence where possible
- the SDK must always preserve an escape hatch to low-level `re` / `skse`
- the SDK should depend on public Rust surfaces, not call `ffi` directly
- advanced or partially translated engine families should not become part of
  the early default user surface

The SDK must also tolerate future upstream growth:

- new CommonLibVR translations may enrich `re` without forcing churn in `sdk`
- new community-contributed `re` or `skse` types should only be wrapped after a
  stable user-facing domain boundary becomes clear
- runtime-divergent fields or low-level implementation details should remain in
  `re`, not leak into stable SDK signatures unless there is no practical
  alternative

## What Belongs Here

Good SDK candidates:

- plugin lifecycle helpers
- structured message registration
- Papyrus registration sugar
- settings and config helpers
- common form lookup and filtering workflows
- owner-proven safe event registration helpers
- gameplay-centric helpers around actors, player state, camera, inventory,
  magic, quests, and menus
- task queue convenience wrappers
- selective hook helpers where the safety/intent boundary is clearer than raw
  trampoline usage

Poor SDK candidates:

- direct mirrors of `RE::*` layout types
- ad-hoc wrappers around unstable runtime-data fields
- heavy `Ni*`, `hk*`, `bhk*`, or `hkb*` types before there is a clear consumer
  use case and a stable abstraction boundary
- wrappers that bypass the low-level public Rust API and reach into `ffi`

## Domain Admission Rules

Not every translated low-level type belongs in the SDK. A new wrapper should
usually satisfy all of the following:

- it serves a recurring gameplay, plugin, UI, Papyrus, or interoperability
  workflow
- it can be explained in terms of behavior rather than raw field access
- it can hide runtime selection or ownership complexity behind a stable API
- it still leaves a straightforward path back to `re` / `skse`
- it remains useful even if more `RE` types are translated later

When these conditions are not met yet, the type should stay exclusively in the
low-level layer until a clearer domain abstraction emerges.

## Native Ownership Model

The SDK should prefer Bethesda-native ownership and identity types instead of
inventing replacement pointer systems.

The intended model is:

- borrowed access inside a callback or hook:
  `&T`, `&mut T`, and small SDK wrappers such as `GameRef<'a, T>`
- engine identity across frames or save/load boundaries:
  native handle types such as `ActorHandle`, `ObjectRefHandle`, and similar
  `BSPointerHandle`-style wrappers
- engine-owned refcounted retention:
  native smart pointers such as `NiPointer<T>`, `BSTSmartPointer<T>`, and
  `hkRefPtr<T>`

The SDK should compose these, not erase them. High-level helpers may wrap them
for ergonomics, but the native Bethesda types remain first-class values.

Planned shared SDK support types:

- `GameRef<'a, T>`
  borrowed nullable engine reference used across Papyrus, events, and hooks
- `GameRefMut<'a, T>`
  nullable mutable borrowed engine reference for APIs that can honestly hand
  out mutable access
- `ResolvableHandle`
  trait for native handles that can resolve to an engine-owned smart pointer
- `ResolvedHandle<H>`
  pair of a native handle and its resolved native smart pointer
- `HandleFamilyTarget`
  trait for runtime object families that share a canonical Bethesda handle root
- `Resolved<T>`
  typed resolved runtime object built on a canonical handle family, for example
  `Resolved<Actor>`, `Resolved<Hazard>`, or `Resolved<MissileProjectile>`
- `NativeOwner`
  small trait surface for native smart-pointer families
- `NativeOwnerFamily<T>`
  relationship between one native owner family and another pointee type in the
  same ownership model
- `DynamicCastExt` / `DynamicCastMutExt`
  shared RTTI-backed cast helpers for borrowed refs, native owners, and
  resolved handles

## Domain Layout

The SDK is split into domain-first modules:

```text
sdk/
  mod.rs
  prelude.rs

  core/
    refs.rs
    owners.rs
    handles.rs
    cast.rs
    ptr.rs
    phase.rs
    error.rs

  plugin/
    entry.rs
    lifecycle.rs
    messaging.rs
    log.rs
    config.rs
    task.rs
    serialization.rs

  papyrus/
    registry.rs
    context.rs
    types.rs
    macros.rs

  forms/
    lookup.rs
    keywords.rs
    lists.rs
    settings.rs

  gameplay/
    actors.rs
    player.rs
    inventory.rs
    magic.rs
    combat.rs
    camera.rs
    input.rs
    quests.rs

  events/
    game.rs
    input.rs
    skse.rs

  ui/
    menus.rs
    notifications.rs
    controls.rs
    scaleform.rs

  interop/
    messaging.rs
    external_api.rs

  hooks/
    trampoline.rs
    patch.rs
    patterns.rs

  persistence/
    cosave.rs

  advanced/
    scene.rs
    physics.rs
    vm.rs
    render.rs
```

## Expected Stability

The first waves of SDK work should focus on:

1. `plugin`
2. `papyrus`
3. `forms`
4. `events`
5. `gameplay`

The following domains should come later:

- `hooks`
- `ui::scaleform`
- `advanced::scene`
- `advanced::physics`
- `advanced::vm`
- `advanced::render`

These later domains depend more heavily on engine internals, runtime splits, or
ongoing `RE` translation work.

## Evolution Rules

The intended migration path is:

1. translate and verify the low-level `re` / `skse` surface first
2. identify repeated consumer workflows across real plugins
3. add a narrow SDK wrapper that composes existing public Rust surfaces
4. keep the wrapper small until real usage suggests a broader abstraction
5. only then consider prelude exports or stronger stability expectations

This keeps the SDK resilient even while the low-level translation continues to
grow, split, and receive community contributions.

## Escape Hatch Policy

Every meaningful SDK wrapper should preserve a way back to the low-level layer.
Typical patterns:

- `raw()` for underlying pointers
- `abi()` or `as_re()` / `as_skse()` accessors
- explicit generic parameters over `re` / `skse` types when needed

The SDK should reduce friction, not block advanced users from dropping to the
ABI layer when they need functionality that has not been wrapped yet.

## Hooking Design

The hook stack is intentionally split into two layers:

- low-level hook primitives in `libskyrim::relocation`
- high-level ergonomic hooks in `libskyrim::sdk::hooks`

The low-level layer remains raw and explicit. The high-level SDK layer is where
null guards, handle resolution, smart-pointer adapters, and user-friendly
signatures belong.

### High-Level Hook Syntax

The high-level user-facing SDK syntax is attribute-driven. `sdk::hooks`
re-exports four separate attributes:

- `function_hook`
- `call_hook`
- `vtable_hook`
- `vcall_hook`

Users can import the module and apply the attribute with any ordinary free
function name; the hook function does not need to be called `detour`.

The `hooks` namespace also re-exports the common hook-facing wrapper types:

- `GameRef`
- `GameRefMut`
- `Resolved`
- `ResolvedHandle`

```rust
use libskyrim::sdk::hooks;

#[hooks::function_hook(
    target = RelocationID::new(123, 456),
    guard = hooks::guards::default(),
    convert_fail = return_(false)
)]
fn invert_actor_bool(
    original: hooks::Original<fn(&RE::Actor, bool) -> bool>,
    actor: &RE::Actor,
    value: bool,
) -> bool {
    !original.call(actor, value)
}

#[hooks::call_hook(
    target = RelocationID::new(123, 456),
    offset = VariantOffset::new(0x15, 0x18, 0x15),
    size = 5,
    invalid = skip
)]
fn maybe_skip_call(
    original: hooks::Original<fn(GameRef<'_, RE::TESObjectREFR>, f32)>,
    refr: &RE::TESObjectREFR,
    delta: f32,
) {
    original.call(refr, delta);
}

#[hooks::vtable_hook(vtable = RE::VTABLE_PlayerCharacter, slot = 0x518, invalid = original)]
fn patch_player_vfunc(
    original: hooks::Original<fn(&mut RE::PlayerCharacter)>,
    player: &mut RE::PlayerCharacter,
) {
    original.call(player);
}

#[hooks::vcall_hook(
    target = RelocationID::new(123, 456),
    offset = 0x54,
    size = 6,
    receiver = actor,
    slot = 0x2E0,
    invalid = return_(0usize)
)]
fn patch_virtual_call_site(
    original: hooks::Original<fn(*mut RE::Actor, Resolved<RE::Actor>, u32) -> usize>,
    actor: *mut RE::Actor,
    target: Resolved<RE::Actor>,
    source: u32,
) -> usize {
    original.call(actor, target, source)
}
```

`offset`, `index`, and `slot` accept ordinary constants as well as runtime-aware
expressions such as `VariantOffset`.

Each attributed function generates a sibling hook module named
`<function_name>_hook` with install helpers:

- `is_installed() -> bool`
- `try_install() -> Result<(), HookInstallError>`
- `install() -> Result<(), HookInstallError>`
- `INSTALLER: HookInstaller`
- `installer() -> HookInstaller`
- `install_or_fatal()`

This keeps installation fallible by default while still allowing plugin startup
code to opt into fatal install semantics.

### Guard Presets

For the common “all invalid hook arguments should behave the same way” case, the
attribute layer supports a named `guard = ...` preset:

```rust
#[hooks::function_hook(target = RelocationID::new(123, 456), guard = hooks::guards::original())]
#[hooks::function_hook(target = RelocationID::new(123, 456), guard = hooks::guards::default())]
#[hooks::call_hook(target = RelocationID::new(123, 456), offset = 0x10, size = 5, guard = hooks::guards::skip())]
```

Currently supported preset functions are:

- `hooks::guards::original()`
- `hooks::guards::default()`
- `hooks::guards::skip()`

Presets set the base invalidity policy and can still be refined with explicit
overrides such as `null = ...`, `unresolved = ...`, or `convert_fail = ...`.

### Hook Function Rules

The current high-level attributes are intentionally narrower than the raw hook
layer. They support ordinary free functions with simple identifier parameters.
They do not support:

- methods
- generic hook functions
- `async`
- `const`
- `extern`
- variadics

The escape hatch for those cases remains the low-level relocation macro layer.

### Guard and Conversion Rules

Hook argument adaptation is driven by the user-visible signature:

- `&T` / `&mut T`
  strict non-null borrowed argument; failure triggers hook guard policy
- `Option<&T>` / `Option<&mut T>`
  null is accepted and delivered as `None`
- `GameRef<'_, T>`
  nullable borrowed SDK wrapper
- `GameRefMut<'_, T>`
  nullable mutable SDK wrapper
- `NiPointer<T>` / `BSTSmartPointer<T>` / `hkRefPtr<T>`
  native owner form for retention-capable APIs
- native handle types such as `ActorHandle`
  delivered directly when the ABI already uses handles
- `ResolvedHandle<H>`
  requires successful handle resolution before user code runs
- `Option<ResolvedHandle<H>>`
  unresolved handles are accepted as `None`
- `Resolved<T>`
  requires canonical handle-family resolution plus a successful RTTI cast
- `Option<Resolved<T>>`
  null is accepted as `None`; failed resolution/cast still triggers guard policy
- raw pointer or plain `Copy` ABI types
  passed through unchanged for low-level or hybrid hooks

The guard policy is wider than just `on_null`. Current invalidity kinds:

- `null`
- `unresolved`
- `convert_fail`

Current policy spelling:

```rust
#[hooks::function_hook(target = RelocationID::new(123, 456), invalid = original)]
#[hooks::function_hook(target = RelocationID::new(123, 456), invalid = default)]
#[hooks::function_hook(target = RelocationID::new(123, 456), invalid = return_(false))]
#[hooks::function_hook(
    target = RelocationID::new(123, 456),
    null = original,
    unresolved = return_(false),
    convert_fail = default
)]
```

If a strict parameter cannot be validated or converted, user code is skipped and
the configured guard policy runs instead.

Policy meanings:

- `original`
  call the original target/call-site/vfunc/vcall
- `skip`
  early-return without calling the original; only valid for hooks returning `()`
- `default`
  return `Default::default()` for the hook return type
- `return_(expr)`
  return the given expression directly

There is intentionally no implicit `skip` for non-void hooks. Use `default` or
`return_(...)` instead.

### Error Handling

Hook installation is explicitly fallible. `try_install()` and `install()`
return [`HookInstallError`](/S:/Programming/CommonLib-SkyrimLib/libskyrim/src/sdk/hooks/runtime.rs),
which currently wraps:

- relocation/backend install failures
- duplicate installation attempts

Those errors can be handled in normal plugin initialization code:

```rust
if let Err(error) = my_hook_hook::try_install() {
    // log, downgrade feature, or abort plugin load
}
```

Runtime argument-adaptation failures do not bubble out as `Result`. They are
handled inside the generated wrapper by the configured guard policy, so hot hook
paths stay predictable and allocation-free.

### Install Batching

The hook layer now includes typed installers and batch helpers. Every generated
hook module exposes an `INSTALLER` constant of type `HookInstaller`, and
`sdk::hooks` exposes both function-based and macro-based batch installation:

```rust
hooks::try_install_all(&[
    some_hook_hook::INSTALLER,
    other_hook_hook::INSTALLER,
])?;

hooks::install_all![some_hook_hook, other_hook_hook]?;
hooks::install_all_or_fatal![some_hook_hook, other_hook_hook];
```

Batch installation returns `HookBatchError`, which preserves both the hook name
and the underlying `HookInstallError`.

## Low-Level Hook Escape Hatch

For users who want raw hooks without the future SDK layer, `libskyrim` exposes a
uniform low-level `hook! { ... }` macro in the relocation layer.

This hatch complements the existing `define_call_hook!` and
`define_vtable_hook!` macros. It currently supports:

- `function`
  universal function-entry detour hook backed by MinHook when `size` is omitted
- `detour`
  first-party auto-sized function-entry detour hook backed by `iced-x86`
- `universal`
  alias for the MinHook-backed universal function-entry detour hook
- `call`
  call-site hook
- `vcall`
  virtual call-site hook for patching one specific indirect virtual call while
  preserving dynamic dispatch in `original(...)`
- `vtable`
  vtable slot hook

### `hook!` Function Syntax

```rust
hook! {
    pub function InvertHook {
        target: RelocationID::new(123, 456),
        size: 12,
        fn detour(actor: *mut RE::Actor, value: bool) -> bool {
            !original(actor, value)
        }
    }
}
```

Supported aliases:

- `target: ...`
- `address: ...`

`size` is the exact number of original prologue bytes stolen from the target
function and relocated into the trampoline returned as `original(...)`.

If `size` is omitted, `hook! { function ... }` uses the universal MinHook
backend:

```rust
hook! {
    pub function UniversalHook {
        target: RelocationID::new(123, 456),
        fn detour(actor: *mut RE::Actor, value: bool) -> bool {
            !original(actor, value)
        }
    }
}
```

Equivalent alias:

```rust
hook! {
    pub universal UniversalHook {
        target: RelocationID::new(123, 456),
        fn detour(actor: *mut RE::Actor, value: bool) -> bool {
            !original(actor, value)
        }
    }
}
```

If you want the first-party `iced-x86` backend instead, use `detour`:

```rust
hook! {
    pub detour AutoDetourHook {
        target: RelocationID::new(123, 456),
        fn detour(actor: *mut RE::Actor, value: bool) -> bool {
            !original(actor, value)
        }
    }
}
```

### `hook!` Call Syntax

```rust
hook! {
    pub call SomeCallHook {
        target: RelocationID::new(123, 456),
        offset: 0x2A,
        size: 5,
        fn detour(this: *mut RE::TESObjectREFR, value: u32) {
            original(this, value);
        }
    }
}
```

### `hook!` Virtual Call Syntax

```rust
hook! {
    pub vcall SomeVirtualCallHook {
        target: RelocationID::new(123, 456),
        offset: 0x54,
        size: 6,
        receiver: actor,
        slot: 0x2E0,
        fn detour(actor: *mut RE::Actor, source: u32) -> *mut RE::MagicCaster {
            original(actor, source)
        }
    }
}
```

Supported aliases:

- `target: ...`
- `address: ...`
- `index: ...`
- `slot: ...`

`vcall` is for the case where one specific instruction performs a virtual call,
for example `call qword ptr [rax+2E0h]`, and you want to hook only that call
site rather than all uses of the same vtable slot.

`receiver` names the callback parameter that holds the virtual receiver object.
`original(...)` re-reads the current vtable from that receiver and dispatches
through the configured slot or index, instead of using one fixed original code
address.

`slot` is the byte offset inside the vtable, matching disassembly output.
`index` is the pointer-sized vtable entry index. Both may be provided as
runtime-aware offset expressions such as `VariantOffset`; when `slot` is used,
`libskyrim` validates that it is aligned to pointer size before converting it
to an index.

### `hook!` VTable Syntax

```rust
hook! {
    pub vtable SomeVTableHook {
        vtable: RE::VTABLE_PlayerCharacter,
        index: 0xA3,
        fn detour(this: *mut RE::PlayerCharacter) {
            original(this);
        }
    }
}
```

Supported aliases:

- `index: ...`
- `offset: ...`

Each generated hook module exposes:

- `install()`
- `original(...)`
- `original_relocation()`

This keeps the raw hatch small and predictable while leaving room for a richer
SDK hook layer on top.

## Why `sdk`, Not `api`

`skse::api` already exists as a low-level typed getter layer for SKSE services.
Using `sdk` for the high-level domain layer avoids confusion between:

- low-level `skse::api`
- future high-level consumer-facing APIs

## Instruction Files

This README is documentation for humans and for deliberate design work.

For Codex in this repository, repository-local instruction lookup is governed by
`.codex/AGENTS.md` and `.codex/skills/*`. Adding an `AGENTS.md` inside
`sdk/` would not automatically make it an authoritative instruction source for
Codex during normal work in this repository.

If SDK-specific Codex behavior is needed, put it in:

- `.codex/AGENTS.md` when it should apply repository-wide
- a dedicated `.codex/skills/...` skill when it should be opt-in or task-shaped

## Current Status

The SDK is no longer documentation-only.

Implemented foundation:

- `sdk::core`
  shared `GameRef` / `GameRefMut`, native-owner traits, handle resolution, and
  shared RTTI cast helpers
- `sdk::papyrus`
  SDK-facing facade over the current high-level Papyrus authoring layer in
  `skse::papyrus`, including callback-facing `GameRef`, `GameRefMut`,
  `ResolvedHandle<H>`, `Resolved<T>`, and `Option<...>` parameter/base support
- `sdk::core::handles`
  family-based resolved runtime-object model with immediate support for the
  `Actor`, `TESObjectREFR`, and `Projectile` handle families
- `sdk::plugin::log`
  facade over the current SKSE logging convenience layer
- `sdk::plugin::task`
  facade over the current SKSE task helpers
- `sdk::plugin::messaging`
  facade over the current message-listener registration layer
- `sdk::plugin::entry`
  light SDK wrappers for common `LoadInterface` initialization paths
- `sdk::plugin::config`
  early INI-focused helpers built on the existing `Ini` surface

Still intentionally placeholder-heavy:

- `sdk::lifecycle`
- `sdk::serialization`
- `sdk::forms`
- `sdk::events`
- `sdk::gameplay`
- `sdk::ui`
- `sdk::interop`
- most of `sdk::hooks`
- all of `sdk::advanced`

The migration strategy is to move existing ergonomic layers into `sdk` first as
thin, compatibility-friendly facades, then gradually converge on more
domain-oriented implementations once real plugin usage shapes the stable API.
