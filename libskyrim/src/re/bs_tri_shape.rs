use crate::offsets::offsets_nirtti::NiRTTI_BSTriShape;
use crate::offsets::offsets_rtti::RTTI_BSTriShape;
use crate::offsets::offsets_vtable::VTABLE_BSTriShape;
use crate::re::{NiAVObject, NiRef};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type BSTriShape; }

impl RttiType for BSTriShape {
    const RTTI: VariantID = RTTI_BSTriShape;
}

impl BSTriShape {
    pub const RTTI: VariantID = RTTI_BSTriShape;
    pub const NI_RTTI: VariantID = NiRTTI_BSTriShape;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSTriShape;
}

impl NiRef for BSTriShape {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const Self as *const NiAVObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const Self as *const NiAVObject)).dec_ref() }
    }
}

// TODO: This is currently a pointer-compatible partial translation of `RE::BSTriShape` backed by
// `BSTriShape.h` inheritance (`BSTriShape : BSGeometry : NiAVObject`). Replace this opaque
// stand-in with an honest runtime-layout translation when code needs `BSTriShape` fields or
// virtual surface.
