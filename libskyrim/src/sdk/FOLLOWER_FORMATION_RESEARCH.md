# Follower Formation Research

This note captures reverse-engineering findings for a Rust/SKSE implementation of
BioWare-style companion movement on Skyrim SE 1.5.97.0. The goal is not "better
package AI", but a layered system where followers:

- start moving with the player immediately
- hold stable slots around the player
- react quickly to sneak/combat/reposition changes
- recover cleanly when navmesh or package state gets them stuck
- escalate to teleport rescue only when needed

## Scope and runtime

- Engine target: SkyrimSE 1.5.97.0
- Main RE reference: `CommonLibVR`
- Binary RE target: `SkyrimSE.exe` loaded in IDA
- Address library mapping source: `E:\Reverse\offsets-1.5.97.0.txt`

## Source set

Primary local sources:

- Papyrus mod source:
  - `S:\SkyrimPapyrus\FormattionWithFollower\source\scripts\FWF_scrCore.psc`
  - `S:\SkyrimPapyrus\FormattionWithFollower\source\scripts\FWF_scrFormation.psc`
  - `S:\SkyrimPapyrus\FormattionWithFollower\source\scripts\FWF_scrFollowerAlias.psc`
  - `S:\SkyrimPapyrus\FormattionWithFollower\source\scripts\FWF_scrPlayerAlias.psc`
  - `S:\SkyrimPapyrus\FormattionWithFollower\source\scripts\FWF_scrMCM.psc`
- Engine-facing C++ reference:
  - `CommonLibVR/include/RE/A/Actor.h`
  - `CommonLibVR/src/RE/A/Actor.cpp`
  - `CommonLibVR/include/RE/A/AIProcess.h`
  - `CommonLibVR/include/RE/P/PlayerCharacter.h`
  - `CommonLibVR/include/RE/P/PositionPlayerEvent.h`
  - `CommonLibVR/include/RE/E/ExtraFollower.h`
  - `CommonLibVR/src/RE/T/TESObjectREFR.cpp`
- SKSE project patterns:
  - `S:\Programming\SKSEProjects\SCAR-main\src\Function.cpp`
  - `S:\Programming\SKSEProjects\SCAR-main\src\Hook_AttackStart.cpp`
  - `S:\Programming\SKSEProjects\Acheron\src\Acheron\Defeat.cpp`
  - `S:\Programming\SKSEProjects\OpenAnimationReplacer-main\src\Utils.cpp`
  - `S:\Programming\SKSEProjects\NextGenDecapitations-master\src\MiscUtils.hpp`

Useful external references:

- KeepOffsetFromActor CK wiki:
  https://ck.uesp.net/wiki/KeepOffsetFromActor_-_Actor
- SetPosition CK wiki:
  https://ck.uesp.net/wiki/SetPosition_-_ObjectReference

## High-level conclusion

The best implementation path is not to replace follower AI with custom path
logic. The engine already exposes a native movement-controller seam for
formation-like behavior through `KeepOffsetFromActor`. The ideal plugin should:

1. use the native keep-offset controller as the primary movement layer
2. drive slot updates from engine events, not coarse polling
3. detect stuck/path failure states with lightweight heuristics
4. fall back to package reevaluation or teleport only when recovery requires it

This keeps the plugin responsive without fighting the engine on every frame.

## What the Papyrus mod gets right

`Formation with Followers` is a strong proof of concept because it does not try
to rewrite follower AI. Its core loop relies on:

- `Actor.KeepOffsetFromActor(...)`
- `Actor.ClearKeepOffsetFromActor()`
- state-specific offsets for normal / sneak / combat modes
- forced reevaluation on player state changes
- fallback release when the follower is too far or the slot is invalid

The central idea is correct: treat formation as an overlay on top of engine
movement, not a replacement for vanilla packages.

Where the Papyrus version is limited:

- updates are driven by script events and timers, not by internal engine timing
- rescue logic is necessarily conservative
- interior / navmesh edge cases are hard to handle perfectly from Papyrus
- persistence and control UX are good, but the movement core stays outside the
  engine's lowest-level update path

The SKSE/Rust version can keep the same conceptual model while moving the core
control loop down into engine-native seams.

## Engine-native keep-offset path

### Papyrus registration

IDA identifies the Actor Papyrus registration function as:

- `papyrus__Actor::RegisterFuncs_14094DF30`

The relevant native entrypoints registered there are:

- `papyrus__Actor::KeepOffsetFromActor_14094B5B0`
- `papyrus__Actor::ClearKeepOffsetFromActor_140958290`

### KeepOffsetFromActor call chain

The native call path is:

1. Papyrus wrapper converts angle degrees to radians
2. Target actor is converted to a `RefHandle`
3. Call forwards into an internal Actor helper
4. Actor helper talks to `MovementControllerNPC`
5. Movement controller activates a named keep-offset interface

Internal function recovered in IDA:

