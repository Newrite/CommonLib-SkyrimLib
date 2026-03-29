//! Plugin-to-plugin messaging helpers built on `SKSE::MessagingInterface`.
//!
//! This module focuses on two recurring patterns:
//!
//! - install-once listeners for non-SKSE plugin messages
//! - synchronous request / response protocols built on message payloads
//!
//! Example server:
//!
//! ```rust,ignore
//! use libskyrim::sdk::interop::messaging::{
//!     ApiVersion, QueryResponse, RequestMessageIds, RequestServer,
//! };
//!
//! #[repr(C)]
//! #[derive(Clone, Copy)]
//! struct PingRequest {
//!     value: u32,
//! }
//!
//! #[repr(C)]
//! #[derive(Clone, Copy)]
//! struct PongResponse {
//!     doubled: u32,
//! }
//!
//! const MSG_VERSION: u32 = 0x9000;
//! const MSG_QUERY: u32 = 0x9001;
//! const REQ_PING: u32 = 1;
//!
//! fn install_server(
//! ) -> Result<(), libskyrim::sdk::interop::messaging::ListenerInstallError> {
//!     let _server = RequestServer::builder(
//!         ApiVersion::new(1, 0),
//!         RequestMessageIds::new(MSG_VERSION, MSG_QUERY),
//!     )
//!     .handle_value(REQ_PING, |_context, request: &PingRequest| {
//!         Ok(PongResponse {
//!             doubled: request.value * 2,
//!         })
//!     })
//!     .install()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Matching client:
//!
//! ```rust,ignore
//! use libskyrim::sdk::interop::messaging::{ApiVersion, ClientError, RequestClient, RequestMessageIds};
//!
//! # #[repr(C)]
//! # #[derive(Clone, Copy)]
//! # struct PingRequest {
//! #     value: u32,
//! # }
//! # #[repr(C)]
//! # #[derive(Clone, Copy)]
//! # struct PongResponse {
//! #     doubled: u32,
//! # }
//! # const MSG_VERSION: u32 = 0x9000;
//! # const MSG_QUERY: u32 = 0x9001;
//! # const REQ_PING: u32 = 1;
//! fn ping_server() -> Result<u32, ClientError> {
//!     let client = RequestClient::new_str(
//!         "ExampleServer",
//!         ApiVersion::new(1, 0),
//!         RequestMessageIds::new(MSG_VERSION, MSG_QUERY),
//!     )
//!     .expect("plugin names used for SKSE messaging cannot contain NUL");
//!
//!     client.check_version()?;
//!     let response: PongResponse = client.query(REQ_PING, &PingRequest { value: 21 })?;
//!     Ok(response.doubled)
//! }
//! ```

use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::ffi::{CString, NulError};
use alloc::vec::Vec;
use core::ffi::{CStr, c_void};
use core::fmt;
use core::marker::PhantomData;
use core::mem::{MaybeUninit, align_of, size_of};
use core::slice;

use core_util::RacyCell;
use spin::Mutex;

use crate::sdk::events::skse::messages::MessageRef;
use crate::skse::{self, Message};

type DynInteropHandler = dyn FnMut(&Message) + Send + 'static;

struct RegisteredListener {
    filter: MessageFilter,
    callback: Box<DynInteropHandler>,
}

static INTEROP_LISTENERS: RacyCell<Vec<RegisteredListener>> = RacyCell::new(Vec::new());
static INTEROP_LISTENER_INSTALLED: Mutex<bool> = Mutex::new(false);

/// Install-time filtering for inter-plugin message listeners.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageFilter {
    message_type: Option<u32>,
    sender: Option<CString>,
}

impl MessageFilter {
    #[inline(always)]
    pub const fn any() -> Self {
        Self {
            message_type: None,
            sender: None,
        }
    }

    #[inline(always)]
    pub const fn for_type(message_type: u32) -> Self {
        Self {
            message_type: Some(message_type),
            sender: None,
        }
    }

    #[inline(always)]
    pub fn for_sender(sender: &CStr) -> Self {
        Self {
            message_type: None,
            sender: Some(sender.to_owned()),
        }
    }

    #[inline(always)]
    pub fn for_sender_str(sender: &str) -> Result<Self, NulError> {
        Ok(Self {
            message_type: None,
            sender: Some(CString::new(sender)?),
        })
    }

