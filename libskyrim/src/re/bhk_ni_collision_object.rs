#![allow(non_camel_case_types)]

use core_util::EnumSet;

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_bhkNiCollisionObject;
use crate::offsets::offsets_rtti::RTTI_bhkNiCollisionObject;
use crate::offsets::offsets_vtable::VTABLE_bhkNiCollisionObject;
use crate::re::NiCollisionObject;
use crate::re::bhk_world_object::bhkWorldObject;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::bhkNiCollisionObject::Flag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum bhkNiCollisionObjectFlag {
    Active = 1 << 0,
    Notify = 1 << 2,
    SetLocal = 1 << 3,
    DebugDisplay = 1 << 4,
    UseVelocity = 1 << 5,
    Reset = 1 << 6,
    SyncOnUpdate = 1 << 7,
    AnimTargeted = 1 << 10,
    DismemberLimb = 1 << 11,
}

core_util::impl_enumset_type!(bhkNiCollisionObjectFlag => u32);

/// C++ `RE::bhkNiCollisionObject`
#[repr(C)]
pub struct bhkNiCollisionObject {
    pub base: NiCollisionObject,                       // 00
    pub flags: EnumSet<bhkNiCollisionObjectFlag, u32>, // 18
    pub pad1c: u32,                                    // 1C
    pub body: crate::re::NiPointer<bhkWorldObject>,    // 20
}

const _: () = assert!(core::mem::size_of::<bhkNiCollisionObject>() == 0x28);
const _: () = assert!(core::mem::offset_of!(bhkNiCollisionObject, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(bhkNiCollisionObject, flags) == 0x18);
const _: () = assert!(core::mem::offset_of!(bhkNiCollisionObject, body) == 0x20);

impl RttiType for bhkNiCollisionObject {
    const RTTI: VariantID = RTTI_bhkNiCollisionObject;
}

impl crate::re::NiRef for bhkNiCollisionObject {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(bhkNiCollisionObject : NiCollisionObject, base);

impl bhkNiCollisionObject {
    pub const RTTI: VariantID = RTTI_bhkNiCollisionObject;
    pub const NI_RTTI: VariantID = NiRTTI_bhkNiCollisionObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_bhkNiCollisionObject;
}
