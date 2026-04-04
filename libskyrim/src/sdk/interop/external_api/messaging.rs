use alloc::ffi::{CString, NulError};
use core::ffi::{CStr, c_void};
use core::fmt;
use core::ptr::NonNull;

use crate::sdk::events::skse::messages::MessageRef;
use crate::sdk::interop::messaging::{
    self as plugin_messaging, DispatchError, ListenerInstallError, message_data_as_mut,
};

/// Loader-command kind used by messaging-driven external API negotiation.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceLoaderCommandKind {
    RequestInterface = 0,
}

/// One messaging command envelope used to request an interface.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterfaceLoaderCommand {
    pub header: u32,
    pub kind: InterfaceLoaderCommandKind,
    pub payload: *mut c_void,
}

impl InterfaceLoaderCommand {
    #[inline(always)]
    pub const fn new(header: u32, kind: InterfaceLoaderCommandKind, payload: *mut c_void) -> Self {
        Self {
            header,
            kind,
            payload,
        }
    }

    #[inline(always)]
    pub fn request<V>(header: u32, request: &mut InterfaceLoaderRequest<V>) -> Self {
        Self::new(
            header,
            InterfaceLoaderCommandKind::RequestInterface,
            request as *mut InterfaceLoaderRequest<V> as *mut c_void,
        )
    }

    #[inline(always)]
    pub const fn matches(self, header: u32, kind: InterfaceLoaderCommandKind) -> bool {
        self.header == header && self.kind as u8 == kind as u8
    }

    /// # Safety
    /// The caller must ensure that `payload` points to a live `T` owned by the
    /// sender for the duration of dispatch.
    #[inline(always)]
    pub unsafe fn payload_as_mut<'a, T>(self) -> Option<&'a mut T> {
        unsafe { payload_ptr_as_mut(self.payload) }
    }
}

/// One typed request for a versioned external interface.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterfaceLoaderRequest<V> {
    pub version: V,
}

impl<V> InterfaceLoaderRequest<V> {
    #[inline(always)]
    pub const fn new(version: V) -> Self {
        Self { version }
    }
}

/// Response kind used by messaging-driven external API negotiation.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceLoaderResponseKind {
    Error = 0,
    InterfaceProvider = 1,
}

/// One messaging response envelope used to return an interface.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterfaceLoaderResponse {
    pub kind: InterfaceLoaderResponseKind,
    pub payload: *mut c_void,
}

impl InterfaceLoaderResponse {
    #[inline(always)]
    pub const fn new(kind: InterfaceLoaderResponseKind, payload: *mut c_void) -> Self {
        Self { kind, payload }
    }

    #[inline(always)]
    pub const fn error() -> Self {
        Self::new(InterfaceLoaderResponseKind::Error, core::ptr::null_mut())
    }

    #[inline(always)]
    pub fn provider<V>(provider: &mut InterfaceProvider<V>) -> Self {
        Self::new(
            InterfaceLoaderResponseKind::InterfaceProvider,
            provider as *mut InterfaceProvider<V> as *mut c_void,
        )
    }

    /// # Safety
    /// The caller must ensure that `payload` points to a live
    /// `InterfaceProvider<V>` owned by the sender for the duration of dispatch.
    #[inline(always)]
    pub unsafe fn provider_payload<V>(self) -> Option<InterfaceProvider<V>>
    where
        V: Copy,
    {
        unsafe { payload_ptr_as_ref::<InterfaceProvider<V>>(self.payload.cast_const()) }.copied()
    }
}

/// Provider payload returned by one messaging-driven interface response.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterfaceProvider<V> {
    pub interface: *mut c_void,
    pub version: V,
}

impl<V> InterfaceProvider<V> {
    #[inline(always)]
    pub const fn from_raw(version: V, interface: *mut c_void) -> Self {
        Self { interface, version }
    }

    #[inline(always)]
    pub fn new<T>(version: V, interface: &'static T) -> Self {
        Self::from_raw(version, (interface as *const T).cast_mut().cast())
    }

    #[inline(always)]
    pub const fn is_available(&self) -> bool {
        !self.interface.is_null()
    }

    #[inline(always)]
    pub fn cast<T>(&self) -> Option<NonNull<T>> {
        NonNull::new(self.interface.cast())
    }
}

