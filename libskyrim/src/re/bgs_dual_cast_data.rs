use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSDualCastData;
use crate::offsets::offsets_vtable::VTABLE_BGSDualCastData;
use crate::re::{
    BGSArtObject, BGSExplosion, BGSImpactDataSet, BGSProjectile, FormCastable, FormType,
    TESBoundObject, TESEffectShader, TESFile,
};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BGSDualCastDataDEF::Flags`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSDualCastDataDefFlags {
    None = 0,
    HitEffectInheritScale = 1 << 0,
    ProjectileInheritScale = 1 << 1,
    ExplosionInheritScale = 1 << 2,
}

core_util::impl_enumset_type!(BGSDualCastDataDefFlags => u32);

/// C++ `RE::BGSDualCastDataDEF`
#[repr(C)]
pub struct BGSDualCastDataDef {
    pub projectile: *mut BGSProjectile,               // 00
    pub explosion: *mut BGSExplosion,                 // 08
    pub effect_shader: *mut TESEffectShader,          // 10
    pub hit_effect_art: *mut BGSArtObject,            // 18
    pub impact_data_set: *mut BGSImpactDataSet,       // 20
    pub flags: EnumSet<BGSDualCastDataDefFlags, u32>, // 28
    pub pad2c: u32,                                   // 2C
}

const _: () = assert!(core::mem::size_of::<BGSDualCastDataDef>() == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSDualCastDataDef, projectile) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSDualCastDataDef, explosion) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSDualCastDataDef, effect_shader) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSDualCastDataDef, hit_effect_art) == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSDualCastDataDef, impact_data_set) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSDualCastDataDef, flags) == 0x28);

bitflags! {
    /// C++ `RE::BGSDualCastData::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSDualCastDataRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::BGSDualCastData`
#[repr(C)]
pub struct BGSDualCastData {
    pub base: TESBoundObject,     // 00
    pub data: BGSDualCastDataDef, // 30
}

const _: () = assert!(core::mem::size_of::<BGSDualCastData>() == 0x60);
const _: () = assert!(core::mem::offset_of!(BGSDualCastData, data) == 0x30);

impl RttiType for BGSDualCastData {
    const RTTI: VariantID = RTTI_BGSDualCastData;
}

impl FormCastable for BGSDualCastData {
    const TARGET_FORM_TYPE: FormType = FormType::DualCastData;
}

inherit!(BGSDualCastData : TESBoundObject);

impl BGSDualCastData {
    pub const RTTI: VariantID = RTTI_BGSDualCastData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSDualCastData;
    pub const FORMTYPE: FormType = FormType::DualCastData;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    // override (TESBoundObject)
    virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_file: *mut TESFile) -> bool
    }

    virtual_method! {
        pub const VFUNC_INIT_ITEM_IMPL: usize = 0x13;
        pub fn init_item_impl(&mut self)
    }
}
