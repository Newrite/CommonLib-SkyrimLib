#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_hkpConvexShape;
use crate::offsets::offsets_vtable::VTABLE_hkpConvexShape;
use crate::re::hkpShape;
use crate::relocation::{RttiType, VariantID};

// TODO: SOURCE - replace this opaque stand-in with the full `hkpConvexShape` inheritance
// chain once a consumer needs its layout or virtual surface; source: `hkpShape.h` only uses
// it through pointer parameters in `WeldContactPointFunc`.
crate::core_util::abstract_type! { pub type hkpConvexShape; }

impl AsRef<hkpShape> for hkpConvexShape {
    #[inline(always)]
    fn as_ref(&self) -> &hkpShape {
        unsafe { &*(self as *const Self as *const hkpShape) }
    }
}

impl AsMut<hkpShape> for hkpConvexShape {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut hkpShape {
        unsafe { &mut *(self as *mut Self as *mut hkpShape) }
    }
}

impl RttiType for hkpConvexShape {
    const RTTI: VariantID = RTTI_hkpConvexShape;
}

impl hkpConvexShape {
    pub const RTTI: VariantID = RTTI_hkpConvexShape;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpConvexShape;
}
