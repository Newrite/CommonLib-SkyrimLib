use crate::core_util::inherit;
use crate::re::hkMatrix3;

/// C++ `RE::hkRotation`
#[repr(C, align(16))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkRotation {
    pub base: hkMatrix3, // 00
}

const _: () = assert!(core::mem::size_of::<hkRotation>() == 0x30);
const _: () = assert!(core::mem::offset_of!(hkRotation, base) == 0x00);

inherit!(hkRotation : hkMatrix3, base);