/// Parsed payload from one messaging-driven interface response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceLoaderResponsePayload<V> {
    Error,
    InterfaceProvider(InterfaceProvider<V>),
}

/// Error replying to one incoming interface-loader message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceLoaderReplyError {
    MissingSender,
    Dispatch(DispatchError),
}

impl fmt::Display for InterfaceLoaderReplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSender => write!(f, "incoming message does not expose a sender name"),
            Self::Dispatch(error) => write!(f, "{}", error),
        }
    }
}

impl core::error::Error for InterfaceLoaderReplyError {}

impl From<DispatchError> for InterfaceLoaderReplyError {
    #[inline(always)]
    fn from(value: DispatchError) -> Self {
        Self::Dispatch(value)
    }
}

/// Dispatch one interface-loader request envelope to a receiver plugin.
pub fn dispatch_interface_loader_request<V>(
    receiver: &CStr,
    message_type: u32,
    header: u32,
    version: V,
) -> Result<(), DispatchError> {
    let mut request = InterfaceLoaderRequest::new(version);
    let mut command = InterfaceLoaderCommand::request(header, &mut request);
    plugin_messaging::dispatch_value(receiver, message_type, &mut command)
}

/// Dispatch one interface-loader request envelope to a receiver plugin by
/// sender string.
pub fn dispatch_interface_loader_request_str<V>(
    receiver: &str,
    message_type: u32,
    header: u32,
    version: V,
) -> Result<(), NulErrorOrDispatchError> {
    let receiver = CString::new(receiver)?;
    dispatch_interface_loader_request(receiver.as_c_str(), message_type, header, version)
        .map_err(Into::into)
}

/// Dispatch one provider response back to a waiting plugin.
pub fn dispatch_interface_loader_response<V>(
    receiver: &CStr,
    message_type: u32,
    provider: InterfaceProvider<V>,
) -> Result<(), DispatchError>
where
    V: Copy,
{
    let mut provider = provider;
    let mut response = InterfaceLoaderResponse::provider(&mut provider);
    plugin_messaging::dispatch_value(receiver, message_type, &mut response)
}

/// Dispatch one provider response using a plugin-owned exported interface table.
pub fn dispatch_interface_loader_exported_response<T, V>(
    receiver: &CStr,
    message_type: u32,
    version: V,
    interface: &'static T,
) -> Result<(), DispatchError>
where
    V: Copy,
{
    dispatch_interface_loader_response(
        receiver,
        message_type,
        InterfaceProvider::new(version, interface),
    )
}

/// Dispatch an error response back to a waiting plugin.
pub fn dispatch_interface_loader_error(
    receiver: &CStr,
    message_type: u32,
) -> Result<(), DispatchError> {
    let mut response = InterfaceLoaderResponse::error();
    plugin_messaging::dispatch_value(receiver, message_type, &mut response)
}

/// Reply to an incoming interface-loader request with one exported interface.
pub fn reply_interface_loader_exported_response<T, V>(
    request_message: MessageRef<'_>,
    message_type: u32,
    version: V,
    interface: &'static T,
) -> Result<(), InterfaceLoaderReplyError>
where
    V: Copy,
{
    let sender = request_message
        .sender_cstr()
        .ok_or(InterfaceLoaderReplyError::MissingSender)?;
    dispatch_interface_loader_exported_response(sender, message_type, version, interface)
        .map_err(Into::into)
}

/// Reply to an incoming interface-loader request with an error response.
pub fn reply_interface_loader_error(
    request_message: MessageRef<'_>,
    message_type: u32,
) -> Result<(), InterfaceLoaderReplyError> {
    let sender = request_message
        .sender_cstr()
        .ok_or(InterfaceLoaderReplyError::MissingSender)?;
    dispatch_interface_loader_error(sender, message_type).map_err(Into::into)
}

/// Install a routed listener for one messaging-driven loader response.
pub fn listen_interface_loader_response<V, F>(
    message_type: u32,
    sender: &CStr,
    mut callback: F,
) -> Result<(), ListenerInstallError>
where
    V: Copy + Send + 'static,
    F: FnMut(InterfaceLoaderResponsePayload<V>) + Send + 'static,
{
    plugin_messaging::listen_type_sender(message_type, sender, move |message| {
        let Some(response) = (unsafe { interface_loader_response_from_message::<V>(message) })
        else {
            return;
        };
        callback(response);
    })
}

