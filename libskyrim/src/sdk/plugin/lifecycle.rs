//! Structured plugin lifecycle helpers.
//!
//! This module gives plugin code a domain-oriented vocabulary over the raw
//! SKSE messaging kinds used for plugin bootstrap and save/load transitions.

use crate::sdk::core::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};
use crate::sdk::events::skse::messages::{MessageListener, MessageRef};

#[inline(always)]
pub fn on_plugin_phase<F>(phase: PluginLifecyclePhase, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    crate::sdk::events::skse::messages::on_plugin_phase(phase, callback)
}

#[inline(always)]
pub fn on_game_lifecycle<F>(phase: GameLifecyclePhase, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    crate::sdk::events::skse::messages::on_game_lifecycle(phase, callback)
}

#[inline(always)]
pub fn on_lifecycle<F>(phase: LifecyclePhase, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    crate::sdk::events::skse::messages::on_lifecycle(phase, callback)
}

#[inline(always)]
pub fn on_post_load<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_plugin_phase(PluginLifecyclePhase::PostLoad, callback)
}

#[inline(always)]
pub fn on_post_post_load<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_plugin_phase(PluginLifecyclePhase::PostPostLoad, callback)
}

#[inline(always)]
pub fn on_input_loaded<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_plugin_phase(PluginLifecyclePhase::InputLoaded, callback)
}

#[inline(always)]
pub fn on_data_loaded<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_plugin_phase(PluginLifecyclePhase::DataLoaded, callback)
}

#[inline(always)]
pub fn on_pre_load_game<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle(GameLifecyclePhase::PreLoadGame, callback)
}

#[inline(always)]
pub fn on_post_load_game<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle(GameLifecyclePhase::PostLoadGame, callback)
}

#[inline(always)]
pub fn on_save_game<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle(GameLifecyclePhase::SaveGame, callback)
}

#[inline(always)]
pub fn on_delete_game<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle(GameLifecyclePhase::DeleteGame, callback)
}

#[inline(always)]
pub fn on_new_game<F>(callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle(GameLifecyclePhase::NewGame, callback)
}
