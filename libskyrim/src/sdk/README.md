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
  `&T`, `Option<&T>`, `&mut T`, and `Option<&mut T>`
- engine identity across frames or save/load boundaries:
  native handle types such as `ActorHandle`, `ObjectRefHandle`, and similar
  `BSPointerHandle`-style wrappers
- engine-owned refcounted retention:
  native smart pointers such as `NiPointer<T>`, `BSTSmartPointer<T>`, and
  `hkRefPtr<T>`
- stable engine pointers from trusted sources:
  `GameRef<T>` and `GamePtr<T>`

The SDK should compose these, not erase them. High-level helpers may wrap them
for ergonomics, but the native Bethesda types remain first-class values.

Planned shared SDK support types:

- `GameRef<T>`
  non-null wrapper for stable engine objects sourced from trusted pointers
- `GamePtr<T>`
  nullable wrapper for stable optional engine pointers
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
  shared RTTI-backed cast helpers for SDK wrappers, native owners, and
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
    install.rs
    source.rs
    ui.rs
    bus.rs
    skse/
      mod.rs
      dispatchers.rs
      messages.rs

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

## Event System Design

The SDK event layer should unify authoring style, not pretend that every Skyrim
and SKSE event source follows one runtime model.

The underlying ecosystems are meaningfully different:

- engine-owned `BSTEventSource<T>` values such as `ScriptEventSourceHolder` and
  `UI`
- input dispatch through `BSTEventSource<InputEvent*>`, where the payload is a
  linked event chain rather than a simple `&T`
- SKSE dispatcher-backed `BSTEventSource<T>` values exposed through the SKSE
  API storage
- SKSE plugin messaging through `MessagingInterface::RegisterListener`, which
  behaves more like install-once lifecycle callbacks than ordinary removable
  event sinks

Because of that, `sdk::events` is split by domain:

- `sdk::events::game`
  gameplay and script-owned singleton event sources from
  `ScriptEventSourceHolder`
- `sdk::events::ui`
  `UI`-owned singleton event sources such as `MenuOpenCloseEvent`
- `sdk::events::input`
  `BSInputDeviceManager` registrations plus ergonomic wrappers over
  `InputEvent*` chains
- `sdk::events::source`
  an escape hatch for direct subscription to a known raw
  `BSTEventSource<T>*`
- `sdk::events::skse::dispatchers`
  typed helpers for SKSE dispatcher-backed event sources such as
  `ActionEvent`, `CameraEvent`, `ModCallbackEvent`, `CrosshairRefEvent`, and
  `NiNodeUpdateEvent`
- `sdk::events::skse::messages`
  lifecycle and plugin messaging registration over `MessagingInterface`
- `sdk::events::bus`
  a Rust-local plugin event bus for intra-plugin coordination

### Event Goals

The first implementation wave should focus on a small shared foundation:

- `EventFlow`
  a stable SDK-facing alias over continue/stop semantics
- `EventSubscription`
  an RAII handle for removable event sinks
- `EventInstallError`
  fallible installation instead of fatal-only registration
- `IntoEventFlow`
  callback sugar so `()` naturally means `Continue`
- typed domain `subscribe(...)` functions instead of raw pointer plumbing
- batched installation support similar to `sdk::hooks`

The most important safety boundary is that sink registration should only become
safe/high-level where source-backed code proves synchronous `ProcessEvent`
dispatch against retained sink pointers. `BSTEventSource<T>` satisfies that
contract; `MessagingInterface` does not share the same unregister and ownership
story, so it remains a separate domain.

The current implemented foundation already includes:

- `EventFlow`
- `EventInstallError`
- `IntoEventFlow`
- `EventSubscription`
- raw `sdk::events::source::{subscribe, prepend}` over `BSTEventSource<T>*`
- safe singleton-domain subscriptions in `sdk::events::game`,
  `sdk::events::ui`, and `sdk::events::skse::dispatchers`
- typed lifecycle registration in `sdk::events::skse::messages::{on, on_raw}`
  plus `MessageRef<'_>` sender/payload helpers, lifecycle-phase mapping, and
  sender-filtered helpers
