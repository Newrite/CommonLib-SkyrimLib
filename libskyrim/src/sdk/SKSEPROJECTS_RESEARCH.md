# SKSE Projects Research

Last updated: 2026-03-30

This file captures a focused research pass over `S:\Programming\SKSEProjects`
to keep high-signal C++ plugin patterns available when shaping
`libskyrim::sdk`.

It complements:

- [`README.md`](./README.md) for the current public SDK surface
- [`MEMORY.md`](./MEMORY.md) for the shorter persistent architecture memory

## Scope

This pass focused on projects that repeatedly showed up for one of these
reasons:

- heavy UI / Scaleform work
- plugin-to-plugin interop
- Havok / raycast / collision work
- gameplay utility breadth
- death / recovery flow handling
- actor caches, persistence, or plugin data repositories

Sampled projects and files included:

- `S:\Programming\SKSEProjects\TrueDirectionalMovement-master\src\API\SmoothCamAPI.h`
- `S:\Programming\SKSEProjects\TrueHUD-master\src\TrueHUDAPI.h`
- `S:\Programming\SKSEProjects\TrueHUD-master\src\Scaleform\TrueHUDMenu.cpp`
- `S:\Programming\SKSEProjects\QuickLootIE\src\LootMenu.cpp`
- `S:\Programming\SKSEProjects\QuickLootIE\src\UniversalMenu.cpp`
- `S:\Programming\SKSEProjects\BetterTelekinesis\src\RaycastHelper.cpp`
- `S:\Programming\SKSEProjects\BlinkSpell\include\RayCast.h`
- `S:\Programming\SKSEProjects\OpenAnimationReplacer-RaySense\src\RaySenseLogic.cpp`
- `S:\Programming\SKSEProjects\Precision-main\src\PrecisionAPI.h`
- `S:\Programming\SKSEProjects\Acheron\src\Acheron\Interface\Interface.h`
- `S:\Programming\SKSEProjects\Acheron\src\Acheron\Resolution.cpp`
- `S:\Programming\SKSEProjects\TrueFlasksNG\src\API\TrueFlasksAPI.h`
- `S:\Programming\SKSEProjects\TrueFlasksNG\src\Core\ActorsCache.cpp`
- `S:\Programming\SKSEProjects\SkyrimSE-SmoothCam\SmoothCam\source\hooks.cpp`
- `S:\Programming\SKSEProjects\Reflyem\include\UI.hpp`
- `S:\Programming\SKSEProjects\death-alternative-mod-main\src\main.cpp`
- `S:\Programming\SKSEProjects\injury-alt-death\src\main.cpp`
- `S:\Programming\SKSEProjects\CLibUtil-master\include\CLIBUtil\editorID.hpp`
- `S:\Programming\SKSEProjects\CLibUtil-master\include\CLIBUtil\distribution.hpp`
- `S:\Programming\SKSEProjects\CLibUtil-master\include\CLIBUtil\hotkeys.hpp`
- `S:\Programming\SKSEProjects\CLibUtil-master\include\CLIBUtil\simpleINI.hpp`
- `S:\Programming\SKSEProjects\StyyxUtils-main\include\st-forms.h`
- `S:\Programming\SKSEProjects\StyyxUtils-main\include\st-menu.h`
- `S:\Programming\SKSEProjects\StyyxUtils-main\include\st-actor.h`
- `S:\Programming\SKSEProjects\StyyxUtils-main\include\st-hooks.h`
- `S:\Programming\SKSEProjects\UselessFenixUtils-master\include\UselessFenixUtils.h`
- `S:\Programming\SKSEProjects\UselessFenixUtils-master\include\UselessImguiUtils.h`
- `S:\Programming\SKSEProjects\UselessFenixUtils-master\include\UselessDebugRenderUtils.h`
- `S:\Programming\SKSEProjects\NewProjectilesTMP-master\src\Positioning.h`
- `S:\Programming\SKSEProjects\NewProjectilesTMP-master\src\Positioning.cpp`
- `S:\Programming\SKSEProjects\NewProjectilesTMP-master\src\Homing.cpp`
- `S:\Programming\SKSEProjects\NewProjectilesTMP-master\src\Followers.cpp`
- `S:\Programming\SKSEProjects\NewProjectilesTMP-master\src\Emitters.cpp`
- `S:\Programming\SKSEProjects\NewProjectilesTMP-master\src\Multicast.cpp`
- `S:\Programming\SKSEProjects\NewProjectilesTMP-master\src\Hooks.h`

