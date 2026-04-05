# SDK Plugin Audit

Last updated: 2026-04-05

This note compares the current `libskyrim::sdk` surface against repeated
patterns in local SKSE plugin code. The goal is not to prove ABI correctness;
it is to answer a different question:

- does a given SDK layer match real plugin workflows?
- is the abstraction missing an important layer?
- did we build something that looks neat in Rust but does not line up with how
  plugins actually work?

## Legend

- `green`: the SDK layer matches repeated real-world plugin patterns well
- `yellow`: the SDK layer is directionally right, but still misses an important
  higher-level pattern or has likely over/under-abstraction
- `red`: the SDK layer is still placeholder-heavy or does not yet reflect the
  dominant plugin pattern

## Sampled plugin evidence

The audit was based on local plugin code under `S:\Programming\SKSEProjects`.
The most important sampled files were:

- `TrueHUD-master/src/HUDHandler.h`
- `TrueHUD-master/src/HUDHandler.cpp`
- `QuickLootIE/src/LootMenuManager.cpp`
- `QuickLootIE/src/Observers/MenuObserver.cpp`
- `TrueDirectionalMovement-master/src/API/APIManager.cpp`
- `TrueDirectionalMovement-master/src/API/SmoothCamAPI.h`
- `Precision-main/src/PrecisionAPI.h`
- `death-alternative-mod-main/src/ResurrectionAPI.h`
- `death-alternative-mod-main/src/main.cpp`
- `Unblockable/include/SKSEMCP/SKSEMenuFramework.hpp`
- `Acheron/src/Serialization/EventManager.h`
- `Acheron/src/Serialization/EventManager.cpp`
- `Acheron/src/Util/FormLookup.h`
- `CustomDodge/src/plugin.cpp`
- `CustomDodge/src/SimpleDodge.cpp`
- `NavigationRestrictions-master/src/Main.cpp`
- `NavigationRestrictions-master/src/papyrus.cpp`
- `BetterTelekinesis/src/Config.cpp`
- `SkyrimSE-SmoothCam/SmoothCam/source/raycast.cpp`

These plugins were chosen because together they cover the main recurring SDK
concerns: messaging, exported APIs, papyrus registration sets, serialization,
Scaleform/UI runtime, input sinks, hooks, raycasts, targeting, INI/config,
form lookup, and gameplay helpers.

## Layer Assessment

### `sdk::core`

Status: `green`

Why:

- This layer is mostly infrastructure, so plugin code does not mirror it
  directly. Nothing in the sampled plugins suggests that `core::{refs,handles,
  owners,cast,containers,phase}` is conceptually wrong.
- `core::phase` matches the repeated "do not run gameplay work during
  load/fade/menu capture" pattern that shows up indirectly in HUD/menu plugins
  and task deferral code.

Risk:

- Low direct risk, but this layer still needs continued pressure-testing by
  higher-level SDK passes instead of assuming internal elegance implies real
  plugin value.

### `sdk::events`

Status: `green`

Why:

- Repeated event sink patterns in sampled plugins line up well with
  `sdk::events::{source,bus,install,input,game,ui,skse}`.
- `CustomDodge`, `NavigationRestrictions`, `QuickLootIE`, and `TrueHUD` all use
  classic SKSE/engine event wiring that the current SDK surface already models
  closely.

Risk:

- Low. The main remaining risk is ergonomics drift rather than conceptual
  mismatch.

### `sdk::papyrus`

Status: `green/yellow`

Why:

- `Acheron` strongly validates the current `PapyrusEventRegistry`,
  targeted registries, grouped save/load/revert/form-delete support, and the
  runtime-installed registry pattern.
- `NavigationRestrictions` and `PapyrusTweaks` validate the need for compact
  registration macros and `callable_from_tasklets`-style metadata.

What still looks incomplete:

- The direct-registration and onboarding gap is much smaller now that the SDK
  has `PapyrusScript`, `register_script::<T>()`, `papyrus_script!`, broader
  rustdoc, and archetype-guided examples.
- The main remaining issue is now deeper plugin-shaped install/lifecycle
  recipes: plugin authors still need more end-to-end guidance for when to
  choose direct script binding, `PapyrusModule`, or persistent event-runtime
  registries inside a larger plugin bootstrap flow.

Repair direction:

- Focus on deeper end-to-end examples rather than another foundational rewrite.

### `sdk::plugin::{serialization,task,config}`

Status: `green/yellow`

Why:

- `Acheron`, `NavigationRestrictions`, and many plugin `main.cpp` files repeat
  the same save/load/revert/delete lifecycle pattern. The current
  `serialization` surface aligns well with that.
- `plugin::task` and `core::phase` match the common "handoff from callback into
  safe queued work" pattern visible in HUD and event-heavy plugins.
- `plugin::config` matches the repeated INI/hotkey/form-string workflow seen in
  `BetterTelekinesis` and utility-heavy plugins.

What still looks incomplete:

- Many plugins combine config parsing, form lookup, and gameplay installation
  in one place. Our pieces and docs now exist, but the actual workflow helper
  path across them is still a bit fragmented.

Repair direction:

- Add workflow bridges first, then deepen examples around them.

### `sdk::forms`

Status: `green/yellow`

Why:

- `Acheron/src/Util/FormLookup.h` and config-heavy plugins validate the
  importance of persistent editor-id lookup, `plugin:FormID` parsing, and typed
  lookup helpers.
- `keywords`, `lists`, `persistent`, and `settings` are all directionally
  correct and match repeated plugin utility code.

What still looks incomplete:

- The real plugin pattern often mixes string parsing, form resolution,
  persistent storage, and validation in one thin workflow. The current SDK has
  the pieces and much better docs now, but the top-level path can still feel
  more library-shaped than plugin-shaped.

Repair direction:

- Add more "workflow" helpers and a few end-to-end examples that bridge
  settings/config strings directly into typed forms and collections.

### `sdk::gameplay::input`

Status: `green`

Why:

- `CustomDodge/src/SimpleDodge.cpp` directly validates the current input layer:
  user-event matching, directional intent, thumbstick interpretation, and
  temporary handler disabling/restoring.
- The repeated pattern of "read event chain, infer direction, suppress
  gameplay input for a moment, then restore" lines up closely with the current
  `input` API.

Risk:

- Low. This is one of the most source-backed SDK domains right now.

### `sdk::gameplay::{magic,inventory,quests}`

Status: `yellow/green`

Why:

- These helpers map to common plugin-local utility code: worn armor scans,
  magic effect/spell queries, quest and alias inspection, and form-driven
  gameplay state.
- `Acheron` in particular validates that worn armor caching and form/keyword
  workflows are recurring needs.

What still looks incomplete:

- Many real plugins still combine these helpers into richer plugin-specific
  state machines. The current SDK provides building blocks, but not as many
  "compound workflows" as the plugin code often ends up reinventing.

Repair direction:

- Keep building compound patterns from live plugins instead of assuming the
  current query/mutation helpers are the final abstraction boundary.

### `sdk::gameplay::projectiles`

Status: `yellow`

Why:

- `NewProjectilesTMP`-style targeting, homing, and retarget behavior is
  directionally matched by the current projectile manager, target acquisition,
  and steering helpers.
- `Precision` and physics-heavy plugins validate that collision and
  projectile-adjacent logic belongs in the SDK.

What still looks incomplete:

- Real projectile plugins often go further into trigger-driven, data-driven, or
  multicast behavior systems. The current SDK intentionally stops earlier.
- That is not a bug, but it means the layer is still foundational rather than a
  near-complete plugin toolkit.

Repair direction:

- More audit passes against `NewProjectilesTMP` before widening the API further.

### `sdk::gameplay::{world,pathing}`

Status: `yellow/green`

Why:

- World scans, nearby reference collection, and navmesh portal helpers match
  recurring utility code in combat/camera/AI plugins.
- `SmoothCam` and physics/targeting code validate the usefulness of reference
  collection and world-query primitives.

What still looks incomplete:

- Real plugins often combine world traversal with richer filters and
  domain-specific collection logic. The SDK still exposes more primitives than
  opinionated workflows here.

### `sdk::ui::{menus,scaleform,controls,notifications,widgets,hud_runtime,driver,controller}`

Status: `green/yellow`

Why:

- `menus` and `scaleform` match real low-level building blocks very well:
  `UIMessageQueue`, `IMenu`, `GFxMovieView`, and menu open/close event plumbing
  are used exactly as expected in `TrueHUD` and `QuickLootIE`.
- `controls` and `notifications` match real HUD/menu state and UI update needs.
- `widgets`, `hud_runtime`, `driver`, and `controller` now match the repeated
  menu-owned widget runtime pattern: deferred menu tasks, menu open/close
  synchronization, visibility/mode coordination, one-cycle request drains, and
  request-bundle dispatch into concrete handlers.

Where the gap is:

