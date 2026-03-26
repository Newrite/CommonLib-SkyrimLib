use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::{
    RTTI_BGSPerk, RTTI_BGSPerk__AddPerkVisitor, RTTI_BGSPerk__ApplyPerksVisitor,
    RTTI_BGSPerk__FindPerkInRanksVisitor,
};
use crate::offsets::offsets_vtable::{
    VTABLE_BGSPerk, VTABLE_BGSPerk__AddPerkVisitor, VTABLE_BGSPerk__ApplyPerksVisitor,
    VTABLE_BGSPerk__FindPerkInRanksVisitor,
};
use crate::re::{
    BGSPerkEntry, BSTArray, FormCastable, FormType, PerkRankVisitor, TESCondition, TESDescription,
    TESFile, TESForm, TESFullName, TESIcon,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::PerkData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PerkData {
    pub trait_flag: bool, // 00
    pub level: u8,        // 01
    pub num_ranks: u8,    // 02
    pub playable: bool,   // 03
    pub hidden: bool,     // 04
}

const _: () = assert!(core::mem::size_of::<PerkData>() == 0x5);

bitflags! {
    /// C++ `RE::BGSPerk::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSPerkRecordFlags: u32 {
        const NON_PLAYABLE = 1 << 2;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::BGSPerk::FindPerkInRanksVisitor`
#[repr(C)]
pub struct BGSPerkFindPerkInRanksVisitor {
    pub base: PerkRankVisitor, // 00
}

const _: () = assert!(core::mem::size_of::<BGSPerkFindPerkInRanksVisitor>() == 0x8);

impl RttiType for BGSPerkFindPerkInRanksVisitor {
    const RTTI: VariantID = RTTI_BGSPerk__FindPerkInRanksVisitor;
}

inherit!(BGSPerkFindPerkInRanksVisitor : PerkRankVisitor, base);

impl BGSPerkFindPerkInRanksVisitor {
    pub const RTTI: VariantID = RTTI_BGSPerk__FindPerkInRanksVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSPerk__FindPerkInRanksVisitor;
}

/// C++ `RE::BGSPerk::ApplyPerksVisitor`
#[repr(C)]
pub struct BGSPerkApplyPerksVisitor {
    pub base: PerkRankVisitor, // 00
}

const _: () = assert!(core::mem::size_of::<BGSPerkApplyPerksVisitor>() == 0x8);

impl RttiType for BGSPerkApplyPerksVisitor {
    const RTTI: VariantID = RTTI_BGSPerk__ApplyPerksVisitor;
}

inherit!(BGSPerkApplyPerksVisitor : PerkRankVisitor, base);

impl BGSPerkApplyPerksVisitor {
    pub const RTTI: VariantID = RTTI_BGSPerk__ApplyPerksVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSPerk__ApplyPerksVisitor;
}

/// C++ `RE::BGSPerk::AddPerkVisitor`
#[repr(C)]
pub struct BGSPerkAddPerkVisitor {
    pub base: PerkRankVisitor, // 00
}

const _: () = assert!(core::mem::size_of::<BGSPerkAddPerkVisitor>() == 0x8);

impl RttiType for BGSPerkAddPerkVisitor {
    const RTTI: VariantID = RTTI_BGSPerk__AddPerkVisitor;
}

inherit!(BGSPerkAddPerkVisitor : PerkRankVisitor, base);

impl BGSPerkAddPerkVisitor {
    pub const RTTI: VariantID = RTTI_BGSPerk__AddPerkVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSPerk__AddPerkVisitor;
}

/// C++ `RE::BGSPerk`
#[repr(C)]
pub struct BGSPerk {
    pub base: TESForm,                             // 00
    pub full_name: TESFullName,                    // 20
    pub description: TESDescription,               // 30
    pub icon: TESIcon,                             // 40
    pub data: PerkData,                            // 50
    pub pad55: u8,                                 // 55
    pub pad56: u16,                                // 56
    pub perk_conditions: TESCondition,             // 58
    pub perk_entries: BSTArray<*mut BGSPerkEntry>, // 60
    pub next_perk: *mut BGSPerk,                   // 78
}

const _: () = assert!(core::mem::size_of::<BGSPerk>() == 0x80);

impl RttiType for BGSPerk {
    const RTTI: VariantID = RTTI_BGSPerk;
}

impl FormCastable for BGSPerk {
    const TARGET_FORM_TYPE: FormType = FormType::Perk;
}

inherit!(BGSPerk : TESForm);
inherit!(BGSPerk => TESFullName, full_name);
inherit!(BGSPerk => TESDescription, description);
inherit!(BGSPerk => TESIcon, icon);

impl BGSPerk {
    pub const RTTI: VariantID = RTTI_BGSPerk;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSPerk;
    pub const FORMTYPE: FormType = FormType::Perk;

    crate::virtual_method! {
        pub const VFUNC_INITIALIZE_DATA: usize = 0x04;
        pub fn initialize_data(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_CLEAR_DATA: usize = 0x05;
        pub fn clear_data(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_file: *mut TESFile) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_ITEM_IMPL: usize = 0x13;
        pub fn init_item_impl(&mut self)
    }
}
