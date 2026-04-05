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
- `PhotoMode-master/src/ImGui/Renderer.cpp`
- `wheeler-main/src/bin/Rendering/RenderManager.cpp`
- `dMenu-main/src/bin/Renderer.cpp`
- `MaxsuDetectionMeter-main/src/Renderer.cpp`
- `SCAR-main/src/DataHandler.cpp`
- `BehaviorDataInjector-master/src/Hook.cpp`
- `BehaviorDataInjector-master/src/DataHandler.cpp`
- `CombatPathingRevolution-master/src/PayloadInterpreter/hooks.h`
- `DynamicAnimationCasting-main/src/Framework.cpp`
- `TK_Dodge_RE-main/src/TKRE.cpp`
- `MaxsuIFrame-main/src/Functions.cpp`
- `Simple-Power-Attack-main/include/SKSEMenuFramework.h`
- `Simple-Power-Attack-main/src/main.cpp`
- `skyrim-firmament-1.5/src/main.cpp`
- `skyrim-firmament-1.5/include/CustomSkills/Interfaces.inl`

These plugins were chosen because together they cover the main recurring SDK
concerns: messaging, exported APIs, papyrus registration sets, serialization,
Scaleform/UI runtime, input sinks, hooks, raycasts, targeting, INI/config,
form lookup, gameplay helpers, animation/behavior graph workflows, and native
overlay/render patterns.

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
- A second UI family is now clearly repeated in local plugins:
  `PhotoMode`, `wheeler`, `dMenu`, and `MaxsuDetectionMeter` all build native
  DX11 + ImGui overlays through `BSRenderManager`, swap-chain/device/context
  acquisition, `WndProc` hooks, and per-present render loops.
- That overlay family does not belong inside the current Scaleform/widget stack.
  It is better treated as a render/overlay SDK gap than as a failure of
  `sdk::ui`.

Repair direction:

- Keep `sdk::ui` focused on reusable menu/widget runtime patterns and avoid
  overfitting the next layer to one specific HUD mod architecture.
- Pair it with a separate `sdk::advanced::render` pass for native overlay mods
  instead of trying to force DX11/ImGui concerns into `sdk::ui`.

### `sdk::interop::external_api`

Status: `green/yellow`

Why:

- The current layer now matches three major plugin interop patterns found in
  local code:
  - `RequestPluginAPI` / exported getter style (`Precision`, `TrueHUD`, TDM)
  - messaging-driven interface loading (`SmoothCam` via TDM)
  - flat exported symbol families and callback/subscriber symbols
    (`SKSEMenuFramework`, `ResurrectionAPI`)
- The new local plugin set validates those same foundations again through
  `SKSEMenuFramework`, `CustomSkills`, `OpenAnimationReplacer`, `po3_Tweaks`,
  `TrueHUD`, and `Precision`.

Important boundary:

- C++-heavy APIs like `Precision` still require a plugin-local C++ shim when
  the interface uses `virtual`, `std::function`, `std::vector`, or overloaded
  methods. That is a correct boundary, not necessarily an SDK mistake.

What still looks incomplete:

- `skyrim-firmament-1.5` shows a lightweight but repeated fourth pattern:
  message-broadcast pointer APIs where a plugin listens for a named sender and
  decodes one interface pointer from `msg->data` (`CustomSkills`).
- `SKSEMenuFramework`-style framework wrappers are now common enough that the
  SDK should provide a clearer recipe layer on top of low-level export loading,
  instead of stopping at generic `symbol()` helpers.

Repair direction:

- Keep documenting the intended split clearly:
  `external_api` loads and organizes interop, but does not erase the need for a
  C++ shim for non-flat C++ APIs. The next examples should show that boundary
  through thin wrapper patterns, not hide it.
- Add one more recipe/helper layer for message-broadcast pointer APIs and for
  framework-shaped flat export sets.

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

### `sdk::advanced::render`

Status: `yellow/red`

Why:

- This is still implementation-light, but the local evidence is now strong and
  repeated rather than speculative.
- `PhotoMode`, `wheeler`, `dMenu`, and `MaxsuDetectionMeter` all rebuild the
  same overlay pipeline:
  - hook renderer or D3D initialization
  - acquire `BSRenderManager` / swap chain / device / context
  - install a `WndProc` hook
  - initialize ImGui Win32 + DX11 backends
  - render one overlay pass per frame/present
  - load fonts/textures after lifecycle messages

