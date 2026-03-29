#![allow(non_camel_case_types)]

/// C++ `RE::GAcquireInterface`
#[repr(C)]
pub struct GAcquireInterface {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GAcquireInterface>() == 0x8);

impl GAcquireInterface {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_CAN_ACQUIRE: usize = 0x01; pub fn can_acquire() -> bool }
    crate::virtual_method! { pub const VFUNC_TRY_ACQUIRE: usize = 0x02; pub fn try_acquire() -> bool }
    crate::virtual_method! { pub const VFUNC_TRY_ACQUIRE_COMMIT: usize = 0x03; pub fn try_acquire_commit() -> bool }
    crate::virtual_method! { pub const VFUNC_TRY_ACQUIRE_CANCEL: usize = 0x04; pub fn try_acquire_cancel() -> bool }
}

pub trait GAcquireInterfaceExt: AsRef<GAcquireInterface> + AsMut<GAcquireInterface> {
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn can_acquire(&mut self) -> bool {
        self.as_mut().can_acquire()
    }

    #[inline(always)]
    fn try_acquire(&mut self) -> bool {
        self.as_mut().try_acquire()
    }

    #[inline(always)]
    fn try_acquire_commit(&mut self) -> bool {
        self.as_mut().try_acquire_commit()
    }

    #[inline(always)]
    fn try_acquire_cancel(&mut self) -> bool {
        self.as_mut().try_acquire_cancel()
    }
}

impl<T> GAcquireInterfaceExt for T where T: AsRef<GAcquireInterface> + AsMut<GAcquireInterface> {}
