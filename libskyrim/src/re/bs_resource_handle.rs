use core::ffi::c_void;

/// Source-backed layout subset for `RE::ModelDBHandle`.
#[repr(transparent)]
pub struct ModelDBHandle {
    pub entry: *mut c_void, // 00
}

const _: () = assert!(core::mem::size_of::<ModelDBHandle>() == 0x8);
