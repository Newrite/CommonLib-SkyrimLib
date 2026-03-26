#![allow(non_camel_case_types)]

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_ButtonEvent;
use crate::offsets::offsets_vtable::VTABLE_ButtonEvent;
use crate::re::{BSFixedString, IDEvent, INPUT_DEVICE, INPUT_EVENT_TYPE, InputEvent, VRWandEvent};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct ButtonEventRuntimeData {
    pub value: f32,          // 00
    pub held_down_secs: f32, // 04
}

const _: () = assert!(core::mem::size_of::<ButtonEventRuntimeData>() == 0x08);

/// Honest common-prefix C++ `RE::ButtonEvent`.
#[repr(C)]
pub struct ButtonEvent {
    pub base: InputEvent, // 00
}

const _: () = assert!(core::mem::size_of::<ButtonEvent>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ButtonEvent, base) == 0x00);

impl RttiType for ButtonEvent {
    const RTTI: VariantID = RTTI_ButtonEvent;
}

inherit!(ButtonEvent : InputEvent, base);

impl ButtonEvent {
    pub const RTTI: VariantID = RTTI_ButtonEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ButtonEvent;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x28, 0x28, 0x30);

    crate::runtime_data_accessor! {
        pub fn runtime_data() -> ButtonEventRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn runtime_data_mut() -> ButtonEventRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    #[inline(always)]
    pub fn value(&self) -> f32 {
        self.runtime_data().value
    }

    #[inline(always)]
    pub fn held_duration(&self) -> f32 {
        self.runtime_data().held_down_secs
    }

    #[inline(always)]
    pub fn is_pressed(&self) -> bool {
        self.value() > 0.0
    }

    #[inline(always)]
    pub fn is_repeating(&self) -> bool {
        self.held_duration() > 0.0
    }

    #[inline(always)]
    pub fn is_down(&self) -> bool {
        self.is_pressed() && self.held_duration() == 0.0
    }

    #[inline(always)]
    pub fn is_held(&self) -> bool {
        self.is_pressed() && self.is_repeating()
    }

    #[inline(always)]
    pub fn is_up(&self) -> bool {
        self.value() == 0.0 && self.is_repeating()
    }

    #[inline(always)]
    pub fn as_id_event(&self) -> &IDEvent {
        unsafe { &*(self as *const Self).cast() }
    }

    #[inline(always)]
    pub fn as_id_event_mut(&mut self) -> &mut IDEvent {
        unsafe { &mut *(self as *mut Self).cast() }
    }

    #[inline(always)]
    pub fn as_vr_wand_event(&self) -> Option<&VRWandEvent> {
        crate::runtime::is_vr().then(|| unsafe { &*(self as *const Self).cast() })
    }

    #[inline(always)]
    pub fn as_vr_wand_event_mut(&mut self) -> Option<&mut VRWandEvent> {
        crate::runtime::is_vr().then(|| unsafe { &mut *(self as *mut Self).cast() })
    }

    #[inline(always)]
    pub fn get_id_code(&self) -> u32 {
        self.as_id_event().id_code
    }

    #[inline(always)]
    pub fn set_id_code(&mut self, id_code: u32) {
        self.as_id_event_mut().id_code = id_code;
    }

    #[inline(always)]
    pub fn get_user_event(&self) -> &BSFixedString {
        &self.as_id_event().user_event
    }

    #[inline(always)]
    pub fn set_user_event(&mut self, user_event: BSFixedString) {
        self.as_id_event_mut().user_event = user_event;
    }

    #[inline(always)]
    pub fn init(
        &mut self,
        device: INPUT_DEVICE,
        id: i32,
        value: f32,
        duration: f32,
        user_event: BSFixedString,
    ) {
        self.runtime_data_mut().value = value;
        self.runtime_data_mut().held_down_secs = duration;
        self.base.device = EnumSet::from_underlying(device as u32);
        self.base.event_type = EnumSet::from_underlying(INPUT_EVENT_TYPE::kButton as u32);
        self.set_id_code(id as u32);
        self.set_user_event(user_event);
    }

    #[inline(always)]
    pub fn create(
        input_device: INPUT_DEVICE,
        user_event: &BSFixedString,
        id_code: u32,
        value: f32,
        held_down_secs: f32,
    ) -> *mut Self {
        unsafe {
            crate::ffi::commonlib_button_event_create(
                input_device as i32,
                (user_event as *const BSFixedString).cast(),
                id_code,
                value,
                held_down_secs,
            )
            .cast()
        }
    }

    // TODO: `create(...)` returns a raw engine-allocated `ButtonEvent*`, but this repo still lacks
    // a source-backed owner/delete wrapper for input-event factories. Keep the factory raw until the
    // engine-side destruction/queue ownership contract is translated honestly.
}