    #[inline(always)]
    pub fn for_type_sender(message_type: u32, sender: &CStr) -> Self {
        Self::for_type(message_type).with_sender(sender)
    }

    #[inline(always)]
    pub fn for_type_sender_str(message_type: u32, sender: &str) -> Result<Self, NulError> {
        Ok(Self::for_type(message_type).with_sender_str(sender)?)
    }

    #[inline(always)]
    pub const fn message_type(&self) -> Option<u32> {
        self.message_type
    }

    #[inline(always)]
    pub fn sender(&self) -> Option<&CStr> {
        self.sender.as_deref()
    }

    #[inline(always)]
    pub const fn with_message_type(mut self, message_type: u32) -> Self {
        self.message_type = Some(message_type);
        self
    }

    #[inline(always)]
    pub fn with_sender(mut self, sender: &CStr) -> Self {
        self.sender = Some(sender.to_owned());
        self
    }

    #[inline(always)]
    pub fn with_sender_str(mut self, sender: &str) -> Result<Self, NulError> {
        self.sender = Some(CString::new(sender)?);
        Ok(self)
    }

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
    InterfaceUnavailable,
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
    InterfaceUnavailable,
    DataTooLarge(usize),
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
    pub const ZERO: Self = Self { major: 0, minor: 0 };

    #[inline(always)]
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        self.major == 0 && self.minor == 0
    }

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
    #[inline(always)]
    pub const fn new(client: ApiVersion) -> Self {
        Self {
            client,
            server: ApiVersion::ZERO,
        }
    }

    #[inline(always)]
    pub const fn is_compatible(&self) -> bool {
        self.server.satisfies(self.client)
    }

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
    #[inline(always)]
    pub const fn new(version: u32, query: u32) -> Self {
        Self { version, query }
    }
}

/// Raw callback signature used for array-style query responses.
pub type QueryArrayCallback = unsafe extern "system" fn(*mut c_void, usize, *const c_void);

/// Generic synchronous query envelope.
///
/// This is intentionally low-level and honest: server-side decoding and
/// response writing remain `unsafe` because the remote plugin controls the raw
/// payload pointers.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct QueryEnvelope {
    pub version: ApiVersion,
    pub request_type: u32,
    pub request_data: *const c_void,
    pub request_data_len: u32,
    pub response_code: u32,
    pub response_data: *mut c_void,
    pub response_data_len: u32,
    pub response_callback: Option<QueryArrayCallback>,
    pub response_callback_user: *mut c_void,
    pub response_count: usize,
}

impl QueryEnvelope {
    #[inline(always)]
    const fn new(
        version: ApiVersion,
        request_type: u32,
        request_data: *const c_void,
        request_data_len: u32,
        response_data: *mut c_void,
        response_data_len: u32,
        response_callback: Option<QueryArrayCallback>,
        response_callback_user: *mut c_void,
    ) -> Self {
        Self {
            version,
            request_type,
            request_data,
            request_data_len,
            response_code: QueryResponse::Success as u32,
            response_data,
            response_data_len,
            response_callback,
            response_callback_user,
            response_count: 0,
        }
    }

    #[inline(always)]
    pub fn response_code(&self) -> Option<QueryResponse> {
        QueryResponse::from_raw(self.response_code)
    }

    #[inline(always)]
    pub fn set_response_code(&mut self, code: QueryResponse) {
        self.response_code = code as u32;
    }

    /// # Safety
    /// The caller must ensure the remote plugin provided a valid request buffer
    /// for `T`.
    pub unsafe fn request_as<T>(&self) -> Option<&T> {
        unsafe { payload_ref(self.request_data, self.request_data_len as usize) }
    }

    /// # Safety
    /// The caller must ensure the remote plugin provided a valid response
    /// buffer for `T`.
    pub unsafe fn response_buffer_as_mut<T>(&mut self) -> Option<&mut T> {
        unsafe { payload_mut(self.response_data, self.response_data_len as usize) }
    }

