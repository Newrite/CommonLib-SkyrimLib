use crate::offsets::offsets_rtti::RTTI_BGSMenuDisplayObject;
use crate::offsets::offsets_vtable::VTABLE_BGSMenuDisplayObject;
use crate::re::TESBoundObject;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

/// C++ `RE::BGSMenuDisplayObject`
#[repr(C)]
pub struct BGSMenuDisplayObject {
    pub base: BaseFormComponent,                  // 0x00
    pub menu_display_object: *mut TESBoundObject, // 0x08 - MDOB
}

const _: () = assert!(core::mem::size_of::<BGSMenuDisplayObject>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSMenuDisplayObject, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSMenuDisplayObject, menu_display_object) == 0x08);

impl RttiType for BGSMenuDisplayObject {
    const RTTI: VariantID = RTTI_BGSMenuDisplayObject;
}

inherit!(BGSMenuDisplayObject : BaseFormComponent);

impl BGSMenuDisplayObject {
    pub const RTTI: VariantID = RTTI_BGSMenuDisplayObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSMenuDisplayObject;

    virtual_method! {
        pub const VFUNC_GET_MENU_DISPLAY_OBJECT: usize = 0x04;
        pub fn get_menu_display_object() -> *mut TESBoundObject
    }

    #[inline]
    pub fn get_menu_display_object_ref(&self) -> Option<&TESBoundObject> {
        unsafe { self.get_menu_display_object().as_ref() }
    }
}

pub trait BGSMenuDisplayObjectExt {
    fn get_menu_display_object(&self) -> *mut TESBoundObject;
    fn get_menu_display_object_ref(&self) -> Option<&TESBoundObject>;
}

impl<T: AsRef<BGSMenuDisplayObject>> BGSMenuDisplayObjectExt for T {
    fn get_menu_display_object(&self) -> *mut TESBoundObject {
        self.as_ref().get_menu_display_object()
    }

    fn get_menu_display_object_ref(&self) -> Option<&TESBoundObject> {
        self.as_ref().get_menu_display_object_ref()
    }
}