- `Actor::SetKeepOffsetFromActor_140603990`
- Address Library ID: `36870`
- 1.5.97 offset: `0x603990`

Behavior observed in decompilation:

- acquires `target->movementController`
- toggles a controller mode bit through `MovementControllerNPC::sub_1406F0050`
- queries a named movement interface keyed by the fixed string
  `"Keep Offset From Actor"`
- builds and submits a parameter block containing:
  - target handle
  - offset xyz
  - angle xyz in radians
  - catch-up radius
  - follow radius

This is the most important RE result in the whole investigation. The mechanic is
not a high-level script helper. It is a real movement-controller feature.

### ClearKeepOffsetFromActor call chain

Recovered internal clear helper:

- `Actor::ClearKeepOffsetFromActor_140603AC0`
- Address Library ID: `36871`
- 1.5.97 offset: `0x603AC0`

Observed behavior:

- acquires `movementController`
- calls `MovementControllerNPC::sub_1406F0050(..., 0)`
- releases controller references

### Movement-controller seam

Recovered internal toggle helper:

- `MovementControllerNPC::sub_1406F0050`
- Address Library ID: `40723`
- 1.5.97 offset: `0x6F0050`

Recovered related types / strings:

- `IMovementSetKeepOffsetFromActor`
- `MovementPlannerAgentKeepOffset`
- `"Keep Offset From Actor"`
- `"Planner Keep Offset"`

This strongly suggests the engine has a planner/agent architecture for
keep-offset movement and that the Papyrus native is only one thin entrypoint into
that system.

## PositionPlayerEvent is the right reevaluation window

`PlayerCharacter` embeds `BSTEventSource<PositionPlayerEvent>` and exposes it via
`AsPositionPlayerEventSource()` in CommonLib.

`PositionPlayerEvent` enum values:

- `kPreUpdatePackages = 1`
- `kPostUpdatePackages = 2`

IDA findings:

- `PathManagerPositionPlayerAdapter::ProcessPositionPlayerEvent_1405BD2C0`

Observed behavior:

- on `kPreUpdatePackages`: set an internal "position/player update in progress"
  flag
- on `kPostUpdatePackages`: run a post-update refresh helper and then clear the
  flag

This matters because it shows that engine systems themselves synchronize path
logic around this event. For a follower-formation plugin, `kPostUpdatePackages`
is the best first-choice hook for immediate slot reevaluation after the engine
has already repositioned or repackaged player movement state.

Practical implication:

- prefer event-driven reevaluation from `PositionPlayerEvent`
- use `kPostUpdatePackages` as the main "rebuild formation now" moment
- keep timer-based fallback logic only as redundancy

## Other engine primitives worth using

### Actor-side state and path heuristics

Useful CommonLib-visible Actor helpers:

- `Actor::EvaluatePackage(bool immediate = false, bool resetAI = false)`
  - Address Library ID: `36407`
- `Actor::IsPathing() const`
  - Address Library ID: `36812`
- `Actor::RequestLOS(Actor*, float)`
  - Address Library ID: `36752`
- `Actor::CanNavigateToPosition(...) const`
  - Address Library ID: `46050`

These appear in real SKSE project code, especially SCAR, as cheap signals for
"can the actor actually move through the intended pathing state right now?"

### Follower identity signals

Useful engine fields:

- `AIProcess::followTarget`
- `AIProcess::escortingPlayer`
- `AIProcess::lowProcessFlags.kFollower`
- `ExtraFollower::actorFollowers`
- `ExtraFollower::FollowerInfo::intendedFollowDistance`

Recommended detection strategy is composite, not single-signal:

- vanilla teammate check
- `followTarget == player`
- AI-process follower flags
- player extra-data follower list
- optional compatibility fallbacks for custom follower frameworks

### Teleport / reposition primitives

Useful `TESObjectREFR` helpers:

- `MoveToNearestNavmesh(...)`
- `SetPosition(...)`
- `MoveTo_Impl(...)`

Important internal seam:

- `TESObjectREFR::MoveTo_Impl`
- Address Library ID: `56227`
- 1.5.97 offset: `0x9AE5C0`

IDA notes show the 1.5.97 function at that target is a shared reposition helper
used by move/set-position/angle functors and includes player-specific relocation
side effects. Existing SKSE projects also call `MoveTo_Impl` directly.

Teleport guidance:

- prefer a navmesh-aware rescue before hard teleport
- prefer engine move helpers over raw position writes when changing cells or
  performing larger corrections
- `SetPosition` is acceptable for short local nudges, but less ideal as a
  general rescue primitive

## Evidence of built-in follower failure logic

The binary contains engine settings strings including:

- `fFollowerSpacingAtDoors`
- `fFollowerTeleportOffsetFudge:Pathfinding`
- `bWarpOnConsecutiveFailures:Pathfinding`

Even without fully unwinding every call site yet, this is strong evidence that
the engine already contains follower-specific spacing and warp recovery policy.
This supports a design that cooperates with engine systems instead of trying to
replace them entirely.