    /// # Safety
    /// The caller must ensure the remote plugin provided a valid response
    /// buffer for `T`.
    pub unsafe fn write_response<T>(&mut self, value: T) -> QueryResponse
    where
        T: Copy,
    {
        let Some(response) = (unsafe { self.response_buffer_as_mut::<T>() }) else {
            self.set_response_code(QueryResponse::InvalidArguments);
            return QueryResponse::InvalidArguments;
        };

        *response = value;
        self.response_count = 1;
        self.set_response_code(QueryResponse::Success);
        QueryResponse::Success
    }

    /// # Safety
    /// The caller must ensure the remote plugin provided a valid array callback
    /// contract for `T`.
    pub unsafe fn send_response_slice<T>(&mut self, values: &[T]) -> QueryResponse {
        let Some(callback) = self.response_callback else {
            self.set_response_code(QueryResponse::InvalidArguments);
            return QueryResponse::InvalidArguments;
        };

        let data = if values.is_empty() {
            core::ptr::null()
        } else {
            values.as_ptr().cast()
        };

        unsafe {
            callback(self.response_callback_user, values.len(), data);
        }

        self.response_count = values.len();
        self.set_response_code(QueryResponse::Success);
        QueryResponse::Success
    }
}

/// Client-side errors for synchronous request / response protocols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientError {
    Dispatch(DispatchError),
    NoVersionResponse,
    VersionMismatch {
        requested: ApiVersion,
        server: ApiVersion,
    },
    QueryRejected(QueryResponse),
    UnknownQueryResponse(u32),
    CallbackNotInvoked,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dispatch(error) => write!(f, "{}", error),
            Self::NoVersionResponse => write!(f, "server did not respond to the version handshake"),
            Self::VersionMismatch { requested, server } => write!(
                f,
                "version mismatch: requested {}.{}, server replied with {}.{}",
                requested.major, requested.minor, server.major, server.minor
            ),
            Self::QueryRejected(code) => write!(f, "query rejected: {}", code),
            Self::UnknownQueryResponse(code) => {
                write!(f, "query rejected with unknown response code {}", code)
            }
            Self::CallbackNotInvoked => {
                write!(
                    f,
                    "array query succeeded but the response callback was not invoked"
                )
            }
        }
    }
}

impl core::error::Error for ClientError {}

impl From<DispatchError> for ClientError {
    #[inline(always)]
    fn from(value: DispatchError) -> Self {
        Self::Dispatch(value)
    }
}

/// Small client wrapper for a versioned synchronous query protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestClient {
    receiver: CString,
    version: ApiVersion,
    ids: RequestMessageIds,
}

impl RequestClient {
    #[inline(always)]
    pub fn new(receiver: &CStr, version: ApiVersion, ids: RequestMessageIds) -> Self {
        Self {
            receiver: receiver.to_owned(),
            version,
            ids,
        }
    }

    #[inline(always)]
    pub fn new_str(
        receiver: &str,
        version: ApiVersion,
        ids: RequestMessageIds,
    ) -> Result<Self, NulError> {
        Ok(Self {
            receiver: CString::new(receiver)?,
            version,
            ids,
        })
    }

    #[inline(always)]
    pub fn receiver(&self) -> &CStr {
        self.receiver.as_c_str()
    }

    #[inline(always)]
    pub const fn version(&self) -> ApiVersion {
        self.version
    }

    #[inline(always)]
    pub const fn message_ids(&self) -> RequestMessageIds {
        self.ids
    }

    pub fn handshake(&self) -> Result<VersionHandshake, ClientError> {
        let mut handshake = VersionHandshake::new(self.version);
        dispatch_value(self.receiver(), self.ids.version, &mut handshake)?;
        if handshake.server.is_zero() {
            return Err(ClientError::NoVersionResponse);
        }
        Ok(handshake)
    }

    pub fn check_version(&self) -> Result<ApiVersion, ClientError> {
        let handshake = self.handshake()?;
        if handshake.is_compatible() {
            Ok(handshake.server)
        } else {
            Err(ClientError::VersionMismatch {
                requested: handshake.client,
                server: handshake.server,
            })
        }
    }

