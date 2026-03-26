use crate::re::NiPoint3;

/// C++ `RE::NiBound`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiBound {
    pub center: NiPoint3, // 00
    pub radius: f32,      // 0C
}

const _: () = assert!(core::mem::size_of::<NiBound>() == 0x10);
const _: () = assert!(core::mem::offset_of!(NiBound, center) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiBound, radius) == 0x0C);

impl NiBound {
    #[inline(always)]
    pub const fn new(center: NiPoint3, radius: f32) -> Self {
        Self { center, radius }
    }
}