What still looks incomplete:

- The SDK currently has almost no real surface here, so plugin authors still
  rebuild the whole pipeline themselves.

Repair direction:

- Promote `advanced::render` from placeholder status and build it from those
  repeated overlay/render patterns instead of leaving it deferred indefinitely.

### `sdk::advanced::scene`

Status: `red`

Why:

- `scene` is still much weaker than `render`. The new plugin pack did not
  uncover a comparably clear, repeated `Ni*` scene-graph abstraction boundary.

Repair direction:

- Keep `scene` behind `render` and `animation` in priority.

## Missing Domain: Animation / Behavior Graph

Status: `red` (missing)

Why:

- The new plugin pack repeatedly uses the same animation-heavy workflows:
  - install `BSTEventSink<BSAnimationGraphEvent>` hooks or sinks
  - read/write animation graph variables
  - call `NotifyAnimationGraph(...)`
  - inspect `BSAnimationGraphManager`, `hkbBehaviorGraph`, and `activeNodes`
  - walk `hkbClipGenerator` bindings and animation annotation tracks
  - parse payloads or JSON from animation annotations/events
  - inject behavior variables/events into `hkbBehaviorGraph` data
- This shows up in `SCAR`, `BehaviorDataInjector`, `CombatPathingRevolution`,
  `DynamicAnimationCasting`, `TK_Dodge_RE`, `MaxsuIFrame`,
  `OneClickPowerAttack`, and `skyrim-firmament-1.5`.

Why it matters:

- This is no longer a one-off "advanced engine hack". It is one of the most
  repeated modern plugin domains, and the SDK currently has no dedicated
  animation/behavior layer at all.

Repair direction:

- Add a new animation-focused SDK domain rather than trying to smuggle these
  helpers into unrelated gameplay or event modules.
- The likely shape is:
  - animation graph event install/helpers
  - graph variable get/set helpers
  - clip/annotation/payload helpers
  - behavior graph traversal helpers
  - optional interop recipes for OAR-style APIs

## Most Likely Mismatches

The most likely places where the SDK is still incomplete or at risk of
misalignment are:

1. A whole animation / behavior graph SDK domain is missing even though modern
   plugins repeatedly depend on it.
2. `sdk::advanced::render` should no longer be treated as speculative; native
   overlay/render patterns now have strong repeated local evidence.
3. `sdk::interop::external_api` has the right foundation, but still needs
   framework-shaped wrapper recipes and a cleaner path for message-broadcast
   pointer APIs like `CustomSkills`.
4. `sdk::forms` and `sdk::plugin::config` still need more direct "plugin
   workflow" bridges from config strings into resolved, validated forms.
5. `sdk::gameplay::{magic,inventory,quests,projectiles}` provide good building
   blocks but still stop earlier than some real plugin-side orchestration
   layers.

## Recommended Repair Order

Recommended next passes, in order:

1. Add a new animation / behavior graph SDK layer.
2. Promote `sdk::advanced::render` into a real overlay/render helper domain.
3. Add framework-shaped interop recipes on top of `sdk::interop::external_api`
   for `CustomSkills`-style message APIs and `SKSEMenuFramework`-style flat
   export sets.
4. Add config-to-form workflow helpers on top of `sdk::forms` and
   `sdk::plugin::config`.
5. Continue auditing `projectiles` and `physics` against
   `NewProjectilesTMP`-style data-driven behavior.
6. Leave `sdk::advanced::scene` for later, after the render and animation
   domains are grounded.

## Bottom Line

The SDK is not drifting blindly. A large portion of the current surface already
matches repeated SKSE plugin patterns well, especially:

- `events`
- `interop::external_api`
- `gameplay::input`
- `hooks`
- `plugin::serialization`

The main remaining gaps are no longer low-level interop or the menu-owned UI
runtime itself. The biggest newly confirmed holes are:

- missing animation / behavior graph helpers
- missing native overlay/render helpers
- missing higher-level interop recipes for framework-shaped plugin APIs

After those, the remaining work shifts back toward config/form workflows and a
few higher-level plugin install recipes.
