use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiAVObjectPalette;
use crate::offsets::offsets_rtti::RTTI_NiAVObjectPalette;
use crate::offsets::offsets_vtable::VTABLE_NiAVObjectPalette;
use crate::re::NiAVObject;
use crate::re::NiObject;
use crate::re::NiRef;
use crate::re::bs_fixed_string::BSFixedString;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::NiAVObjectPalette`
#[repr(C)]
pub struct NiAVObjectPalette {
    pub base: NiObject, // 00
}

const _: () = assert!(core::mem::size_of::<NiAVObjectPalette>() == 0x10);
const _: () = assert!(core::mem::offset_of!(NiAVObjectPalette, base) == 0x00);

impl RttiType for NiAVObjectPalette {
    const RTTI: VariantID = RTTI_NiAVObjectPalette;
}

impl NiRef for NiAVObjectPalette {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiAVObjectPalette : NiObject);

impl NiAVObjectPalette {
    pub const RTTI: VariantID = RTTI_NiAVObjectPalette;
    pub const NI_RTTI: VariantID = NiRTTI_NiAVObjectPalette;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiAVObjectPalette;

    // override (NiObject)
    // const NiRTTI* GetRTTI() const override;                                  // 02
    // void          LoadBinary(NiStream& a_stream) override;                    // 18
    // void          LinkObject(NiStream& a_stream) override;                    // 19
    // void          SaveBinary(NiStream& a_stream) override;                    // 1B

    virtual_method! {
        pub const VFUNC_GET_AV_OBJECT: usize = 0x25;
        pub fn get_av_object(name: *mut BSFixedString) -> *mut NiAVObject
    }

    virtual_method! {
        pub const VFUNC_SET_AV_OBJECT: usize = 0x26;
        pub fn set_av_object(name: *mut BSFixedString, object: *mut NiAVObject)
    }

    virtual_method! {
        pub const VFUNC_RESET_AND_FILL_FROM_SCENEGRAPH: usize = 0x27;
        pub fn reset_and_fill_from_scenegraph()
    }
}

pub trait NiAVObjectPaletteExt {
    fn get_av_object(&mut self, name: *mut BSFixedString) -> *mut NiAVObject;
    fn set_av_object(&mut self, name: *mut BSFixedString, object: *mut NiAVObject);
    fn reset_and_fill_from_scenegraph(&mut self);
}

impl<T: AsRef<NiAVObjectPalette> + AsMut<NiAVObjectPalette>> NiAVObjectPaletteExt for T {
    #[inline(always)]
    fn get_av_object(&mut self, name: *mut BSFixedString) -> *mut NiAVObject {
        NiAVObjectPalette::get_av_object(self.as_mut(), name)
    }

    #[inline(always)]
    fn set_av_object(&mut self, name: *mut BSFixedString, object: *mut NiAVObject) {
        NiAVObjectPalette::set_av_object(self.as_mut(), name, object)
    }

    #[inline(always)]
    fn reset_and_fill_from_scenegraph(&mut self) {
        NiAVObjectPalette::reset_and_fill_from_scenegraph(self.as_mut())
    }
}
