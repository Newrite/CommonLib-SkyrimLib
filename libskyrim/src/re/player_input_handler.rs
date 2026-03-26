use crate::offsets::offsets_rtti::RTTI_PlayerInputHandler;
use crate::offsets::offsets_vtable::VTABLE_PlayerInputHandler;
use crate::re::{
    BSFixedString, ButtonEvent, InputEvent, MouseMoveEvent, PlayerControlsData, ThumbstickEvent,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::PlayerInputHandler`
#[repr(C)]
pub struct PlayerInputHandler {
    pub vtable: *const usize,               // 00
    pub input_event_handling_enabled: bool, // 08
    pub pad09: u8,                          // 09
    pub pad0a: u16,                         // 0A
    pub pad0c: u32,                         // 0C
}

const _: () = assert!(core::mem::size_of::<PlayerInputHandler>() == 0x10);
const _: () =
    assert!(core::mem::offset_of!(PlayerInputHandler, input_event_handling_enabled) == 0x08);

impl RttiType for PlayerInputHandler {
    const RTTI: VariantID = RTTI_PlayerInputHandler;
}

impl PlayerInputHandler {
    pub const RTTI: VariantID = RTTI_PlayerInputHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PlayerInputHandler;

    pub const VR_UNK10_OFFSET: VariantOffset = VariantOffset::new_se_ae(0x0, 0x10);
    pub const VR_UNK18_OFFSET: VariantOffset = VariantOffset::new_se_ae(0x0, 0x18);
    pub const VR_UNK20_OFFSET: VariantOffset = VariantOffset::new_se_ae(0x0, 0x20);

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_CAN_PROCESS: usize = 0x01;
        pub fn can_process(event: *mut InputEvent) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_PROCESS_THUMBSTICK: usize = 0x02;
        pub fn process_thumbstick(event: *mut ThumbstickEvent, data: *mut PlayerControlsData)
    }

    crate::virtual_method! {
        pub const VFUNC_PROCESS_MOUSE_MOVE: usize = 0x03;
        pub fn process_mouse_move(event: *mut MouseMoveEvent, data: *mut PlayerControlsData)
    }

    crate::virtual_method! {
        pub const VFUNC_PROCESS_BUTTON: usize = 0x04;
        pub fn process_button(event: *mut ButtonEvent, data: *mut PlayerControlsData)
    }

    #[inline(always)]
    pub fn unk_05(&mut self) {
        crate::runtime::require_vr("PlayerInputHandler::unk_05");
        crate::relocate_virtual!(
            extern "C" fn(*mut Self),
            self as *mut Self,
            VariantOffset::new_se_ae(0x0, 0x05)
        )
    }

    #[inline(always)]
    pub fn unk_06(&mut self) {
        crate::runtime::require_vr("PlayerInputHandler::unk_06");
        crate::relocate_virtual!(
            extern "C" fn(*mut Self),
            self as *mut Self,
            VariantOffset::new_se_ae(0x0, 0x06)
        )
    }

    #[inline(always)]
    pub const fn is_input_event_handling_enabled(&self) -> bool {
        self.input_event_handling_enabled
    }

    #[inline(always)]
    pub fn set_input_event_handling_enabled(&mut self, enabled: bool) {
        self.input_event_handling_enabled = enabled;
    }

    crate::runtime_data_accessor! {
        pub fn vr_unk10() -> u64 {
            offset: Self::VR_UNK10_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        pub fn vr_unk18() -> BSFixedString {
            offset: Self::VR_UNK18_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        pub fn vr_unk20() -> u64 {
            offset: Self::VR_UNK20_OFFSET
        }
    }
}

pub trait PlayerInputHandlerExt {
    fn can_process(&self, event: *mut InputEvent) -> bool;
    fn process_thumbstick(&mut self, event: *mut ThumbstickEvent, data: *mut PlayerControlsData);
    fn process_mouse_move(&mut self, event: *mut MouseMoveEvent, data: *mut PlayerControlsData);
    fn process_button(&mut self, event: *mut ButtonEvent, data: *mut PlayerControlsData);
    fn unk_05(&mut self);
    fn unk_06(&mut self);
    fn is_input_event_handling_enabled(&self) -> bool;
    fn set_input_event_handling_enabled(&mut self, enabled: bool);
}

impl<T> PlayerInputHandlerExt for T
where
    T: AsRef<PlayerInputHandler> + AsMut<PlayerInputHandler>,
{
    #[inline(always)]
    fn can_process(&self, event: *mut InputEvent) -> bool {
        PlayerInputHandler::can_process(self.as_ref(), event)
    }

    #[inline(always)]
    fn process_thumbstick(&mut self, event: *mut ThumbstickEvent, data: *mut PlayerControlsData) {
        PlayerInputHandler::process_thumbstick(self.as_mut(), event, data)
    }

    #[inline(always)]
    fn process_mouse_move(&mut self, event: *mut MouseMoveEvent, data: *mut PlayerControlsData) {
        PlayerInputHandler::process_mouse_move(self.as_mut(), event, data)
    }

    #[inline(always)]
    fn process_button(&mut self, event: *mut ButtonEvent, data: *mut PlayerControlsData) {
        PlayerInputHandler::process_button(self.as_mut(), event, data)
    }

    #[inline(always)]
    fn unk_05(&mut self) {
        PlayerInputHandler::unk_05(self.as_mut())
    }

    #[inline(always)]
    fn unk_06(&mut self) {
        PlayerInputHandler::unk_06(self.as_mut())
    }

    #[inline(always)]
    fn is_input_event_handling_enabled(&self) -> bool {
        PlayerInputHandler::is_input_event_handling_enabled(self.as_ref())
    }

    #[inline(always)]
    fn set_input_event_handling_enabled(&mut self, enabled: bool) {
        PlayerInputHandler::set_input_event_handling_enabled(self.as_mut(), enabled)
    }
}
