#![allow(non_camel_case_types)]

use crate::re::{hkRefPtr, hkReferencedObject};

/// Partial C++ `RE::hkRefVariant`.
///
/// CommonLib models this as a thin derived wrapper over `hkRefPtr<hkReferencedObject>`.
#[repr(transparent)]
pub struct hkRefVariant {
    pub object: hkRefPtr<hkReferencedObject>, // 00
}

const _: () = assert!(core::mem::size_of::<hkRefVariant>() == 0x08);
