use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESShout;
use crate::offsets::offsets_vtable::VTABLE_TESShout;
use crate::re::{
    BGSEquipSlot, BGSEquipType, BGSMenuDisplayObject, FormCastable, FormType, SpellItem,
    TESDescription, TESFile, TESForm, TESFullName, TESWordOfPower,
};
use crate::relocation::{RttiType, VariantID};

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESShoutRecordFlags: u32 {
        const DELETED = 1 << 5;
        const TREAT_SPELLS_AS_POWERS = 1 << 7;
        const IGNORED = 1 << 12;
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESShoutVariationID {
    None = u32::MAX,
    One = 0,
    Two = 1,
    Three = 2,
}

pub const TES_SHOUT_VARIATION_TOTAL: usize = 3;

#[repr(C)]
pub struct TESShoutVariation {
    pub word: *mut TESWordOfPower,
    pub spell: *mut SpellItem,
    pub recovery_time: f32,
    pub pad14: u32,
}

const _: () = assert!(core::mem::size_of::<TESShoutVariation>() == 0x18);
const _: () = assert!(core::mem::offset_of!(TESShoutVariation, word) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESShoutVariation, spell) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESShoutVariation, recovery_time) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESShoutVariation, pad14) == 0x14);

#[repr(C)]
pub struct TESShout {
    pub base: TESForm,
    pub full_name: TESFullName,
    pub menu_display_object: BGSMenuDisplayObject,
    pub equip_type: BGSEquipType,
    pub description: TESDescription,
    pub variations: [TESShoutVariation; TES_SHOUT_VARIATION_TOTAL],
}

const _: () = assert!(core::mem::size_of::<TESShout>() == 0xA8);
const _: () = assert!(core::mem::offset_of!(TESShout, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESShout, menu_display_object) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESShout, equip_type) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESShout, description) == 0x50);
const _: () = assert!(core::mem::offset_of!(TESShout, variations) == 0x60);

impl RttiType for TESShout {
    const RTTI: VariantID = RTTI_TESShout;
}

impl FormCastable for TESShout {
    const TARGET_FORM_TYPE: FormType = FormType::Shout;
}

inherit!(TESShout : TESForm);
inherit!(TESShout => TESFullName, full_name);
inherit!(TESShout => BGSMenuDisplayObject, menu_display_object);
inherit!(TESShout => BGSEquipType, equip_type);
inherit!(TESShout => TESDescription, description);

impl TESShout {
    pub const RTTI: VariantID = RTTI_TESShout;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESShout;
    pub const FORMTYPE: FormType = FormType::Shout;

    // override (TESForm)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_: *mut TESFile) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_ITEM_IMPL: usize = 0x13;
        pub fn init_item_impl(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_KNOWN: usize = 0x17;
        pub fn get_known(&self) -> bool
    }

    // override (BGSEquipType)
    crate::virtual_method! {
        pub const VFUNC_GET_EQUIP_SLOT: usize = 0x04;
        pub fn get_equip_slot(&self) -> *mut BGSEquipSlot
    }

    crate::virtual_method! {
        pub const VFUNC_SET_EQUIP_SLOT: usize = 0x05;
        pub fn set_equip_slot(&mut self, slot: *mut BGSEquipSlot)
    }
}
