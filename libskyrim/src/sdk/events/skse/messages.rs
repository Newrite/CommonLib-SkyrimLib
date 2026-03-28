//! High-level helpers for SKSE plugin messaging and lifecycle listeners.
//!
//! This domain remains distinct from ordinary `BSTEventSource<T>` subscriptions
//! because `MessagingInterface::RegisterListener` is install-once style and
//! does not offer the same RAII removal semantics as engine event sinks.

use alloc::ffi::CString;
use core::ffi::{CStr, c_void};
use core::fmt;
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::ops::Deref;
use core::slice;
use core::str;

use crate::sdk::core::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};
use crate::skse::{self, Message};

/// Stable SDK-facing SKSE lifecycle message kinds.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    PostLoad = Message::SKSE_POST_LOAD,
    PostPostLoad = Message::SKSE_POST_POST_LOAD,
    PreLoadGame = Message::SKSE_PRE_LOAD_GAME,
    PostLoadGame = Message::SKSE_POST_LOAD_GAME,
    SaveGame = Message::SKSE_SAVE_GAME,
    DeleteGame = Message::SKSE_DELETE_GAME,
    InputLoaded = Message::SKSE_INPUT_LOADED,
    NewGame = Message::SKSE_NEW_GAME,
    DataLoaded = Message::SKSE_DATA_LOADED,
}

impl MessageKind {
    pub const ALL: [Self; 9] = [
        Self::PostLoad,
        Self::PostPostLoad,
        Self::PreLoadGame,
        Self::PostLoadGame,
        Self::SaveGame,
        Self::DeleteGame,
        Self::InputLoaded,
        Self::NewGame,
        Self::DataLoaded,
    ];

    #[inline(always)]
    pub const fn raw(self) -> u32 {
        self as u32
    }

    #[inline(always)]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            Message::SKSE_POST_LOAD => Some(Self::PostLoad),
            Message::SKSE_POST_POST_LOAD => Some(Self::PostPostLoad),
            Message::SKSE_PRE_LOAD_GAME => Some(Self::PreLoadGame),
            Message::SKSE_POST_LOAD_GAME => Some(Self::PostLoadGame),
            Message::SKSE_SAVE_GAME => Some(Self::SaveGame),
            Message::SKSE_DELETE_GAME => Some(Self::DeleteGame),
            Message::SKSE_INPUT_LOADED => Some(Self::InputLoaded),
            Message::SKSE_NEW_GAME => Some(Self::NewGame),
            Message::SKSE_DATA_LOADED => Some(Self::DataLoaded),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn plugin_phase(self) -> Option<PluginLifecyclePhase> {
        match self {
            Self::PostLoad => Some(PluginLifecyclePhase::PostLoad),
            Self::PostPostLoad => Some(PluginLifecyclePhase::PostPostLoad),
            Self::InputLoaded => Some(PluginLifecyclePhase::InputLoaded),
            Self::DataLoaded => Some(PluginLifecyclePhase::DataLoaded),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn game_lifecycle_phase(self) -> Option<GameLifecyclePhase> {
        match self {
            Self::PreLoadGame => Some(GameLifecyclePhase::PreLoadGame),
            Self::PostLoadGame => Some(GameLifecyclePhase::PostLoadGame),
            Self::SaveGame => Some(GameLifecyclePhase::SaveGame),
            Self::DeleteGame => Some(GameLifecyclePhase::DeleteGame),
            Self::NewGame => Some(GameLifecyclePhase::NewGame),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn lifecycle_phase(self) -> Option<LifecyclePhase> {
        match self.plugin_phase() {
            Some(phase) => Some(LifecyclePhase::Plugin(phase)),
            None => match self.game_lifecycle_phase() {
                Some(phase) => Some(LifecyclePhase::Game(phase)),
                None => None,
            },
        }
    }
}

/// Borrowed high-level view over one SKSE `Message`.
#[derive(Clone, Copy)]
pub struct MessageRef<'a> {
    raw: &'a Message,
}

/// Typed borrowed payload view over one SKSE message.
#[derive(Clone, Copy)]
pub struct TypedMessageRef<'a, T> {
    message: MessageRef<'a>,
    payload: &'a T,
}

impl<'a, T> TypedMessageRef<'a, T> {
    #[inline(always)]
    pub const fn message(self) -> MessageRef<'a> {
        self.message
    }

    #[inline(always)]
    pub const fn payload(self) -> &'a T {
        self.payload
    }
}

impl<T> Deref for TypedMessageRef<'_, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.payload
    }
}

impl<T: fmt::Debug> fmt::Debug for TypedMessageRef<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypedMessageRef")
            .field("message", &self.message)
            .field("payload", &self.payload)
            .finish()
    }
}

/// Typed borrowed slice payload view over one SKSE message.
#[derive(Clone, Copy)]
pub struct TypedMessageSliceRef<'a, T> {
    message: MessageRef<'a>,
    payload: &'a [T],
}

impl<'a, T> TypedMessageSliceRef<'a, T> {
    #[inline(always)]
    pub const fn message(self) -> MessageRef<'a> {
        self.message
    }

    #[inline(always)]
    pub const fn payload(self) -> &'a [T] {
        self.payload
    }
}

