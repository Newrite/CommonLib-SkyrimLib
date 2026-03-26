use crate::offsets::offsets_rtti::RTTI_BSLight;
use crate::offsets::offsets_vtable::VTABLE_BSLight;
use crate::re::{NiRef, NiRefObject};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type BSLight; }

impl RttiType for BSLight {
    const RTTI: VariantID = RTTI_BSLight;
}

impl BSLight {
    pub const RTTI: VariantID = RTTI_BSLight;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLight;
}

impl NiRef for BSLight {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const Self as *const NiRefObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const Self as *const NiRefObject)).dec_ref() }
    }
}

// TODO: This is currently a pointer-compatible partial translation of `RE::BSLight` backed by
// `BSLight.h` inheritance (`BSLight : NiRefObject`). Replace this opaque stand-in with the full
// `BSLight` layout when code needs light fields or virtual methods beyond `NiPointer<BSLight>`.
