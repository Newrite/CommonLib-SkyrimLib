use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BGSCollisionLayer;
use crate::offsets::offsets_vtable::VTABLE_BGSCollisionLayer;
use crate::re::BSFixedString;
use crate::re::BSTArray;
use crate::re::Color;
use crate::re::FormCastable;
use crate::re::FormType;
use crate::re::TESDescription;
use crate::re::TESForm;
use crate::relocation::{RttiType, VariantID};
use core_util::EnumSet;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollisionLayerFlag {
    None = 0,
    TriggerVolume = 1 << 0,
    Sensor = 1 << 1,
    NavmeshObstacle = 1 << 2,
}

core_util::impl_enumset_type!(CollisionLayerFlag => u32);

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CollisionLayerRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct BGSCollisionLayer {
    pub base: TESForm,                                   // 0x00
    pub description: TESDescription,                     // 0x20
    pub collision_idx: u32,                              // 0x30 - BNAM
    pub debug_color: Color,                              // 0x34 - FNAM
    pub flags: EnumSet<CollisionLayerFlag, u32>,         // 0x38 - GNAM
    pub pad3c: u32,                                      // 0x3C
    pub name: BSFixedString,                             // 0x40 - MNAM
    pub collides_with: BSTArray<*mut BGSCollisionLayer>, // 0x48 - CNAM
}

const _: () = assert!(core::mem::size_of::<BGSCollisionLayer>() == 0x60);
const _: () = assert!(core::mem::offset_of!(BGSCollisionLayer, description) == 0x20);

impl RttiType for BGSCollisionLayer {
    const RTTI: VariantID = RTTI_BGSCollisionLayer;
}

impl FormCastable for BGSCollisionLayer {
    const TARGET_FORM_TYPE: FormType = FormType::CollisionLayer;
}

inherit!(BGSCollisionLayer : TESForm);
inherit!(BGSCollisionLayer => TESDescription, description);

impl BGSCollisionLayer {
    pub const RTTI: VariantID = RTTI_BGSCollisionLayer;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSCollisionLayer;
    pub const FORMTYPE: FormType = FormType::CollisionLayer;

    // override (TESForm)
    // void ClearData() override;            // 05
    // bool Load(TESFile* a_mod) override;   // 06
    // void InitItemImpl() override;         // 13
    // void SetDelete(bool a_set) override;  // 23
}