- Real plugins still add one more plugin-shaped layer on top:
  - concrete install/observer glue
  - mod-specific widget registries
  - domain-specific show/hide/refresh recipes
- The SDK now covers the reusable runtime/driver/controller foundation, but not
  those final plugin-shaped install recipes.

Repair direction:

- Keep `sdk::ui` focused on reusable menu/widget runtime patterns and avoid
  overfitting the next layer to one specific HUD mod architecture.

### `sdk::interop::external_api`

Status: `green`

Why:

- The current layer now matches three major plugin interop patterns found in
  local code:
  - `RequestPluginAPI` / exported getter style (`Precision`, `TrueHUD`, TDM)
  - messaging-driven interface loading (`SmoothCam` via TDM)
  - flat exported symbol families and callback/subscriber symbols
    (`SKSEMenuFramework`, `ResurrectionAPI`)

Important boundary:

- C++-heavy APIs like `Precision` still require a plugin-local C++ shim when
  the interface uses `virtual`, `std::function`, `std::vector`, or overloaded
  methods. That is a correct boundary, not necessarily an SDK mistake.

Repair direction:

- Keep documenting the intended split clearly:
  `external_api` loads and organizes interop, but does not erase the need for a
  C++ shim for non-flat C++ APIs. The next examples should show that boundary
  through thin wrapper patterns, not hide it.

### `sdk::hooks`

Status: `green/yellow`

Why:

- The current `hooks::{trampoline,patch,patterns}` maps well onto repeated
  CommonLib-style patterns used by plugins like `BetterTelekinesis`,
  `NavigationRestrictions`, and other hook-heavy mods.

What still looks incomplete:

- The low-level hook vocabulary is there, but some repeated higher-level install
  recipes still live plugin-side.

Repair direction:

- Add a few more install recipes and examples, but the current foundation is
  sound.

### `sdk::persistence`

Status: `green/yellow`

Why:

- Small typed record persistence and save/load pipelines are strongly validated
  by sampled plugin serialization code.
- `cosave` and plugin serialization abstractions match recurring workflows.

What still looks incomplete:

- Some plugins still keep more bespoke versioned migration logic in their own
  code. The SDK should stay honest about where it ends.

### `sdk::advanced::{physics,vm}`

Status: `yellow/green`

Why:

- `physics` aligns with `SmoothCam`, `Precision`, and targeting-heavy plugin
  patterns. Raycasts, LOS, and filtering belong here.
- `vm` matches repeated Papyrus VM access and dispatch needs seen in
  Papyrus-heavy plugins.

What still looks incomplete:

- `physics` is currently a solid foundation but not yet a full mirror of richer
  collector/filter ecosystems in live plugins.
- `vm` needs more continued comparison against complex Papyrus-heavy plugins to
  make sure the dispatch/binding surface stays practical.

### `sdk::advanced::{render,scene}`

Status: `red`

Why:

- These remain the weakest major SDK domains. The local plugin evidence does not
  currently support a mature abstraction there, and the implementation is still
  comparatively placeholder-heavy.

Repair direction:

- Do not expand these speculatively. Audit more plugin code first and build only
  from repeated patterns.

## Most Likely Mismatches

The most likely places where the SDK is still incomplete or at risk of
misalignment are:

1. `sdk::forms` and `sdk::plugin::config` have the right primitives, but still
   need more direct "plugin workflow" bridges from config strings into resolved,
   validated forms.
2. `sdk::gameplay::{magic,inventory,quests,projectiles}` provide good building
   blocks but still stop earlier than some real plugin-side orchestration
   layers.
3. `sdk::advanced::{render,scene}` should not be treated as mature layers yet.

## Recommended Repair Order

Recommended next passes, in order:

1. Add config-to-form workflow helpers on top of `sdk::forms` and
   `sdk::plugin::config`.
2. Continue auditing `projectiles` and `physics` against
   `NewProjectilesTMP`-style data-driven behavior.
3. Leave `advanced::render` and `advanced::scene` for later, after collecting
   stronger plugin evidence.

## Bottom Line

The SDK is not drifting blindly. A large portion of the current surface already
matches repeated SKSE plugin patterns well, especially:

- `events`
- `interop::external_api`
- `gameplay::input`
- `hooks`
- `plugin::serialization`

The main remaining gaps are no longer low-level interop or the menu-owned UI
runtime itself. The reusable UI foundation is now present and the SDK now has
broader rustdoc plus archetype-guided onboarding. The remaining work is more
about workflow-shaped gaps around Papyrus authoring and config/form
orchestration, plus a few higher-level plugin install recipes.
