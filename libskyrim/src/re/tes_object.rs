use crate::offsets::offsets_rtti::RTTI_TESObject;
use crate::offsets::offsets_vtable::VTABLE_TESObject;
use crate::re::ni_av_object::NiAVObject;
use crate::re::tes_form::TESForm;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::re::tes_water_form::TESWaterForm;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use bitflags::bitflags;
use core_util::inherit;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChangeFlags: u32 {
        const NONE = 0;
        const OBJECT_VALUE = 1 << 1;
        const OBJECT_FULL_NAME = 1 << 2;
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

    // override (TESForm)
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_IS_OBJECT: usize = 0x28;
        pub fn is_object() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_REF_COUNT: usize = 0x2D;
        pub fn get_ref_count() -> u32
    }

    virtual_method! {
        pub const UNK_3B: usize = 0x3B;
        pub fn unk_3b(&mut self)
    }

    virtual_method! {
        pub const IS_BOUND_ANIM_OBJECT: usize = 0x3C;
        pub fn is_bound_anim_object(&self) -> bool
    }

    virtual_method! {
        pub const GET_WATER_TYPE: usize = 0x3D;
        pub fn get_water_type(&self) -> *mut TESWaterForm
    }

    virtual_method! {
        pub const IS_AUTO_CALC: usize = 0x3E;
        pub fn is_auto_calc(&self) -> bool
    }

    virtual_method! {
        pub const SET_AUTO_CALC: usize = 0x3F;
        pub fn set_auto_calc(&mut self, auto_calc: bool)
    }

    virtual_method! {
        pub const CLONE_3D: usize = 0x40;
        pub fn clone_3d(&mut self, a_ref: *mut TESObjectREFR, a_arg3: bool) -> *mut NiAVObject
    }

    virtual_method! {
        pub const UN_CLONE_3D: usize = 0x41;
        pub fn un_clone_3d(&mut self, a_ref: *mut TESObjectREFR)
    }

    virtual_method! {
        pub const IS_MARKER: usize = 0x42;
        pub fn is_marker(&self) -> bool
    }

    virtual_method! {
        pub const IS_OCCLUSION_MARKER: usize = 0x43;
        pub fn is_occlusion_marker(&self) -> bool
    }

    virtual_method! {
        pub const REPLACE_MODEL: usize = 0x44;
        pub fn replace_model(&mut self) -> bool
    }

    virtual_method! {
        pub const INC_REF: usize = 0x45;
        pub fn inc_ref(&mut self) -> u32
    }

    virtual_method! {
        pub const DEC_REF: usize = 0x46;
        pub fn dec_ref(&mut self) -> u32
    }

    virtual_method! {
        pub const LOAD_GRAPHICS: usize = 0x47;
        pub fn load_graphics(&mut self, a_ref: *mut TESObjectREFR) -> *mut NiAVObject
    }
}

pub trait TESObjectExt {
    fn dtor(&mut self);
    fn is_object(&self) -> bool;
    fn get_ref_count(&self) -> u32;
    fn unk_3b(&mut self);
    fn is_bound_anim_object(&self) -> bool;
    fn get_water_type(&self) -> *mut TESWaterForm;
    fn is_auto_calc(&self) -> bool;
    fn set_auto_calc(&mut self, auto_calc: bool);
    fn clone_3d(&mut self, a_ref: *mut TESObjectREFR, a_arg3: bool) -> *mut NiAVObject;
    fn un_clone_3d(&mut self, a_ref: *mut TESObjectREFR);
    fn is_marker(&self) -> bool;
    fn is_occlusion_marker(&self) -> bool;
    fn replace_model(&mut self) -> bool;
    fn inc_ref(&mut self) -> u32;
    fn dec_ref(&mut self) -> u32;
    fn load_graphics(&mut self, a_ref: *mut TESObjectREFR) -> *mut NiAVObject;
}

impl<T: AsRef<TESObject> + AsMut<TESObject>> TESObjectExt for T {
    fn dtor(&mut self) {
        TESObject::dtor(self.as_mut())
    }

    fn is_object(&self) -> bool {
        TESObject::is_object(self.as_ref())
    }

    fn get_ref_count(&self) -> u32 {
        TESObject::get_ref_count(self.as_ref())
    }

    fn unk_3b(&mut self) {
        self.as_mut().unk_3b()
    }

    fn is_bound_anim_object(&self) -> bool {
        self.as_ref().is_bound_anim_object()
    }

    fn get_water_type(&self) -> *mut TESWaterForm {
        self.as_ref().get_water_type()
    }

    fn is_auto_calc(&self) -> bool {
        self.as_ref().is_auto_calc()
    }

    fn set_auto_calc(&mut self, auto_calc: bool) {
        self.as_mut().set_auto_calc(auto_calc)
    }

    fn clone_3d(&mut self, a_ref: *mut TESObjectREFR, a_arg3: bool) -> *mut NiAVObject {
        self.as_mut().clone_3d(a_ref, a_arg3)
    }

    fn un_clone_3d(&mut self, a_ref: *mut TESObjectREFR) {
        self.as_mut().un_clone_3d(a_ref)
    }

    fn is_marker(&self) -> bool {
        self.as_ref().is_marker()
    }

    fn is_occlusion_marker(&self) -> bool {
        self.as_ref().is_occlusion_marker()
    }

    fn replace_model(&mut self) -> bool {
        self.as_mut().replace_model()
    }

    fn inc_ref(&mut self) -> u32 {
        self.as_mut().inc_ref()
    }

    fn dec_ref(&mut self) -> u32 {
        self.as_mut().dec_ref()
    }

    fn load_graphics(&mut self, a_ref: *mut TESObjectREFR) -> *mut NiAVObject {
        self.as_mut().load_graphics(a_ref)
    }
}
