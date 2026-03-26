#![allow(non_camel_case_types)]

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESAmmo;
use crate::offsets::offsets_vtable::VTABLE_TESAmmo;
use crate::re::{
    BGSDestructibleObjectForm, BGSKeywordForm, BGSMessageIcon, BGSPickupPutdownSounds,
    BGSProjectile, BSFixedString, FormCastable, FormType, TESBoundObject, TESDescription,
    TESFullName, TESIcon, TESModelTextureSwap, TESValueForm, TESWeightForm,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESAmmoDataFlag {
    None = 0,
    IgnoresNormalWeaponResistance = 1 << 0,
    NonPlayable = 1 << 1,
    NonBolt = 1 << 2,
}

core_util::impl_enumset_type!(TESAmmoDataFlag => u8);

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESAmmoRecordFlags: u32 {
        const NON_PLAYABLE = 1 << 2;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct TESAmmoData {
    pub projectile: *mut BGSProjectile,
    pub flags: EnumSet<TESAmmoDataFlag, u8>,
    pub pa09: u8,
    pub pa0a: u16,
    pub damage: f32,
}

const _: () = assert!(core::mem::size_of::<TESAmmoData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESAmmoData, projectile) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESAmmoData, flags) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESAmmoData, damage) == 0x0C);

#[repr(C)]
pub struct TESAmmoRuntimeData {
    pub data: TESAmmoData,
    pub short_desc: BSFixedString,
}

const _: () = assert!(core::mem::size_of::<TESAmmoRuntimeData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(TESAmmoRuntimeData, data) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESAmmoRuntimeData, short_desc) == 0x10);

#[repr(C)]
pub struct TESAmmo {
    pub base: TESBoundObject,
    pub full_name: TESFullName,
    pub model_texture_swap: TESModelTextureSwap,
    pub icon: TESIcon,
    pub message_icon: BGSMessageIcon,
    pub value_form: TESValueForm,
}

const _: () = assert!(core::mem::size_of::<TESAmmo>() == 0xB0);
const _: () = assert!(core::mem::offset_of!(TESAmmo, full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESAmmo, model_texture_swap) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESAmmo, icon) == 0x78);
const _: () = assert!(core::mem::offset_of!(TESAmmo, message_icon) == 0x88);
const _: () = assert!(core::mem::offset_of!(TESAmmo, value_form) == 0xA0);

inherit!(TESAmmo : TESBoundObject);
inherit!(TESAmmo => TESFullName, full_name);
inherit!(TESAmmo => TESModelTextureSwap, model_texture_swap);
inherit!(TESAmmo => TESIcon, icon);
inherit!(TESAmmo => BGSMessageIcon, message_icon);
inherit!(TESAmmo => TESValueForm, value_form);

impl RttiType for TESAmmo {
    const RTTI: VariantID = RTTI_TESAmmo;
}

impl FormCastable for TESAmmo {
    const TARGET_FORM_TYPE: FormType = FormType::Ammo;
}

impl TESAmmo {
    pub const RTTI: VariantID = RTTI_TESAmmo;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESAmmo;
    pub const FORMTYPE: FormType = FormType::Ammo;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x110, 0x110, 0x100);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x128, 0x128, 0x118);
    pub const WEIGHT_FORM_OFFSET: VariantOffset = VariantOffset::new(0xB0, 0xB0, 0x0);
    pub const DESTRUCTIBLE_OBJECT_FORM_OFFSET: VariantOffset = VariantOffset::new(0xC0, 0xC0, 0xB0);
    pub const PICKUP_PUTDOWN_SOUNDS_OFFSET: VariantOffset = VariantOffset::new(0xD0, 0xD0, 0xC0);
    pub const DESCRIPTION_FORM_OFFSET: VariantOffset = VariantOffset::new(0xE8, 0xE8, 0xD8);
    pub const KEYWORD_FORM_OFFSET: VariantOffset = VariantOffset::new(0xF8, 0xF8, 0xE8);

    crate::runtime_data_accessor! {
        pub fn get_runtime_data() -> TESAmmoRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_runtime_data_mut() -> TESAmmoRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_optional_data_accessor! {
        pub fn as_weight_form() -> TESWeightForm {
            offset: Self::WEIGHT_FORM_OFFSET
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn as_weight_form_mut() -> TESWeightForm {
            offset: Self::WEIGHT_FORM_OFFSET
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_destructible_object_form() -> BGSDestructibleObjectForm {
            offset: Self::DESTRUCTIBLE_OBJECT_FORM_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_destructible_object_form_mut() -> BGSDestructibleObjectForm {
            offset: Self::DESTRUCTIBLE_OBJECT_FORM_OFFSET
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_pickup_putdown_sounds_form() -> BGSPickupPutdownSounds {
            offset: Self::PICKUP_PUTDOWN_SOUNDS_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_pickup_putdown_sounds_form_mut() -> BGSPickupPutdownSounds {
            offset: Self::PICKUP_PUTDOWN_SOUNDS_OFFSET
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_description_form() -> TESDescription {
            offset: Self::DESCRIPTION_FORM_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_description_form_mut() -> TESDescription {
            offset: Self::DESCRIPTION_FORM_OFFSET
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_keyword_form() -> BGSKeywordForm {
            offset: Self::KEYWORD_FORM_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_keyword_form_mut() -> BGSKeywordForm {
            offset: Self::KEYWORD_FORM_OFFSET
        }
    }

    #[inline(always)]
    pub fn ignores_normal_weapon_resistance(&self) -> bool {
        self.get_runtime_data()
            .data
            .flags
            .all(TESAmmoDataFlag::IgnoresNormalWeaponResistance)
    }

    #[inline(always)]
    pub fn is_bolt(&self) -> bool {
        !self
            .get_runtime_data()
            .data
            .flags
            .all(TESAmmoDataFlag::NonBolt)
    }

    // override (TESBoundObject)
    // void        InitializeData() override;                                           // 04
    // bool        Load(TESFile* a_mod) override;                                       // 06
    // void        SaveGame(BGSSaveFormBuffer* a_buf) override;                         // 0E
    // void        LoadGame(BGSLoadFormBuffer* a_buf) override;                         // 0F
    // void        InitItemImpl() override;                                             // 13
    // bool        GetPlayable() const override;                                        // 19
    // NiAVObject* Clone3D(TESObjectREFR* a_ref, bool a_arg3) override;                 // 40
    // void        HandleRemoveItemFromContainer(TESObjectREFR* a_container) override;  // 4E

    // override (BGSKeywordForm) [flat only]
    // [[nodiscard]] BGSKeyword* GetDefaultKeyword() const override;  // 05
}