    pub fn query<TRequest, TResponse>(
        &self,
        request_type: u32,
        request: &TRequest,
    ) -> Result<TResponse, ClientError>
    where
        TRequest: Copy,
        TResponse: Copy,
    {
        let mut response = MaybeUninit::<TResponse>::uninit();
        let (response_ptr, response_len) = maybe_uninit_ptr_and_len(&mut response);

        let mut query = QueryEnvelope::new(
            self.version,
            request_type,
            value_ptr(request),
            checked_len_u32(size_of::<TRequest>())?,
            response_ptr,
            response_len,
            None,
            core::ptr::null_mut(),
        );

        dispatch_value(self.receiver(), self.ids.query, &mut query)?;
        match query.response_code() {
            Some(QueryResponse::Success) => {}
            Some(code) => return Err(ClientError::QueryRejected(code)),
            None => return Err(ClientError::UnknownQueryResponse(query.response_code)),
        }

        Ok(unsafe { response.assume_init() })
    }

    #[inline(always)]
    pub fn notify<TRequest>(&self, request_type: u32, request: &TRequest) -> Result<(), ClientError>
    where
        TRequest: Copy,
    {
        self.query::<TRequest, ()>(request_type, request)
    }

    pub fn query_array<TRequest, TResponse, F>(
        &self,
        request_type: u32,
        request: &TRequest,
        callback: F,
    ) -> Result<(), ClientError>
    where
        TRequest: Copy,
        TResponse: Copy,
        F: FnMut(&[TResponse]),
    {
        let mut context = ArrayResponseContext::<F, TResponse> {
            callback,
            was_invoked: false,
            _marker: PhantomData,
        };

        let mut query = QueryEnvelope::new(
            self.version,
            request_type,
            value_ptr(request),
            checked_len_u32(size_of::<TRequest>())?,
            core::ptr::null_mut(),
            0,
            Some(array_response_bridge::<F, TResponse>),
            (&mut context as *mut ArrayResponseContext<F, TResponse>).cast(),
        );

        dispatch_value(self.receiver(), self.ids.query, &mut query)?;
        match query.response_code() {
            Some(QueryResponse::Success) => {
                if context.was_invoked {
                    Ok(())
                } else {
                    Err(ClientError::CallbackNotInvoked)
                }
            }
            Some(code) => Err(ClientError::QueryRejected(code)),
            None => Err(ClientError::UnknownQueryResponse(query.response_code)),
        }
    }
}

trait RequestHandler: Send {
    fn handle(&mut self, context: &mut QueryContext<'_>) -> QueryResponse;
}

impl<F> RequestHandler for F
where
    F: for<'a> FnMut(&mut QueryContext<'a>) -> QueryResponse + Send + 'static,
{
    #[inline(always)]
    fn handle(&mut self, context: &mut QueryContext<'_>) -> QueryResponse {
        self(context)
    }
}

struct RequestHandlerEntry {
    request_type: u32,
    handler: Box<dyn RequestHandler>,
}

/// Server-side query handling context.
pub struct QueryContext<'a> {
    message: MessageRef<'a>,
    envelope: &'a mut QueryEnvelope,
    server_version: ApiVersion,
}

impl<'a> QueryContext<'a> {
    #[inline(always)]
    const fn new(
        message: MessageRef<'a>,
        envelope: &'a mut QueryEnvelope,
        server_version: ApiVersion,
    ) -> Self {
        Self {
            message,
            envelope,
            server_version,
        }
    }