/// Install a routed listener for one messaging-driven loader response using a
/// sender string.
pub fn listen_interface_loader_response_str<V, F>(
    message_type: u32,
    sender: &str,
    callback: F,
) -> Result<(), NulErrorOrListenerInstallError>
where
    V: Copy + Send + 'static,
    F: FnMut(InterfaceLoaderResponsePayload<V>) + Send + 'static,
{
    let sender = CString::new(sender)?;
    listen_interface_loader_response(message_type, sender.as_c_str(), callback).map_err(Into::into)
}

/// # Safety
/// The caller must ensure that the sender encoded the loader command envelope
/// honestly and that the nested request pointer is valid for `V`.
pub unsafe fn interface_loader_request_from_message_mut<'a, V>(
    message: MessageRef<'a>,
    expected_header: u32,
) -> Option<&'a mut InterfaceLoaderRequest<V>> {
    let command = unsafe { message_data_as_mut::<InterfaceLoaderCommand>(message) }?;
    if !command.matches(
        expected_header,
        InterfaceLoaderCommandKind::RequestInterface,
    ) {
        return None;
    }

    unsafe { command.payload_as_mut::<InterfaceLoaderRequest<V>>() }
}

/// # Safety
/// The caller must ensure that the sender encoded the loader response envelope
/// honestly and that the nested payload pointer is valid for `V`.
pub unsafe fn interface_loader_response_from_message<V>(
    message: MessageRef<'_>,
) -> Option<InterfaceLoaderResponsePayload<V>>
where
    V: Copy,
{
    let response = unsafe { message_data_as_mut::<InterfaceLoaderResponse>(message) }?;
    match response.kind {
        InterfaceLoaderResponseKind::Error => Some(InterfaceLoaderResponsePayload::Error),
        InterfaceLoaderResponseKind::InterfaceProvider => unsafe {
            response
                .provider_payload::<V>()
                .map(InterfaceLoaderResponsePayload::InterfaceProvider)
        },
    }
}

/// Combined failure returned by string-based request helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NulErrorOrDispatchError {
    Nul(NulError),
    Dispatch(DispatchError),
}

impl fmt::Display for NulErrorOrDispatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nul(error) => write!(f, "{}", error),
            Self::Dispatch(error) => write!(f, "{}", error),
        }
    }
}

impl core::error::Error for NulErrorOrDispatchError {}

impl From<NulError> for NulErrorOrDispatchError {
    #[inline(always)]
    fn from(value: NulError) -> Self {
        Self::Nul(value)
    }
}

impl From<DispatchError> for NulErrorOrDispatchError {
    #[inline(always)]
    fn from(value: DispatchError) -> Self {
        Self::Dispatch(value)
    }
}

/// Combined failure returned by string-based listener helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NulErrorOrListenerInstallError {
    Nul(NulError),
    ListenerInstall(ListenerInstallError),
}

impl fmt::Display for NulErrorOrListenerInstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nul(error) => write!(f, "{}", error),
            Self::ListenerInstall(error) => write!(f, "{}", error),
        }
    }
}

impl core::error::Error for NulErrorOrListenerInstallError {}

impl From<NulError> for NulErrorOrListenerInstallError {
    #[inline(always)]
    fn from(value: NulError) -> Self {
        Self::Nul(value)
    }
}

impl From<ListenerInstallError> for NulErrorOrListenerInstallError {
    #[inline(always)]
    fn from(value: ListenerInstallError) -> Self {
        Self::ListenerInstall(value)
    }
}

unsafe fn payload_ptr_as_ref<'a, T>(raw: *const c_void) -> Option<&'a T> {
    let ptr = NonNull::new(raw.cast_mut().cast::<T>())?;
    if !(ptr.as_ptr() as usize).is_multiple_of(core::mem::align_of::<T>()) {
        return None;
    }

    Some(unsafe { &*ptr.as_ptr() })
}

unsafe fn payload_ptr_as_mut<'a, T>(raw: *mut c_void) -> Option<&'a mut T> {
    let ptr = NonNull::new(raw.cast::<T>())?;
    if !(ptr.as_ptr() as usize).is_multiple_of(core::mem::align_of::<T>()) {
        return None;
    }

    Some(unsafe { &mut *ptr.as_ptr() })
}