- `sdk::events::input::subscribe(...)` / `prepend(...)` plus `InputEvents<'_>`
  for iterating and mutating one `InputEvent*` chain without manual raw linked
  list traversal, device filtering, and per-device iteration helpers
- `sdk::events::bus::{Bus, BusSubscription, SubscriberPriority}` for
  synchronous plugin-local mutable event dispatch with deterministic priority
  ordering, owned-payload publish, `publish_with(...)`, and priority
  convenience helpers
- `sdk::events::{EventBatch, EventInstaller}` for owning RAII event
  registrations and install-once message listeners together in one place
- `sdk::events::install_all!(&mut batch, ...)` and `try_install_all(...)` for
  typed batched installation
- attribute-driven event modules via:
  `#[events::game_event]`, `#[events::ui_event]`,
  `#[events::dispatcher_event]`, `#[events::input_event]`, and
  `#[events::message_event]`, plus `#[events::bus_event]` for local
  `Bus<T>` subscriptions

### Event Installers

Unlike hooks, event registrations are not modeled as permanent global installs.
Engine subscriptions are RAII-owned, and dropping them should unsubscribe
cleanly. Because of that, the high-level event authoring surface is centered on
`EventBatch` instead of a global `is_installed()` flag.

`EventBatch` owns:

- retained `BSTEventSource<T>` subscriptions
- local `Bus<T>` subscriptions
- install-once SKSE messaging listeners

Dropping the batch removes everything removable and leaves message listeners in
their already-registered SKSE state.

The typed installer layer looks like this:

```rust
let mut batch = sdk::events::EventBatch::new();

batch.game::<RE::TESHitEvent, _, _>("hit", |event| {
    let _ = event;
})?;

batch.message(
    "data_loaded",
    sdk::events::skse::messages::MessageKind::DataLoaded,
    |message| {
        let _ = message.sender();
    },
);

sdk::events::install_all!(&mut batch, some_game_event, some_input_event)?;
```

When a plugin subsystem already owns a borrowed dynamic source, use
`EventSourceRef<'a, T>` instead of spelling raw `*mut BSTEventSource<T>`:

```rust
let mut source = RE::BSTEventSource::<RE::TESHitEvent>::new();
let _sub = sdk::events::EventSourceRef::new(&mut source).subscribe(|event| {
    let _ = event;
})?;
```

Each attribute-generated event module exposes:

- `try_install(&mut EventBatch<'_>) -> Result<(), EventBatchError>`
- `install(&mut EventBatch<'_>) -> Result<(), EventBatchError>`
- `INSTALLER: EventInstaller`
- `installer() -> EventInstaller`
- `install_or_fatal(&mut EventBatch<'_>)`

This keeps installation explicit about ownership and makes duplicate installs a
user choice instead of a hidden global state machine.

### Event Attributes

The first high-level authoring sugar on top of the batch/install foundation is
attribute-driven event registration.

Supported attributes:

- `#[events::game_event(event = RE::TESHitEvent)]`
- `#[events::ui_event(event = RE::MenuOpenCloseEvent)]`
- `#[events::dispatcher_event(event = skse::ActionEvent)]`
- `#[events::input_event]`
- `#[events::message_event(kind = events::skse::messages::MessageKind::DataLoaded)]`
- `#[events::message_event(plugin_phase = sdk::core::PluginLifecyclePhase::DataLoaded)]`
- `#[events::message_event(game_phase = sdk::core::GameLifecyclePhase::PostLoadGame)]`
- `#[events::message_event(phase = sdk::core::LifecyclePhase::Plugin(...))]`
- `#[events::bus_event(bus = some_bus())]`
- `#[events::bus_event(bus = some_bus(), early)]`
- `#[events::bus_event(bus = some_bus(), priority = events::SubscriberPriority::LAST)]`

For gameplay/UI/dispatcher events, the callback parameter may currently be:

- `&Event`
- `Option<&Event>`
- or no parameter at all

For input events, the callback parameter may be `InputEvents<'_>` or omitted.
For SKSE messages, the callback parameter may be `MessageRef<'_>`, `&Message`,
or omitted. `message_event` also accepts `sender = "..."` for common sender
filtering without hand-written closure plumbing. For local bus events, the
callback parameter must be exactly one payload parameter of type `&Payload` or
`&mut Payload`; `bus_event` infers the payload type from that signature, and
its `bus = ...` expression must evaluate to a stable `&Bus<Payload>`.

