#![allow(non_camel_case_types)]

/// C++ `RE::hkArrayBase<T>`
#[repr(C)]
pub struct hkArrayBase<T> {
    pub data: *mut T,            // 00
    pub size: i32,               // 08
    pub capacity_and_flags: i32, // 0C
}

const _: () = assert!(core::mem::size_of::<hkArrayBase<*mut core::ffi::c_void>>() == 0x10);

/// C++ `RE::hkArray<T>`
#[repr(C)]
pub struct hkArray<T, Allocator = ()> {
    pub base: hkArrayBase<T>, // 00
    pub _allocator: core::marker::PhantomData<Allocator>,
}

const _: () = assert!(core::mem::size_of::<hkArray<*mut core::ffi::c_void>>() == 0x10);
