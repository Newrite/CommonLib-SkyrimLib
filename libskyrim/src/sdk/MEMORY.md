<!--
  Working memory for libskyrim SDK direction.
  Keep this file short enough to stay readable, but update it whenever a
  research pass, API redesign, or large SDK implementation changes priorities.
-->

# SDK Memory

Last updated: 2026-04-03

This file is a persistent working memory for `libskyrim/src/sdk`.

Use it to record:

- repeated patterns observed in real Skyrim / SKSE plugins
- current SDK design decisions that should not be re-litigated from scratch
- high-priority gaps between the current SDK and real plugin needs
- source projects worth revisiting during future research passes

## Current Foundation Decisions

These decisions are already important enough to treat as part of the SDK
direction, not as temporary discussion notes.

- `GameRef<T>` means a stable, trusted, non-null game object pointer.
- `GamePtr<T>` means a stable, trusted, nullable game object pointer.
- `Resolved<T>` means a retained runtime object backed by handle-family
  semantics and owner retention.
- Borrowed hook / event / owner-backed data should prefer `&T`,
  `Option<&T>`, `&mut T`, and `Option<&mut T>`.
- `GameRef<T>` / `GamePtr<T>` should not be used as generic borrowed callback
  wrappers. The safety boundary is the pointer source, not the pointee type.
- `Resolved<T>` is runtime-object oriented and should stay compile-time limited
  to the sealed handle-family surface.
- Low-level `NiPointer`, `BSTSmartPointer`, and `hkRefPtr` remain the honest
  ownership truth for many engine-facing APIs. Do not overwrap them just to
  make everything look uniform.

## Research Sources

Detailed notes from the `S:\Programming\SKSEProjects` research pass live in
[`SKSEPROJECTS_RESEARCH.md`](./SKSEPROJECTS_RESEARCH.md).

### ersh1 GitHub repos relevant to Skyrim / SKSE

High-signal repos from `https://github.com/ersh1`:

- `CommonLibVR`
- `TrueDirectionalMovement`
- `SkyrimSE-SmoothCam`
- `TrueHUD`
- `DodgeFramework`
- `Precision`
- `OpenAnimationReplacer`
- `OpenAnimationReplacer-ExamplePlugin`
- `OpenAnimationReplacer-Math`
- `MCM-Helper`
- `HighFPSPhysicsFix`
- `AnimationQueueFix`
- `DynamicCollisionAdjustment`
- `iEquipUtil`
  - `NoFurnitureCamera`
  - `PairedAnimationImprovements`

These repos are useful because they repeatedly cover the same plugin-side
problems the SDK should smooth out:

- exported plugin APIs and interface negotiation
- SKSE messaging handshakes
  - camera / input / widget / HUD integration
  - physics and collision queries
  - animation event and animation system integration

### Local `S:\Programming\SKSEProjects` folder

High-signal projects sampled from the consolidated local folder:

- `Acheron`
- `BetterTelekinesis`
- `BlinkSpell`
- `CLibUtil-master`
- `OpenAnimationReplacer-RaySense`
- `Precision-main`
- `QuickLootIE`
- `SkyrimSE-SmoothCam`
- `StyyxUtils-main`
- `TrueDirectionalMovement-master`
- `TrueFlasksNG`
- `TrueHUD-master`
- `UselessFenixUtils-master`
- `NewProjectilesTMP-master`
- `CustomDodge`
- `NavigationRestrictions-master`
- `PapyrusTweaks-main`
- `Unblockable`
- `LoadingScreenTruce01`
- `LoadingScreenTruce02`
- `injury-alt-death`
- `death-alternative-mod-main`
- `Reflyem`

This folder is now the fastest local source for recurring plugin-side patterns
that should inform the SDK.
- MCM / config / Papyrus-facing frameworks

### Local C++ project roots sampled

High-signal local projects under `S:\Programming` and
`G:\Programming\Projects\CPP`:

- `S:\Programming\tdm\TrueDirectionalMovement`
- `S:\Programming\QuickLootIE`
- `S:\Programming\MCM-Helper-XMAKE`
- `S:\Programming\SkyPatcher`
- `S:\Programming\TrueFlasksNG`
- `S:\Programming\GigaWidget`
- `S:\Programming\SkyrimCrashGuard\Source`
- `S:\Programming\Reflyem`
- `S:\Programming\PowerAttackNG-XMAKE`
- `S:\Programming\Input Support`

Additional local repos may still be mined later, but the list above already
shows the main repeated SDK patterns.

### Concrete files sampled in this pass

- `S:\Programming\tdm\TrueDirectionalMovement\src\API\APIManager.cpp`
- `S:\Programming\tdm\TrueDirectionalMovement\src\Raycast.cpp`
- `S:\Programming\QuickLootIE\include\PluginRequests\RequestClient.h`
- `S:\Programming\QuickLootIE\include\QuickLootAPI.h`
- `S:\Programming\MCM-Helper-XMAKE\src\ConfigStore.cpp`
- `S:\Programming\TrueFlasksNG\src\Core\ModsAPIRepository.cpp`
- `S:\Programming\GigaWidget\src\GigaWidgetAPI.hpp`
- `S:\Programming\GigaWidget\src\MenuHandler.cpp`
- `S:\Programming\SkyrimCrashGuard\Source\src\ConfigMenu.cpp`
- `S:\Programming\Reflyem\src\Hooks.cpp`

### Concrete files sampled in the 2026-04-01 follow-up pass