All event attributes support ordinary free functions only. They do not support
methods, generics, `async`, `const`, `extern`, or variadics.

If the callback returns `EventFlow`, that flow is used directly. If it returns
`()`, the SDK treats it as `EventFlow::Continue`.

### Planned Domain Examples

The intended authoring style is:

```rust
let _hit = sdk::events::game::subscribe::<RE::TESHitEvent>(|event| {
    let _ = event;
    sdk::events::EventFlow::Continue
})?;

let _menu = sdk::events::ui::subscribe::<RE::MenuOpenCloseEvent>(|event| {
    let _ = event;
    sdk::events::EventFlow::Continue
})?;

let _action = sdk::events::skse::dispatchers::subscribe::<skse::events::ActionEvent>(|event| {
    let _ = event;
    sdk::events::EventFlow::Continue
})?;

sdk::events::skse::messages::on(
    sdk::events::skse::messages::MessageKind::DataLoaded,
    |message| {
        let _ = message.sender();
    },
);

sdk::events::skse::messages::on_sender_str(
    sdk::events::skse::messages::MessageKind::DataLoaded,
    "SKSE",
    |message| {
        let _ = message.typed::<u32>();
    },
);

sdk::plugin::on_data_loaded(|message| {
    let _ = message.lifecycle_phase();
});

sdk::plugin::on_game_lifecycle(
    sdk::core::GameLifecyclePhase::PostLoadGame,
    |message| {
        let _ = message.kind();
    },
);

let _input = sdk::events::input::subscribe(|mut events| {
    if events.contains_device(RE::INPUT_DEVICE::kKeyboard) {
        for button in events.buttons_mut() {
            let _ = button;
        }
    }

    sdk::events::EventFlow::Continue
})?;

let local_bus = sdk::events::Bus::<u32>::new();
let _sub = local_bus.subscribe_early(|value| {
    *value += 1;
});
let published = local_bus.publish_owned(10);
assert_eq!(published.flow(), sdk::events::EventFlow::Continue);

let built = local_bus.publish_with(|value| {
    *value = 41;
});
assert_eq!(built.into_event(), 42);

fn local_bus() -> &'static sdk::events::Bus<u32> {
    # use alloc::boxed::Box;
    # use spin::Once;
    static PTR: Once<usize> = Once::new();
    let ptr =
        *PTR.call_once(|| Box::into_raw(Box::new(sdk::events::Bus::new())) as usize);
    unsafe { &*(ptr as *const sdk::events::Bus<u32>) }
}

#[sdk::events::bus_event(bus = local_bus(), early)]
fn on_local_bus(value: &mut u32) -> sdk::events::EventFlow {
    *value += 1;
    sdk::events::EventFlow::Continue
}

#[sdk::events::input_event(prepend)]
fn on_input(mut events: sdk::events::InputEvents<'_>) {
    for keyboard_event in events.keyboard_mut() {
        let _ = keyboard_event;
    }
}

#[sdk::events::message_event(
    kind = sdk::events::skse::messages::MessageKind::DataLoaded
)]
fn on_data_loaded(message: sdk::events::skse::messages::MessageRef<'_>) {
    let _ = message.data_len();
}

#[sdk::events::message_event(
    plugin_phase = sdk::core::PluginLifecyclePhase::DataLoaded,
    sender = "SKSE"
)]
fn on_skse_data_loaded(message: sdk::events::skse::messages::MessageRef<'_>) {
    let _ = message.sender();
}

let mut batch = sdk::events::EventBatch::new();
sdk::events::install_all!(
    &mut batch,
    on_data_loaded_event,
    on_skse_data_loaded_event,
    on_input_event,
    on_local_bus_event
)?;
```

This keeps one recognizable SDK style while preserving the real ownership and
lifetime differences underneath.

### Rust-Local Event Bus

`sdk::events::bus` is intentionally plugin-local. It is not a replacement for
SKSE messaging or engine-backed event sources. Its job is to let one Rust
plugin subsystem publish typed events to other Rust subsystems without forcing
those subsystems to know about the original hook or engine callback site.

