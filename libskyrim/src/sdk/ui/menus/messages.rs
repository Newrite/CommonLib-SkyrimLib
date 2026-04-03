use alloc::ffi::CString;
use core::ffi::c_void;
use core::ptr::NonNull;

use crate::re::{
    BSFixedString, BSUIMessageData, BSUIMessageDataData, BSUIScaleformData, GFxEvent,
    IUIMessageData, UIMessageType,
};

use super::surface::message_queue;
use super::{NamedMenu, TypedMenuMessageData};

#[inline(always)]
pub fn process_commands() {
    unsafe { message_queue().with_mut_unchecked(crate::re::UIMessageQueue::process_commands) };
}

#[inline(always)]
fn menu_name_cstring(_caller: &str, menu_name: &str) -> Option<CString> {
    let Ok(menu_name) = CString::new(menu_name) else {
        crate::defensive_sdk_warn!(
            "sdk::ui::menus::{}() rejected menu name with interior NUL",
            _caller
        );
        return None;
    };

    Some(menu_name)
}

#[inline(always)]
fn message_data_class_name<T>() -> Option<CString>
where
    T: TypedMenuMessageData,
{
    let Ok(class_name) = CString::new(T::CLASS_NAME) else {
        crate::defensive_sdk_error!(
            "sdk::ui::menus::create_message_data<{}>() rejected class name with interior NUL",
            core::any::type_name::<T>()
        );
        return None;
    };

    Some(class_name)
}

/// # Safety
/// `data` must either be null or point to a live `IUIMessageData` instance
/// suitable for the target menu and owned according to the engine's UI message
/// contract.
pub unsafe fn queue_message_data_unchecked(
    menu_name: &str,
    message_type: UIMessageType,
    data: *mut IUIMessageData,
) {
    let Some(menu_name) = menu_name_cstring("queue_message_data_unchecked", menu_name) else {
        return;
    };

    unsafe {
        crate::ffi::commonlib_ui_message_queue_add_message(
            menu_name.as_ptr(),
            message_type as i32,
            data.cast(),
        );
    }
}

#[inline(always)]
fn create_message_data<T>() -> Option<NonNull<T>>
where
    T: TypedMenuMessageData,
{
    let class_name = message_data_class_name::<T>()?;

    let message_data = NonNull::new(unsafe {
        crate::ffi::commonlib_create_ui_message_data(class_name.as_ptr()).cast()
    });

    if message_data.is_none() {
        crate::defensive_sdk_warn!(
            "sdk::ui::menus::create_message_data<{}>() failed to allocate engine UI message data",
            core::any::type_name::<T>()
        );
    }

    message_data
}

pub fn queue_message_with<T>(
    menu_name: &str,
    message_type: UIMessageType,
    init: impl FnOnce(&mut T),
) -> bool
where
    T: TypedMenuMessageData,
{
    let Some(mut data) = create_message_data::<T>() else {
        return false;
    };

    unsafe { init(data.as_mut()) };

    let Some(menu_name) = menu_name_cstring(core::any::type_name::<T>(), menu_name) else {
        return false;
    };

    unsafe {
        crate::ffi::commonlib_ui_message_queue_add_message(
            menu_name.as_ptr(),
            message_type as i32,
            data.as_ptr().cast(),
        )
    }
}

#[inline(always)]
pub fn queue_named_message_with<M, T>(
    message_type: UIMessageType,
    init: impl FnOnce(&mut T),
) -> bool
where
    M: NamedMenu,
    T: TypedMenuMessageData,
{
    queue_message_with::<T>(M::MENU_NAME, message_type, init)
}

#[inline(always)]
pub fn queue_message(menu_name: &str, message_type: UIMessageType) {
    unsafe { queue_message_data_unchecked(menu_name, message_type, core::ptr::null_mut()) };
}

#[inline(always)]
pub fn open_menu(menu_name: &str) {
    queue_message(menu_name, UIMessageType::Show);
}

#[inline(always)]
pub fn close_menu(menu_name: &str) {
    queue_message(menu_name, UIMessageType::Hide);
}

#[inline(always)]
pub fn force_close_menu(menu_name: &str) {
    queue_message(menu_name, UIMessageType::ForceHide);
}

#[inline(always)]
pub fn reshow_menu(menu_name: &str) {
    queue_message(menu_name, UIMessageType::Reshow);
}

#[inline(always)]
pub fn toggle_menu(menu_name: &str, open: bool) {
    if open {
        open_menu(menu_name);
    } else {
        close_menu(menu_name);
    }
}