- `S:\Programming\SKSEProjects\CustomDodge\src\plugin.cpp`
- `S:\Programming\SKSEProjects\CustomDodge\src\SimpleDodge.cpp`
- `S:\Programming\SKSEProjects\NavigationRestrictions-master\src\Main.cpp`
- `S:\Programming\SKSEProjects\NavigationRestrictions-master\src\Hooks.cpp`
- `S:\Programming\SKSEProjects\NavigationRestrictions-master\src\papyrus.cpp`
- `S:\Programming\SKSEProjects\PapyrusTweaks-main\src\main.cpp`
- `S:\Programming\SKSEProjects\PapyrusTweaks-main\src\Papyrus.cpp`
- `S:\Programming\SKSEProjects\PapyrusTweaks-main\src\ExperimentalHooks.h`
- `S:\Programming\SKSEProjects\Unblockable\src\Events.cpp`
- `S:\Programming\SKSEProjects\Acheron\src\Acheron\EventSink.cpp`
- `S:\Programming\SKSEProjects\Acheron\src\Papyrus\Events.h`
- `S:\Programming\SKSEProjects\Acheron\src\Serialization\EventManager.h`
- `S:\Programming\SKSEProjects\Acheron\src\Serialization\EventManager.cpp`
- `S:\Programming\SKSEProjects\death-alternative-mod-main\src\effectEvent.cpp`
- `S:\Programming\SKSEProjects\death-alternative-mod-main\src\sleepEvent.cpp`
- `S:\Programming\SKSEProjects\injury-alt-death\src\utility.h`
- `S:\Programming\SKSEProjects\LoadingScreenTruce01\src\main.cpp`
- `S:\Programming\SKSEProjects\LoadingScreenTruce02\src\main.cpp`

## What Real Plugins Repeatedly Need

### 1. External plugin API interop

Seen in:

- `TrueDirectionalMovement`
- `TrueHUD`
- `DodgeFramework`
- `GigaWidget`
- `TrueFlasksNG`

Repeated pattern:

- request a typed interface from another plugin
- negotiate version compatibility
- cache the interface lazily
- sometimes discover the interface through SKSE messaging callbacks
- keep the call site terse and resilient when the dependency is absent

SDK implication:

- `sdk::interop::external_api` should become a real typed interop layer
- support both `GetProcAddress("RequestPluginAPI")` style APIs and
  SKSE-messaging-mediated interface loaders
- provide version-aware helpers and small typed error enums
- support cached optional interfaces cleanly

### 2. Plugin-to-plugin request / response messaging

Seen strongly in:

- `QuickLootIE`
- several API-heavy UI mods

Repeated pattern:

- client/server messaging on top of `SKSE::MessagingInterface`
- version handshake
- request type enum
- trivial request / response payloads
- optional array response callbacks
- standardized error reporting

SDK implication:

- `sdk::interop::messaging` should grow into a typed request client/server
  helper layer instead of a placeholder
- useful primitives:
  - version handshake
  - query helpers for trivial request / response payloads
  - callback-backed array responses
  - reusable response enums / errors
  - ergonomic registration / dispatch helpers

### 3. Menu lifecycle and Scaleform / widget integration

Seen strongly in:

- `QuickLootIE`
- `GigaWidget`
- `SkyrimCrashGuard`
- `TrueHUD`
- `SmoothCam`

Repeated pattern:

- register a menu
- open / close / toggle it
- push data into GFx
- bind callbacks from GFx into native code
- react to `MenuOpenCloseEvent`
- hide / show widgets based on UI state and input context stack

SDK implication:

- expand `sdk::ui::menus`
- expand `sdk::ui::scaleform`
- likely add a widget-oriented helper layer instead of forcing every project to
  rebuild the same menu-state logic

Concrete helpers worth adding:

- menu registration helpers
- `open_menu`, `close_menu`, `toggle_menu`
- menu visibility predicates for common gameplay / HUD logic
- Scaleform object/member/invoke helpers
- `FxDelegate` callback registration helpers
- UI state sync helpers for HUD widgets

### 4. Input context and gameplay visibility state

Seen strongly in:

- `GigaWidget`
- `SmoothCam`
- `TrueDirectionalMovement`
- `CrashGuard`

Repeated pattern:

- inspect current `ControlMap` context stack
- hide widgets or block actions in menu-heavy states
- temporarily disable gameplay controls
- reason about gameplay vs menu vs cursor vs inventory context

SDK implication:

- current `sdk::gameplay::input` is a good start, but it should grow richer
- likely add:
  - context stack inspection helpers
  - predicates like `is_gameplay_context()`, `is_menu_context()`
  - temporary scoped control toggles
  - higher-level "HUD should be visible" style helpers

### 5. Runtime actor / reference scans and caches

Seen strongly in:

- `Reflyem`
- `TrueFlasksNG`
- `TrueDirectionalMovement`

Repeated pattern:

- scan process lists
- find nearby or interesting actors
- track actor state over time
- cache runtime facts keyed by handle or form ID
- repeatedly answer "who is hostile / teammate / summon / current target"

SDK implication:

- `sdk::gameplay::actors` and `sdk::gameplay::combat` should grow beyond thin
  wrappers
- add high-level actor scan helpers, cache-friendly utilities, and common
  hostility / teammate / summon predicates
- add stable patterns for "update all relevant actors" loops

### 6. Physics and spatial query refinement

Seen strongly in:

- `TrueDirectionalMovement`
- `Precision`
- `DynamicCollisionAdjustment`
- blink / teleport / dodge style mods

Repeated pattern:

- closest-hit raycasts
- all-hit raycasts
- backoff from impact point
- downward snap to ground
- headroom / clearance tests
- line-of-sight queries
- resolve hit collidable to `TESObjectREFR`

SDK implication:

- `sdk::advanced::physics` should become a richer spatial query layer
- the old single-hit helper was not enough; the SDK needed explicit
  world-locked multi-hit, layer-mask, and hit-resolution helpers to support
  teleport / dodge / camera / lock-on style mods

Concrete helpers worth adding:

- multi-hit raycast helpers
- downward ground snap helper
- capsule-like clearance approximation from ray bundles
- LOS helper
- impact backoff helper
- "candidate relocation validation" helper

### 7. Config, MCM, and data-driven plugin infrastructure

Seen strongly in:

- `MCM-Helper`
- `SkyPatcher`
- `CrashGuard`
- `Reflyem`

Repeated pattern:

- load TOML / INI / JSON
- version and validate config
- expose config into UI or Papyrus-friendly layers
- map editor IDs / forms / keywords from config into live game forms

SDK implication:

- `sdk::plugin::config` should grow beyond simple ini load / write
- good targets:
  - typed config loaders
  - schema validation and better error reporting
  - reload flow
  - path conventions for SKSE plugin configs

