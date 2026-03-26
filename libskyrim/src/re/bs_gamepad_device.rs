use crate::offsets::offsets_rtti::RTTI_BSGamepadDevice;
use crate::offsets::offsets_vtable::VTABLE_BSGamepadDevice;
use crate::re::{BSGamepadEvent, BSInputDevice, BSTEventSink, BSTEventSource};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSGamepadDevice`
#[repr(C)]
pub struct BSGamepadDevice {
    pub base: BSInputDevice,                                  // 00
    pub gamepad_event_source: BSTEventSource<BSGamepadEvent>, // 70
    pub user_index: i32,                                      // C8
    pub connected: bool,                                      // CC
    pub listening_for_input: bool,                            // CD
    pub padce: u16,                                           // CE
}

const _: () = assert!(core::mem::size_of::<BSGamepadDevice>() == 0xD0);
const _: () = assert!(core::mem::offset_of!(BSGamepadDevice, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSGamepadDevice, gamepad_event_source) == 0x70);
const _: () = assert!(core::mem::offset_of!(BSGamepadDevice, user_index) == 0xC8);

impl RttiType for BSGamepadDevice {
    const RTTI: VariantID = RTTI_BSGamepadDevice;
}

core_util::inherit!(BSGamepadDevice : BSInputDevice, base);
core_util::inherit!(BSGamepadDevice => BSTEventSource<BSGamepadEvent>, gamepad_event_source);

impl BSGamepadDevice {
    pub const RTTI: VariantID = RTTI_BSGamepadDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSGamepadDevice;

    crate::virtual_method! {
        pub const VFUNC_SET_VIBRATION: usize = 0x09;
        pub fn set_vibration(&mut self, large_motor: f32, small_motor: f32)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_DEVICE_LIGHT: usize = 0x0A;
        pub fn set_device_light(&mut self, rgb: &[u32; 3])
    }

    crate::virtual_method! {
        pub const VFUNC_RESET_DEVICE_LIGHT: usize = 0x0B;
        pub fn reset_device_light(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_IS_REMOTE_CONTROLLER: usize = 0x0C;
        pub fn is_remote_controller(&mut self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_NORMALIZE_THUMBSTICK_VALUE: usize = 0x0D;
        pub fn normalize_thumbstick_value(
            &mut self,
            thumb_x: i32,
            thumb_y: i32,
            x_out: &mut f32,
            y_out: &mut f32
        )
    }

    crate::virtual_method! {
        pub const VFUNC_DO_ENABLE_LISTENING_MODE: usize = 0x0E;
        pub fn do_enable_listening_mode(&mut self)
    }

    #[inline(always)]
    pub unsafe fn add_gamepad_event_sink(&mut self, sink: *mut BSTEventSink<BSGamepadEvent>) {
        unsafe { self.gamepad_event_source.add_event_sink(sink) }
    }

    #[inline(always)]
    pub unsafe fn remove_gamepad_event_sink(&mut self, sink: *mut BSTEventSink<BSGamepadEvent>) {
        unsafe { self.gamepad_event_source.remove_event_sink(sink) }
    }
}

pub trait BSGamepadDeviceExt {
    fn set_vibration(&mut self, large_motor: f32, small_motor: f32);
    fn set_device_light(&mut self, rgb: &[u32; 3]);
    fn reset_device_light(&mut self);
    fn is_remote_controller(&mut self) -> bool;
    fn normalize_thumbstick_value(
        &mut self,
        thumb_x: i32,
        thumb_y: i32,
        x_out: &mut f32,
        y_out: &mut f32,
    );
    fn do_enable_listening_mode(&mut self);
    unsafe fn add_gamepad_event_sink(&mut self, sink: *mut BSTEventSink<BSGamepadEvent>);
    unsafe fn remove_gamepad_event_sink(&mut self, sink: *mut BSTEventSink<BSGamepadEvent>);
}

impl<T> BSGamepadDeviceExt for T
where
    T: AsRef<BSGamepadDevice> + AsMut<BSGamepadDevice>,
{
    #[inline(always)]
    fn set_vibration(&mut self, large_motor: f32, small_motor: f32) {
        BSGamepadDevice::set_vibration(self.as_mut(), large_motor, small_motor)
    }

    #[inline(always)]
    fn set_device_light(&mut self, rgb: &[u32; 3]) {
        BSGamepadDevice::set_device_light(self.as_mut(), rgb)
    }

    #[inline(always)]
    fn reset_device_light(&mut self) {
        BSGamepadDevice::reset_device_light(self.as_mut())
    }

    #[inline(always)]
    fn is_remote_controller(&mut self) -> bool {
        BSGamepadDevice::is_remote_controller(self.as_mut())
    }

    #[inline(always)]
    fn normalize_thumbstick_value(
        &mut self,
        thumb_x: i32,
        thumb_y: i32,
        x_out: &mut f32,
        y_out: &mut f32,
    ) {
        BSGamepadDevice::normalize_thumbstick_value(self.as_mut(), thumb_x, thumb_y, x_out, y_out)
    }

    #[inline(always)]
    fn do_enable_listening_mode(&mut self) {
        BSGamepadDevice::do_enable_listening_mode(self.as_mut())
    }

    #[inline(always)]
    unsafe fn add_gamepad_event_sink(&mut self, sink: *mut BSTEventSink<BSGamepadEvent>) {
        unsafe { BSGamepadDevice::add_gamepad_event_sink(self.as_mut(), sink) }
    }

    #[inline(always)]
    unsafe fn remove_gamepad_event_sink(&mut self, sink: *mut BSTEventSink<BSGamepadEvent>) {
        unsafe { BSGamepadDevice::remove_gamepad_event_sink(self.as_mut(), sink) }
    }
}
