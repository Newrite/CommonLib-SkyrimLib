use core::ffi::c_char;

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_ExtraTextDisplayData;
use crate::offsets::offsets_vtable::VTABLE_ExtraTextDisplayData;
use crate::re::{
    BGSMessage, BSExtraData, BSFixedString, ExtraDataType, ExtraDataTyped, TESBoundObject, TESQuest,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::ExtraTextDisplayData::DisplayDataType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisplayDataType {
    Uninitialized = -1,
    CustomName = -2,
}

core_util::impl_enumset_type!(DisplayDataType => i32);

/// C++ `RE::ExtraTextDisplayData`
#[repr(C)]
pub struct ExtraTextDisplayData {
    pub base: BSExtraData,                             // 00
    pub display_name: BSFixedString,                   // 10
    pub display_name_text: *mut BGSMessage,            // 18
    pub owner_quest: *mut TESQuest,                    // 20
    pub owner_instance: EnumSet<DisplayDataType, i32>, // 28
    pub temper_factor: f32,                            // 2C
    pub custom_name_length: u16,                       // 30
    pub pad32: u16,                                    // 32
    pub pad34: u32,                                    // 34
}

const _: () = assert!(core::mem::size_of::<ExtraTextDisplayData>() == 0x38);
const _: () = assert!(core::mem::offset_of!(ExtraTextDisplayData, display_name) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraTextDisplayData, display_name_text) == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraTextDisplayData, owner_quest) == 0x20);
const _: () = assert!(core::mem::offset_of!(ExtraTextDisplayData, owner_instance) == 0x28);
const _: () = assert!(core::mem::offset_of!(ExtraTextDisplayData, temper_factor) == 0x2C);
const _: () = assert!(core::mem::offset_of!(ExtraTextDisplayData, custom_name_length) == 0x30);
const _: () = assert!(core::mem::offset_of!(ExtraTextDisplayData, pad32) == 0x32);
const _: () = assert!(core::mem::offset_of!(ExtraTextDisplayData, pad34) == 0x34);

impl RttiType for ExtraTextDisplayData {
    const RTTI: VariantID = RTTI_ExtraTextDisplayData;
}

impl ExtraDataTyped for ExtraTextDisplayData {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::TextDisplayData;
}

inherit!(ExtraTextDisplayData : BSExtraData);

impl ExtraTextDisplayData {
    pub const RTTI: VariantID = RTTI_ExtraTextDisplayData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraTextDisplayData;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::TextDisplayData;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kTextDisplayData; }

    #[inline(always)]
    pub fn owner_instance_uninitialized() -> EnumSet<DisplayDataType, i32> {
        EnumSet::from_underlying(DisplayDataType::Uninitialized as i32)
    }

    #[inline(always)]
    pub fn owner_instance_custom_name() -> EnumSet<DisplayDataType, i32> {
        EnumSet::from_underlying(DisplayDataType::CustomName as i32)
    }

    #[inline(always)]
    pub fn new_empty() -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            display_name: BSFixedString::from_str(""),
            display_name_text: core::ptr::null_mut(),
            owner_quest: core::ptr::null_mut(),
            owner_instance: Self::owner_instance_uninitialized(),
            temper_factor: 1.0,
            custom_name_length: 0,
            pad32: 0,
            pad34: 0,
        }
    }

    #[inline(always)]
    pub fn new_with_name(name: *const c_char) -> Self {
        let mut text = Self::new_empty();
        text.set_name(name);
        text
    }

    #[inline(always)]
    pub fn new_from_base_object(base_object: *mut TESBoundObject, temper_factor: f32) -> Self {
        let mut text = Self::new_empty();
        let _ = text.get_display_name(base_object, temper_factor);
        text
    }

    crate::relocation_func! {
        pub fn get_display_name(&mut self, base_object: *mut TESBoundObject, temper_factor: f32) -> *const c_char => RelocationID::new(12626, 12768)
    }

    #[inline(always)]
    pub fn is_player_set(&self) -> bool {
        self.owner_instance == Self::owner_instance_custom_name()
    }

    pub fn set_name(&mut self, name: *const c_char) {
        if !self.display_name_text.is_null() {
            return;
        }

        self.display_name = BSFixedString::new(name);
        self.custom_name_length = self.display_name.len() as u16;
        self.owner_instance = Self::owner_instance_custom_name();
        self.temper_factor = 1.0;
    }
}