## Recommended plugin architecture

### Layer 1: native formation controller

For each active follower, maintain a desired slot relative to the player:

- lateral slot
- longitudinal depth
- height bias if needed
- orientation mode
- per-state variants for normal / sneak / combat / sprint

Drive the slot with:

- `Actor::SetKeepOffsetFromActor_140603990`
- `Actor::ClearKeepOffsetFromActor_140603AC0`

This should be the normal movement mode.

### Layer 2: immediate event-driven reevaluation

Subscribe to:

- `PlayerCharacter::AsPositionPlayerEventSource()`
- menu / scene / wait / sleep state changes as needed
- combat and sneak transitions
- load-door or cell/world transitions

Minimum recommended trigger points:

- `PositionPlayerEvent::kPostUpdatePackages`
- player sneak toggle
- player combat enter/exit
- follower combat enter/exit
- door/cell transition completion

### Layer 3: slot solver

Use a deterministic slot allocator:

- front-left / front-right / rear-left / rear-right
- caster/archer/melee role bias if available
- collision-aware local perturbation if two followers fight for one slot
- temporary slot widening while sprinting or near doors

Keep the solver separate from transport/recovery logic. The slot solver should
answer "where should this follower be?" not "how do we force them there?"

### Layer 4: stuck detection

Per follower, track:

- current distance to slot
- velocity toward slot
- time since last meaningful progress
- `IsPathing()`
- `CanNavigateToPosition(...)`
- optional `RequestLOS(player)` or `RequestLOS(slot proxy)`
- whether current package state obviously conflicts with formation

Escalation tiers:

1. healthy: continue native keep-offset
2. mild drift: resend keep-offset with fresh slot
3. likely AI conflict: clear keep-offset and `EvaluatePackage()`
4. path failure: navmesh rescue
5. hard failure: teleport rescue and reapply formation

### Layer 5: rescue transport

Recommended rescue order:

1. `MoveToNearestNavmesh(...)` when the actor is close but locally stuck
2. short local `SetPosition(...)` or equivalent only for same-cell nudges
3. `MoveTo_Impl(...)` for full relocation / reliable rescue
4. reapply keep-offset after the actor is settled

Do not teleport aggressively on every lag spike. Teleport should be policy-based
and stateful, not just distance-based.

## Why not custom path-follow from scratch

The binary exposes much heavier path-follow machinery:

- `MovementAgentPathFollowerStandard`
- `MovementTweenerAgentNodeFollower`
- `PathFollowerStateFollowPathToParameter`
- `PathFollowerStateTurnToAngle`
- `PathFollowerStateKeepLastDirection`

Those systems are real, but they are far more complex and tightly coupled to the
engine's planner stack. They are useful RE evidence, but they are not the first
implementation target for a practical SKSE plugin. The keep-offset seam already
solves the main problem at a much lower integration cost.

## Practical Rust implementation plan

Suggested implementation order:

1. Add thin FFI wrappers for the engine calls we already trust:
   - keep-offset set/clear
   - `EvaluatePackage`
   - `IsPathing`
   - `CanNavigateToPosition`
   - `RequestLOS`
   - `MoveTo_Impl` or a safer high-level relocation wrapper
2. Add a plugin-local follower tracker:
   - actor handles
   - slot assignment
   - state machine
   - stuck counters
3. Add a `BSTEventSink<PositionPlayerEvent>` bridge
4. Implement normal slot maintenance
5. Add rescue escalation logic
6. Add persistence and configuration after movement is proven stable

This order is important. Reliable movement should come before MCM, serialization,
or formation presets.

## Suggested first Rust-facing API surface

Plugin-local first, SDK later:

- `formation::FollowerSet`
- `formation::SlotLayout`
- `formation::FormationController`
- `formation::RecoveryPolicy`
- `formation::PositionPlayerSink`

Start with plugin-local wrappers. Only promote them into `libskyrim::sdk` after
the behavioral model is stable and the FFI surface is battle-tested.

## Open RE questions

Useful next passes:

1. Fully unwind the engine call sites for:
   - `fFollowerSpacingAtDoors`
   - `fFollowerTeleportOffsetFudge:Pathfinding`
   - `bWarpOnConsecutiveFailures:Pathfinding`
2. Identify whether an internal equivalent of `Character::WarpFollowers` can be
   recovered cleanly on 1.5.97
3. Map the minimum-safe relocation path for cross-cell rescue without invoking
   extra Papyrus functor machinery unnecessarily
4. Verify whether follower wait/sleep adapters should suspend formation rather
   than just widening slots

## Final recommendation

The "ideal" solution for Skyrim is a hybrid:

- native keep-offset for continuous formation
- engine event timing for responsiveness
- lightweight heuristics for failure detection
- teleport as a controlled recovery tool, not the primary transport

That design is close to what the Papyrus mod was trying to achieve, but it moves
the core movement decisions into the same layer where the engine already manages
player repositioning and path-state updates.