### 8. Form lookup, editor ID, and patching helpers

Seen strongly in:

- `SkyPatcher`
- many config-driven mods

Repeated pattern:

- resolve form by plugin + form ID
- resolve by editor ID
- traverse forms by type
- patch forms declaratively

SDK implication:

- `sdk::forms::lookup` is the natural home for deeper form lookup helpers
- later, a separate patching layer may be justified, but the SDK should first
  nail lookup, traversal, and typed filtering

### 9. Papyrus registration ergonomics

Seen strongly in:

- `MCM-Helper`
- `QuickLootIE`
- `TrueFlasksNG`
- many gameplay frameworks

Repeated pattern:

- repetitive Papyrus class registration
- native function registration boilerplate
- type adaptation and result conversion boilerplate

SDK implication:

- `sdk::papyrus` should continue growing macro and registry ergonomics
- this is especially important once plugins start mixing DLL logic with ESP /
  Papyrus glue

### 10. Reusable menu / event / lifecycle registrars

Seen strongly in:

- `GigaWidget`
- `QuickLootIE`
- `CrashGuard`
- `Reflyem`

Repeated pattern:

- install event sinks during plugin init
- register menu handlers on data-loaded or post-load
- request external APIs when dependencies are ready
- keep all of that synchronized with plugin lifecycle phases

SDK implication:

- `sdk::plugin::lifecycle`, `sdk::events`, and `sdk::interop` should become
  more cohesive
- likely worth adding small registrars for:
  - deferred API acquisition
  - event sink install / uninstall
  - menu registration
  - one-time phase-gated initialization

### 11. Input gestures and scoped control-handler locks still sit below the current SDK surface

Seen strongly in:

- `CustomDodge`
- `Acheron`
- `SkyrimSE-SmoothCam`

Repeated pattern:

- inspect `InputEvent*` chains directly for both buttons and thumbsticks
- use `userEvent` names for directional intent instead of hard-coded scancodes
- layer tap / release / modifier semantics on top of simple hotkeys
- temporarily disable movement or attack handlers and later restore the previous state
- gate gameplay input behind menu, pause, and `ControlMap` checks

SDK implication:

- `sdk::gameplay::input` already covers snapshots and combo parsing, but it
  still lacks the gesture and handler-control layer that gameplay mods keep
  rebuilding

### 12. Papyrus-side event registries are a recurring framework shape

Seen strongly in:

- `Acheron`
- `NavigationRestrictions-master`
- `PapyrusTweaks-main`

Repeated pattern:

- repeated `RegisterFunction(...)` blocks
- `SKSE::RegistrationSet` managers over forms, aliases, and active effects
- save/load/revert/form-delete plumbing for those registrations
- deferring real work from Papyrus callbacks onto the task interface

SDK implication:

- `sdk::papyrus` now has a real higher-level registry/event layer on top of the
  low-level `skse::registration_set*` support
- the next Papyrus-facing gaps are deferred task handoff ergonomics, richer
  runtime install/lifecycle glue, and more compact `RegisterFor...` authoring
  sugar

### 13. Magic, active-effect, and injury workflows still fall into plugin-local utilities

Seen strongly in:

- `injury-alt-death`
- `death-alternative-mod-main`
- `Unblockable`
- `Acheron`

Repeated pattern:

- scan active effects by keyword or exact `EffectSetting`
- iterate actor spells through `VisitSpells(...)`
- cast immediate spells through `GetMagicCaster(...)->CastSpellImmediate(...)`
- react to sleep or magic-effect events to clean up, upgrade, or downgrade state

SDK implication:

- `sdk::gameplay::magic` is now a real high-value helper layer
- the next gaps are more compound cast/cleanup workflows and continued API
  tightening as real plugin usage shakes out the most important spell/effect
  flows

### 14. Inventory and equipment workflows are still repeatedly reconstructed

Seen strongly in:

- `Acheron`
- `NavigationRestrictions-master`
- `Unblockable`

Repeated pattern:

- scan inventory with filters for worn, quest, playable, keyword, or value state
- cache worn armor and re-equip it later
- grouped remove / transfer / unequip flows with repeated reason selection
- item-count gates for gameplay behavior

SDK implication:

- `sdk::gameplay::inventory` now has a real first helper layer
- the next gaps are richer equip/unequip/re-equip workflows, more opinionated
  transfer/remove scenarios, and better worn-state restoration sugar

### 15. Tiny UI and lifecycle safety predicates are a real repeated need

Seen strongly in:

- `LoadingScreenTruce01`
- `LoadingScreenTruce02`
- `CustomDodge`
- `Acheron`
- `NavigationRestrictions-master`

Repeated pattern:

- bail out while loading, fading, paused, or console/menu states are active
- combine those checks with `ControlMap` or `PlayerControls` state

SDK implication:

- `sdk::ui::controls` now covers the first compact UI/input coordination layer
  for these checks
- follow-up work should deepen widget/notification glue instead of rebuilding
  the same loading/fader/menu predicates yet again

## Current SDK Coverage vs Gaps

Current SDK already has meaningful foundations in:

- hooks
- event subscription
- borrowed vs stable pointer semantics
- serialization / cosave schema
- basic gameplay wrappers
- basic menu fade helpers
- a first Havok raycast helper
- a first usable external plugin API helper layer
- a first usable plugin-to-plugin messaging helper layer

Current weak or placeholder areas:

- high-level actor cache / scan helpers
- deeper widget / notification / HUD synchronization helpers
- richer spatial query helpers
- gesture and control-handler helpers in `sdk::gameplay::input`
- projectile-targeting helpers
- provider-side service registries above the current `sdk::interop::external_api`
  publication helpers

## Progress Notes

### 2026-03-29: interop first pass implemented

The SDK now has a first usable implementation for:

- `sdk::interop::external_api`
  loaded module lookup, export lookup, and `RequestPluginAPI`-style helpers
- `sdk::interop::messaging`
  install-once inter-plugin listeners, typed dispatch helpers, version
  handshake types, synchronous query envelopes, and a reusable `RequestClient`