    #[inline(always)]
    pub const fn message(&self) -> MessageRef<'a> {
        self.message
    }

    #[inline(always)]
    pub fn sender(&self) -> Option<&'a str> {
        self.message.sender()
    }

    #[inline(always)]
    pub fn sender_cstr(&self) -> Option<&'a CStr> {
        self.message.sender_cstr()
    }

    #[inline(always)]
    pub const fn server_version(&self) -> ApiVersion {
        self.server_version
    }

    #[inline(always)]
    pub const fn client_version(&self) -> ApiVersion {
        self.envelope.version
    }

    #[inline(always)]
    pub const fn request_type(&self) -> u32 {
        self.envelope.request_type
    }

    #[inline(always)]
    pub fn envelope(&self) -> &QueryEnvelope {
        self.envelope
    }

    #[inline(always)]
    pub fn envelope_mut(&mut self) -> &mut QueryEnvelope {
        self.envelope
    }

    /// # Safety
    /// The caller must ensure the remote plugin provided a valid request buffer
    /// for `T`.
    #[inline(always)]
    pub unsafe fn request_as<T>(&self) -> Option<&'a T> {
        unsafe {
            payload_ref::<'a, T>(
                self.envelope.request_data,
                self.envelope.request_data_len as usize,
            )
        }
    }

    /// # Safety
    /// The caller must ensure the remote plugin provided a valid response
    /// buffer for `T`.
    #[inline(always)]
    pub unsafe fn response_buffer_as_mut<T>(&mut self) -> Option<&'a mut T> {
        unsafe {
            payload_mut::<'a, T>(
                self.envelope.response_data,
                self.envelope.response_data_len as usize,
            )
        }
    }

    /// # Safety
    /// The caller must ensure the remote plugin provided a valid response
    /// buffer for `T`.
    #[inline(always)]
    pub unsafe fn write_response<T>(&mut self, value: T) -> QueryResponse
    where
        T: Copy,
    {
        unsafe { self.envelope.write_response(value) }
    }

    /// # Safety
    /// The caller must ensure the remote plugin provided a valid array callback
    /// contract for `T`.
    #[inline(always)]
    pub unsafe fn send_response_slice<T>(&mut self, values: &[T]) -> QueryResponse {
        unsafe { self.envelope.send_response_slice(values) }
    }

    #[inline(always)]
    pub fn reject(&mut self, code: QueryResponse) -> QueryResponse {
        self.envelope.set_response_code(code);
        code
    }
}

impl fmt::Debug for QueryContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QueryContext")
            .field("sender", &self.sender())
            .field("server_version", &self.server_version)
            .field("client_version", &self.client_version())
            .field("request_type", &self.request_type())
            .field("response_code", &self.envelope.response_code())
            .finish()
    }
}

/// Installed request server marker.
///
/// Request servers are install-once because the underlying SKSE messaging
/// listener surface does not support removal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestServer {
    version: ApiVersion,
    ids: RequestMessageIds,
    sender: Option<CString>,
}

impl RequestServer {
    #[inline(always)]
    pub const fn builder(version: ApiVersion, ids: RequestMessageIds) -> RequestServerBuilder {
        RequestServerBuilder::new(version, ids)
    }

    #[inline(always)]
    pub const fn version(&self) -> ApiVersion {
        self.version
    }

    #[inline(always)]
    pub const fn message_ids(&self) -> RequestMessageIds {
        self.ids
    }

    #[inline(always)]
    pub fn sender(&self) -> Option<&CStr> {
        self.sender.as_deref()
    }
}

/// Builder for one versioned request / response protocol.
pub struct RequestServerBuilder {
    version: ApiVersion,
    ids: RequestMessageIds,
    sender: Option<CString>,
    enforce_version: bool,
    unknown_request: QueryResponse,
    handlers: Vec<RequestHandlerEntry>,
}

