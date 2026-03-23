use core::ffi::c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BSResourceFileID {
    pub file: u32,        // 0x00
    pub ext: [c_char; 4], // 0x04
}

const _: () = assert!(core::mem::size_of::<BSResourceFileID>() == 0x8);