### 2026-03-29: interop server builder pass implemented

`sdk::interop::messaging` now also includes a reusable server-side protocol
builder:

- `RequestServer`
- `RequestServerBuilder`
- `QueryContext`

This closes the earlier gap where server-side interop still required raw
`listen_*` glue and manual `VersionHandshake` / `QueryEnvelope` decoding.
Client/server protocols can now be declared through typed handler registration
for raw, value, notify, and array-style requests.

### 2026-03-29: `sdk::ui::menus` first workflow pass implemented

`sdk::ui::menus` now covers the menu operations we already have enough RE
surface to support cleanly:

- `UI` / `UIMessageQueue` singleton access
- menu and movie-view lookup
- top-most menu lookup
- menu visibility and pause/application/item/modal state predicates
- menu registration
- queued open / close / force-hide / reshow messaging
- named-menu sugar through a lightweight `NamedMenu` trait
- `MenuOpenCloseEvent` subscriptions with filtered open / close helpers
- `FaderMenu`-specific helpers such as `is_fader_open()` and
  `is_fader_active()`

### 2026-03-29: `sdk::ui::scaleform` first honest pass implemented

`sdk::ui::scaleform` now covers the Scaleform surfaces we can already expose
without inventing fake `GFxValue` support:

- owner-backed `MenuSurface` lookup for open menus
- `GFxMovieView` lookup from named or top-most menus
- `FxDelegate` lookup from named or top-most menus
- menu-side Scaleform state predicates and pointer accessors

What is still intentionally deferred until more RE translation lands:

- `GFxValue` construction and traversal
- `GFxMovieView::Invoke`-style helpers
- typed variable get/set helpers
- `FxDelegate` callback registration sugar

### 2026-03-29: `sdk::gameplay::input` workflow pass implemented

`sdk::gameplay::input` now covers the most common plugin-facing input
workflows we already have enough RE surface to support cleanly:

- `ControlMap`, `PlayerControls`, and `UserEvents` singleton access
- context-stack snapshot and top-context predicates
- menu-like context classification helpers
- enabled-control snapshots plus restore-on-drop guards
- scoped gameplay-input suppression and scoped context pushes
- text-input request guards
- simple user-event mapping and button-name lookup helpers

### 2026-03-29: `sdk::gameplay::actors` / `combat` first useful pass implemented

`sdk::gameplay::actors` now covers the most common actor-scanning workflows we
can already expose honestly from `ProcessLists` and translated `Actor` helpers:

- loaded/high actor traversal with early-exit `ControlFlow`
- retained `Vec<Resolved<Actor>>` collection helpers
- point/player proximity helpers
- player-command / summon / teammate categorization that is source-backed today

`sdk::gameplay::combat` now composes those helpers into a few reusable combat
operations:

- hostile-nearby collection through `ProcessLists::are_hostile_actors_near`
- cached faction-fight cleanup
- stop-combat helpers for a single actor, the player, retained actor slices,
  nearby actors, and nearby hostiles

Current intentional limitation:

- keep the current helpers explicitly player-centric; we now have
  `is_hostile_to_actor`, but broader faction/relationship abstractions still
  need more deliberate API shaping than a quick wrapper pass

### 2026-03-29: gameplay/UI SDK refactor after `TESObjectREFR` / `Actor` / `PlayerCharacter` / `GFx` upgrades

The strengthened RE layer enabled a second-pass SDK refinement rather than just
new wrappers:

- `sdk::gameplay::actors` now has honest hostility helpers:
  `is_hostile_to`, `is_hostile_to_player`, `is_player_ally`,
  `collect_hostile_actors`, `collect_hostile_nearby_*`, and
  `collect_player_allies`
- `sdk::gameplay::combat` now composes that into
  `is_player_in_combat` and radius-based hostile combat cleanup
- `sdk::gameplay::player` now exposes `is_in_combat` and the current
  `actor_doing_player_command`
- `sdk::ui::scaleform` is no longer just lookup-oriented; it now exposes
  menu-side `is_available`, `get_variable`, `set_variable`,
  bool/number variable setters, and `invoke` / `invoke_no_return` through
  `MenuSurface`

New intentional limitation after this pass:

- Scaleform object/member traversal still lives mostly at the RE layer; the SDK
  now covers the movie-level workflows, but not yet a richer object/member DSL
- `sdk::ui::menus` now has first-pass typed payload helpers, but broader menu
  payload coverage beyond `FaderData`, `LoadingMenuData`, `BSUIMessageData`,
  and `BSUIScaleformData` is still mostly absent

### 2026-03-29: `sdk::ui::menus` typed payload helpers added

`sdk::ui::menus` now exposes a shared typed `IUIMessageData` path over
`UIMessageQueue::create_ui_message_data`, instead of forcing every workflow
through raw `*mut IUIMessageData`.

Current surface:

- `TypedMenuMessageData`
- `queue_message_with::<T>(...)`
- `queue_named_message_with::<M, T>(...)`
- `queue_fade(...)` rebuilt on top of the shared typed-message helper
- `LoadingMenuRequest`, `queue_loading_menu(...)`, and `show_loading_menu(...)`
- `queue_bsui_bool_message(...)`, `queue_bsui_uint_message(...)`,
  `queue_bsui_ptr_message(...)`, `queue_bsui_string_message(...)`,
  `queue_bsui_string_bool_message(...)`,
  `queue_bsui_string_float_message(...)`, and
  `queue_bsui_string_uint_message(...)`
- `queue_scaleform_event_message(...)`

Intentional scope:

- keep this layer centered on already translated message-data classes
- do not expose a safe public API that constructs queue-owned payloads and lets
  them escape before submission
- keep the API menu-oriented rather than turning `sdk::ui::menus` into a
  generic UI object factory

### 2026-03-29: consolidated `SKSEProjects` research pass

The new `S:\Programming\SKSEProjects` folder made it easier to compare patterns
across many real plugins in one place. Detailed notes live in
[`SKSEPROJECTS_RESEARCH.md`](./SKSEPROJECTS_RESEARCH.md).

Highest-signal repeated findings from this pass:

- exported `RequestPluginAPI` headers are still the dominant interop shape
- messaging-based interface loaders are common when lifecycle ordering matters
- Scaleform work repeatedly turns into an object/member DSL built manually on
  top of `GFxMovieView` and `GFxValue`
- custom menu work repeatedly needs menu registration, SWF load helpers,
  `ProcessMessage`, and scoped function injection
- Havok/raycast plugins repeatedly want collision masks, ignore filters,
  multi-hit collectors, hit resolution, split rays, and backoff helpers
- actor/gameplay mods repeatedly scan `ProcessLists`, read current combat
  targets, and maintain small versioned actor caches

Implication for SDK direction:

- `sdk::ui::scaleform` object/member helpers and function binding now look like
  one of the highest-value next passes
- `sdk::advanced::physics` needed a first serious pass for multi-hit, layer
  masks, hit resolution, and line-of-sight helpers
- `sdk::interop` should grow an API-repository / interface-loader layer
- `sdk::ui::menus` should keep broadening typed payload coverage and eventually
  grow custom-menu scaffolding

### 2026-03-29: `sdk::advanced::physics` first serious pass

`sdk::advanced::physics` now covers the first real gameplay-quality Havok query
surface instead of a single thin closest-hit helper.

Current surface:

- `LayerMask` as an explicit post-query collision-layer blocker mask
- `RaycastHit` with resolved world-space point / normal, raw Havok pointers,
  raw `TESObjectREFR` / `NiAVObject` pointers, and on-demand
  `resolved_reference()`
- world-locked `raycast_segment(...)` and `raycast_all_segment(...)`
- layer-filtered closest helpers like `raycast_segment_layers(...)`
- actor-aware query filters via `actor_filter(...)`,
  `actor_line_of_sight_filter(...)`, and `actor_item_pick_filter(...)`
- line helpers like `segment_is_clear(...)` / `has_line_of_sight(...)`
- relocation helpers like `backoff_point(...)` and `offset_point_along_normal(...)`
- downward helpers like `raycast_down_layers(...)` and `ground_snap_point(...)`

Intentional limits after this pass:

- there is still no first-class ignore-list/object-filter layer like the one
  many camera / blink / telekinesis mods build on top of raw hit arrays
- split-ray and bundled clearance/headroom validation are not in the SDK yet
- relocation-candidate validation still needs a higher-level helper that
  combines raycast, navmesh, and gameplay constraints
- low-level Ni/Havok pointers returned in `RaycastHit` intentionally stay raw;
  only runtime references are upgradeable into `Resolved<TESObjectREFR>`

### 2026-03-30: utility-library and projectile-framework research pass

Follow-up research over `CLibUtil-master`, `StyyxUtils-main`,
`UselessFenixUtils-master`, and `NewProjectilesTMP-master` sharpened three
strong SDK directions.

Highest-signal findings:

- form-string parsing, editor-ID lookup, plugin-loaded checks, and typed config
  parsing are still rebuilt in many utility libraries
- data-driven projectile mods repeatedly decompose into target acquisition,
  pattern generation, projectile runtime state, and composable behaviors like
  homing, followers, emitters, and multicast
- imgui/debug-render/behavior-graph helpers are useful, but they still look
  more like optional future utility layers than core SDK material

Implication for SDK direction:

- `sdk::forms::lookup` moves up in priority now that more local projects repeat
  the same form parsing and editor-ID patterns
- `sdk::plugin::config` should likely gain small typed helpers for config
  access, hotkey parsing, and config-to-form resolution glue
- a future `sdk::gameplay::projectiles` module now looks justified, but it
  should start with narrow building blocks instead of copying one mod's whole
  data-driven architecture
- `sdk::gameplay::{actors, combat, magic}` can continue cherry-picking narrow
  utility helpers from broad C++ utility libraries without introducing a giant
  catch-all `utils` module

Intentional non-goals after this pass:

- do not rush a core `imgui` or generic debug-render framework into `sdk`
- do not bake `NewProjectilesTMP`'s exact JSON schema or hook topology into the
  SDK surface

### 2026-03-30: `sdk::forms::lookup` first pass implemented

`sdk::forms::lookup` now covers the highest-frequency form-resolution workflows
that were repeatedly showing up in local utility libraries.

Current surface:

- `TESDataHandler` singleton access through `data_handler()`
- plugin file / load-order checks like `plugin_file(...)`,
  `loaded_plugin_file(...)`, and `plugin_loaded(...)`
- local/raw form-ID resolution through `resolve_form_id(...)` and
  `resolve_raw_form_id(...)`
- typed and untyped lookup for local/raw form IDs
- editor-ID string lookup through `lookup_by_editor_id(...)`
- community-standard `po3_Tweaks` fallback for `form -> editor_id` through
  `try_editor_id(...)` / `editor_id(...)`
- `"Plugin.esp|0x123"` parsing and direct lookup through
  `parse_plugin_form_id(...)`, `form_from_string(...)`, and raw/typed variants

Intentional limits after this pass:

- `form_from_string(...)` currently follows the common `"plugin|local form id"`
  interpretation rather than trying to guess every possible raw/ESL encoding
- this pass is lookup-oriented; it does not attempt to become a patching or
  distribution framework
- config loaders and hotkey parsing still belong in the next
  `sdk::plugin::config` pass

### 2026-04-01: C++ control / Papyrus / recovery pass reprioritized the next SDK gaps

The follow-up pass over `CustomDodge`, `PapyrusTweaks-main`,
`NavigationRestrictions-master`, `Unblockable`, `Acheron`,
`death-alternative-mod-main`, `injury-alt-death`, and
`LoadingScreenTruce01/02` changed the near-term SDK picture.

Highest-signal findings:

- simple config hotkeys are no longer the main input gap; the repeated missing
  layer is gesture handling plus scoped movement / attack handler locks
- the repository already has low-level `skse::registration_set*`, but plugins
  still rebuild their own persistent Papyrus event registries above it
- `sdk::gameplay::magic` and `sdk::gameplay::inventory` remain far emptier than
  the real plugin demand shown by recovery, combat, and utility mods
