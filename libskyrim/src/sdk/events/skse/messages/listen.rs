use core::ffi::CStr;

use crate::sdk::core::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};
use crate::skse::{self, Message};

use super::types::{MessageKind, MessageListener, MessageRef};

#[inline(always)]
pub fn on_raw(kind: MessageKind, callback: fn(&Message)) -> MessageListener {
    skse::register_listener(kind.raw(), callback);
    MessageListener(core::marker::PhantomData)
}

pub fn on<F>(kind: MessageKind, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    let mut callback = callback;
    skse::register_listener_dyn(kind.raw(), move |message| {
        callback(MessageRef::new(message))
    });
    MessageListener(core::marker::PhantomData)
}

pub fn on_filtered<P, F>(kind: MessageKind, predicate: P, callback: F) -> MessageListener
where
    P: for<'a> Fn(MessageRef<'a>) -> bool + 'static,
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    let mut callback = callback;
    on(kind, move |message| {
        if predicate(message) {
            callback(message);
        }
    })
}

#[inline(always)]
pub fn on_sender<F>(kind: MessageKind, sender: &'static CStr, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_filtered(
        kind,
        move |message| message.sender_matches(sender),
        callback,
    )
}

#[inline(always)]
pub fn on_sender_str<F>(kind: MessageKind, sender: &'static str, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_filtered(kind, move |message| message.sender_equals(sender), callback)
}

#[inline(always)]
pub fn on_plugin_phase_raw(phase: PluginLifecyclePhase, callback: fn(&Message)) -> MessageListener {
    on_raw(message_kind_for_plugin_phase(phase), callback)
}

#[inline(always)]
pub fn on_plugin_phase<F>(phase: PluginLifecyclePhase, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on(message_kind_for_plugin_phase(phase), callback)
}

pub fn on_plugin_phase_filtered<P, F>(
    phase: PluginLifecyclePhase,
    predicate: P,
    callback: F,
) -> MessageListener
where
    P: for<'a> Fn(MessageRef<'a>) -> bool + 'static,
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_filtered(message_kind_for_plugin_phase(phase), predicate, callback)
}

#[inline(always)]
pub fn on_plugin_phase_sender<F>(
    phase: PluginLifecyclePhase,
    sender: &'static CStr,
    callback: F,
) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_plugin_phase_filtered(
        phase,
        move |message| message.sender_matches(sender),
        callback,
    )
}

#[inline(always)]
pub fn on_plugin_phase_sender_str<F>(
    phase: PluginLifecyclePhase,
    sender: &'static str,
    callback: F,
) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_plugin_phase_filtered(
        phase,
        move |message| message.sender_equals(sender),
        callback,
    )
}

#[inline(always)]
pub fn on_game_lifecycle_raw(phase: GameLifecyclePhase, callback: fn(&Message)) -> MessageListener {
    on_raw(message_kind_for_game_lifecycle(phase), callback)
}

#[inline(always)]
pub fn on_game_lifecycle<F>(phase: GameLifecyclePhase, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on(message_kind_for_game_lifecycle(phase), callback)
}

pub fn on_game_lifecycle_filtered<P, F>(
    phase: GameLifecyclePhase,
    predicate: P,
    callback: F,
) -> MessageListener
where
    P: for<'a> Fn(MessageRef<'a>) -> bool + 'static,
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_filtered(message_kind_for_game_lifecycle(phase), predicate, callback)
}

#[inline(always)]
pub fn on_game_lifecycle_sender<F>(
    phase: GameLifecyclePhase,
    sender: &'static CStr,
    callback: F,
) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle_filtered(
        phase,
        move |message| message.sender_matches(sender),
        callback,
    )
}

#[inline(always)]
pub fn on_game_lifecycle_sender_str<F>(
    phase: GameLifecyclePhase,
    sender: &'static str,
    callback: F,
) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_game_lifecycle_filtered(
        phase,
        move |message| message.sender_equals(sender),
        callback,
    )
}

#[inline(always)]
pub fn on_lifecycle_raw(phase: LifecyclePhase, callback: fn(&Message)) -> MessageListener {
    match phase {
        LifecyclePhase::Plugin(phase) => on_plugin_phase_raw(phase, callback),
        LifecyclePhase::Game(phase) => on_game_lifecycle_raw(phase, callback),
    }
}

#[inline(always)]
pub fn on_lifecycle<F>(phase: LifecyclePhase, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    match phase {
        LifecyclePhase::Plugin(phase) => on_plugin_phase(phase, callback),
        LifecyclePhase::Game(phase) => on_game_lifecycle(phase, callback),
    }
}

pub fn on_lifecycle_filtered<P, F>(
    phase: LifecyclePhase,
    predicate: P,
    callback: F,
) -> MessageListener
where
    P: for<'a> Fn(MessageRef<'a>) -> bool + 'static,
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    match phase {
        LifecyclePhase::Plugin(phase) => on_plugin_phase_filtered(phase, predicate, callback),
        LifecyclePhase::Game(phase) => on_game_lifecycle_filtered(phase, predicate, callback),
    }
}

#[inline(always)]
pub fn on_lifecycle_sender<F>(
    phase: LifecyclePhase,
    sender: &'static CStr,
    callback: F,
) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_lifecycle_filtered(
        phase,
        move |message| message.sender_matches(sender),
        callback,
    )
}

#[inline(always)]
pub fn on_lifecycle_sender_str<F>(
    phase: LifecyclePhase,
    sender: &'static str,
    callback: F,
) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    on_lifecycle_filtered(
        phase,
        move |message| message.sender_equals(sender),
        callback,
    )
}

#[inline(always)]
const fn message_kind_for_plugin_phase(phase: PluginLifecyclePhase) -> MessageKind {
    match phase {
        PluginLifecyclePhase::PostLoad => MessageKind::PostLoad,
        PluginLifecyclePhase::PostPostLoad => MessageKind::PostPostLoad,
        PluginLifecyclePhase::InputLoaded => MessageKind::InputLoaded,
        PluginLifecyclePhase::DataLoaded => MessageKind::DataLoaded,
    }
}

#[inline(always)]
const fn message_kind_for_game_lifecycle(phase: GameLifecyclePhase) -> MessageKind {
    match phase {
        GameLifecyclePhase::PreLoadGame => MessageKind::PreLoadGame,
        GameLifecyclePhase::PostLoadGame => MessageKind::PostLoadGame,
        GameLifecyclePhase::SaveGame => MessageKind::SaveGame,
        GameLifecyclePhase::DeleteGame => MessageKind::DeleteGame,
        GameLifecyclePhase::NewGame => MessageKind::NewGame,
    }
}
