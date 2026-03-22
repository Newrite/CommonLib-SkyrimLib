#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct NiPoint3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

const _: () = assert!(core::mem::size_of::<NiPoint3>() == 0xC);
