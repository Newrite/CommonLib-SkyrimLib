use crate::re::{NiPoint3, NiPointer, bhkRigidBody};

/// C++ `RE::DamageImpactData`
#[repr(C)]
#[derive(Clone)]
pub struct DamageImpactData {
    pub body: NiPointer<bhkRigidBody>, // 00
    pub location: NiPoint3,            // 08
    pub normal: NiPoint3,              // 14
    pub velocity: NiPoint3,            // 20
    pub unk2c: f32,                    // 2C
    pub unk30: f32,                    // 30
    pub pad34: u32,                    // 34
}

const _: () = assert!(core::mem::size_of::<DamageImpactData>() == 0x38);
const _: () = assert!(core::mem::offset_of!(DamageImpactData, body) == 0x00);
const _: () = assert!(core::mem::offset_of!(DamageImpactData, location) == 0x08);
const _: () = assert!(core::mem::offset_of!(DamageImpactData, normal) == 0x14);
const _: () = assert!(core::mem::offset_of!(DamageImpactData, velocity) == 0x20);
const _: () = assert!(core::mem::offset_of!(DamageImpactData, unk2c) == 0x2C);
const _: () = assert!(core::mem::offset_of!(DamageImpactData, unk30) == 0x30);
