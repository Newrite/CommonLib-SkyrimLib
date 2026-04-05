use alloc::borrow::ToOwned;
use alloc::ffi::{CString, NulError};
use core::ffi::CStr;
use core::fmt;

use crate::sdk::events::skse::messages::MessageRef;

/// Install-time filtering for inter-plugin message listeners.
///
/// Use this when the listener should stay installed once but only react to a
/// narrow subset of custom plugin messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageFilter {
    message_type: Option<u32>,
    sender: Option<CString>,
}

impl MessageFilter {
    /// Match any inter-plugin message.
    #[inline(always)]
    pub const fn any() -> Self {
        Self {
            message_type: None,
            sender: None,
        }
    }

    /// Match one exact message type.
    #[inline(always)]
    pub const fn for_type(message_type: u32) -> Self {
        Self {
            message_type: Some(message_type),
            sender: None,
        }
    }

    /// Match any message sent by one sender.
    #[inline(always)]
    pub fn for_sender(sender: &CStr) -> Self {
        Self {
            message_type: None,
            sender: Some(sender.to_owned()),
        }
    }

    /// String-based variant of [`Self::for_sender`].
    #[inline(always)]
    pub fn for_sender_str(sender: &str) -> Result<Self, NulError> {
        Ok(Self {
            message_type: None,
            sender: Some(CString::new(sender)?),
        })
    }

    /// Match one exact message type from one sender.
    #[inline(always)]
    pub fn for_type_sender(message_type: u32, sender: &CStr) -> Self {
        Self::for_type(message_type).with_sender(sender)
    }

    /// String-based variant of [`Self::for_type_sender`].
    #[inline(always)]
    pub fn for_type_sender_str(message_type: u32, sender: &str) -> Result<Self, NulError> {
        Ok(Self::for_type(message_type).with_sender_str(sender)?)
    }

    /// Optional exact message type currently configured on the filter.
    #[inline(always)]
    pub const fn message_type(&self) -> Option<u32> {
        self.message_type
    }

    /// Optional exact sender currently configured on the filter.
    #[inline(always)]
    pub fn sender(&self) -> Option<&CStr> {
        self.sender.as_deref()
    }

    /// Add or replace the exact message type match.
    #[inline(always)]
    pub const fn with_message_type(mut self, message_type: u32) -> Self {
        self.message_type = Some(message_type);
        self
    }

    /// Add or replace the exact sender match.
    #[inline(always)]
    pub fn with_sender(mut self, sender: &CStr) -> Self {
        self.sender = Some(sender.to_owned());
        self
    }

    /// String-based variant of [`Self::with_sender`].
    #[inline(always)]
    pub fn with_sender_str(mut self, sender: &str) -> Result<Self, NulError> {
        self.sender = Some(CString::new(sender)?);
        Ok(self)
    }

    /// Check whether a received message satisfies the filter.
    #[inline(always)]
    pub fn matches(&self, message: MessageRef<'_>) -> bool {
        if let Some(expected) = self.message_type {
            if message.kind_raw() != expected {
                return false;
            }
        }

        match &self.sender {
            Some(expected) => message.sender_cstr() == Some(expected.as_c_str()),
            None => true,
        }
    }
}

/// Failure installing the shared inter-plugin listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListenerInstallError {
    /// The SKSE messaging interface was unavailable.
    InterfaceUnavailable,
    /// Registering the shared listener with SKSE failed.
    RegisterFailed,
}

impl fmt::Display for ListenerInstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InterfaceUnavailable => write!(f, "SKSE messaging interface is unavailable"),
            Self::RegisterFailed => write!(f, "failed to register the inter-plugin listener"),
        }
    }
}

impl core::error::Error for ListenerInstallError {}

/// Failure dispatching one custom plugin message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchError {
    /// The SKSE messaging interface was unavailable.
    InterfaceUnavailable,
    /// The payload length exceeded the `u32`-sized SKSE payload contract.
    DataTooLarge(usize),
    /// The SKSE dispatch call reported failure.
    DispatchFailed,
}

impl fmt::Display for DispatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InterfaceUnavailable => write!(f, "SKSE messaging interface is unavailable"),
            Self::DataTooLarge(len) => {
                write!(f, "message payload length {} exceeds u32::MAX", len)
            }
            Self::DispatchFailed => write!(f, "SKSE message dispatch failed"),
        }
    }
}

impl core::error::Error for DispatchError {}

/// Semantic version pair commonly used by plugin APIs.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ApiVersion {
    pub major: u16,
    pub minor: u16,
}

impl ApiVersion {
    /// Zero/unspecified protocol version.
    pub const ZERO: Self = Self { major: 0, minor: 0 };

    /// Construct an explicit semantic version pair.
    #[inline(always)]
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    /// Whether the version is the zero/unspecified sentinel.
    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        self.major == 0 && self.minor == 0
    }

    /// Whether this server version satisfies the requested client version.
    ///
    /// The current policy is "same major, equal-or-greater minor".
    #[inline(always)]
    pub const fn satisfies(self, requested: Self) -> bool {
        self.major == requested.major && self.minor >= requested.minor
    }
}

/// Basic version handshake payload for synchronous plugin API negotiation.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VersionHandshake {
    pub client: ApiVersion,
    pub server: ApiVersion,
}

impl VersionHandshake {
    /// Construct a client-side handshake with an empty server response slot.
    #[inline(always)]
    pub const fn new(client: ApiVersion) -> Self {
        Self {
            client,
            server: ApiVersion::ZERO,
        }
    }

    /// Whether the server version recorded in the handshake is compatible with
    /// the client request.
    #[inline(always)]
    pub const fn is_compatible(&self) -> bool {
        self.server.satisfies(self.client)
    }

    /// Fill in the responding server version.
    #[inline(always)]
    pub fn respond(&mut self, server: ApiVersion) {
        self.server = server;
    }
}

/// Default response codes for synchronous inter-plugin query protocols.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryResponse {
    Success = 0,
    NotReady = 1,
    InvalidArguments = 2,
    VersionMismatch = 3,
    UnsupportedRequest = 4,
    HandlerFailed = 5,
    CallbackNotInvoked = 6,
}

impl QueryResponse {
    /// Decode a raw protocol response code.
    #[inline(always)]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Success),
            1 => Some(Self::NotReady),
            2 => Some(Self::InvalidArguments),
            3 => Some(Self::VersionMismatch),
            4 => Some(Self::UnsupportedRequest),
            5 => Some(Self::HandlerFailed),
            6 => Some(Self::CallbackNotInvoked),
            _ => None,
        }
    }
}

impl fmt::Display for QueryResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::NotReady => write!(f, "server is not ready"),
            Self::InvalidArguments => write!(f, "invalid request or response arguments"),
            Self::VersionMismatch => write!(f, "protocol version mismatch"),
            Self::UnsupportedRequest => write!(f, "unsupported request type"),
            Self::HandlerFailed => write!(f, "request handler reported failure"),
            Self::CallbackNotInvoked => write!(f, "array response callback was not invoked"),
        }
    }
}

/// Shared message IDs for a versioned request/response protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestMessageIds {
    pub version: u32,
    pub query: u32,
}

impl RequestMessageIds {
    /// Construct one `version/query` message-ID pair.
    #[inline(always)]
    pub const fn new(version: u32, query: u32) -> Self {
        Self { version, query }
    }
}
