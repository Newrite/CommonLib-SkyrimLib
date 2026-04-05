//! Structured plugin lifecycle helpers.
//!
//! This module gives plugin code a domain-oriented vocabulary over the raw
//! SKSE messaging kinds used for plugin bootstrap and save/load transitions.
//!
//! Reach for this module when the plugin only cares about lifecycle *meaning*
//! rather than raw messaging transport details.
//!
//! Decision guide:
//!
//! - use [`on_post_load`] / [`on_data_loaded`] for the most common bootstrap
//!   boundaries
//! - use [`on_pre_load_game`], [`on_post_load_game`], [`on_save_game`], and
//!   friends for save/load lifecycle
//! - use [`on_plugin_phase`], [`on_game_lifecycle`], or [`on_lifecycle`] when
//!   a helper should stay parameterized over lifecycle enums instead of one
//!   hard-coded phase
//! - fall back to [`crate::sdk::plugin::messaging`] when sender filtering, raw
//!   payload access, or non-lifecycle message kinds are the real concern
//!
//! Typical bootstrap pattern:
//!
//! 1. use [`on_post_load`] for early one-time bootstrap that should happen
//!    after plugin discovery
//! 2. use [`on_data_loaded`] for work that depends on forms/data being ready
//! 3. use [`crate::sdk::plugin::task`] when the callback should only schedule
//!    later gameplay/UI work instead of doing it inline

use crate::sdk::core::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};
use crate::sdk::events::skse::messages::{MessageListener, MessageRef};

/// Register a callback for one plugin-lifecycle phase.
///
/// Prefer the more specific helpers like [`on_post_load`] or
/// [`on_data_loaded`] when the phase is fixed at the call site.
#[inline(always)]
pub fn on_plugin_phase<F>(phase: PluginLifecyclePhase, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    crate::sdk::events::skse::messages::on_plugin_phase(phase, callback)
}

/// Register a callback for one save/load lifecycle phase.
///
/// This is the parameterized entrypoint for helpers that stay generic across
/// save/load boundaries.
#[inline(always)]
pub fn on_game_lifecycle<F>(phase: GameLifecyclePhase, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    crate::sdk::events::skse::messages::on_game_lifecycle(phase, callback)
}

/// Register a callback for either plugin or save/load lifecycle phases through
/// one shared enum surface.
///
/// Use this when higher-level plugin code wants to stay generic over
/// bootstrap-time and save/load-time lifecycle phases.
#[inline(always)]
pub fn on_lifecycle<F>(phase: LifecyclePhase, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    crate::sdk::events::skse::messages::on_lifecycle(phase, callback)
}

/// Register a callback for the `PostLoad` plugin phase.
///
/// This is a common place to wire inter-plugin listeners or lightweight early
/// bootstrap that does not yet need loaded game data.
#[inline(always)]
pub fn on_post_load<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_plugin_phase(PluginLifecyclePhase::PostLoad, callback)
}

/// Register a callback for the `PostPostLoad` plugin phase.
///
/// This is useful for “after everyone else has seen PostLoad” style bootstrap
/// glue, especially when inter-plugin contracts expect a slightly later phase.
#[inline(always)]
pub fn on_post_post_load<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_plugin_phase(PluginLifecyclePhase::PostPostLoad, callback)
}

/// Register a callback for the `InputLoaded` plugin phase.
///
/// Use this when installation depends on input mappings or device-layer
/// bootstrap having completed.
#[inline(always)]
pub fn on_input_loaded<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_plugin_phase(PluginLifecyclePhase::InputLoaded, callback)
}

/// Register a callback for the `DataLoaded` plugin phase.
///
/// This is the usual boundary for form-driven installation, config-to-form
/// resolution, and deferred gameplay/UI startup.
#[inline(always)]
pub fn on_data_loaded<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_plugin_phase(PluginLifecyclePhase::DataLoaded, callback)
}

/// Register a callback for `PreLoadGame`.
///
/// This is the usual place to clear transient state before one save begins
/// loading.
#[inline(always)]
pub fn on_pre_load_game<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle(GameLifecyclePhase::PreLoadGame, callback)
}

/// Register a callback for `PostLoadGame`.
///
/// Use this when runtime state should be rebuilt from the newly loaded save.
#[inline(always)]
pub fn on_post_load_game<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle(GameLifecyclePhase::PostLoadGame, callback)
}

/// Register a callback for `SaveGame`.
///
/// This is the normal boundary for state capture or serialization-adjacent
/// cleanup that should happen alongside game saving.
#[inline(always)]
pub fn on_save_game<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle(GameLifecyclePhase::SaveGame, callback)
}

/// Register a callback for `DeleteGame`.
///
/// Use this when plugin-owned per-save state should be discarded permanently.
#[inline(always)]
pub fn on_delete_game<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle(GameLifecyclePhase::DeleteGame, callback)
}

/// Register a callback for `NewGame`.
///
/// This is the clean boundary for initializing per-save state that should only
/// exist for newly started games.
#[inline(always)]
pub fn on_new_game<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle(GameLifecyclePhase::NewGame, callback)
}
