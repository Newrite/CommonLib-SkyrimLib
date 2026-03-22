use bitflags::bitflags;
use core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_TESObject;
use crate::offsets::offsets_vtable::VTABLE_TESObject;
use crate::re::tes_form::TESForm;
use crate::re::tes_water_form::TESWaterForm;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::re::ni_av_object::NiAVObject;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChangeFlags: u32 {
        const NONE = 0;
        const FORM_FLAGS = 1 << 0;
    }
}

unsafe impl bytemuck::Zeroable for ChangeFlags {}

#[repr(C)]
pub struct TESObject {
    pub base: TESForm,
}

const _: () = assert!(core::mem::size_of::<TESObject>() == 0x20);

impl RttiType for TESObject {
    const RTTI: VariantID = RTTI_TESObject;
}

inherit!(TESObject : TESForm);

impl TESObject {
    pub const RTTI: VariantID = RTTI_TESObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObject;

    virtual_method! {
        pub const UNK_3B: usize = 0x3B;
        pub fn unk_3b(this: &TESObject) -> ()
    }

    virtual_method! {
        pub const IS_BOUND_ANIM_OBJECT: usize = 0x3C;
        pub fn is_bound_anim_object(this: &TESObject) -> bool
    }

    virtual_method! {
        pub const GET_WATER_TYPE: usize = 0x3D;
        pub fn get_water_type(this: &TESObject) -> *mut TESWaterForm
    }

    virtual_method! {
        pub const IS_AUTO_CALC: usize = 0x3E;
        pub fn is_auto_calc(this: &TESObject) -> bool
    }

    virtual_method! {
        pub const SET_AUTO_CALC: usize = 0x3F;
        pub fn set_auto_calc(this: &TESObject, auto_calc: bool) -> ()
    }

    virtual_method! {
        pub const CLONE_3D: usize = 0x40;
        pub fn clone_3d(this: &TESObject, a_ref: *mut TESObjectREFR, a_arg3: bool) -> *mut NiAVObject
    }

    virtual_method! {
        pub const UN_CLONE_3D: usize = 0x41;
        pub fn un_clone_3d(this: &TESObject, a_ref: *mut TESObjectREFR) -> ()
    }

    virtual_method! {
        pub const IS_MARKER: usize = 0x42;
        pub fn is_marker(this: &TESObject) -> bool
    }

    virtual_method! {
        pub const IS_OCCLUSION_MARKER: usize = 0x43;
        pub fn is_occlusion_marker(this: &TESObject) -> bool
    }

    virtual_method! {
        pub const REPLACE_MODEL: usize = 0x44;
        pub fn replace_model(this: &TESObject) -> bool
    }

    virtual_method! {
        pub const INC_REF: usize = 0x45;
        pub fn inc_ref(this: &TESObject) -> u32
    }

    virtual_method! {
        pub const DEC_REF: usize = 0x46;
        pub fn dec_ref(this: &TESObject) -> u32
    }

    virtual_method! {
        pub const LOAD_GRAPHICS: usize = 0x47;
        pub fn load_graphics(this: &TESObject, a_ref: *mut TESObjectREFR) -> *mut NiAVObject
    }
}