impl<T> Deref for TypedMessageSliceRef<'_, T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.payload
    }
}

impl<T: fmt::Debug> fmt::Debug for TypedMessageSliceRef<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypedMessageSliceRef")
            .field("message", &self.message)
            .field("payload", &self.payload)
            .finish()
    }
}

impl<'a> MessageRef<'a> {
    #[inline(always)]
    pub const fn new(raw: &'a Message) -> Self {
        Self { raw }
    }

    #[inline(always)]
    pub const fn raw(self) -> &'a Message {
        self.raw
    }

    #[inline(always)]
    pub const fn kind_raw(self) -> u32 {
        self.raw.msg_type
    }

    #[inline(always)]
    pub const fn kind(self) -> Option<MessageKind> {
        MessageKind::from_raw(self.kind_raw())
    }

    #[inline(always)]
    pub const fn plugin_phase(self) -> Option<PluginLifecyclePhase> {
        match self.kind() {
            Some(kind) => kind.plugin_phase(),
            None => None,
        }
    }

    #[inline(always)]
    pub const fn game_lifecycle_phase(self) -> Option<GameLifecyclePhase> {
        match self.kind() {
            Some(kind) => kind.game_lifecycle_phase(),
            None => None,
        }
    }

    #[inline(always)]
    pub const fn lifecycle_phase(self) -> Option<LifecyclePhase> {
        match self.kind() {
            Some(kind) => kind.lifecycle_phase(),
            None => None,
        }
    }

    #[inline(always)]
    pub const fn data_len(self) -> usize {
        self.raw.data_len as usize
    }

    #[inline(always)]
    pub const fn data_ptr(self) -> *mut c_void {
        self.raw.data
    }

    #[inline(always)]
    pub fn sender_cstr(self) -> Option<&'a CStr> {
        unsafe {
            self.raw
                .sender
                .as_ref()
                .map(|_| CStr::from_ptr(self.raw.sender))
        }
    }

    #[inline(always)]
    pub fn sender(self) -> Option<&'a str> {
        self.sender_cstr()?.to_str().ok()
    }

    #[inline(always)]
    pub fn sender_matches(self, sender: &CStr) -> bool {
        self.sender_cstr() == Some(sender)
    }

    #[inline(always)]
    pub fn sender_equals(self, sender: &str) -> bool {
        self.sender() == Some(sender)
    }

    #[inline(always)]
    pub fn sender_cstring(self) -> Option<CString> {
        self.sender_cstr().map(CString::from)
    }

    #[inline(always)]
    pub fn data_bytes(self) -> Option<&'a [u8]> {
        let len = self.data_len();
        if len == 0 {
            return Some(&[]);
        }

        let data = self.raw.data.cast::<u8>();
        if data.is_null() {
            return None;
        }

        Some(unsafe { slice::from_raw_parts(data, len) })
    }

    #[inline(always)]
    pub fn data_str(self) -> Option<&'a str> {
        str::from_utf8(self.data_bytes()?).ok()
    }

    #[inline(always)]
    pub fn data_as<T>(self) -> Option<&'a T> {
        if self.data_len() != size_of::<T>() {
            return None;
        }

        let data = self.raw.data.cast::<T>();
        if data.is_null() || !(data as usize).is_multiple_of(align_of::<T>()) {
            return None;
        }

        unsafe { data.as_ref() }
    }

    #[inline(always)]
    pub fn data_slice<T>(self) -> Option<&'a [T]> {
        let len = self.data_len();
        if len == 0 {
            return Some(&[]);
        }

        let stride = size_of::<T>();
        if stride == 0 || len % stride != 0 {
            return None;
        }

        let data = self.raw.data.cast::<T>();
        if data.is_null() || !(data as usize).is_multiple_of(align_of::<T>()) {
            return None;
        }

        Some(unsafe { slice::from_raw_parts(data, len / stride) })
    }

    #[inline(always)]
    pub fn data_copy<T>(self) -> Option<T>
    where
        T: Copy,
    {
        Some(*self.data_as::<T>()?)
    }

    #[inline(always)]
    pub fn typed<T>(self) -> Option<TypedMessageRef<'a, T>> {
        Some(TypedMessageRef {
            message: self,
            payload: self.data_as::<T>()?,
        })
    }

    #[inline(always)]
    pub fn typed_slice<T>(self) -> Option<TypedMessageSliceRef<'a, T>> {
        Some(TypedMessageSliceRef {
            message: self,
            payload: self.data_slice::<T>()?,
        })
    }
}

impl fmt::Debug for MessageRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MessageRef")
            .field("kind_raw", &self.kind_raw())
            .field("kind", &self.kind())
            .field("sender", &self.sender())
            .field("data_len", &self.data_len())
            .finish()
    }
}

/// Marker object for installed message listeners.
///
/// SKSE message listeners are install-once and do not support removal, so this
/// type is intentionally zero-sized.
#[derive(Debug, Clone, Copy, Default)]
pub struct MessageListener(PhantomData<()>);

