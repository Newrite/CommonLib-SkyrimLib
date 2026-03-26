/// C++ `RE::VRResetHMDHeight`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VRResetHMDHeight {
    // TODO: CommonLib's header leaves `VRResetHMDHeight` empty and comments out
    // its size assert with "may be unused?". Keep this 1-byte C++ empty-struct
    // ABI stand-in until a source-backed constructor or call site proves the
    // event payload has different semantics.
    pub _empty: u8, // 00
}

const _: () = assert!(core::mem::size_of::<VRResetHMDHeight>() == 0x1);