- many mods repeatedly hand-roll tiny "unsafe phase" checks for loading, fader,
  pause, console, and control-map state
- custom exported service symbols such as subscriber registries show that
  interop needs provider-side helpers, not just API request clients

Implication for SDK direction:

- the next work should favor cross-cutting gameplay and Papyrus glue layers
  over another content-heavy one-mod-shaped subsystem
- `projectiles`, deeper Scaleform DSL work, and more physics are still valid,
  but they no longer outrank the missing `magic`, `inventory`, Papyrus event,
  and input-control surfaces

### 2026-04-03: first-pass cross-cutting SDK gaps landed

Several of the highest-priority gaps from the 2026-04-01 reprioritization pass
now have real SDK surfaces:

- `sdk::papyrus`
  persistent event registries, grouped save/load/revert/form-delete helpers,
  runtime registration sets, and event-oriented macro sugar over
  `skse::registration_set*`
- `sdk::gameplay::magic`
  active-effect queries, item/spell inspection, actor-spell traversal and
  mutation, immediate-cast helpers, and source-slot runtime control
- `sdk::gameplay::inventory`
  inventory-entry snapshots, typed/filtered collection, equipped/worn queries,
  and grouped remove/transfer helpers
- `sdk::gameplay::quests`
  quest/objective/stage/alias snapshots, alias/objective lookup, and common
  quest lifecycle wrappers
- `sdk::ui::controls`
  compact UI/input coordination helpers such as `UiControlSnapshot`,
  `is_ui_capturing_input()`, `is_gameplay_input_available()`, and
  `should_show_hud_widgets()`
- `sdk::interop::external_api`
  provider-side publication helpers, version selection, and export macros in
  addition to the existing request/client surface
- `sdk::forms::{keywords, lists, settings}`
  no longer placeholder-heavy; these now cover first-pass traversal, lookup,
  mutation, typed value access, and store iteration workflows

This shifts the immediate SDK backlog away from "add any layer at all" and
toward deepening the already-started surfaces:

- richer input gestures and handler guards
- widget/notification glue on top of `ui::controls`, `ui::menus`, and
  `ui::scaleform`
- projectile and targeting helpers
- higher-level interop registrars and provider-side service patterns

## Recommended Near-Term SDK Backlog

Priority order as of this pass:

1. Deepen `sdk::gameplay::input` with gesture detection, directional snapshots,
   and scoped movement / attack handler guards.
2. Expand `sdk::ui` beyond menus/controls/scaleform with notifications and
   stronger widget-state synchronization helpers.
3. Add `sdk::gameplay::projectiles` once the cross-cutting input/UI helpers
   above are stable enough to support targeting-heavy mods cleanly.
4. Continue the deeper `sdk::ui::scaleform` object/member and callback pass
   once the more repeated widget/HUD gaps above are covered.
5. Continue broadening `sdk::advanced::physics` where new gameplay helpers
   still need lower-level collision support.
6. Add higher-level interop registrars and provider-side service patterns above
   the current `external_api` publication/export helpers.

### 2026-03-31: respawn / spatial-query backlog clarified

The current local research pass over `GhostOfDeath`, `SKSEProjects`, and the
existing SDK surface sharpened what should happen next for spatial-query and
respawn-oriented helpers.

Confirmed near-term SDK additions:

- bundled spawn-candidate validation helpers that combine:
  - ground snap
  - headroom
  - local clearance
  - optional LOS checks
  - optional navmesh sanity / relocation checks
- `sdk::gameplay::world` scene helpers such as:
  - richer `snapshot_scene_in_range(...)`
  - `collect_hostile_positions_in_range(...)`
  - `collect_actor_positions_in_range(...)`

Progress since this note:

- `sdk::gameplay::movement` now exposes source-backed translation helpers over
  `ExtraRefrPath` and `TESObjectCELL::AddTranslateObject`
- `sdk::gameplay::world::snapshot_scene_in_range(...)` now exists as the first
  reusable scene-snapshot surface
- `sdk::gameplay::world::snapshot_scene_in_cell_range(...)` now exists for
  point-based local scene queries
- `sdk::gameplay::actors` now exposes `collect_actor_positions_in_range(...)`
  and `collect_hostile_positions_in_range(...)`
- `sdk::advanced::physics` now exposes:
  - `split_raycast(...)`
  - hit filtering / `best_hit(...)` over `raycast_all_*`
  - a first bundled `validate_spawn_point(...)`
  - `RaycastHitFilter` plus filtered split-ray / LOS / spawn-validation flows
  - `SpawnPointValidationOptions::{new, respawn_default, with_*}`
  - richer clearance bundles through `clearance_probe_segments*` and
    `clearance_is_clear_with_filter(...)`
  - multi-origin LOS aggregation through
    `line_of_sight_from_points_with_filter(...)`
- `sdk::gameplay::spatial` now exposes a higher-level candidate
  evaluation workflow that composes:
  - point-based scene snapshotting
  - nearest actor / hostile distance heuristics
  - filtered physics validation
  - weighted scoring / best-candidate selection
  - candidate-set evaluation
  - fallback-tier selection
  - typed rejection diagnostics per tier / candidate
- `sdk::gameplay::navmesh` now exposes the first honest point-based navmesh SDK
  surface:
  - `snapshot_cell_navmeshes(...)`
  - `snapshot_reference_navmeshes(...)`
  - `collect_navmesh_vertices_in_cell_range(...)`
  - `collect_navmesh_triangle_centers_in_cell_range(...)`
  - `query_point_in_cell(...)`
  - nearest vertex / triangle-center helpers
  - `has_navmesh_support_in_cell(...)`
- `sdk::gameplay::navmesh` now also exposes conservative path-cost /
  reachability heuristics:
  - `NavMeshReachabilityHeuristicsOptions`
  - `NavMeshReachabilityHeuristics`
  - `evaluate_reachability_in_cell(...)`
  - `evaluate_reachability_from_reference(...)`
  These now cover same-mesh triangle-center graph heuristics plus a first
  cross-mesh approximation via navmesh-info adjacency, but still do not claim
  full Bethesda pathfinding parity.
