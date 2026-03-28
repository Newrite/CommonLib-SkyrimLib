use crate::offsets::offsets_nirtti::NiRTTI_NiSkinInstance;
use crate::offsets::offsets_rtti::RTTI_NiSkinInstance;
use crate::offsets::offsets_vtable::VTABLE_NiSkinInstance;
use crate::re::{NiObject, NiRef};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type NiSkinInstance; }

impl RttiType for NiSkinInstance {
    const RTTI: VariantID = RTTI_NiSkinInstance;
}

impl NiRef for NiSkinInstance {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const Self as *const NiObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const Self as *const NiObject)).dec_ref() }
    }
}

impl NiSkinInstance {
    pub const RTTI: VariantID = RTTI_NiSkinInstance;
    pub const NI_RTTI: VariantID = NiRTTI_NiSkinInstance;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiSkinInstance;
}

// TODO: This is currently a pointer-compatible partial translation of `RE::NiSkinInstance`
// backed by `NiSkinInstance.h`. Replace the opaque stand-in with the full cross-runtime layout
// when Rust needs the skinner fields or `Create()` helper surface.
