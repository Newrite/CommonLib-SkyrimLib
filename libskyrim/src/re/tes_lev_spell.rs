use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESLevSpell;
use crate::offsets::offsets_vtable::VTABLE_TESLevSpell;
use crate::re::{
    BGSLoadFormBuffer, BGSSaveFormBuffer, FormCastable, FormType, TESBoundObject, TESFile,
    TESLeveledList,
};
use crate::relocation::{RttiType, VariantID};

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESLevSpellChangeFlags: u32 {
        const ADDED_OBJECT = 1u32 << 31;
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESLevSpellRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct TESLevSpell {
    pub base: TESBoundObject,
    pub leveled_list: TESLeveledList,
}

const _: () = assert!(core::mem::size_of::<TESLevSpell>() == 0x58);
const _: () = assert!(core::mem::offset_of!(TESLevSpell, leveled_list) == 0x30);

impl RttiType for TESLevSpell {
    const RTTI: VariantID = RTTI_TESLevSpell;
}

impl FormCastable for TESLevSpell {
    const TARGET_FORM_TYPE: FormType = FormType::LeveledSpell;
}

inherit!(TESLevSpell : TESBoundObject);
inherit!(TESLevSpell => TESLeveledList, leveled_list);

impl TESLevSpell {
    pub const RTTI: VariantID = RTTI_TESLevSpell;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESLevSpell;
    pub const FORMTYPE: FormType = FormType::LeveledSpell;

    // override (TESBoundObject)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_: *mut TESFile) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x0E;
        pub fn save_game(&mut self, buf: *mut BGSSaveFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x0F;
        pub fn load_game(&mut self, buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_REVERT: usize = 0x12;
        pub fn revert(&mut self, buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_ITEM_IMPL: usize = 0x13;
        pub fn init_item_impl(&mut self)
    }

    // override (TESLeveledList)
    crate::virtual_method! {
        pub const VFUNC_GET_CAN_CONTAIN_FORMS_OF_TYPE: usize = 0x07;
        pub fn get_can_contain_forms_of_type(&self, form_type: FormType) -> bool
    }
}
