use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiCollisionObject;
use crate::offsets::offsets_rtti::RTTI_NiCollisionObject;
use crate::offsets::offsets_vtable::VTABLE_NiCollisionObject;
use crate::re::{NiAVObject, NiObject};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::NiCollisionObject`
#[repr(C)]
pub struct NiCollisionObject {
    pub base: NiObject,                // 00
    pub scene_object: *mut NiAVObject, // 10
}

const _: () = assert!(core::mem::size_of::<NiCollisionObject>() == 0x18);
const _: () = assert!(core::mem::offset_of!(NiCollisionObject, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiCollisionObject, scene_object) == 0x10);

impl RttiType for NiCollisionObject {
    const RTTI: VariantID = RTTI_NiCollisionObject;
}

impl crate::re::NiRef for NiCollisionObject {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiCollisionObject : NiObject, base);

impl NiCollisionObject {
    pub const RTTI: VariantID = RTTI_NiCollisionObject;
    pub const NI_RTTI: VariantID = NiRTTI_NiCollisionObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiCollisionObject;
}
