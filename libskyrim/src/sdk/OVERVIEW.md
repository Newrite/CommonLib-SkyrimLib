# `libskyrim::sdk`

`libskyrim::sdk` is the Rust-first authoring layer built on top of the
source-backed [`re`](crate::re) and [`skse`](crate::skse) layers.

Use this module when the low-level engine translations are already correct, but
you want a plugin-shaped API that matches recurring SKSE workflows:

- plugin bootstrap and lifecycle wiring
- Papyrus registration and persistent Papyrus event sets
- form lookup and settings/config driven workflows
- gameplay-centric helpers for input, inventory, magic, quests, and projectiles
- event installation and plugin-local event buses
- task queues, serialization, and inter-plugin APIs
- menu/runtime/HUD coordination for UI-heavy plugins

The SDK is intentionally domain-oriented. It groups helpers by plugin author
intent instead of mirroring reverse-engineered class names one-to-one.

## Choosing A Layer

Use the lowest layer that already matches your workflow well:

- [`crate::re`]
  when you need a source-backed RE type, raw layout access, direct engine
  wrappers, or a capability that the SDK does not expose yet.
- [`crate::skse`]
  when the task is mostly about SKSE service interfaces and raw registration
  surfaces.
- [`crate::sdk`]
  when you want a stable, ergonomic, plugin-facing API over those lower layers.

The SDK should reduce boilerplate and make common code paths harder to misuse,
but it must always leave a straightforward escape hatch back to `re` / `skse`.

## Module Map

### `sdk::core`

Shared foundations used across the rest of the SDK:

- `GameRef<T>` / `GamePtr<T>` for trusted stable engine pointers
- `Resolved<T>` / `ResolvedHandle<H>` for handle-backed runtime objects
- native-owner traits for `NiPointer`, `BSTSmartPointer`, `hkRefPtr`, ...
- RTTI-backed cast helpers
- contiguous-container traversal/snapshot helpers
- runtime-phase snapshots and gameplay/HUD gating

If you are building a new domain helper, start here first.

### `sdk::plugin`

The plugin-runtime surface that usually corresponds to `main.cpp` / bootstrap
code in C++ plugins:

- entry/init wrappers
- logging
- lifecycle registration
- typed SKSE messaging listeners
- config / INI helpers
- co-save schema/model registration
- queued task handoff helpers

### `sdk::papyrus`

Papyrus authoring has three distinct layers:

- direct script binding via `PapyrusScript`, `Registry<'_>`, and
  `register_script::<T>()`
- richer class/module registration via `PapyrusModule`, `ModuleRegistry<'_>`,
  and `papyrus_module!`
- persistent `RegisterFor...` style event sets via `PapyrusEventRegistry*`,
  `register_event_set(...)`, `papyrus_event_functions!`, and
  `papyrus_event_module!`

This split mirrors real plugin code:

- simple utility plugins often just bind a few functions to one script name
- larger plugins often want grouped module/class registration
- persistent event-driven plugins need installable/runtime-owned Papyrus event
  registries

### `sdk::forms`

Form-centric workflows for plugin authors:

- editor-ID and `plugin:FormID` lookup
- persistent form wrappers
- list/keyword helpers
- typed `Setting` access

This domain is especially useful when config parsing and gameplay installation
need to resolve forms repeatedly.

### `sdk::gameplay`

Gameplay-oriented helpers built on top of the RE gameplay surface. Current
coverage includes:

- actors, player, camera, combat, world scans
- input gestures and direction/state helpers
- inventory and equipment workflows
- magic inspection/casting helpers
- quest/objective/alias/stage helpers
- projectile acquisition/steering/launch helpers

These helpers are intentionally domain-first rather than class-first.

### `sdk::events`

User-facing event installation and dispatch helpers. This layer is split by
runtime semantics instead of pretending every event source is the same:

- gameplay/UI/dispatcher-backed `BSTEventSource<T>` owners
- input-event chain helpers
- plugin messaging listeners
- install batching
- plugin-local synchronous event buses

### `sdk::ui`

UI support is layered from low-level building blocks up to menu/HUD runtime
recipes:

- `menus`
  queued UI messages, menu-state queries, typed `IUIMessageData`
- `scaleform`
  `GFxMovieView` / `FxDelegate` access and movie-variable helpers
- `controls`
  UI/input context and gameplay/HUD visibility state
- `notifications`
  HUD notification/message builders
- `widgets`
  deferred menu-surface task runtime