impl RequestServerBuilder {
    #[inline(always)]
    pub const fn new(version: ApiVersion, ids: RequestMessageIds) -> Self {
        Self {
            version,
            ids,
            sender: None,
            enforce_version: true,
            unknown_request: QueryResponse::UnsupportedRequest,
            handlers: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn with_sender(mut self, sender: &CStr) -> Self {
        self.sender = Some(sender.to_owned());
        self
    }

    #[inline(always)]
    pub fn with_sender_str(mut self, sender: &str) -> Result<Self, NulError> {
        self.sender = Some(CString::new(sender)?);
        Ok(self)
    }

    #[inline(always)]
    pub const fn with_version_check(mut self, enforce: bool) -> Self {
        self.enforce_version = enforce;
        self
    }

    #[inline(always)]
    pub const fn with_unknown_request_response(mut self, response: QueryResponse) -> Self {
        self.unknown_request = response;
        self
    }

    pub fn handle_raw<F>(mut self, request_type: u32, handler: F) -> Self
    where
        F: for<'a> FnMut(&mut QueryContext<'a>) -> QueryResponse + Send + 'static,
    {
        self.upsert_handler(request_type, Box::new(handler) as Box<dyn RequestHandler>);
        self
    }

    pub fn handle_value<TRequest, TResponse, F>(self, request_type: u32, mut handler: F) -> Self
    where
        TRequest: 'static,
        TResponse: Copy + 'static,
        F: for<'a> FnMut(&mut QueryContext<'a>, &'a TRequest) -> Result<TResponse, QueryResponse>
            + Send
            + 'static,
    {
        self.handle_raw(request_type, move |context| {
            let Some(request) = (unsafe { context.request_as::<TRequest>() }) else {
                return context.reject(QueryResponse::InvalidArguments);
            };

            match handler(context, request) {
                Ok(response) => unsafe { context.write_response(response) },
                Err(code) => context.reject(code),
            }
        })
    }

    pub fn handle_notify<TRequest, F>(self, request_type: u32, mut handler: F) -> Self
    where
        TRequest: 'static,
        F: for<'a> FnMut(&mut QueryContext<'a>, &'a TRequest) -> Result<(), QueryResponse>
            + Send
            + 'static,
    {
        self.handle_raw(request_type, move |context| {
            let Some(request) = (unsafe { context.request_as::<TRequest>() }) else {
                return context.reject(QueryResponse::InvalidArguments);
            };

            match handler(context, request) {
                Ok(()) => unsafe { context.write_response(()) },
                Err(code) => context.reject(code),
            }
        })
    }

    pub fn handle_array<TRequest, TResponse, F>(self, request_type: u32, mut handler: F) -> Self
    where
        TRequest: 'static,
        F: for<'a> FnMut(
                &mut QueryContext<'a>,
                &'a TRequest,
            ) -> Result<Vec<TResponse>, QueryResponse>
            + Send
            + 'static,
    {
        self.handle_raw(request_type, move |context| {
            let Some(request) = (unsafe { context.request_as::<TRequest>() }) else {
                return context.reject(QueryResponse::InvalidArguments);
            };

            match handler(context, request) {
                Ok(response) => unsafe { context.send_response_slice(response.as_slice()) },
                Err(code) => context.reject(code),
            }
        })
    }

    pub fn install(self) -> Result<RequestServer, ListenerInstallError> {
        let version_filter = self.version_filter();
        let query_filter = self.query_filter();
        let sender = self.sender.clone();
        let server_version = self.version;
        let ids = self.ids;
        let enforce_version = self.enforce_version;
        let unknown_request = self.unknown_request;
        let mut handlers = self.handlers;

        listen(version_filter, move |message| {
            let Some(handshake) = (unsafe { version_handshake_from_message_mut(message) }) else {
                return;
            };
            handshake.respond(server_version);
        })?;

        listen(query_filter, move |message| {
            let Some(envelope) = (unsafe { query_envelope_from_message_mut(message) }) else {
                return;
            };

            if enforce_version && !server_version.satisfies(envelope.version) {
                envelope.set_response_code(QueryResponse::VersionMismatch);
                return;
            }

            let Some(entry) = handlers
                .iter_mut()
                .find(|entry| entry.request_type == envelope.request_type)
            else {
                envelope.set_response_code(unknown_request);
                return;
            };

            let mut context = QueryContext::new(message, envelope, server_version);
            let response = entry.handler.handle(&mut context);
            context.envelope.set_response_code(response);
        })?;

        Ok(RequestServer {
            version: server_version,
            ids,
            sender,
        })
    }

    fn upsert_handler(&mut self, request_type: u32, handler: Box<dyn RequestHandler>) {
        if let Some(entry) = self
            .handlers
            .iter_mut()
            .find(|entry| entry.request_type == request_type)
        {
            entry.handler = handler;
        } else {
            self.handlers.push(RequestHandlerEntry {
                request_type,
                handler,
            });
        }
    }

    fn version_filter(&self) -> MessageFilter {
        self.filter_for(self.ids.version)
    }

    fn query_filter(&self) -> MessageFilter {
        self.filter_for(self.ids.query)
    }

    fn filter_for(&self, message_type: u32) -> MessageFilter {
        match &self.sender {
            Some(sender) => MessageFilter::for_type_sender(message_type, sender.as_c_str()),
            None => MessageFilter::for_type(message_type),
        }
    }
}