#[inline(always)]
pub fn on_raw(kind: MessageKind, callback: fn(&Message)) -> MessageListener {
    skse::register_listener(kind.raw(), callback);
    MessageListener(PhantomData)
}

pub fn on<F>(kind: MessageKind, callback: F) -> MessageListener
where
    F: for<'a> FnMut(MessageRef<'a>) + 'static,
{
    let mut callback = callback;
    skse::register_listener_dyn(kind.raw(), move |message| {
        callback(MessageRef::new(message))
    });
    MessageListener(PhantomData)
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

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;
    use core::cell::Cell;
    use core::ffi::c_char;

    use super::*;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Pair {
        left: u32,
        right: u32,
    }

    #[test]
    fn message_ref_views_sender_and_payload() {
        let sender = c"SKSE";
        let payload = Pair { left: 1, right: 2 };
        let message = Message {
            sender: sender.as_ptr().cast::<c_char>(),
            msg_type: Message::SKSE_DATA_LOADED,
            data_len: size_of::<Pair>() as u32,
            data: (&payload as *const Pair).cast_mut().cast(),
        };

        let view = MessageRef::new(&message);
        assert_eq!(view.kind(), Some(MessageKind::DataLoaded));
        assert_eq!(view.sender(), Some("SKSE"));
        assert_eq!(view.data_as::<Pair>(), Some(&payload));
        assert_eq!(view.data_copy::<Pair>(), Some(payload));
        assert_eq!(view.typed::<Pair>().map(|typed| *typed), Some(payload));
    }

    #[test]
    fn message_ref_rejects_misaligned_payloads() {
        let mut bytes = [0u8; 8];
        let message = Message {
            sender: core::ptr::null(),
            msg_type: Message::SKSE_POST_LOAD,
            data_len: 4,
            data: unsafe { bytes.as_mut_ptr().add(1) }.cast(),
        };

        let view = MessageRef::new(&message);
        assert_eq!(view.data_as::<u32>(), None);
    }

    #[test]
    fn message_kinds_map_to_lifecycle_phases() {
        assert_eq!(
            MessageKind::PostLoad.plugin_phase(),
            Some(PluginLifecyclePhase::PostLoad)
        );
        assert_eq!(
            MessageKind::NewGame.game_lifecycle_phase(),
            Some(GameLifecyclePhase::NewGame)
        );
        assert_eq!(
            MessageKind::DataLoaded.lifecycle_phase(),
            Some(LifecyclePhase::Plugin(PluginLifecyclePhase::DataLoaded))
        );
        assert_eq!(
            MessageKind::DeleteGame.lifecycle_phase(),
            Some(LifecyclePhase::Game(GameLifecyclePhase::DeleteGame))
        );
    }

    #[test]
    fn message_ref_exposes_lifecycle_phases() {
        let message = Message {
            sender: core::ptr::null(),
            msg_type: Message::SKSE_SAVE_GAME,
            data_len: 0,
            data: core::ptr::null_mut(),
        };

        let view = MessageRef::new(&message);
        assert_eq!(view.plugin_phase(), None);
        assert_eq!(
            view.game_lifecycle_phase(),
            Some(GameLifecyclePhase::SaveGame)
        );
        assert_eq!(
            view.lifecycle_phase(),
            Some(LifecyclePhase::Game(GameLifecyclePhase::SaveGame))
        );
    }

    #[test]
    fn sender_filtered_helpers_only_fire_for_matching_sender() {
        let seen = Rc::new(Cell::new(0u32));
        let seen_ref = Rc::clone(&seen);
        let sender = c"SKSE";

        let callback = {
            let sender = sender;
            move |message: MessageRef<'_>| {
                if message.sender_matches(sender) {
                    seen_ref.set(seen_ref.get() + 1);
                }
            }
        };

        let good = Message {
            sender: sender.as_ptr().cast::<c_char>(),
            msg_type: Message::SKSE_DATA_LOADED,
            data_len: 0,
            data: core::ptr::null_mut(),
        };
        let bad_sender = c"Other";
        let bad = Message {
            sender: bad_sender.as_ptr().cast::<c_char>(),
            msg_type: Message::SKSE_DATA_LOADED,
            data_len: 0,
            data: core::ptr::null_mut(),
        };

        callback(MessageRef::new(&good));
        assert_eq!(seen.get(), 1);
        if MessageRef::new(&bad).sender_matches(sender) {
            seen.set(seen.get() + 1);
        }
        assert_eq!(seen.get(), 1);
    }

    #[test]
    fn typed_slice_view_wraps_payload_slice() {
        let payload = [1u32, 2, 3, 4];
        let message = Message {
            sender: core::ptr::null(),
            msg_type: Message::SKSE_POST_LOAD,
            data_len: (size_of::<u32>() * payload.len()) as u32,
            data: payload.as_ptr().cast_mut().cast(),
        };

        let view = MessageRef::new(&message);
        let typed = view
            .typed_slice::<u32>()
            .expect("slice payload should decode");
        assert_eq!(typed.message().kind(), Some(MessageKind::PostLoad));
        assert_eq!(typed.payload(), payload.as_slice());
    }
}