The motivating case is a hook that wants to publish a mutable gameplay payload,
for example an `on_weapon_hit` event whose subscribers can inspect and mutate
hit data before later systems observe it.

The currently implemented bus already supports:

- mutable event payload delivery through `FnMut(&mut T) -> EventFlow`
- explicit stop/continue flow control
- deterministic `SubscriberPriority` ordering
- `subscribe_first/early/late/last` convenience helpers
- `publish_owned(...)` / `publish_default(...)` for hook-owned mutable payloads
- `publish_with(...)` for default-constructible payloads that want one
  initialization pass before dispatch
- RAII unsubscription through `BusSubscription`
- deferred add/remove semantics so dispatch can stay stable while callbacks
  subscribe or unsubscribe

That priority model is especially attractive for gameplay mutation chains:

- early subscribers can normalize or clamp engine data
- mid-priority subscribers can apply gameplay rules
- late subscribers can observe the final value after previous edits

This bus should still stay synchronous and explicit by default. The first goal
is deterministic intra-plugin orchestration, not a general async framework.
The current implementation also intentionally does not re-enter the same
subscriber during a nested publish; if one callback republishes while it is
already active, that callback is skipped for the nested pass instead of being
invoked recursively.

For owner-bound dynamic sources, the SDK now also exposes
`EventSourceExt<T>` on `BSTEventSource<T>` itself, so the common case does not
need to spell `EventSourceRef::new(...)` manually:

```rust
let mut source = RE::BSTEventSource::<RE::TESHitEvent>::new();
let _subscription = source.subscribe_sdk(|event| {
    let _ = event;
})?;
```

## Serialization Design

The serialization stack is intentionally split in two:

- `sdk::plugin::serialization`
  owns SKSE callback registration and the high-level persistent-model API
- `sdk::persistence::cosave`
  owns typed record IDs, bounded record IO, and reusable codecs

The intended high-level path is model-driven:

```rust
#[derive(Default, Cosave)]
struct SaveState {
    count: u32,
    cache: BTreeMap<String, BoundedVec<u32, 64>>,
}

impl sdk::plugin::serialization::Model for SaveState {
    const UNIQUE_ID: sdk::plugin::serialization::UniqueId =
        sdk::plugin::serialization::unique_id!("TFNG");

    fn schema(schema: &mut sdk::plugin::serialization::Schema<Self>) {
        sdk::plugin::serialization::schema_fields!(schema, {
            sdk::plugin::serialization::record_id!("CNT1") => 1 => count,
            sdk::plugin::serialization::record_id!("CACH") => 1 => cache,
        });
    }
}
```

The `u32` after `record_id!(...)` is the version of that specific top-level
record format, not the whole plugin.

For payload structs, `#[derive(Cosave)]` generates both `CosaveEncode` and
`CosaveDecode` in field order for ordinary structs and tuple structs. That is
the intended default path for primitive fields and nested containers such as
`Option<T>`, `[T; N]`, `Vec<T>`, `BoundedVec<T, N>`, and `BTreeMap<K, V>`.

The derive also supports field-level sugar for common serialization cases:

- `#[cosave(skip)]`
  keeps a field out of the serialized payload and restores it with
  `Default::default()` during load
- `#[cosave(default)]`
  decodes the field normally when bytes are present, but falls back to
  `Default::default()` when older payloads end before that field
- `#[cosave(with = path::to::codec)]`
  routes one field through custom `encode(&T, &mut RecordWriter)` /
  `decode(&mut RecordReader<'_>) -> Result<T, LoadError>` helpers

`#[cosave(default)]` is primarily intended for newly added trailing fields in a
record payload. It is not a replacement for full record-version migrations when
the wire layout itself changes incompatibly.

For unusual binary layouts, `Schema::record(...)` exposes a custom hook that
receives the full loaded record and a `LoadContext`, so version dispatch and
manual decoding can stay local to that record.

When one record ID needs multiple load paths across save versions, the schema
can use migration sugar instead of a hand-written `match`:

```rust
schema
    .migrating_record(
        sdk::plugin::serialization::record_id!("CDAD"),
        2,
        save_v2,
    )
    .load(1, load_v1)
    .load(2, load_v2);
```

