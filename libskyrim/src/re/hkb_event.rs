#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{hkbEventBase, hkbNode};

/// C++ `RE::hkbEvent`
#[repr(C)]
pub struct hkbEvent {
    pub base: hkbEventBase,   // 00
    pub sender: *mut hkbNode, // 10
}

const _: () = assert!(core::mem::size_of::<hkbEvent>() == 0x18);

inherit!(hkbEvent : hkbEventBase, base);
