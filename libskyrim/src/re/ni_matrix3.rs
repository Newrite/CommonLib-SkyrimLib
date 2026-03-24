/// C++ `RE::NiMatrix3`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiMatrix3 {
    pub entry: [[f32; 3]; 3], // 00
}

const _: () = assert!(core::mem::size_of::<NiMatrix3>() == 0x24);
const _: () = assert!(core::mem::offset_of!(NiMatrix3, entry) == 0x00);