## Utility Libraries And Projectile Framework Pass

This follow-up pass focused on four kinds of repositories that are especially
useful when deciding what belongs in `sdk` versus what should stay plugin-local:

- broad gameplay utility libraries
- forms/config/hotkey helper libraries
- debug/imgui/render helper libraries
- data-driven projectile frameworks

The strongest signal came from:

- `CLibUtil-master`
- `StyyxUtils-main`
- `UselessFenixUtils-master`
- `NewProjectilesTMP-master`

## Additional Repeated Patterns

### 11. Form lookup and editor-ID ergonomics are still repeatedly rebuilt by utility libraries

Observed in:

- `CLibUtil-master`
- `StyyxUtils-main`

Repeated patterns:

- parse `"Plugin.esp|0x123"` strings into forms
- editor-ID lookup with optional fallback to external providers like
  `po3_Tweaks.dll`
- small typed wrappers like `GetSpellFromString(...)` and
  `GetWeaponFromString(...)`
- plugin-loaded checks before using optional cross-mod lookups

What this means for `sdk`:

- `sdk::forms::lookup` should move higher in priority
- the important value is not just `LookupForm`, but the ergonomic layer around:
  - plugin-name + raw-form-id lookup
  - form-string parsing
  - editor-ID lookup
  - optional dependency-aware fallback paths

Recommended additions:

- `form_from_string("Mod.esp|0x123")`
- `lookup_form(plugin_name, raw_form_id)`
- `lookup_by_editor_id<T>(...)`
- `plugin_loaded(plugin_name)`
- maybe optional interop-backed editor-ID resolution when native coverage is
  insufficient

### 12. Distribution, key maps, and typed config parsing are recurring small frameworks

Observed in:

- `CLibUtil-master`
- `NewProjectilesTMP-master`

Repeated patterns:

- parse small data-driven DSLs from INI/JSON/TOML-like sources
- keep typed string-to-key registries for reusable symbolic identifiers
- maintain per-file namespacing so multiple configs can coexist
- turn human strings into stable indices or `FormID`s early

What this means for `sdk`:

- `sdk::plugin::config` should stay intentionally small, but it can still solve
  the repeated low-level pain
- `sdk` does not need to ship a giant distribution framework, but typed parsing
  helpers and config/form-cache glue look widely reusable

Recommended additions:

- typed INI accessors with defaults
- hotkey-combination parsing helpers
- form-string and editor-ID parsing helpers reusable from config code
- tiny namespaced key registries for data-driven systems
- lightweight helpers for config + cached form resolution

### 13. Data-driven projectile mods decompose into a reusable set of building blocks

Observed in:

- `NewProjectilesTMP-master`

Repeated patterns:

- per-projectile runtime state stored in projectile-side fields/paddings
- JSON-backed key registries and config-time symbolic indirection
- homing target acquisition using hostility + LOS + cursor-angle logic
- positional pattern generation for line/circle/fill/sphere/cylinder layouts
- follow/emit/multicast features implemented as composable behaviors
- hook points that mutate projectile behavior over time instead of only on spawn

What this means for `sdk`:

- there is likely room for a future `sdk::gameplay::projectiles` module
- the SDK should not copy the whole `NewProjectilesTMP` framework shape
- the reusable part is the building-block layer:
  - target acquisition
  - spatial pattern generation
  - projectile runtime helpers
  - composable projectile behavior helpers

Recommended additions:

- `collect_projectile_targets(...)` with hostility / LOS / view-cone filters
- pattern generators for `line`, `circle`, `half_circle`, `fill_circle`,
  `sphere`, `cylinder`
- helpers for `Projectile` direction, speed, desired-target, and shooter data
- maybe small runtime-state helpers once the correct persistence/save boundary
  is clearer

Important note:

- projectile-constructor / save-load padding hooks are real and useful, but
  they are still closer to plugin code than to first-pass `sdk`

### 14. Utility libraries repeatedly package tiny gameplay sugar, not giant architectures

Observed in:

- `StyyxUtils-main`
- `UselessFenixUtils-master`

Repeated patterns:

- active-effect presence checks by archetype, keyword, or exact `MGEF`
- actor max AV helpers and full-heal helpers
- equipped-item category checks like heavy/light armor or shield presence
- lightweight spell application helpers
- small menu and UI queue wrappers

What this means for `sdk`:

- `sdk::gameplay::{actors, combat, player}` should keep absorbing narrow,
  source-backed helpers where the workflow repeats across projects
- avoid adding a giant catch-all `utils` namespace; the right shape is still
  domain modules with small helpers

Recommended additions:

- active-effect query helpers in `sdk::gameplay::actors` or `magic`
- more equipment categorization helpers
- a small `sdk::gameplay::magic` module if the repeated surface grows enough

### 15. ImGui overlays, debug rendering, and behavior-graph traversal are useful but not yet core-SDK material

Observed in:

- `UselessFenixUtils-master`
- parts of `StyyxUtils-main`

Repeated patterns:

- hook input dispatch and DX11 present for imgui overlays
- translate keyboard/input events into overlay input state
- debug draw lines, spheres, capsules, and moving collision shapes
- traverse `hkbBehaviorGraph` trees and resolve nodes/events/variables
- generic hook helpers over trampolines and prologue patches

What this means for `sdk`:

- these patterns are important to remember, but most of them should stay out of
  the core SDK until a stronger shared shape appears
- the best near-term action is to keep targeted notes and only extract narrow,
  well-bounded helpers when a second or third real plugin confirms the same need

Recommended additions:

- maybe future `sdk::animation::behavior` targeted helpers if more repos confirm
  the same traversal workflows
- maybe future optional debug utilities, but not a generic imgui framework in
  the core SDK right now

### 16. Reference traversal tends to split into local scene scans vs rare full-world scans

Observed in:

- `Reflyem`
- `Acheron`
- `NewProjectilesTMP`

Repeated patterns:

- `TES::ForEachReferenceInRange(...)` is used as a local scene scan around a
  center/origin during gameplay logic
- full `TES::ForEachReference(...)` still appears, but usually for one-shot or
  specialized workflows rather than frequent background polling
- projects typically keep the callback short, filter aggressively, and only do
  heavier logic after a local candidate set has been found

What this means for `sdk`:

- `sdk::gameplay::world` should expose closure-first wrappers for reference
  traversal, with `in_range` as the primary ergonomic path
- nullable origin/radius seams should fail soft in SDK space instead of falling
  back to full-world traversal implicitly
- snapshot helpers over `ObjectRefHandle` are useful because they shorten the
  borrow on live engine storage and let plugins perform heavier follow-up work
  after the traversal

Recommended additions:

- `for_each_reference(...)`
- `for_each_reference_in_range(...)`
- pointer-friendly soft-fail variants for nullable origins
- snapshot / collect helpers built on `ObjectRefHandle` or `Resolved<TESObjectREFR>`

## Repeated Patterns

### 1. Exported plugin APIs are still the dominant cross-plugin contract

Observed in:

- `TrueHUD`
- `TrueDirectionalMovement`
- `TrueFlasksNG`

Common shape:

- `extern "C"` `RequestPluginAPI`
- versioned interface enums like `InterfaceVersion::V1..V4`
- `APIResult` enums modeling ownership, capability, or error state
- public header meant to be copied into another plugin project

What this means for `sdk`:

- `sdk::interop::external_api` is core, not optional sugar
- we should keep supporting both:
  - plain `RequestPluginAPI` DLL export lookup
  - versioned interface selection
- ergonomic Rust wrappers are useful, but the real product is a stable ABI

Recommended additions:

- helper for exporting C ABI function-table style APIs from Rust plugins
- `ApiResult`-oriented helpers for ownership-style APIs
- documentation templates for Rust-to-C++ API publication

### 2. Messaging-based API acquisition is common when the provider wants async or lifecycle safety

Observed most clearly in:

- `SmoothCamAPI.h`

Common shape:

- request packet dispatched through `SKSE::MessagingInterface`
- loader callback registered during `PostLoad`
- request sent during `PostPostLoad`
- provider responds with interface pointer + version

What this means for `sdk`:

- `sdk::interop::messaging` should not stop at raw request/response helpers
- we need higher-level helpers specifically for:
  - interface request packets
  - interface loader callbacks
  - versioned provider responses

Recommended additions:

- `interop::messaging::interface_loader`
- typed request/response envelopes for pointer-bearing API handshakes
- an `ApiRepository<T>` pattern for caching foreign APIs after SKSE messages

### 3. Scaleform work repeatedly becomes a hand-rolled object DSL

Observed in:

- `QuickLootIE`
- `TrueHUD`
- `Acheron`

Repeated patterns:

- `GetVariable` by path strings
- `Invoke` by method-name strings
- `SetVariable` / `SetMember`
- `CreateObject`, `CreateArray`, `CreateFunction`
- bind functions into `_global` or a specific object scope
- cache important object handles after menu load

What this means for `sdk`:

- current `sdk::ui::scaleform` movie-level helpers are useful but too low
- the next high-value step is an object/member layer, not more raw movie lookup

Recommended additions:

- `ScaleformObject` wrapper over `GFxValue`
- `movie.object(\"_root.foo\")`
- `object.member(\"bar\")`
- `object.invoke(\"method\", args)`
- `movie.create_object()`, `movie.create_array()`
- `bind_global_function(...)`
- `bind_scope_function(scope, name, handler)`

This is one of the clearest repeated pain points in the sampled projects.

### 4. Custom menu lifecycle has a recurring shape

Observed in:

- `QuickLootIE`
- `TrueHUD`
- `Acheron`

Repeated patterns:

- register menu with `UI::Register`
- open / close through `UIMessageQueue::AddMessage`
- load SWF through `BSScaleformManager::LoadMovie`
- run initialization only if the menu view exists
- cache SWF views or menu-local handles when appropriate
- override `ProcessMessage`

Special cases:

- `QuickLootIE` has a reusable `UniversalMenu` layer for VR / flat runtime
  divergence
- `Acheron` injects custom SKSE-like functions into Scaleform scope

What this means for `sdk`:

- `sdk::ui::menus` should keep growing around typed message payloads
- there is room for a future `sdk::ui::custom_menu` or menu scaffold layer
- VR-aware menu setup is still too specialized to expose broadly today, but the
  pattern is worth keeping in mind

Recommended additions:

- more typed `IUIMessageData` helpers
- `custom_menu` scaffolding once more `IMenu` / `BSScaleformManager` surface is
  translated
- menu registration helpers that pair menu name, creator, and SWF loading
- Scaleform function-binding helpers

### 5. UI state and camera systems often depend on menu events and direct GFx interception

Observed in:

- `SkyrimSE-SmoothCam`

Repeated patterns:

- listen to `MenuOpenCloseEvent`
- map menu names to mod-local semantic states
- hook specific `GFxInvoke` or HUD methods when event-based observation is not
  enough
- coordinate camera/input behavior around menu transitions

What this means for `sdk`:

- `sdk::ui::menus` and `sdk::gameplay::input` should keep leaning into
  transition-oriented helpers
- we should be careful not to promise a full generic “camera ownership”
  framework inside `sdk`

Recommended additions:

- more transition helpers: fade + menu visibility + input state composition
- maybe lightweight menu-name enum mapping helpers
- keep low-level detour logic in plugin code unless the pattern becomes truly
  universal

### 6. Havok raycast work always wants more than a single `pick_object`

Observed in:

- `BetterTelekinesis`
- `BlinkSpell`
- `OpenAnimationReplacer-RaySense`
- `Precision`

Repeated patterns:

- collision masks and layer filtering
- ignore sets for the player or owned nodes
- hit collectors returning many hits, not just closest
- hit-to-`NiAVObject` or hit-to-`TESObjectREFR` resolution
- split raycasts and backoff from hit fraction
- careful world-scale conversion
- in one project, explicit world locking before `CastRay`

What this means for `sdk`:

- `sdk::advanced::physics` is one of the highest-value next modules
- good helpers here can immediately simplify several plugin styles

Recommended additions:

- `CollisionMask` builder / convenience constants
- `raycast_closest(...)`
- `raycast_all(...)`
- `split_raycast(...)`
- `resolve_hit_av_object(...)`
- `resolve_hit_reference(...)`
- `backoff_hit_position(...)`
- `raycast_ignore_nodes(...)`
- explicit helper documenting world-scale conversion and thread-safety

Important note:

- full contact-listener / collision-object creation patterns from `Precision`
  and `DynamicCollisionAdjustment` are valuable, but they are still lower-level
  than the first `sdk::advanced::physics` pass should expose

### 7. Actor utilities repeatedly combine process-list scans, hostility checks, and current combat state

Observed in:

- `Acheron`
- `injury-alt-death`
- `death-alternative`
- `Reflyem`

Repeated patterns:

- iterate `ProcessLists`
- filter dead / loaded / teammate / hostile actors
- inspect `currentCombatTarget`
- stop combat on selected actors
- handle follower-rescue or nearby-enemy cleanup

What this means for `sdk`:

- `sdk::gameplay::actors` and `sdk::gameplay::combat` are on the right path
- the strongest next additions are incremental, not architectural

Recommended additions:

- `current_combat_target(actor)`
- `is_targeting_actor(actor, target)`
- better nearby hostile scans using the new hostility helpers
- maybe `for_each_loaded_actor_matching(...)` if it improves ergonomics over the
  current collection helpers

### 8. Persistence patterns are often just small typed stores with versioned save records

Observed in:

- `TrueFlasksNG`
- `Acheron`

Repeated patterns:

- per-actor or per-system caches keyed by `FormID`
- explicit version labels
- garbage collection before save
- `ResolveFormID` on load

What this means for `sdk`:

- `sdk::plugin::serialization` can likely offer reusable small-record helpers
- the repeated need is not “serialize everything”, but
  “make versioned save records ergonomic”

Recommended additions:

- tiny helpers for labeled record writing/reading
- `FormID` map serialization helpers
- maybe a reusable `ResolvedFormIdMap<T>` or similar once the right shape is
  clearer

### 9. Form lookup ergonomics still matter a lot

Observed in:

- `Acheron`
- `injury-alt-death`
- `death-alternative`

Repeated patterns:

- `TESDataHandler::LookupForm`
- form-from-string parsing
- plugin-name + form-id lookup
- “optional dependency plugin loaded?” checks

What this means for `sdk`:

- `sdk::forms::lookup` is still a high-value target

Recommended additions:

- `lookup_form(plugin_name, form_id)`
- `lookup_form_id(plugin_name, form_id)`
- `plugin_loaded(plugin_name)`
- `form_from_string(\"Mod.esp|0x123\")`
- maybe editor ID lookup once coverage is ready

### 10. Config layers are usually small, typed, and project-local

Observed in:

- `TrueFlasksNG`
- `death-alternative`
- `injury-alt-death`
- `Reflyem`

Repeated patterns:

- typed INI/TOML settings
- refresh-on-save / hot reload style behavior
- form caches loaded from config

What this means for `sdk`:

- a good config SDK should probably stay intentionally small
- the win is not replacing every config library, but smoothing the repetitive
  parts around Skyrim plugin lifecycle

Recommended additions:

- small lifecycle glue for config load / reload
- integration helpers for config + form caches
- do not overbuild a huge config framework without a clear repeated shape

## What Looks Worth Building Next

Priority order after this pass:

1. Expand `sdk::forms::lookup` with form-string parsing, plugin-aware lookup,
   editor-ID helpers, and optional dependency-aware fallbacks.
2. Add small `sdk::plugin::config` helpers for typed INI access, hotkey parsing,
   and config-to-form resolution glue.
3. Start a narrow `sdk::gameplay::projectiles` pass focused on target
   acquisition, spatial pattern generation, and projectile runtime helpers.
4. Deepen `sdk::ui::scaleform` with object/member wrappers and function
   binding.
5. Expand `sdk::advanced::physics` with ignore filters, split rays, clearance,
   and relocation-candidate helpers.

## Things That Should Probably Stay Out Of The SDK For Now

- full mod-specific widget frameworks
- large ownership-heavy camera frameworks
- full collision-object creation / active-ragdoll APIs
- giant gameplay frameworks that bake in one mod’s domain language
- invasive hook packs whose only commonality is “they exist in many mods”

## Revisit Targets

Projects worth returning to when implementing the next passes:

- `TrueHUD-master`
  for object-level Scaleform and widget lifecycle
- `QuickLootIE`
  for custom menu scaffolding and GFx object ergonomics
- `BetterTelekinesis`
  for split-raycast and hit filtering
- `OpenAnimationReplacer-RaySense`
  for world-lock and hit-to-reference resolution
- `Precision-main`
  for future deeper Havok work
- `TrueDirectionalMovement-master`
  for API repository and external API patterns
- `SkyrimSE-SmoothCam`
  for menu/camera/input transition observations
- `TrueFlasksNG`
  for compact API + actor cache + config integration patterns
- `Acheron`
  for death-flow actor scans, menu use, and consequence selection
- `CLibUtil-master`
  for form-string parsing, editor-ID fallback, distribution, and hotkeys
- `StyyxUtils-main`
  for compact gameplay/menu/forms helper patterns
- `UselessFenixUtils-master`
  for geometry, behavior-graph traversal, and optional debug/imgui patterns
- `NewProjectilesTMP-master`
  for data-driven projectile runtime and target-selection patterns
