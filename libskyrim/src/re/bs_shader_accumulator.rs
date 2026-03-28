use crate::offsets::offsets_nirtti::NiRTTI_BSShaderAccumulator;
use crate::offsets::offsets_rtti::RTTI_BSShaderAccumulator;
use crate::offsets::offsets_vtable::VTABLE_BSShaderAccumulator;
use crate::re::{NiObject, NiRef};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type BSShaderAccumulator; }

impl RttiType for BSShaderAccumulator {
    const RTTI: VariantID = RTTI_BSShaderAccumulator;
}

impl NiRef for BSShaderAccumulator {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const Self as *const NiObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const Self as *const NiObject)).dec_ref() }
    }
}

impl BSShaderAccumulator {
    pub const RTTI: VariantID = RTTI_BSShaderAccumulator;
    pub const NI_RTTI: VariantID = NiRTTI_BSShaderAccumulator;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSShaderAccumulator;
}

// TODO: This is currently a pointer-compatible partial translation of `RE::BSShaderAccumulator`
// backed by `BSShaderAccumulator.h`. Replace the opaque stand-in with the real runtime-tail
// translation when Rust needs accumulator fields or its render-path virtual surface.
