use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpMaxSizeMotion;
use crate::offsets::offsets_vtable::VTABLE_hkpMaxSizeMotion;
use crate::re::hkpKeyframedRigidMotion;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::hkpMaxSizeMotion`
#[repr(C)]
pub struct hkpMaxSizeMotion {
    pub base: hkpKeyframedRigidMotion, // 00
}

const _: () = assert!(core::mem::size_of::<hkpMaxSizeMotion>() == 0x140);
const _: () = assert!(core::mem::offset_of!(hkpMaxSizeMotion, base) == 0x00);

impl RttiType for hkpMaxSizeMotion {
    const RTTI: VariantID = RTTI_hkpMaxSizeMotion;
}

inherit!(hkpMaxSizeMotion : hkpKeyframedRigidMotion, base);

impl hkpMaxSizeMotion {
    pub const RTTI: VariantID = RTTI_hkpMaxSizeMotion;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpMaxSizeMotion;
}
