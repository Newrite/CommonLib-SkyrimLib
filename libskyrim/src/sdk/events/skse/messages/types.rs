use alloc::ffi::CString;
use core::ffi::{CStr, c_void};
use core::fmt;
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::ops::Deref;
use core::slice;
use core::str;

use crate::sdk::core::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};
use crate::skse::Message;

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
pub struct MessageListener(pub(super) PhantomData<()>);
