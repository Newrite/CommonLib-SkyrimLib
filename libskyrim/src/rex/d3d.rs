//! Minimal D3D ABI surface used by RE layout translations.
//!
//! Keep this module source-backed and scoped to the translated CommonLib REX
//! usage instead of mirroring the full Windows SDK.

#[allow(non_camel_case_types)]
pub type D3DBLEND = i32;
const _: () = assert!(core::mem::size_of::<D3DBLEND>() == 0x4);

#[allow(non_camel_case_types)]
pub type D3DBLENDOP = i32;
const _: () = assert!(core::mem::size_of::<D3DBLENDOP>() == 0x4);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum D3DCMPFUNC {
    Never = 1,
    Less = 2,
    Equal = 3,
    LessEqual = 4,
    Greater = 5,
    NotEqual = 6,
    GreaterEqual = 7,
    Always = 8,
}

const _: () = assert!(core::mem::size_of::<D3DCMPFUNC>() == 0x4);
