#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GAcquireInterface, GMutexImpl, GWaitable};

/// C++ `RE::GMutex`
#[repr(C)]
pub struct GMutex {
    pub base: GWaitable,                      // 00
    pub acquire_interface: GAcquireInterface, // 18
    pub impl_: *mut GMutexImpl,               // 20
}

const _: () = assert!(core::mem::size_of::<GMutex>() == 0x28);
const _: () = assert!(core::mem::offset_of!(GMutex, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GMutex, acquire_interface) == 0x18);
const _: () = assert!(core::mem::offset_of!(GMutex, impl_) == 0x20);

inherit!(GMutex : GWaitable, base);
inherit!(GMutex => GAcquireInterface, acquire_interface);