- `hud_runtime`
  visibility modes and aggregated refresh requests
- `driver`
  event/policy/task orchestration over `widgets` and `hud_runtime`
- `controller`
  controller-style recipes that translate request bundles into runtime calls

This stack is designed around repeated patterns from UI-heavy SKSE plugins such
as custom HUDs and menu-owned widgets.

### `sdk::interop`

Inter-plugin communication and external API negotiation:

- plugin-to-plugin messaging/request-response protocols
- `RequestPluginAPI`-style DLL exports
- messaging-driven interface loaders
- flat exported symbol helpers for subscriber/callback patterns

### `sdk::hooks`

High-level hook authoring over the relocation layer:

- typed hook macros
- batch installers
- named guard presets
- patch/trampoline/thunk helpers

### `sdk::advanced`

More engine-heavy or volatile domains that intentionally stay separate from the
default early SDK surface.

## Common Workflows

### Plugin Bootstrap

Start in `sdk::plugin`:

- initialize with `sdk::plugin::init(...)` / `init_with_log(...)`
- install lifecycle listeners through `sdk::plugin::lifecycle`
- hand work off through `sdk::plugin::task`
- register co-save models through `sdk::plugin::serialization`

### Lightweight Papyrus Utility Script

Use `sdk::papyrus::PapyrusScript` when the plugin looks like classic
`Bind(VM*) -> RegisterFunction(...)` C++ code:

```rust,ignore
use libskyrim::sdk::papyrus;

papyrus::papyrus_script! {
    pub MyBindings("MyScript") {
        fn "GetVersion" => get_version => fn(base: *mut libskyrim::re::StaticFunctionTag) -> i32;
    }
}

fn install_papyrus() -> bool {
    papyrus::register_script::<MyBindings>()
}
```

### Persistent Papyrus Event Set

Use `PapyrusEventRegistry*` plus `register_event_set(...)` when the plugin owns
registered handles that must survive save/load/revert/form-delete flows.

### UI-Heavy Menu/HUD Plugin

Combine:

- `sdk::ui::menus`
- `sdk::ui::scaleform`
- `sdk::ui::widgets`
- `sdk::ui::hud_runtime`
- `sdk::ui::driver`
- `sdk::ui::controller`

This gives you a menu-owned runtime with deferred tasks, visibility policy,
request aggregation, and controller-style orchestration.

### External Dependency

Start in `sdk::interop`:

- `external_api` for `RequestPluginAPI`, exported symbols, and callback-style
  flat APIs
- `messaging` for SKSE messaging handshakes and request/response protocols

## Decision Guides

### Papyrus: Which Registration Surface?

Choose the lightest Papyrus layer that still matches the plugin:

- `PapyrusScript` plus `register_script::<T>()`
  when the plugin mostly binds a few native functions to one script/class name
- `PapyrusModule` plus `papyrus_module!`
  when registration should stay grouped by feature/module instead of by one
  monolithic script type
- `PapyrusEventRegistry*` plus `register_event_set(...)`
  when the plugin owns persistent `RegisterFor...` style registrations that
  must survive save/load/revert/form-delete flows

If the plugin ends up needing both native calls and persistent event sets, use
both layers instead of trying to force everything through one registration
style.

### Interop: `external_api` or `messaging`?

Use `sdk::interop::external_api` when another plugin exposes a stable
request/getter surface:

- `RequestPluginAPI`
- flat exported symbols
- callback/subscriber exports

Use `sdk::interop::messaging` when the dependency relationship is lifecycle- or
ordering-sensitive:

- SKSE messaging handshakes
- request/response protocols
- interface publication that depends on `PostLoad`/`DataLoaded`

Real plugins often need both:

- messaging to learn *when* an external service is ready
- `external_api` to load or cache the actual callable interface

### UI: Where To Enter The Stack?

The UI stack is intentionally layered. Start as low as your plugin actually
needs:

- `menus`
  when you only need menu state, queued messages, or typed `IUIMessageData`
- `scaleform`
  when you need `GFxMovieView`, variable writes, or `invoke(...)`
- `widgets`
  when work must wait until the menu surface is really available
- `hud_runtime`
  when the plugin owns visibility modes or refresh aggregation
- `driver`
  when menu events, cached menu state, and pending tasks must be orchestrated
  together
- `controller`
  when you want request bundles translated into concrete show/hide/refresh
  actions through a handler object