#[inline(always)]
pub fn open_named_menu<M>()
where
    M: NamedMenu,
{
    open_menu(M::MENU_NAME)
}

#[inline(always)]
pub fn close_named_menu<M>()
where
    M: NamedMenu,
{
    close_menu(M::MENU_NAME)
}

#[inline(always)]
pub fn force_close_named_menu<M>()
where
    M: NamedMenu,
{
    force_close_menu(M::MENU_NAME)
}

#[inline(always)]
pub fn toggle_named_menu<M>(open: bool)
where
    M: NamedMenu,
{
    toggle_menu(M::MENU_NAME, open)
}

#[inline(always)]
fn queue_bsui_message(
    _caller: &str,
    menu_name: &str,
    message_type: UIMessageType,
    init: impl FnOnce(&mut BSUIMessageData),
) -> bool {
    let queued = queue_message_with::<BSUIMessageData>(menu_name, message_type, init);

    if !queued {
        crate::defensive_sdk_warn!(
            "sdk::ui::menus::{}() failed for menu '{}'",
            _caller,
            menu_name
        );
    }

    queued
}

#[inline(always)]
pub fn queue_bsui_bool_message(menu_name: &str, message_type: UIMessageType, data: bool) -> bool {
    queue_bsui_message(
        "queue_bsui_bool_message",
        menu_name,
        message_type,
        |msg_data| {
            msg_data.data = BSUIMessageDataData { b: data };
        },
    )
}

#[inline(always)]
pub fn queue_bsui_uint_message(menu_name: &str, message_type: UIMessageType, data: u32) -> bool {
    queue_bsui_message(
        "queue_bsui_uint_message",
        menu_name,
        message_type,
        |msg_data| {
            msg_data.data = BSUIMessageDataData { u: data };
        },
    )
}

#[inline(always)]
pub fn queue_bsui_ptr_message(
    menu_name: &str,
    message_type: UIMessageType,
    data: *mut c_void,
) -> bool {
    queue_bsui_message(
        "queue_bsui_ptr_message",
        menu_name,
        message_type,
        |msg_data| {
            msg_data.data = BSUIMessageDataData { p: data };
        },
    )
}

#[inline(always)]
pub fn queue_bsui_string_message(
    menu_name: &str,
    message_type: UIMessageType,
    value: &str,
) -> bool {
    queue_bsui_message(
        "queue_bsui_string_message",
        menu_name,
        message_type,
        |msg_data| {
            msg_data.fixed_str = BSFixedString::from_str(value);
        },
    )
}

#[inline(always)]
pub fn queue_bsui_string_bool_message(
    menu_name: &str,
    message_type: UIMessageType,
    value: &str,
    data: bool,
) -> bool {
    queue_bsui_message(
        "queue_bsui_string_bool_message",
        menu_name,
        message_type,
        |msg_data| {
            msg_data.fixed_str = BSFixedString::from_str(value);
            msg_data.data = BSUIMessageDataData { b: data };
        },
    )
}

#[inline(always)]
pub fn queue_bsui_string_float_message(
    menu_name: &str,
    message_type: UIMessageType,
    value: &str,
    data: f32,
) -> bool {
    queue_bsui_message(
        "queue_bsui_string_float_message",
        menu_name,
        message_type,
        |msg_data| {
            msg_data.fixed_str = BSFixedString::from_str(value);
            msg_data.data = BSUIMessageDataData { f: data };
        },
    )
}

#[inline(always)]
pub fn queue_bsui_string_uint_message(
    menu_name: &str,
    message_type: UIMessageType,
    value: &str,
    data: u32,
) -> bool {
    queue_bsui_message(
        "queue_bsui_string_uint_message",
        menu_name,
        message_type,
        |msg_data| {
            msg_data.fixed_str = BSFixedString::from_str(value);
            msg_data.data = BSUIMessageDataData { u: data };
        },
    )
}

#[inline(always)]
pub fn queue_scaleform_event_message(
    menu_name: &str,
    message_type: UIMessageType,
    scaleform_event: *mut GFxEvent,
) -> bool {
    let queued = queue_message_with::<BSUIScaleformData>(menu_name, message_type, |msg_data| {
        msg_data.scaleform_event = scaleform_event;
    });

    if !queued {
        crate::defensive_sdk_warn!(
            "sdk::ui::menus::queue_scaleform_event_message() failed for menu '{}'",
            menu_name
        );
    }

    queued
}
