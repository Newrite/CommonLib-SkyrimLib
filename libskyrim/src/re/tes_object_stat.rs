use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESObjectSTAT;
use crate::offsets::offsets_vtable::VTABLE_TESObjectSTAT;
use crate::re::bgs_material_object::BGSMaterialObject;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::relocation::{RttiType, VariantID};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESObjectSTATDataFlag {
    None = 0,
    ConsideredSnow = 1 << 0,
}

core_util::impl_enumset_type!(TESObjectSTATDataFlag => u32);

#[repr(C)]
pub struct TESObjectSTATData {
    pub material_threshold_angle: f32,              // 00
    pub pad04: u32,                                 // 04
    pub material_obj: *mut BGSMaterialObject,       // 08
    pub flags: EnumSet<TESObjectSTATDataFlag, u32>, // 10
    pub pad14: u32,                                 // 14
}

const _: () = assert!(core::mem::size_of::<TESObjectSTATData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(TESObjectSTATData, material_threshold_angle) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESObjectSTATData, material_obj) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESObjectSTATData, flags) == 0x10);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESObjectSTATRecordFlags: u32 {
        const NEVER_FADES = 1 << 2;
        const DELETED = 1 << 5;
        const IS_SKY_OBJECT = 1 << 5;
        const HAS_TREE_LOD = 1 << 6;
        const ADD_ON_LOD_OBJECT = 1 << 7;
        const HIDDEN_FROM_LOCAL_MAP = 1 << 9;
        const HAS_DISTANT_LOD = 1 << 15;
        const USES_HDLOD_TEXTURE = 1 << 17;
        const HAS_CURRENTS = 1 << 19;
        const IS_MARKER = 1 << 23;
        const OBSTACLE = 1 << 25;
        const NAV_MESH_GENERATION_FILTER = 1 << 26;
        const NAV_MESH_GENERATION_BOUNDING_BOX = 1 << 27;
        const SHOW_IN_WORLD_MAP = 1 << 28;
        const NAV_MESH_GENERATION_GROUND = 1 << 30;
    }
}

#[repr(C)]
pub struct TESObjectSTAT {
    pub base: TESBoundObject,                    // 00
    pub model_texture_swap: TESModelTextureSwap, // 30
    pub data: TESObjectSTATData,                 // 68
}

const _: () = assert!(core::mem::size_of::<TESObjectSTAT>() == 0x80);
const _: () = assert!(core::mem::offset_of!(TESObjectSTAT, model_texture_swap) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESObjectSTAT, data) == 0x68);

impl RttiType for TESObjectSTAT {
    const RTTI: VariantID = RTTI_TESObjectSTAT;
}

impl FormCastable for TESObjectSTAT {
    const TARGET_FORM_TYPE: FormType = FormType::Static;
}

inherit!(TESObjectSTAT : TESBoundObject);
inherit!(TESObjectSTAT => TESModelTextureSwap, model_texture_swap);

impl TESObjectSTAT {
    pub const RTTI: VariantID = RTTI_TESObjectSTAT;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectSTAT;
    pub const FORMTYPE: FormType = FormType::Static;

    // override (TESBoundObject)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    #[inline(always)]
    pub const fn get_playable(&self) -> bool {
        true
    }

    #[inline(always)]
    pub fn is_heading_marker(&self) -> bool {
        (self.base.base.base.form_flags.bits() & TESObjectSTATRecordFlags::NEVER_FADES.bits()) != 0
    }

    #[inline(always)]
    pub fn has_tree_lod(&self) -> bool {
        (self.base.base.base.form_flags.bits() & TESObjectSTATRecordFlags::HAS_TREE_LOD.bits()) != 0
    }

    #[inline(always)]
    pub fn is_sky_object(&self) -> bool {
        (self.base.base.base.form_flags.bits() & TESObjectSTATRecordFlags::IS_SKY_OBJECT.bits())
            != 0
    }

    #[inline(always)]
    pub fn is_snow_object(&self) -> bool {
        self.data.flags.all(TESObjectSTATDataFlag::ConsideredSnow)
    }

    // void ClearData() override;              // 05
    // bool Load(TESFile* a_mod) override;     // 06
    // void InitItemImpl() override;           // 13
    // bool GetPlayable() const override;      // 19
    // bool IsHeadingMarker() const override;  // 1A
}

pub trait TESObjectSTATExt {
    fn dtor(&mut self);
    fn get_playable(&self) -> bool;
    fn is_heading_marker(&self) -> bool;
    fn has_tree_lod(&self) -> bool;
    fn is_sky_object(&self) -> bool;
    fn is_snow_object(&self) -> bool;
}

impl<T: AsRef<TESObjectSTAT> + AsMut<TESObjectSTAT>> TESObjectSTATExt for T {
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    fn get_playable(&self) -> bool {
        self.as_ref().get_playable()
    }

    fn is_heading_marker(&self) -> bool {
        self.as_ref().is_heading_marker()
    }

    fn has_tree_lod(&self) -> bool {
        self.as_ref().has_tree_lod()
    }

    fn is_sky_object(&self) -> bool {
        self.as_ref().is_sky_object()
    }

    fn is_snow_object(&self) -> bool {
        self.as_ref().is_snow_object()
    }
}
