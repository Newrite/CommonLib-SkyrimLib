use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_AlchemyItem;
use crate::offsets::offsets_vtable::VTABLE_AlchemyItem;
use crate::re::bgs_destructible_object_form::BGSDestructibleObjectForm;
use crate::re::bgs_equip_type::BGSEquipType;
use crate::re::bgs_message_icon::BGSMessageIcon;
use crate::re::bgs_pickup_putdown_sounds::BGSPickupPutdownSounds;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::magic_item::MagicItem;
use crate::re::spell_item::SpellItem;
use crate::re::tes_icon::TESIcon;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::re::tes_weight_form::TESWeightForm;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::AlchemyItem::AlchemyFlag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlchemyFlag {
    None = 0,
    CostOverride = 1 << 0,
    FoodItem = 1 << 1,
    ExtendDuration = 1 << 3,
    Medicine = 1 << 16,
    Poison = 1 << 17,
}

core_util::impl_enumset_type!(AlchemyFlag => u32);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AlchemyItemRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
        const MEDICINE = 1 << 29;
    }
}

/// C++ `RE::AlchemyItem::Data`
#[repr(C)]
pub struct AlchemyItemData {
    pub cost_override: i32,                             // 00
    pub flags: EnumSet<AlchemyFlag, u32>,               // 04
    pub addiction_item: *mut SpellItem,                 // 08
    pub addiction_chance: f32,                          // 10
    pub pad14: u32,                                     // 14
    pub consumption_sound: *mut BGSSoundDescriptorForm, // 18
}

const _: () = assert!(core::mem::size_of::<AlchemyItemData>() == 0x20);
const _: () = assert!(core::mem::offset_of!(AlchemyItemData, cost_override) == 0x00);
const _: () = assert!(core::mem::offset_of!(AlchemyItemData, flags) == 0x04);
const _: () = assert!(core::mem::offset_of!(AlchemyItemData, addiction_item) == 0x08);
const _: () = assert!(core::mem::offset_of!(AlchemyItemData, addiction_chance) == 0x10);
const _: () = assert!(core::mem::offset_of!(AlchemyItemData, pad14) == 0x14);
const _: () = assert!(core::mem::offset_of!(AlchemyItemData, consumption_sound) == 0x18);

/// C++ `RE::AlchemyItem`
#[repr(C)]
pub struct AlchemyItem {
    pub base: MagicItem,                                     // 000
    pub model_texture_swap: TESModelTextureSwap,             // 090
    pub icon: TESIcon,                                       // 0C8
    pub message_icon_component: BGSMessageIcon,              // 0D8
    pub weight_form: TESWeightForm,                          // 0F0
    pub equip_type: BGSEquipType,                            // 100
    pub destructible_object_form: BGSDestructibleObjectForm, // 110
    pub pickup_putdown_sounds: BGSPickupPutdownSounds,       // 120
    pub data: AlchemyItemData,                               // 138 - ENIT
    pub message_icon: TESIcon,                               // 158
}

const _: () = assert!(core::mem::size_of::<AlchemyItem>() == 0x168);
const _: () = assert!(core::mem::offset_of!(AlchemyItem, model_texture_swap) == 0x90);
const _: () = assert!(core::mem::offset_of!(AlchemyItem, icon) == 0xC8);
const _: () = assert!(core::mem::offset_of!(AlchemyItem, message_icon_component) == 0xD8);
const _: () = assert!(core::mem::offset_of!(AlchemyItem, weight_form) == 0xF0);
const _: () = assert!(core::mem::offset_of!(AlchemyItem, equip_type) == 0x100);
const _: () = assert!(core::mem::offset_of!(AlchemyItem, destructible_object_form) == 0x110);
const _: () = assert!(core::mem::offset_of!(AlchemyItem, pickup_putdown_sounds) == 0x120);
const _: () = assert!(core::mem::offset_of!(AlchemyItem, data) == 0x138);
const _: () = assert!(core::mem::offset_of!(AlchemyItem, message_icon) == 0x158);

impl RttiType for AlchemyItem {
    const RTTI: VariantID = RTTI_AlchemyItem;
}

impl FormCastable for AlchemyItem {
    const TARGET_FORM_TYPE: FormType = FormType::AlchemyItem;
}

inherit!(AlchemyItem : MagicItem);
inherit!(AlchemyItem => TESModelTextureSwap, model_texture_swap);
inherit!(AlchemyItem => TESIcon, icon);
inherit!(AlchemyItem => BGSMessageIcon, message_icon_component);
inherit!(AlchemyItem => TESWeightForm, weight_form);
inherit!(AlchemyItem => BGSEquipType, equip_type);
inherit!(AlchemyItem => BGSDestructibleObjectForm, destructible_object_form);
inherit!(AlchemyItem => BGSPickupPutdownSounds, pickup_putdown_sounds);

impl AlchemyItem {
    pub const RTTI: VariantID = RTTI_AlchemyItem;
    pub const VTABLE: &'static [VariantID] = &VTABLE_AlchemyItem;
    pub const FORMTYPE: FormType = FormType::AlchemyItem;

    // override (MagicItem)
    // void                                   InitializeData() override;                                             // 04
    // void                                   ClearData() override;                                                  // 05
    // void                                   InitItemImpl() override;                                               // 13
    // [[nodiscard]] MagicSystem::SpellType   GetSpellType() const override;                                         // 53
    // [[nodiscard]] MagicSystem::CastingType GetCastingType() const override;                                       // 55
    // [[nodiscard]] MagicSystem::Delivery    GetDelivery() const override;                                          // 57
    // [[nodiscard]] bool                     IsFood() const override;                                               // 5D
    // [[nodiscard]] bool                     IsPoison() const override;                                             // 61
    // [[nodiscard]] bool                     IsMedicine() const override;                                           // 62
    // [[nodiscard]] ActorValue               GetAssociatedSkill() const override;                                   // 66
    // [[nodiscard]] std::uint32_t            GetChunkID() override;                                                 // 68
    // void                                   CopyMagicItemData(MagicItem* a_src) override;                          // 69
    // void                                   LoadMagicItemChunk(TESFile* a_mod, std::uint32_t a_chunkID) override;  // 6A
    // [[nodiscard]] const MagicItem::Data*   GetData1() const override;                                             // 6C
    // [[nodiscard]] MagicItem::Data*         GetData2() override;                                                   // 6D
    // [[nodiscard]] std::uint32_t            GetDataSize() const override;                                          // 6E
    // void                                   InitFromChunk(TESFile* a_mod) override;                                // 6F
    // void                                   InitChunk() override;                                                  // 70

    // override (BGSKeywordForm)
    // [[nodiscard]] BGSKeyword* GetDefaultKeyword() const override;  // 05
}