/// Install a routed listener for arbitrary inter-plugin messages.
pub fn listen<F>(filter: MessageFilter, mut callback: F) -> Result<(), ListenerInstallError>
where
    F: for<'a> FnMut(MessageRef<'a>) + Send + 'static,
{
    ensure_listener_installed()?;

    let wrapped = move |raw: &Message| callback(MessageRef::new(raw));
    unsafe {
        (*INTEROP_LISTENERS.get()).push(RegisteredListener {
            filter,
            callback: Box::new(wrapped),
        });
    }

    Ok(())
}

#[inline(always)]
pub fn listen_type<F>(message_type: u32, callback: F) -> Result<(), ListenerInstallError>
where
    F: for<'a> FnMut(MessageRef<'a>) + Send + 'static,
{
    listen(MessageFilter::for_type(message_type), callback)
}

#[inline(always)]
pub fn listen_sender<F>(sender: &CStr, callback: F) -> Result<(), ListenerInstallError>
where
    F: for<'a> FnMut(MessageRef<'a>) + Send + 'static,
{
    listen(MessageFilter::for_sender(sender), callback)
}

#[inline(always)]
pub fn listen_type_sender<F>(
    message_type: u32,
    sender: &CStr,
    callback: F,
) -> Result<(), ListenerInstallError>
where
    F: for<'a> FnMut(MessageRef<'a>) + Send + 'static,
{
    listen(
        MessageFilter::for_type_sender(message_type, sender),
        callback,
    )
}

/// Dispatch a raw plugin message to one receiver.
pub fn dispatch_raw(
    receiver: &CStr,
    message_type: u32,
    data: *mut c_void,
    data_len: usize,
) -> Result<(), DispatchError> {
    let Some(messaging) = (unsafe { skse::get_messaging_interface().as_ref() }) else {
        return Err(DispatchError::InterfaceUnavailable);
    };

    let data_len = checked_len_u32(data_len)?;
    if messaging.dispatch(message_type, data, data_len, receiver.as_ptr()) {
        Ok(())
    } else {
        Err(DispatchError::DispatchFailed)
    }
}

/// Dispatch a mutable value payload.
#[inline(always)]
pub fn dispatch_value<T>(
    receiver: &CStr,
    message_type: u32,
    value: &mut T,
) -> Result<(), DispatchError> {
    let (data, len) = value_ptr_and_len(value);
    dispatch_raw(receiver, message_type, data, len)
}

/// Dispatch a mutable slice payload.
#[inline(always)]
pub fn dispatch_slice<T>(
    receiver: &CStr,
    message_type: u32,
    values: &mut [T],
) -> Result<(), DispatchError> {
    let (data, len) = slice_ptr_and_len(values);
    dispatch_raw(receiver, message_type, data, len)
}

/// Dispatch one empty payload.
#[inline(always)]
pub fn dispatch_empty(receiver: &CStr, message_type: u32) -> Result<(), DispatchError> {
    dispatch_raw(receiver, message_type, core::ptr::null_mut(), 0)
}

/// # Safety
/// The caller must ensure the remote sender provided a valid mutable payload
/// buffer for `T`.
pub unsafe fn message_data_as_mut<'a, T>(message: MessageRef<'a>) -> Option<&'a mut T> {
    unsafe { payload_mut(message.data_ptr().cast(), message.data_len()) }
}

/// # Safety
/// The caller must ensure the remote sender provided a valid mutable payload
/// buffer for `VersionHandshake`.
#[inline(always)]
pub unsafe fn version_handshake_from_message_mut<'a>(
    message: MessageRef<'a>,
) -> Option<&'a mut VersionHandshake> {
    unsafe { message_data_as_mut::<VersionHandshake>(message) }
}

/// # Safety
/// The caller must ensure the remote sender provided a valid mutable payload
/// buffer for `QueryEnvelope`.
#[inline(always)]
pub unsafe fn query_envelope_from_message_mut<'a>(
    message: MessageRef<'a>,
) -> Option<&'a mut QueryEnvelope> {
    unsafe { message_data_as_mut::<QueryEnvelope>(message) }
}

fn ensure_listener_installed() -> Result<(), ListenerInstallError> {
    let mut installed = INTEROP_LISTENER_INSTALLED.lock();
    if *installed {
        return Ok(());
    }

    let Some(messaging) = (unsafe { skse::get_messaging_interface().as_ref() }) else {
        return Err(ListenerInstallError::InterfaceUnavailable);
    };

    if !messaging.register_listener_for(core::ptr::null(), interop_message_listener) {
        return Err(ListenerInstallError::RegisterFailed);
    }

    *installed = true;
    Ok(())
}

