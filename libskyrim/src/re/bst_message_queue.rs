use core::marker::PhantomData;
use core::mem::MaybeUninit;

use core_util::inherit;

use crate::virtual_method;

/// C++ `RE::BSTMessageQueue<T>`
#[repr(C)]
pub struct BSTMessageQueue<T> {
    pub vtable: *const usize, // 00
    pub _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<BSTMessageQueue<*mut core::ffi::c_void>>() == 0x8);
const _: () =
    assert!(core::mem::offset_of!(BSTMessageQueue<*mut core::ffi::c_void>, vtable) == 0x00);

impl<T> BSTMessageQueue<T> {
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_PUSH: usize = 0x01;
        pub fn push(&mut self, obj: *mut T) -> bool
    }

    virtual_method! {
        pub const VFUNC_TRY_PUSH: usize = 0x02;
        pub fn try_push(&mut self, obj: *mut T) -> bool
    }

    virtual_method! {
        pub const VFUNC_POP: usize = 0x03;
        pub fn pop(&mut self, obj: *mut T) -> bool
    }

    virtual_method! {
        pub const VFUNC_TRY_POP: usize = 0x04;
        pub fn try_pop(&mut self, obj: *mut T) -> bool
    }
}

/// C++ `RE::BSTCommonMessageQueue<T>`
#[repr(C)]
pub struct BSTCommonMessageQueue<T> {
    pub base: BSTMessageQueue<T>, // 00
    pub lock: u32,                // 08
    pub pad0c: u32,               // 0C
}

const _: () =
    assert!(core::mem::size_of::<BSTCommonMessageQueue<*mut core::ffi::c_void>>() == 0x10);
const _: () =
    assert!(core::mem::offset_of!(BSTCommonMessageQueue<*mut core::ffi::c_void>, base) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(BSTCommonMessageQueue<*mut core::ffi::c_void>, lock) == 0x08);

inherit!(for[T] BSTCommonMessageQueue<T> : BSTMessageQueue<T>);

impl<T> BSTCommonMessageQueue<T> {
    // override (BSTMessageQueue<T>)
    // ~BSTCommonMessageQueue() override; // 00
    // bool Push(T* a_obj) override;      // 01
    // bool TryPush(T* a_obj) override;   // 02
    // bool Pop(T* a_obj) override;       // 03
    // bool TryPop(T* a_obj) override;    // 04

    virtual_method! {
        pub const VFUNC_PUSH_INTERNAL: usize = 0x05;
        pub fn push_internal(&mut self, obj: *mut T) -> bool
    }

    virtual_method! {
        pub const VFUNC_POP_INTERNAL: usize = 0x06;
        pub fn pop_internal(&mut self, obj: *mut T) -> bool
    }
}

/// C++ `RE::BSTCommonStaticMessageQueue<T, SIZE>`
#[repr(C)]
pub struct BSTCommonStaticMessageQueue<T, const SIZE: usize> {
    pub base: BSTCommonMessageQueue<T>,       // 00
    pub queue_buffer: [MaybeUninit<T>; SIZE], // 10
    pub num_entries: u32,                     // 10 + sizeof(T) * SIZE
    pub push_idx: u32,                        // ...
    pub pop_idx: u32,                         // ...
}

const _: () =
    assert!(core::mem::size_of::<BSTCommonStaticMessageQueue<*mut core::ffi::c_void, 1>>() == 0x28);
const _: () = assert!(
    core::mem::offset_of!(BSTCommonStaticMessageQueue<*mut core::ffi::c_void, 1>, base) == 0x00
);
const _: () = assert!(
    core::mem::offset_of!(
        BSTCommonStaticMessageQueue<*mut core::ffi::c_void, 1>,
        queue_buffer
    ) == 0x10
);
const _: () = assert!(
    core::mem::offset_of!(
        BSTCommonStaticMessageQueue<*mut core::ffi::c_void, 1>,
        num_entries
    ) == 0x18
);

inherit!(for[T, const SIZE: usize] BSTCommonStaticMessageQueue<T, SIZE> : BSTCommonMessageQueue<T>);

impl<T, const SIZE: usize> BSTCommonStaticMessageQueue<T, SIZE> {
    // override (BSTCommonMessageQueue<T>)
    // ~BSTCommonStaticMessageQueue() override; // 00
    // bool PushInternal(T* a_obj) override;    // 05
    // bool PopInternal(T* a_obj) override;     // 06
}