- `sdk::gameplay::pathing` now exposes the first thin SDK layer over the
  already-translated pathing RE seams:
  - strict `Pathing` singleton access
  - `exterior_cell_width()`
  - `inspect_pathing_cell(...)`
  - `is_pathing_cell_ready(...)`
  - `inspect_concrete_pathing_cell(...)`
  - `inspect_navmesh_info_concrete_pathing_cell(...)`
  - `collect_loaded_pathing_cells(...)`
  - `collect_recent_pathing_cells(...)`
  - `pathing_cells_share_space(...)`
  - `make_pathing_location(...)`
  - `get_pathing_cell(...)`
  - `get_pathing_cell_for_reference(...)`
  - `find_closest_point_on_navmesh(...)`
  - `lookup_navmesh_info(...)`
  - `collect_navmesh_infos(...)`
  - `collect_connected_navmesh_infos(...)`
  - `collect_potential_navmeshes_for_location(...)`
  - `collect_connected_navmesh_infos_for_location(...)`
  - `collect_connected_navmeshes_for_location(...)`
  - `nav_mesh_info_map()`
  - `collect_adjacent_navmesh_ids(...)`
  - `collect_adjacent_navmesh_infos(...)`
  - `lookup_navmesh_info_for_navmesh(...)`
  - `approximate_navmesh_info_graph_path(...)`
  - precomputed navmesh-info path lookup helpers over
    `BSPrecomputedNavmeshInfoPathMap`
  - `collect_loaded_navmeshes(...)`
  - `collect_loaded_navmesh_infos(...)`
  - `pathing_door_from_collision(...)`
  - portal helpers over `ExtraNavMeshPortal` and `BSNavmesh` edge transitions
  This now has an honest global `Pathing` singleton surface where source-backed
  `re::Pathing` coverage exists, but still does not pretend the full Bethesda
  higher pathing stack is translated.
- `sdk::gameplay::spatial` now folds nearest navmesh support into candidate
  validity and weighted scoring
- `sdk::gameplay::spatial` now also folds optional reachability origin /
  path-cost heuristics into candidate validity and scoring, and can select a
  best candidate through tiered fallback policies instead of only single-pass
  best-score selection
- `sdk::gameplay::spatial` now exposes typed rejection reasons and per-tier
  rejection summaries, so plugin code can log or react to failed selection
  policies without reverse-engineering a pile of boolean fields

Still open in this area:

- richer navmesh-aware scoring beyond nearest support / triangle-center
  heuristics
- stronger path-cost / reachability validation once an honest point-based
  cross-mesh / pathing surface exists
- stronger navmesh-aware spawn validation layered on top of the now-richer
  physics clearance / LOS bundles
- richer `sdk::gameplay::spatial` diagnostics and reusable tier presets for
  respawn-oriented selection policies

These should be treated as reusable SDK material rather than reimplemented in
plugin code, because they now show up as recurring gameplay-quality spatial
workflows rather than one-off hacks.

Important constraint:

- `BSPathingLOSGridMap`
- `BSPathingSearchRayCast`

are not good immediate targets for SDK dependence. In the available
`CommonLibVR` base they are declaration-only, not source-backed behavioral
surfaces we can honestly translate and rely on yet.

Practical consequence:

- do not block the respawn/spatial-query roadmap on a full Bethesda pathing
  stack port
- build the near-term SDK on the honest primitives we already have:
  - `sdk::advanced::physics`
  - `sdk::gameplay::world`
  - `sdk::gameplay::actors`
  - `sdk::gameplay::navmesh`
  - `TESObjectREFR::find_nearest_vertex(...)`
  - `TESObjectREFR::move_to_nearest_navmesh(...)`

RE-side candidates still worth pursuing when source-backed coverage is
available:

- `ExtraRefrPath`
- `TESObjectCELL::AddTranslateObject`
- richer `BSNavmesh` coverage

## Candidate API Shapes

These are directional examples, not frozen signatures.

### External API interop

- `request_proc_api::<T>(dll_name, symbol_name)`
- `request_plugin_api::<T, V>(dll_name, version)`
- `request_messaging_api::<T, V>(messaging, dependency_name, version)`
- `LazyApi<T>`
- `ApiError::{DllMissing, SymbolMissing, VersionMismatch, NotReady}`

### Messaging request / response

- `RequestClient::init(...)`
- `RequestClient::query(...)`
- `RequestClient::query_array(...)`
- `RequestServer::register_handler(...)`
- `HandshakeVersion`
- `ResponseCode`

### Menu / widget helpers

- `register_menu::<T>(menu_name, factory)`
- `open_menu(menu_name)`
- `close_menu(menu_name)`
- `toggle_menu(menu_name, open)`
- `on_menu_open_close::<T>(...)`
- `should_show_hud_widgets()`

### Scaleform helpers

- `invoke(root, "method", args)`
- `set_member_bool(...)`
- `set_member_number(...)`
- `get_member_string(...)`
- `register_fx_callback(...)`

### Forms and config helpers

- `lookup_form(plugin_name, raw_form_id)`
- `form_from_string("Mod.esp|0x123")`
- `lookup_by_editor_id<T>(...)`
- `plugin_loaded("SomeDependency.dll")`
- `parse_hotkey_combo("Shift+E")`
- `ini_get_bool(...)`, `ini_get_i32(...)`, `ini_get_f32(...)`
- `Config::bool/f32/u32/...`
- `Config::form_typed(...)`
- `HotkeyCombo::currently_pressed()`
- `HotkeyCombo::just_pressed_in(...)`

### Input helpers

- `InputGesture::tap(combo)`
- `InputGesture::hold(combo, duration)`
- `DirectionalInputSnapshot::capture_now()`
- `movement_input_guard()`
- `attack_input_guard()`
- `can_run_gameplay_input()`

### Papyrus event helpers

- `PapyrusEventRegistry::new("OnActorDefeated")`
- `registry.register_form(form)`
- `registry.register_alias(alias)`
- `registry.register_active_effect(effect)`
- `registry.save/load/revert/form_delete(...)`
- `registry.send((payload, ...))`
- `registry.queue_send((payload, ...))`

