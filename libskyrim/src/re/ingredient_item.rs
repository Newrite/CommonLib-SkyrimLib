use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_IngredientItem;
use crate::offsets::offsets_vtable::VTABLE_IngredientItem;
use crate::re::bgs_destructible_object_form::BGSDestructibleObjectForm;
use crate::re::bgs_equip_type::BGSEquipType;
use crate::re::bgs_pickup_putdown_sounds::BGSPickupPutdownSounds;
use crate::re::effect_setting::EffectSetting;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::magic_item::MagicItem;
use crate::re::tes_icon::TESIcon;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::re::tes_value_form::TESValueForm;
use crate::re::tes_weight_form::TESWeightForm;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::IngredientItem::IngredientFlag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IngredientFlag {
    None = 0,
    CostOverride = 1 << 0,
    FoodItem = 1 << 1,
    ExtendDuration = 1 << 3,
    ReferencesPersist = 1 << 8,
}

core_util::impl_enumset_type!(IngredientFlag => u32);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct IngredientItemChangeFlags: u32 {
        const INGREDIENT_USE = 1 << 31;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct IngredientItemRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::IngredientItem::Data`
#[repr(C)]
pub struct IngredientItemData {
    pub cost_override: i32,                  // 00
    pub flags: EnumSet<IngredientFlag, u32>, // 04
}

const _: () = assert!(core::mem::size_of::<IngredientItemData>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IngredientItemData, cost_override) == 0x00);
const _: () = assert!(core::mem::offset_of!(IngredientItemData, flags) == 0x04);

/// C++ `RE::IngredientItem::GameData`
#[repr(C)]
pub struct IngredientItemGameData {
    pub known_effect_flags: u16, // 00
    pub player_uses: u16,        // 02
}

const _: () = assert!(core::mem::size_of::<IngredientItemGameData>() == 0x4);
const _: () = assert!(core::mem::offset_of!(IngredientItemGameData, known_effect_flags) == 0x00);
const _: () = assert!(core::mem::offset_of!(IngredientItemGameData, player_uses) == 0x02);

/// C++ `RE::IngredientItem`
#[repr(C)]
pub struct IngredientItem {
    pub base: MagicItem,                                     // 000
    pub model_texture_swap: TESModelTextureSwap,             // 090
    pub icon: TESIcon,                                       // 0C8
    pub weight_form: TESWeightForm,                          // 0D8
    pub equip_type: BGSEquipType,                            // 0E8
    pub destructible_object_form: BGSDestructibleObjectForm, // 0F8
    pub pickup_putdown_sounds: BGSPickupPutdownSounds,       // 108
    pub value_form: TESValueForm,                            // 120
    pub data: IngredientItemData,                            // 130 - ENIT
    pub game_data: IngredientItemGameData,                   // 138
    pub pad13c: u32,                                         // 13C
}

const _: () = assert!(core::mem::size_of::<IngredientItem>() == 0x140);
const _: () = assert!(core::mem::offset_of!(IngredientItem, model_texture_swap) == 0x90);
const _: () = assert!(core::mem::offset_of!(IngredientItem, icon) == 0xC8);
const _: () = assert!(core::mem::offset_of!(IngredientItem, weight_form) == 0xD8);
const _: () = assert!(core::mem::offset_of!(IngredientItem, equip_type) == 0xE8);
const _: () = assert!(core::mem::offset_of!(IngredientItem, destructible_object_form) == 0xF8);
const _: () = assert!(core::mem::offset_of!(IngredientItem, pickup_putdown_sounds) == 0x108);
const _: () = assert!(core::mem::offset_of!(IngredientItem, value_form) == 0x120);
const _: () = assert!(core::mem::offset_of!(IngredientItem, data) == 0x130);
const _: () = assert!(core::mem::offset_of!(IngredientItem, game_data) == 0x138);
const _: () = assert!(core::mem::offset_of!(IngredientItem, pad13c) == 0x13C);

impl RttiType for IngredientItem {
    const RTTI: VariantID = RTTI_IngredientItem;
}

impl FormCastable for IngredientItem {
    const TARGET_FORM_TYPE: FormType = FormType::Ingredient;
}

inherit!(IngredientItem : MagicItem);
inherit!(IngredientItem => TESModelTextureSwap, model_texture_swap);
inherit!(IngredientItem => TESIcon, icon);
inherit!(IngredientItem => TESWeightForm, weight_form);
inherit!(IngredientItem => BGSEquipType, equip_type);
inherit!(IngredientItem => BGSDestructibleObjectForm, destructible_object_form);
inherit!(IngredientItem => BGSPickupPutdownSounds, pickup_putdown_sounds);
inherit!(IngredientItem => TESValueForm, value_form);

impl IngredientItem {
    pub const RTTI: VariantID = RTTI_IngredientItem;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IngredientItem;
    pub const FORMTYPE: FormType = FormType::Ingredient;

    // override (MagicItem)
    // void                     InitializeData() override;                                             // 04
    // void                     ClearData() override;                                                  // 05
    // void                     SaveGame(BGSSaveFormBuffer* a_buf) override;                           // 0E
    // void                     LoadGame(BGSLoadFormBuffer* a_buf) override;                           // 0F
    // void                     Revert(BGSLoadFormBuffer* a_buf) override;                             // 12
    // void                     InitItemImpl() override;                                               // 13
    // MagicSystem::SpellType   GetSpellType() const override;                                         // 53
    // MagicSystem::CastingType GetCastingType() const override;                                       // 55
    // MagicSystem::Delivery    GetDelivery() const override;                                          // 57
    // bool                     IsFood() const override;                                               // 5D
    // bool                     GetSkillUsageData(SkillUsageData& a_data) const override;              // 60
    // std::uint32_t            GetMaxEffectCount() const override;                                    // 65
    // ActorValue               GetAssociatedSkill() const override;                                   // 66
    // std::uint32_t            GetChunkID() override;                                                 // 68
    // void                     CopyMagicItemData(MagicItem* a_src) override;                          // 69
    // void                     LoadMagicItemChunk(TESFile* a_mod, std::uint32_t a_chunkID) override;  // 6A
    // const MagicItem::Data*   GetData1() const override;                                             // 6C
    // MagicItem::Data*         GetData2() override;                                                   // 6D
    // std::uint32_t            GetDataSize() const override;                                          // 6E
    // void                     InitFromChunk(TESFile* a_mod) override;                                // 6F
    // void                     InitChunk() override;                                                  // 70

    // override (BGSKeywordForm)
    // [[nodiscard]] BGSKeyword* GetDefaultKeyword() const override;  // 05

    #[inline(always)]
    fn tes_form_mut(&mut self) -> &mut crate::re::TESForm {
        &mut self.base.base.base.base
    }

    #[inline]
    pub fn learn_effect(&mut self, index: u32) -> bool {
        if index < 4 {
            self.game_data.known_effect_flags |= 1u16 << index;
            self.tes_form_mut()
                .add_change(IngredientItemChangeFlags::INGREDIENT_USE.bits());
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn learn_effect_by_setting(&mut self, effect_setting: *mut EffectSetting) -> bool {
        if effect_setting.is_null() {
            return false;
        }

        let mut index = 0;
        for &effect in unsafe { self.base.effects.as_slice() } {
            if !effect.is_null() && unsafe { (*effect).base_effect == effect_setting } {
                let _ = self.learn_effect(index);
                return true;
            }
            index += 1;
        }

        false
    }

    #[inline]
    pub fn learn_next_effect(&mut self) -> Option<u32> {
        let flags = self.game_data.known_effect_flags;
        let mut index = 0;
        while (flags & (1u16 << index)) != 0 {
            index += 1;
            if index >= 4 {
                return None;
            }
        }

        let _ = self.learn_effect(index);
        Some(index)
    }

    #[inline]
    pub fn learn_all_effects(&mut self) {
        for index in 0..4 {
            let _ = self.learn_effect(index);
        }
    }
}