This keeps the current save format explicit while still preserving older
versioned records that the active schema does not yet understand.

### Serialization Safety Goals

The SDK serialization layer tries to be safer than the usual hand-written
`OpenRecord` / `GetNextRecordInfo` loop:

- all top-level records are encoded into memory first, then written to SKSE
- record readers are length-bounded and detect trailing bytes
- `BoundedVec<T, N>` can reject oversized payloads during load
- field-derived payloads report which field failed to encode/decode
- sequence and map codecs report the failing element or entry index
- unknown records in the plugin's own co-save segment are preserved and written
  back untouched, so partially upgraded plugins do not accidentally discard
  unrelated future records
- unknown records whose IDs are now explicitly owned by the active schema are
  not re-emitted, so one logical record ID does not get written twice with two
  conflicting payloads

### SKSE-Aware Codecs

The cosave layer keeps raw numeric codecs available, but also exposes SKSE-aware
resolved wrappers:

- `ResolvedFormId`
- `ResolvedVmHandle`

These write the stored raw value and resolve it through the active
`SerializationInterface` during decode, so model code can opt into correct
save/load remapping without open-coding `ResolveFormID` or `ResolveHandle`.

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

The `hooks` namespace also re-exports the retained runtime-object wrappers used
by high-level hook signatures:

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
    original: hooks::Original<fn(&RE::TESObjectREFR, f32)>,
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
  shared `GameRef` / `GamePtr`, native-owner traits, handle resolution, and
  shared RTTI cast helpers
- `sdk::papyrus`
  SDK-facing facade over the current high-level Papyrus authoring layer in
  `skse::papyrus`, including callback-facing `GameRef`, `GamePtr`,
  `ResolvedHandle<H>`, `Resolved<T>`, and `Option<...>` parameter/base support
- `sdk::core::handles`
  family-based resolved runtime-object model with immediate support for the
  `Actor`, `TESObjectREFR`, and `Projectile` handle families
- `sdk::plugin::log`
  facade over the current SKSE logging convenience layer
- `sdk::plugin::task`
  facade over the current SKSE task helpers
- `sdk::plugin::messaging`
  high-level façade over SKSE plugin messaging with typed `MessageKind`,
  `MessageRef<'_>`, sender-filtered listeners, and lifecycle-aware helpers
- `sdk::plugin::lifecycle`
  structured plugin/game lifecycle registration built on typed SKSE messaging
- `sdk::plugin::entry`
  light SDK wrappers for common `LoadInterface` initialization paths
- `sdk::plugin::config`
  early INI-focused helpers built on the existing `Ini` surface
- `sdk::persistence::cosave`
  typed `RecordId` / `UniqueId`, bounded record readers/writers, and first-class
  codecs for fixed-width primitives, `String`, `Option<T>`, `[T; N]`, `Vec<T>`,
  `BoundedVec<T, N>`, `BTreeMap<K, V>`, and SKSE-aware resolved `FormID` /
  `VMHandle` wrappers
- `sdk::plugin::serialization`
  high-level `Model` / `Schema` registration layer over the typed cosave
  foundation, including save/load/revert/form-delete callback registration,
  typed state access, runtime error capture, passthrough preservation of
  unknown records in the plugin's own co-save segment, `#[derive(Cosave)]`,
  `schema_fields!(...)` sugar for top-level value records, and migrating-record
  sugar for versioned load handlers
- `sdk::events`
  raw `BSTEventSource<T>` subscriptions, singleton gameplay/UI/SKSE dispatcher
  helpers, typed SKSE messaging registration, ergonomic input-chain wrappers,
  `IntoEventFlow` callback sugar, `EventBatch` / `EventInstaller` batching,
  attribute-driven event authoring, and a first synchronous plugin-local
  mutable event bus

Still intentionally placeholder-heavy:

- `sdk::forms`
- `sdk::gameplay`
- `sdk::ui`
- `sdk::interop`
- most of `sdk::hooks`
- all of `sdk::advanced`

The migration strategy is to move existing ergonomic layers into `sdk` first as
thin, compatibility-friendly facades, then gradually converge on more
domain-oriented implementations once real plugin usage shapes the stable API.
