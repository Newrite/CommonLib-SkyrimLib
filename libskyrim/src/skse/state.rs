use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ffi::c_char;

use core_util::{Later, RacyCell};

use super::{InterfaceId, LoadInterface, Message, MessagingInterface, PluginHandle};

type DynMessageHandler = dyn FnMut(&Message) + 'static;

enum MessageHandler {
    Function(fn(&Message)),
    Closure(Box<DynMessageHandler>),
}

static MESSAGE_HANDLERS: RacyCell<[Vec<MessageHandler>; Message::SKSE_MAX]> =
    RacyCell::new([const { Vec::new() }; Message::SKSE_MAX]);
static PLUGIN_HANDLE: Later<PluginHandle> = Later::new();

#[inline(always)]
pub(crate) fn initialize_plugin_handle(handle: PluginHandle) {
    if !PLUGIN_HANDLE.is_init() {
        PLUGIN_HANDLE.init(handle);
    }
}

#[inline(always)]
pub fn plugin_handle() -> PluginHandle {
    *PLUGIN_HANDLE
}

pub fn register_listener(message_type: u32, callback: fn(&Message)) {
    assert!(message_type < Message::SKSE_MAX as u32);
    unsafe {
        (*MESSAGE_HANDLERS.get())[message_type as usize].push(MessageHandler::Function(callback));
    }
}

pub fn register_listener_dyn<F>(message_type: u32, callback: F)
where
    F: FnMut(&Message) + 'static,
{
    assert!(message_type < Message::SKSE_MAX as u32);
    unsafe {
        (*MESSAGE_HANDLERS.get())[message_type as usize]
            .push(MessageHandler::Closure(Box::new(callback)));
    }
}

pub(crate) fn initialize_messaging_listener(load_interface: &LoadInterface) -> bool {
    unsafe {
        let messaging = (load_interface.query_interface)(InterfaceId::Messaging as u32)
            as *mut MessagingInterface;
        if messaging.is_null() {
            crate::skse_fatal!(window, "Failed to acquire the SKSE messaging interface");
            return false;
        }

        if !((*messaging).register_listener)(
            plugin_handle(),
            b"SKSE\0".as_ptr().cast::<c_char>(),
            message_listener,
        ) {
            crate::skse_fatal!(window, "Failed to register the SKSE messaging listener");
            return false;
        }
    }

    true
}

unsafe extern "system" fn message_listener(message: *mut Message) {
    let Some(message) = (unsafe { message.as_ref() }) else {
        crate::skse_fatal!(window, "SKSE delivered a null messaging payload");
        return;
    };

    if message.msg_type >= Message::SKSE_MAX as u32 {
        return;
    }

    for callback in unsafe { (*MESSAGE_HANDLERS.get())[message.msg_type as usize].iter_mut() } {
        match callback {
            MessageHandler::Function(function) => function(message),
            MessageHandler::Closure(closure) => closure(message),
        }
    }
}
