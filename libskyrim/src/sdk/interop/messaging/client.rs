use alloc::borrow::ToOwned;
use alloc::ffi::{CString, NulError};
use core::ffi::CStr;
use core::fmt;
use core::marker::PhantomData;
use core::mem::{MaybeUninit, size_of};

use super::listener::dispatch_value;
use super::payload::{
    ArrayResponseContext, QueryEnvelope, array_response_bridge, checked_len_u32,
    maybe_uninit_ptr_and_len, value_ptr,
};
use super::types::{ApiVersion, DispatchError, QueryResponse, RequestMessageIds, VersionHandshake};

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
