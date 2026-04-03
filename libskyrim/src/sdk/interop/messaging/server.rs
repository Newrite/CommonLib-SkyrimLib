use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::ffi::{CString, NulError};
use alloc::vec::Vec;
use core::ffi::CStr;
use core::fmt;

use crate::sdk::events::skse::messages::MessageRef;

use super::listener::listen;
use super::payload::{
    QueryEnvelope, payload_mut, payload_ref, query_envelope_from_message_mut,
    version_handshake_from_message_mut,
};
use super::types::{
    ApiVersion, ListenerInstallError, MessageFilter, QueryResponse, RequestMessageIds,
};

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
