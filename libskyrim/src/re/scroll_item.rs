use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ScrollItem;
use crate::offsets::offsets_vtable::VTABLE_ScrollItem;
use crate::re::bgs_destructible_object_form::BGSDestructibleObjectForm;
use crate::re::bgs_pickup_putdown_sounds::BGSPickupPutdownSounds;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::spell_item::SpellItem;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::re::tes_value_form::TESValueForm;
use crate::re::tes_weight_form::TESWeightForm;
use crate::relocation::{RttiType, VariantID};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ScrollItemRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::ScrollItem`
#[repr(C)]
pub struct ScrollItem {
    pub base: SpellItem,                                     // 000
    pub model_texture_swap: TESModelTextureSwap,             // 0E8
    pub destructible_object_form: BGSDestructibleObjectForm, // 120
    pub pickup_putdown_sounds: BGSPickupPutdownSounds,       // 130
    pub weight_form: TESWeightForm,                          // 148
    pub value_form: TESValueForm,                            // 158
}

const _: () = assert!(core::mem::size_of::<ScrollItem>() == 0x168);
const _: () = assert!(core::mem::offset_of!(ScrollItem, model_texture_swap) == 0xE8);
const _: () = assert!(core::mem::offset_of!(ScrollItem, destructible_object_form) == 0x120);
const _: () = assert!(core::mem::offset_of!(ScrollItem, pickup_putdown_sounds) == 0x130);
const _: () = assert!(core::mem::offset_of!(ScrollItem, weight_form) == 0x148);
const _: () = assert!(core::mem::offset_of!(ScrollItem, value_form) == 0x158);

impl RttiType for ScrollItem {
    const RTTI: VariantID = RTTI_ScrollItem;
}

impl FormCastable for ScrollItem {
    const TARGET_FORM_TYPE: FormType = FormType::Scroll;
}

inherit!(ScrollItem : SpellItem);
inherit!(ScrollItem => TESModelTextureSwap, model_texture_swap);
inherit!(ScrollItem => BGSDestructibleObjectForm, destructible_object_form);
inherit!(ScrollItem => BGSPickupPutdownSounds, pickup_putdown_sounds);
inherit!(ScrollItem => TESWeightForm, weight_form);
inherit!(ScrollItem => TESValueForm, value_form);

impl ScrollItem {
    pub const RTTI: VariantID = RTTI_ScrollItem;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ScrollItem;
    pub const FORMTYPE: FormType = FormType::Scroll;

    // override (SpellItem)
    // void                                   InitItemImpl() override;                                               // 13
    // [[nodiscard]] MagicSystem::SpellType   GetSpellType() const override;                                         // 53
    // [[nodiscard]] MagicSystem::CastingType GetCastingType() const override;                                       // 55
    // bool                                   GetSkillUsageData(SkillUsageData& a_data) const override;              // 60
    // [[nodiscard]] ActorValue               GetAssociatedSkill() const override;                                   // 66
    // void                                   LoadMagicItemChunk(TESFile* a_mod, std::uint32_t a_chunkID) override;  // 6A
}
