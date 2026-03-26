use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESWordOfPower;
use crate::offsets::offsets_vtable::VTABLE_TESWordOfPower;
use crate::re::{BSFixedString, FormCastable, FormType, TESFile, TESForm, TESFullName};
use crate::relocation::{RttiType, VariantID};

bitflags! {
    /// C++ `RE::TESWordOfPower::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESWordOfPowerRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::TESWordOfPower`
#[repr(C)]
pub struct TESWordOfPower {
    pub base: TESForm,              // 00
    pub full_name: TESFullName,     // 20
    pub translation: BSFixedString, // 30
}

const _: () = assert!(core::mem::size_of::<TESWordOfPower>() == 0x38);
const _: () = assert!(core::mem::offset_of!(TESWordOfPower, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESWordOfPower, translation) == 0x30);

impl RttiType for TESWordOfPower {
    const RTTI: VariantID = RTTI_TESWordOfPower;
}

impl FormCastable for TESWordOfPower {
    const TARGET_FORM_TYPE: FormType = FormType::WordOfPower;
}

inherit!(TESWordOfPower : TESForm);
inherit!(TESWordOfPower => TESFullName, full_name);

impl TESWordOfPower {
    pub const RTTI: VariantID = RTTI_TESWordOfPower;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESWordOfPower;
    pub const FORMTYPE: FormType = FormType::WordOfPower;

    crate::virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_file: *mut TESFile) -> bool
    }
}
