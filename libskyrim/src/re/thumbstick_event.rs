#![allow(non_camel_case_types)]

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_ThumbstickEvent;
use crate::offsets::offsets_vtable::VTABLE_ThumbstickEvent;
use crate::re::{BSFixedString, IDEvent, INPUT_DEVICE, INPUT_EVENT_TYPE};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ThumbstickEvent::InputTypes::InputType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThumbstickInputType {
    kLeftThumbstick = 0x0B,
    kRightThumbstick = 0x0C,
}

/// C++ `RE::ThumbstickEvent`
#[repr(C)]
pub struct ThumbstickEvent {
    pub base: IDEvent, // 00
    pub x_value: f32,  // 28
    pub y_value: f32,  // 2C
}

const _: () = assert!(core::mem::size_of::<ThumbstickEvent>() == 0x30);
const _: () = assert!(core::mem::offset_of!(ThumbstickEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ThumbstickEvent, x_value) == 0x28);
const _: () = assert!(core::mem::offset_of!(ThumbstickEvent, y_value) == 0x2C);

impl RttiType for ThumbstickEvent {
    const RTTI: VariantID = RTTI_ThumbstickEvent;
}

inherit!(ThumbstickEvent : IDEvent, base);

impl ThumbstickEvent {
    pub const RTTI: VariantID = RTTI_ThumbstickEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ThumbstickEvent;

    #[inline(always)]
    pub fn init(&mut self, id: ThumbstickInputType, x_value: f32, y_value: f32) {
        self.init_with_device_user_event(
            id,
            INPUT_DEVICE::kGamepad,
            x_value,
            y_value,
            BSFixedString::default(),
        );
    }

    #[inline(always)]
    pub fn init_with_device(
        &mut self,
        id: ThumbstickInputType,
        device: INPUT_DEVICE,
        x_value: f32,
        y_value: f32,
    ) {
        self.init_with_device_user_event(id, device, x_value, y_value, BSFixedString::default());
    }

    #[inline(always)]
    pub fn init_with_device_user_event(
        &mut self,
        id: ThumbstickInputType,
        device: INPUT_DEVICE,
        x_value: f32,
        y_value: f32,
        user_event: BSFixedString,
    ) {
        self.x_value = x_value;
        self.y_value = y_value;
        self.base.base.device = EnumSet::from_underlying(device as u32);
        self.base.base.event_type = EnumSet::from_underlying(INPUT_EVENT_TYPE::kThumbstick as u32);
        self.base.id_code = id as u32;
        self.base.user_event = user_event;
    }

    #[inline(always)]
    pub fn is_left(&self) -> bool {
        self.base.id_code == ThumbstickInputType::kLeftThumbstick as u32
    }

    #[inline(always)]
    pub fn is_right(&self) -> bool {
        self.base.id_code == ThumbstickInputType::kRightThumbstick as u32
    }

    #[inline(always)]
    pub fn is_off_hand(&self) -> bool {
        self.is_left()
    }

    #[inline(always)]
    pub fn is_main_hand(&self) -> bool {
        self.is_right()
    }
}