unsafe extern "system" fn interop_message_listener(message: *mut Message) {
    let Some(message) = (unsafe { message.as_ref() }) else {
        return;
    };

    let message_ref = MessageRef::new(message);
    for listener in unsafe { (*INTEROP_LISTENERS.get()).iter_mut() } {
        if listener.filter.matches(message_ref) {
            (listener.callback)(message);
        }
    }
}

struct ArrayResponseContext<F, T> {
    callback: F,
    was_invoked: bool,
    _marker: PhantomData<fn() -> T>,
}

unsafe extern "system" fn array_response_bridge<F, T>(
    user_ptr: *mut c_void,
    count: usize,
    data: *const c_void,
) where
    F: FnMut(&[T]),
{
    let Some(context) = (unsafe { (user_ptr as *mut ArrayResponseContext<F, T>).as_mut() }) else {
        return;
    };

    let payload = if count == 0 {
        &[]
    } else {
        let Some(data) = NonNullExt::new_const(data.cast::<T>()) else {
            return;
        };
        if !(data.as_ptr() as usize).is_multiple_of(align_of::<T>()) {
            return;
        }
        unsafe { slice::from_raw_parts(data.as_ptr(), count) }
    };

    (context.callback)(payload);
    context.was_invoked = true;
}

#[inline(always)]
fn checked_len_u32(len: usize) -> Result<u32, DispatchError> {
    u32::try_from(len).map_err(|_| DispatchError::DataTooLarge(len))
}

#[inline(always)]
fn value_ptr<T>(value: &T) -> *const c_void {
    if size_of::<T>() == 0 {
        core::ptr::null()
    } else {
        value as *const T as *const c_void
    }
}

#[inline(always)]
fn value_ptr_and_len<T>(value: &mut T) -> (*mut c_void, usize) {
    if size_of::<T>() == 0 {
        (core::ptr::null_mut(), 0)
    } else {
        (value as *mut T as *mut c_void, size_of::<T>())
    }
}

#[inline(always)]
fn maybe_uninit_ptr_and_len<T>(value: &mut MaybeUninit<T>) -> (*mut c_void, u32) {
    if size_of::<T>() == 0 {
        (core::ptr::null_mut(), 0)
    } else {
        (value.as_mut_ptr().cast(), size_of::<T>() as u32)
    }
}

#[inline(always)]
fn slice_ptr_and_len<T>(values: &mut [T]) -> (*mut c_void, usize) {
    if size_of::<T>() == 0 || values.is_empty() {
        (core::ptr::null_mut(), 0)
    } else {
        (
            values.as_mut_ptr().cast(),
            values.len().saturating_mul(size_of::<T>()),
        )
    }
}

unsafe fn payload_ref<'a, T>(raw: *const c_void, raw_len: usize) -> Option<&'a T> {
    if size_of::<T>() != raw_len {
        return None;
    }
    if size_of::<T>() == 0 {
        return Some(unsafe { &*core::ptr::NonNull::<T>::dangling().as_ptr() });
    }

    let raw = raw.cast::<T>();
    if raw.is_null() || !(raw as usize).is_multiple_of(align_of::<T>()) {
        return None;
    }

    unsafe { raw.as_ref() }
}

unsafe fn payload_mut<'a, T>(raw: *mut c_void, raw_len: usize) -> Option<&'a mut T> {
    if size_of::<T>() != raw_len {
        return None;
    }
    if size_of::<T>() == 0 {
        return Some(unsafe { &mut *core::ptr::NonNull::<T>::dangling().as_ptr() });
    }

    let raw = raw.cast::<T>();
    if raw.is_null() || !(raw as usize).is_multiple_of(align_of::<T>()) {
        return None;
    }

    unsafe { raw.as_mut() }
}

struct NonNullExt;

impl NonNullExt {
    #[inline(always)]
    fn new_const<T>(raw: *const T) -> Option<core::ptr::NonNull<T>> {
        core::ptr::NonNull::new(raw.cast_mut())
    }
}
