#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_ImageSpaceModifierInstanceDOF;
use crate::offsets::offsets_vtable::VTABLE_ImageSpaceModifierInstanceDOF;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type ImageSpaceModifierInstanceDOF; }

impl RttiType for ImageSpaceModifierInstanceDOF {
    const RTTI: VariantID = RTTI_ImageSpaceModifierInstanceDOF;
}

impl ImageSpaceModifierInstanceDOF {
    pub const RTTI: VariantID = RTTI_ImageSpaceModifierInstanceDOF;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ImageSpaceModifierInstanceDOF;
}