### Magic helpers

- `has_active_effect(effect)`
- `has_active_effect_with_keyword("CureInjury")`
- `collect_active_effects_matching(...)`
- `visit_spells(actor, |spell| ...)`
- `collect_spells_matching(...)`
- `cast_spell_immediate(caster, target, spell)`
- `remove_spells_matching(...)`

### Inventory helpers

- `collect_worn_armor(actor, ignored_mask)`
- `collect_inventory_items(container, filter)`
- `remove_items_matching(container, filter, target)`
- `transfer_items_matching(from, to, filter)`
- `item_count(form)`

### Physics helpers

- `raycast_all_segment(...)`
- `raycast_segment_layers(...)`
- `ground_snap_point(...)`
- `has_line_of_sight(...)`
- `actor_line_of_sight_filter(...)`
- `LayerMask`
- `validate_relocation_candidate(...)`
- `backoff_point(...)`

### Projectile helpers

- `collect_projectile_targets(...)`
- `collect_cursor_targets(...)`
- `pattern_positions_line(...)`
- `pattern_positions_circle(...)`
- `pattern_positions_sphere(...)`
- `projectile_shooter(...)`
- `projectile_desired_target(...)`
- `projectile_speed(...)`

### Actor and runtime helpers

- `for_each_nearby_actor(...)`
- `collect_hostile_actors(...)`
- `collect_player_followers(...)`
- `is_player_ally(...)`
- `is_summon_or_commanded_actor(...)`
- `stop_combat_near(...)`

## Guardrails

- Do not overfit the SDK to one mod's naming or architecture.
- Prefer small composable helpers over giant monolithic frameworks.
- Keep `GameRef` / `GamePtr` reserved for truly stable pointer sources.
- Keep retained runtime semantics in `Resolved<T>`.
- Keep low-level `Ni` / `hk` ownership models honest; only add SDK helpers when
  there is a repeated workflow worth smoothing out.
- For low-level `Ni` / `hkb` / `hk` families, prefer targeted helper functions
  over blanket "safe wrappers" unless ownership and lifetime semantics are
  genuinely understood.

## Revisit Queue

Projects and areas worth revisiting in later passes:

- `TrueDirectionalMovement`
  API interop, event installs, raycast helpers, camera / lock-on logic
- `SkyrimSE-SmoothCam`
  camera state, input state, menu / gameplay transitions
- `TrueHUD`
  widget lifecycle, HUD menu integration
- `Precision`
  collision query quality, hit interpretation
- `MCM-Helper`
  config + Papyrus + UI glue
- `QuickLootIE`
  request / response messaging and Scaleform patterns
- `SkyPatcher`
  form lookup and data-driven patching helpers
- `TrueFlasksNG`
  API repository pattern and actor caches
- `GigaWidget`
  widget API export and menu visibility synchronization
- `SkyrimCrashGuard`
  config menu and defensive runtime helpers
- `Reflyem`
  breadth of gameplay utility and actor-state patterns
- `CustomDodge`
  action-mapped input, analog direction capture, and scoped input-handler locks
- `NavigationRestrictions-master`
  menu gating, item-count workflows, and tiny Papyrus + serialization glue
- `PapyrusTweaks-main`
  Papyrus-facing surface design and the boundary between SDK helpers and VM surgery
- `Unblockable`
  combat-state scans, animation-event-driven gameplay reactions, and delayed task handoff
- `death-alternative-mod-main`
  exported subscriber APIs, sleep/effect event sinks, and recovery flow pieces
- `injury-alt-death`
  injury/spell helpers and sleep-gated recovery logic
- `LoadingScreenTruce01`
  minimal loading/fader safety predicates
- `LoadingScreenTruce02`
  the same loading/fader guard pattern in a split hook file
- `CLibUtil-master`
  form-string parsing, editor-ID lookup, distribution, and hotkey helpers
- `StyyxUtils-main`
  compact gameplay/menu/forms helper patterns
- `UselessFenixUtils-master`
  geometry, behavior-graph traversal, and optional debug/imgui patterns
- `NewProjectilesTMP-master`
  data-driven projectile runtime and target-selection patterns

## 2026-03-30: `sdk::plugin::config` first pass implemented

`sdk::plugin::config` now covers the main patterns that were repeatedly showing
up in local utility libraries and config-driven plugins.

Current surface:

- borrowed `Config<'a>` wrapper over `Ini`
- typed field access: `bool`, `i32`, `u32`, `f32`, `parsed<T>`, `*_or(...)`
- field and section presence helpers
- config-to-form glue via `sdk::forms::lookup`
- parsed `HotkeyCombo` with keyboard, mouse, and common gamepad aliases
- current-state matching through translated input-device state
- event-batch activation checks through `sdk::events::input::InputEvents`

Important guardrail:

- this pass intentionally stays INI-first and helper-oriented
- it does not try to become a schema framework, MCM framework, or generic
  config runtime yet

## How To Update This File

When doing future research passes:

- record the concrete repos and files sampled
- write down repeated patterns, not just interesting one-off tricks
- note whether the pattern belongs in `re`, `sdk`, or plugin code
- update the backlog ordering when a new pattern clearly outranks old ones
- keep this file biased toward reusable SDK value, not toward mod-specific
  design notes

## 2026-03-30: direct `FadeOutGame` helper added to `sdk::ui::menus`

While debugging `GhostOfDeath`, `UIMessageQueue::AddMessage` for `Fader Menu`
still crashed even after moving the call onto a `Main::Update` main-thread
dispatcher and even after isolating away the typed `FaderData` payload.

A useful community-backed fallback seam was identified from `CustomSkills`:

- `FadeOutGame` relocation used as a direct engine call
- SE ID `51909`
- AE ID `52847`

`sdk::ui::menus` now exposes:

- `fade_out_game_direct(...)`
- `fade_to_black_direct(...)`
- `fade_from_black_direct(...)`

Important caveat:

- the fourth bool is still source-backed only as an unnamed `arg4` from public
  community references; mapping it to `pauses_game` in the convenience wrappers
  is an inference from `FaderData`, not yet a proven engine symbol name