If a plugin already looks like a `TrueHUD`/`QuickLootIE` style menu-owned
runtime, skip directly to `widgets` or above instead of rebuilding deferred
surface logic by hand.

### Config and Forms: Resolve Once or Stay Persistent?

Use `sdk::forms::lookup` when a plugin needs to resolve editor IDs,
`plugin:FormID` specs, or raw form IDs at install/reload time.

Use `sdk::forms::persistent` when the resolved form must remain as a stable
plugin-owned value across later workflows.

Use `sdk::forms::settings` when the plugin is reading typed `Setting` values,
and combine it with `sdk::plugin::config` when config strings or hotkeys must
be translated into typed forms plus installation-time validation.

### `core::phase` vs `plugin::task`

These layers solve different problems and are strongest when used together:

- `sdk::core::phase`
  answers "is this gameplay/HUD work safe right now?"
- `sdk::plugin::task`
  answers "how do I hand this work off into a safer queued execution point?"

Use `phase` when deciding whether work should run now, be suppressed, or be
deferred. Use `task` when you already know the work should move onto the SKSE
task queue or Papyrus/gameplay-safe queue path.

## Plugin Archetypes

### Papyrus Utility Plugin

Use this stack when the plugin mostly exposes script-callable helpers and only
needs a small runtime footprint:

- `sdk::plugin`
  for bootstrap, logging, lifecycle, and config
- `sdk::papyrus`
  for direct script binding through `PapyrusScript` or grouped module binding
- `sdk::forms`
  when script-facing APIs resolve editor IDs or `plugin:FormID` specs
- `sdk::plugin::task`
  when callbacks need to hand work off into safer queued execution

This matches classic "bind a handful of functions to one script name" plugins,
plus the next step up where the same plugin also owns persistent Papyrus event
registries.

### Event-Driven Gameplay Plugin

Use this stack when the plugin reacts to engine/UI/input events and then
updates actor, inventory, quest, or combat state:

- `sdk::plugin`
  for bootstrap, lifecycle, and deferred work
- `sdk::events`
  for game/UI/input/SKSE subscriptions
- `sdk::gameplay`
  for the domain-specific state and mutation helpers
- `sdk::core::phase`
  when gameplay work should be deferred around loading/fade/menu states

This is the most common shape for combat, input, dodge, quest, and utility
plugins that do not own a custom UI.

### UI / HUD Plugin

Use this stack when the plugin owns a menu, custom HUD widget, or menu-bound
Scaleform runtime:

- `sdk::plugin`
  for bootstrap and lifecycle
- `sdk::events::ui` / `sdk::events::skse::messages`
  for open/close and lifecycle glue
- `sdk::ui::menus` + `sdk::ui::scaleform`
  for low-level menu/movie access
- `sdk::ui::widgets` + `sdk::ui::hud_runtime`
  for deferred menu work, request aggregation, and visibility state
- `sdk::ui::driver` + `sdk::ui::controller`
  for higher-level orchestration recipes

This matches the recurring TrueHUD / QuickLoot-style plugin shape.

### External API Bridge Plugin

Use this stack when the plugin depends on another SKSE plugin or exposes a
typed API for other plugins:

- `sdk::plugin`
  for bootstrap and lifecycle
- `sdk::interop::external_api`
  for exported symbols, service getters, callback registrars, and subscriber
  patterns
- `sdk::interop::messaging`
  when the dependency uses message packets or request/response protocols
- `sdk::events::skse::messages`
  when the dependency handshake is lifecycle-driven instead of export-driven

This matches plugins that integrate with mods such as Precision, SmoothCam,
TrueHUD, or any flat exported-symbol API.

### Spatial / Respawn / Teleport Plugin

Use this stack when the plugin evaluates candidate points, line of sight,
navmesh support, or pathability:

- `sdk::gameplay::world`
  for nearby reference and scene snapshots
- `sdk::advanced::physics`
  for ground, headroom, clearance, and line-of-sight checks
- `sdk::gameplay::navmesh` and `sdk::gameplay::pathing`
  for support and graph/pathing hints
- `sdk::gameplay::spatial`
  for scored/tiered candidate evaluation

This is the usual stack for respawn, blink/teleport, summon placement, and
world-position fallback systems.

For longer archetype-oriented skeletons, see [`README.md`](./README.md).

## Recipe Gallery

### Config-Driven Form Resolution

Use `sdk::plugin::config` to read strings and `sdk::forms` to resolve them into
typed or persistent forms:

```rust,ignore
use libskyrim::re::BGSKeyword;
use libskyrim::sdk::{forms, plugin};

fn load_keyword() -> Result<forms::PersistentForm<BGSKeyword>, ()> {
    let ini = plugin::config::load_ini("Data/SKSE/Plugins/MyPlugin.ini")?;
    let config = plugin::config::config(&ini);
    let spec = config
        .require("Gameplay", "RequiredKeyword")
        .map_err(|_| ())?;

    Ok(forms::lookup::require_persistent_form_spec::<BGSKeyword>(
        spec,
        "Gameplay.RequiredKeyword",
    ))
}
```

### Gameplay-Safe Deferred Work

Use `sdk::plugin::task` when callback code needs to hand work off into a safer
execution phase and resolve handles later:

```rust,ignore
use libskyrim::re::Actor;
use libskyrim::sdk::{core::Resolved, plugin};

fn queue_actor_work(actor: libskyrim::sdk::core::GamePtr<Actor>) {
    let _queued = plugin::task::queue_gameplay_task_resolving_target(actor, |actor: Resolved<Actor>| {
        let _position = actor.get_position();
    });
}
```

### Co-Save Model Registration

Use `sdk::plugin::serialization` when the plugin owns one registered state
model:

```rust,ignore
use libskyrim::sdk::plugin::serialization;

#[derive(Default, serialization::Cosave)]
struct SaveState {
    counter: u32,
}

impl serialization::Model for SaveState {
    const UNIQUE_ID: serialization::UniqueId = serialization::unique_id!("EXMP");

    fn schema(schema: &mut serialization::Schema<Self>) {
        serialization::schema_fields!(schema, {
            serialization::record_id!("CNT1") => 1 => counter,
        });
    }
}

fn install_serialization() -> Result<(), serialization::RegistrationError> {
    serialization::register_model::<SaveState>()
}
```

### External Flat Callback API

Use `sdk::interop::external_api` when another plugin exposes a register /
unregister callback pair through exports:

```rust,ignore
use libskyrim::sdk::interop::external_api;

type HudCallback = unsafe extern "system" fn(i32);

unsafe extern "system" fn on_hud_mode_changed(_mode: i32) {}

fn install_dependency_callback() -> Result<(), external_api::SymbolError> {
    let registrar = unsafe {
        external_api::callback_registrar_for_plugin::<HudCallback, u32>(
            "SomeHudPlugin",
            "RegisterHudCallback",
            "UnregisterHudCallback",
        )?
    };

    let _registration = unsafe { registrar.register(on_hud_mode_changed) };
    Ok(())
}
```

### Menu-Owned HUD Controller

Use `sdk::ui::controller` when plugin code owns a menu/HUD runtime and wants a
controller-style event loop:

```rust,ignore
use libskyrim::re::MenuOpenCloseEvent;
use libskyrim::sdk::ui;

struct MyHudMenu;

impl ui::menus::NamedMenu for MyHudMenu {
    const MENU_NAME: &'static str = "MyHudMenu";
}

#[derive(Default)]
struct MyHudHandler;

impl ui::controller::WidgetRequestHandler for MyHudHandler {}

impl ui::controller::HudRequestHandler for MyHudHandler {}

fn on_menu_event(event: &MenuOpenCloseEvent) {
    let mut controller =
        ui::controller::HudRuntimeController::<MyHudMenu, MyHudHandler>::with_policy(
            MyHudHandler::default(),
            ui::widgets::WidgetVisibilityPolicy::hud_like(),
        );

    let _cycle = controller.drive_menu_event(event);
}
```

## Defensive Model

The SDK is more defensive than `re`, but it does not rewrite real ABI
contracts.

In practice this means:

- strict singleton boundaries often stay strict
- nullable seams fail soft where that preserves honest semantics
- query-style helpers prefer stable snapshots over leaking transient traversal
- task/runtime helpers expose named phase checks instead of forcing every plugin
  to rebuild the same gating logic

## Documentation Strategy

SDK documentation is split across several layers:

- this module overview for rustdoc-facing navigation
- module-level docs on each domain `mod.rs`
- [`README.md`](./README.md) for longer-form human design notes and examples
- [`MEMORY.md`](./MEMORY.md) for persistent architecture/research memory
- [`PLUGIN_AUDIT.md`](./PLUGIN_AUDIT.md) for alignment against real SKSE plugin
  code

When extending the SDK, prefer updating both:

- rustdoc/module docs for the public authoring surface
- markdown notes when the change also shifts design/backlog/research state
