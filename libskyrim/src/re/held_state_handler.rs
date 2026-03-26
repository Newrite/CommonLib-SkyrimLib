use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_HeldStateHandler;
use crate::offsets::offsets_vtable::VTABLE_HeldStateHandler;
use crate::re::{ButtonEvent, PlayerInputHandler};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct HeldStateHandlerData {
    pub held_state_active: bool,
    pub trigger_release_event: bool,
    pub pad12: u16,
    pub pad14: u32,
}

const _: () = assert!(core::mem::size_of::<HeldStateHandlerData>() == 0x08);
const _: () = assert!(core::mem::offset_of!(HeldStateHandlerData, held_state_active) == 0x00);
const _: () = assert!(core::mem::offset_of!(HeldStateHandlerData, trigger_release_event) == 0x01);
const _: () = assert!(core::mem::offset_of!(HeldStateHandlerData, pad14) == 0x04);

#[repr(C)]
pub struct HeldStateHandler {
    pub base: PlayerInputHandler,
}

const _: () = assert!(core::mem::size_of::<HeldStateHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(HeldStateHandler, base) == 0x00);

impl RttiType for HeldStateHandler {
    const RTTI: VariantID = RTTI_HeldStateHandler;
}

inherit!(HeldStateHandler : PlayerInputHandler, base);

impl HeldStateHandler {
    pub const RTTI: VariantID = RTTI_HeldStateHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_HeldStateHandler;
    pub const DATA_OFFSET: VariantOffset = VariantOffset::new(0x10, 0x10, 0x28);

    crate::runtime_data_accessor! {
        pub fn data() -> HeldStateHandlerData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> HeldStateHandlerData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE_HELD_STATE_ACTIVE: usize = 0x05;
        pub fn update_held_state_active(event: *const ButtonEvent)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_HELD_STATE_ACTIVE: usize = 0x06;
        pub fn set_held_state_active(flag: bool)
    }
}

pub trait HeldStateHandlerExt {
    fn data(&self) -> &HeldStateHandlerData;
    fn data_mut(&mut self) -> &mut HeldStateHandlerData;
    fn update_held_state_active(&mut self, event: *const ButtonEvent);
    fn set_held_state_active(&mut self, flag: bool);
}

impl<T> HeldStateHandlerExt for T
where
    T: AsRef<HeldStateHandler> + AsMut<HeldStateHandler>,
{
    #[inline(always)]
    fn data(&self) -> &HeldStateHandlerData {
        HeldStateHandler::data(self.as_ref())
    }

    #[inline(always)]
    fn data_mut(&mut self) -> &mut HeldStateHandlerData {
        HeldStateHandler::data_mut(self.as_mut())
    }

    #[inline(always)]
    fn update_held_state_active(&mut self, event: *const ButtonEvent) {
        HeldStateHandler::update_held_state_active(self.as_mut(), event)
    }

    #[inline(always)]
    fn set_held_state_active(&mut self, flag: bool) {
        HeldStateHandler::set_held_state_active(self.as_mut(), flag)
    }
}
