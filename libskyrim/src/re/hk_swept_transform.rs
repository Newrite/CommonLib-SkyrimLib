use crate::re::{hkQuaternion, hkVector4};

/// C++ `RE::hkSweptTransform`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkSweptTransform {
    pub center_of_mass0: hkVector4,      // 00
    pub center_of_mass1: hkVector4,      // 10
    pub rotation0: hkQuaternion,         // 20
    pub rotation1: hkQuaternion,         // 30
    pub center_of_mass_local: hkVector4, // 40
}

const _: () = assert!(core::mem::size_of::<hkSweptTransform>() == 0x50);
const _: () = assert!(core::mem::offset_of!(hkSweptTransform, center_of_mass0) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkSweptTransform, center_of_mass1) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkSweptTransform, rotation0) == 0x20);
const _: () = assert!(core::mem::offset_of!(hkSweptTransform, rotation1) == 0x30);
const _: () = assert!(core::mem::offset_of!(hkSweptTransform, center_of_mass_local) == 0x40);
