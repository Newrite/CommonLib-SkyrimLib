/// C++ `RE::BGSSaveLoadBuffer`
#[repr(C)]
pub struct BGSSaveLoadBuffer {
    pub buffer: *mut i8, // 00
}

const _: () = assert!(core::mem::size_of::<BGSSaveLoadBuffer>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BGSSaveLoadBuffer, buffer) == 0x00);

/// C++ `RE::BGSLoadGameSubBuffer`
#[repr(C)]
pub struct BGSLoadGameSubBuffer {
    pub buffer: BGSSaveLoadBuffer, // 00
}

const _: () = assert!(core::mem::size_of::<BGSLoadGameSubBuffer>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BGSLoadGameSubBuffer, buffer) == 0x00);
