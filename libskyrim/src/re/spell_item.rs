use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_SpellItem;
use crate::offsets::offsets_vtable::VTABLE_SpellItem;
use crate::re::bgs_equip_type::BGSEquipType;
use crate::re::bgs_menu_display_object::BGSMenuDisplayObject;
use crate::re::bgs_perk::BGSPerk;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::magic_item::MagicItem;
use crate::re::magic_system::{CastingType, Delivery, SpellType};
use crate::re::tes_description::TESDescription;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SpellItem::SpellFlag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpellFlag {
    None = 0,
    CostOverride = 1 << 0,
    FoodItem = 1 << 1,
    ExtendDuration = 1 << 3,
    PCStartSpell = 1 << 17,
    InstantCast = 1 << 18,
    IgnoreLOSCheck = 1 << 19,
    IgnoreResistance = 1 << 20,
    NoAbsorb = 1 << 21,
    NoDualCastMods = 1 << 23,
}

core_util::impl_enumset_type!(SpellFlag => u32);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SpellItemRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::SpellItem::Data`
#[repr(C)]
pub struct SpellItemData {
    pub cost_override: i32,             // 00
    pub flags: EnumSet<SpellFlag, u32>, // 04
    pub spell_type: SpellType,          // 08
    pub charge_time: f32,               // 0C
    pub casting_type: CastingType,      // 10
    pub delivery: Delivery,             // 14
    pub cast_duration: f32,             // 18
    pub range: f32,                     // 1C
    pub casting_perk: *mut BGSPerk,     // 20
}

const _: () = assert!(core::mem::size_of::<SpellItemData>() == 0x28);
const _: () = assert!(core::mem::offset_of!(SpellItemData, cost_override) == 0x00);
const _: () = assert!(core::mem::offset_of!(SpellItemData, flags) == 0x04);
const _: () = assert!(core::mem::offset_of!(SpellItemData, spell_type) == 0x08);
const _: () = assert!(core::mem::offset_of!(SpellItemData, charge_time) == 0x0C);
const _: () = assert!(core::mem::offset_of!(SpellItemData, casting_type) == 0x10);
const _: () = assert!(core::mem::offset_of!(SpellItemData, delivery) == 0x14);
const _: () = assert!(core::mem::offset_of!(SpellItemData, cast_duration) == 0x18);
const _: () = assert!(core::mem::offset_of!(SpellItemData, range) == 0x1C);
const _: () = assert!(core::mem::offset_of!(SpellItemData, casting_perk) == 0x20);

/// C++ `RE::SpellItem`
#[repr(C)]
pub struct SpellItem {
    pub base: MagicItem,                           // 000
    pub equip_type: BGSEquipType,                  // 090
    pub menu_display_object: BGSMenuDisplayObject, // 0A0
    pub description: TESDescription,               // 0B0
    pub data: SpellItemData,                       // 0C0 - SPIT
}

const _: () = assert!(core::mem::size_of::<SpellItem>() == 0xE8);
const _: () = assert!(core::mem::offset_of!(SpellItem, equip_type) == 0x90);
const _: () = assert!(core::mem::offset_of!(SpellItem, menu_display_object) == 0xA0);
const _: () = assert!(core::mem::offset_of!(SpellItem, description) == 0xB0);
const _: () = assert!(core::mem::offset_of!(SpellItem, data) == 0xC0);

impl RttiType for SpellItem {
    const RTTI: VariantID = RTTI_SpellItem;
}

impl FormCastable for SpellItem {
    const TARGET_FORM_TYPE: FormType = FormType::Spell;
}

inherit!(SpellItem : MagicItem);
inherit!(SpellItem => BGSEquipType, equip_type);
inherit!(SpellItem => BGSMenuDisplayObject, menu_display_object);
inherit!(SpellItem => TESDescription, description);

impl SpellItem {
    pub const RTTI: VariantID = RTTI_SpellItem;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SpellItem;
    pub const FORMTYPE: FormType = FormType::Spell;

    // override (MagicItem)
    // void                                   InitializeData() override;                                             // 04
    // void                                   ClearData() override;                                                  // 05
    // void                                   InitItemImpl() override;                                               // 13
    // [[nodiscard]] MagicSystem::SpellType   GetSpellType() const override;                                         // 53
    // void                                   SetCastingType(MagicSystem::CastingType a_type) override;              // 54
    // [[nodiscard]] MagicSystem::CastingType GetCastingType() const override;                                       // 55
    // void                                   SetDelivery(MagicSystem::Delivery a_type) override;                    // 56
    // [[nodiscard]] MagicSystem::Delivery    GetDelivery() const override;                                          // 57
    // [[nodiscard]] float                    GetFixedCastDuration() const override;                                 // 59
    // [[nodiscard]] float                    GetRange() const override;                                             // 5A
    // [[nodiscard]] bool                     IgnoresResistance() const override;                                    // 5B
    // [[nodiscard]] bool                     IgnoreLOS() const override;                                            // 5C
    // [[nodiscard]] bool                     GetNoAbsorb() const override;                                          // 5E
    // [[nodiscard]] bool                     GetNoDualCastModifications() const override;                           // 5F
    // bool                                   GetSkillUsageData(SkillUsageData& a_data) const override;              // 60
    // void                                   AdjustCost(float& a_cost, Actor* a_actor) const override;              // 63
    // [[nodiscard]] float                    GetChargeTime() const override;                                        // 64
    // [[nodiscard]] ActorValue               GetAssociatedSkill() const override;                                   // 66
    // [[nodiscard]] bool                     IsTwoHanded() const override;                                          // 67
    // [[nodiscard]] std::uint32_t            GetChunkID() override;                                                 // 68
    // void                                   CopyMagicItemData(MagicItem* a_src) override;                          // 69
    // void                                   LoadMagicItemChunk(TESFile* a_mod, std::uint32_t a_chunkID) override;  // 6A
    // void                                   LoadChunkDataPostProcess(TESFile* a_mod) override;                     // 6B
    // [[nodiscard]] const MagicItem::Data*   GetData1() const override;                                             // 6C
    // [[nodiscard]] MagicItem::Data*         GetData2() override;                                                   // 6D
    // [[nodiscard]] std::uint32_t            GetDataSize() const override;                                          // 6E
    // void                                   InitFromChunk(TESFile* a_mod) override;                                // 6F
    // void                                   InitChunk() override;                                                  // 70

    // override (BGSMenuDisplayObject)
    // [[nodiscard]] TESBoundObject* GetMenuDisplayObject() const override;  // 04
}
