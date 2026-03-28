use core::ffi::c_char;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ImageSpaceModifierInstance;
use crate::offsets::offsets_vtable::VTABLE_ImageSpaceModifierInstance;
use crate::re::{ImageSpaceModifierInstanceForm, NiAVObject, NiObject, NiPointer, NiRef};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::ImageSpaceModifierInstance`
#[repr(C)]
pub struct ImageSpaceModifierInstance {
    pub base: NiObject,                // 00
    pub unk10: u32,                    // 10
    pub strength: f32,                 // 14
    pub target: NiPointer<NiAVObject>, // 18
    pub age: f32,                      // 20
    pub flags: u32,                    // 24
}

const _: () = assert!(core::mem::size_of::<ImageSpaceModifierInstance>() == 0x28);
const _: () = assert!(core::mem::offset_of!(ImageSpaceModifierInstance, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ImageSpaceModifierInstance, strength) == 0x14);
const _: () = assert!(core::mem::offset_of!(ImageSpaceModifierInstance, target) == 0x18);
const _: () = assert!(core::mem::offset_of!(ImageSpaceModifierInstance, age) == 0x20);
const _: () = assert!(core::mem::offset_of!(ImageSpaceModifierInstance, flags) == 0x24);

inherit!(ImageSpaceModifierInstance : NiObject);

impl RttiType for ImageSpaceModifierInstance {
    const RTTI: VariantID = RTTI_ImageSpaceModifierInstance;
}

impl NiRef for ImageSpaceModifierInstance {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

impl ImageSpaceModifierInstance {
    pub const RTTI: VariantID = RTTI_ImageSpaceModifierInstance;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ImageSpaceModifierInstance;

    // override (NiObject)
    // ~ImageSpaceModifierInstance() override; // 00

    virtual_method! {
        pub const VFUNC_UNK_25: usize = 0x25;
        pub fn unk_25(&mut self) -> bool
    }

    virtual_method! {
        pub const VFUNC_APPLY: usize = 0x26;
        pub fn apply(&mut self)
    }

    virtual_method! {
        pub const VFUNC_IS_FORM: usize = 0x27;
        pub fn is_form(&mut self) -> *mut ImageSpaceModifierInstanceForm
    }

    virtual_method! {
        pub const VFUNC_PRINT_INFO: usize = 0x28;
        pub fn print_info(&mut self, dst_buf: *mut c_char)
    }
}

pub trait ImageSpaceModifierInstanceExt {
    fn unk_25(&mut self) -> bool;
    fn apply(&mut self);
    fn is_form(&mut self) -> *mut ImageSpaceModifierInstanceForm;
    fn print_info(&mut self, dst_buf: *mut c_char);
}

impl<T> ImageSpaceModifierInstanceExt for T
where
    T: AsRef<ImageSpaceModifierInstance> + AsMut<ImageSpaceModifierInstance>,
{
    #[inline(always)]
    fn unk_25(&mut self) -> bool {
        ImageSpaceModifierInstance::unk_25(self.as_mut())
    }

    #[inline(always)]
    fn apply(&mut self) {
        ImageSpaceModifierInstance::apply(self.as_mut())
    }

    #[inline(always)]
    fn is_form(&mut self) -> *mut ImageSpaceModifierInstanceForm {
        ImageSpaceModifierInstance::is_form(self.as_mut())
    }

    #[inline(always)]
    fn print_info(&mut self, dst_buf: *mut c_char) {
        ImageSpaceModifierInstance::print_info(self.as_mut(), dst_buf)
    }
}
