use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ffi::{CStr, c_void};

use core_util::RacyCell;
use spin::Mutex;

use crate::sdk::events::skse::messages::MessageRef;
use crate::skse::{self, Message};

use super::payload::{checked_len_u32, slice_ptr_and_len, value_ptr_and_len};
use super::types::{DispatchError, ListenerInstallError, MessageFilter};

type DynInteropHandler = dyn FnMut(&Message) + Send + 'static;

struct RegisteredListener {
    filter: MessageFilter,
    callback: Box<DynInteropHandler>,
}

static INTEROP_LISTENERS: RacyCell<Vec<RegisteredListener>> = RacyCell::new(Vec::new());
static INTEROP_LISTENER_INSTALLED: Mutex<bool> = Mutex::new(false);

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

pub(crate) fn ensure_listener_installed() -> Result<(), ListenerInstallError> {
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
    crate::skse::crash::guard("sdk::interop message listener", || {
        let Some(message) = (unsafe { message.as_ref() }) else {
            return;
        };

        let message_ref = MessageRef::new(message);
        for listener in unsafe { (*INTEROP_LISTENERS.get()).iter_mut() } {
            if listener.filter.matches(message_ref) {
                (listener.callback)(message);
            }
        }
    });
}
