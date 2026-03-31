use crate::re::hkVector4;

/// C++ `RE::hkpLinearCastInput`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct hkpLinearCastInput {
    pub to: hkVector4,              // 00
    pub max_extra_penetration: f32, // 10
    pub start_point_tolerance: f32, // 14
    pub pad18: u64,                 // 18
}

const _: () = assert!(core::mem::size_of::<hkpLinearCastInput>() == 0x20);
const _: () = assert!(core::mem::offset_of!(hkpLinearCastInput, to) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpLinearCastInput, max_extra_penetration) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkpLinearCastInput, start_point_tolerance) == 0x14);
