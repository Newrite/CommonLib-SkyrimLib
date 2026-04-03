use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::{MaybeUninit, align_of, size_of};
use core::slice;

use crate::sdk::events::skse::messages::MessageRef;

use super::types::{ApiVersion, DispatchError, QueryResponse, VersionHandshake};

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
    pub(crate) const fn new(
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

#[inline(always)]
pub(crate) fn checked_len_u32(len: usize) -> Result<u32, DispatchError> {
    u32::try_from(len).map_err(|_| DispatchError::DataTooLarge(len))
}

#[inline(always)]
pub(crate) fn value_ptr<T>(value: &T) -> *const c_void {
    if size_of::<T>() == 0 {
        core::ptr::null()
    } else {
        value as *const T as *const c_void
    }
}

#[inline(always)]
pub(crate) fn value_ptr_and_len<T>(value: &mut T) -> (*mut c_void, usize) {
    if size_of::<T>() == 0 {
        (core::ptr::null_mut(), 0)
    } else {
        (value as *mut T as *mut c_void, size_of::<T>())
    }
}

#[inline(always)]
pub(crate) fn maybe_uninit_ptr_and_len<T>(value: &mut MaybeUninit<T>) -> (*mut c_void, u32) {
    if size_of::<T>() == 0 {
        (core::ptr::null_mut(), 0)
    } else {
        (value.as_mut_ptr().cast(), size_of::<T>() as u32)
    }
}

#[inline(always)]
pub(crate) fn slice_ptr_and_len<T>(values: &mut [T]) -> (*mut c_void, usize) {
    if size_of::<T>() == 0 || values.is_empty() {
        (core::ptr::null_mut(), 0)
    } else {
        (
            values.as_mut_ptr().cast(),
            values.len().saturating_mul(size_of::<T>()),
        )
    }
}

pub(crate) struct ArrayResponseContext<F, T> {
    pub callback: F,
    pub was_invoked: bool,
    pub _marker: PhantomData<fn() -> T>,
}

pub(crate) unsafe extern "system" fn array_response_bridge<F, T>(
    user_ptr: *mut c_void,
    count: usize,
    data: *const c_void,
) where
    F: FnMut(&[T]),
{
    crate::skse::crash::guard("sdk::interop array response bridge", || {
        let Some(context) = (unsafe { (user_ptr as *mut ArrayResponseContext<F, T>).as_mut() })
        else {
            return;
        };

        let payload = if count == 0 {
            &[]
        } else {
            let Some(data) = core::ptr::NonNull::new(data.cast_mut().cast::<T>()) else {
                return;
            };
            if !(data.as_ptr() as usize).is_multiple_of(align_of::<T>()) {
                return;
            }
            unsafe { slice::from_raw_parts(data.as_ptr(), count) }
        };

        (context.callback)(payload);
        context.was_invoked = true;
    });
}

pub(crate) unsafe fn payload_ref<'a, T>(raw: *const c_void, raw_len: usize) -> Option<&'a T> {
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

pub(crate) unsafe fn payload_mut<'a, T>(raw: *mut c_void, raw_len: usize) -> Option<&'a mut T> {
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
