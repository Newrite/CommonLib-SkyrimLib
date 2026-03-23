use crate::offsets::offsets_rtti::RTTI_BSLensFlareSpriteRenderData;
use crate::offsets::offsets_vtable::VTABLE_BSLensFlareSpriteRenderData;
use crate::re::NiRef;
use crate::re::NiRefObject;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! {
    pub type BSLensFlareSpriteRenderData;
}

impl RttiType for BSLensFlareSpriteRenderData {
    const RTTI: VariantID = RTTI_BSLensFlareSpriteRenderData;
}

impl NiRef for BSLensFlareSpriteRenderData {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).dec_ref() }
    }
}

impl BSLensFlareSpriteRenderData {
    pub const RTTI: VariantID = RTTI_BSLensFlareSpriteRenderData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLensFlareSpriteRenderData;
}
