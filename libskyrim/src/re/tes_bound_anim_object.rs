use crate::offsets::offsets_rtti::RTTI_TESBoundAnimObject;
use crate::offsets::offsets_vtable::VTABLE_TESBoundAnimObject;
use crate::re::tes_bound_object::TESBoundObject;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

#[repr(C)]
pub struct TESBoundAnimObject {
    pub base: TESBoundObject, // 00
}

const _: () = assert!(core::mem::size_of::<TESBoundAnimObject>() == 0x30);

impl RttiType for TESBoundAnimObject {
    const RTTI: VariantID = RTTI_TESBoundAnimObject;
}

inherit!(TESBoundAnimObject : TESBoundObject);

impl TESBoundAnimObject {
    pub const RTTI: VariantID = RTTI_TESBoundAnimObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESBoundAnimObject;

    // override (TESBoundObject)
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_IS_BOUND_ANIM_OBJECT: usize = 0x3C;
        pub fn is_bound_anim_object() -> bool
    }

    virtual_method! {
        pub const VFUNC_REPLACE_MODEL: usize = 0x4B;
        pub fn replace_model(model_path: *const core::ffi::c_char) -> bool
    }
}

pub trait TESBoundAnimObjectExt {
    fn dtor(&mut self);
    fn is_bound_anim_object(&self) -> bool;
    fn replace_model(&mut self, model_path: *const core::ffi::c_char) -> bool;
}

impl<T: AsRef<TESBoundAnimObject> + AsMut<TESBoundAnimObject>> TESBoundAnimObjectExt for T {
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    fn is_bound_anim_object(&self) -> bool {
        self.as_ref().is_bound_anim_object()
    }

    fn replace_model(&mut self, model_path: *const core::ffi::c_char) -> bool {
        self.as_mut().replace_model(model_path)
    }
}
