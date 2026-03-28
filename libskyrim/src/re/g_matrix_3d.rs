/// C++ `RE::GMatrix3D`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GMatrix3D {
    pub data: [[f32; 4]; 4], // 00
}

const _: () = assert!(core::mem::size_of::<GMatrix3D>() == 0x40);
const _: () = assert!(core::mem::offset_of!(GMatrix3D, data) == 0x0);

impl GMatrix3D {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn set_matrix(&mut self, rhs: &Self) {
        self.data = rhs.data;
    }
}
