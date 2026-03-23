use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESSpellList;
use crate::offsets::offsets_vtable::VTABLE_TESSpellList;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::spell_item::SpellItem;
use crate::re::tes_lev_spell::TESLevSpell;
use crate::re::tes_shout::TESShout;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct SpellData {
    pub spells: *mut *mut SpellItem,        // 0x00
    pub lev_spells: *mut *mut TESLevSpell,  // 0x08
    pub shouts: *mut *mut TESShout,         // 0x10
    pub num_spells: u32,                    // 0x18
    pub num_lev_spells: u32,                // 0x1C
    pub num_shouts: u32,                    // 0x20
    pub pad24: u32,                         // 0x24
}

const _: () = assert!(core::mem::size_of::<SpellData>() == 0x28);

#[repr(C)]
pub struct TESSpellList {
    pub base: BaseFormComponent,       // 0x00
    pub actor_effects: *mut SpellData, // 0x08 - SPLO
}

const _: () = assert!(core::mem::size_of::<TESSpellList>() == 0x10);

impl RttiType for TESSpellList {
    const RTTI: VariantID = RTTI_TESSpellList;
}

inherit!(TESSpellList : BaseFormComponent);

impl TESSpellList {
    pub const RTTI: VariantID = RTTI_TESSpellList;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESSpellList;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
