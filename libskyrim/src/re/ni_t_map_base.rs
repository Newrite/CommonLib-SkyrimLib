use core::ffi::{CStr, c_char, c_void};
use core::marker::PhantomData;
use core::ptr;

use crate::offsets::offsets_rtti::RTTI_NiTMapBase_DFALL_NiTMapItem_char_const___Setting_____char_const___Setting___;
use crate::offsets::offsets_vtable::VTABLE_NiTMapBase_DFALL_NiTMapItem_char_const___Setting_____char_const___Setting___;
use crate::re::Setting;
use crate::re::ni_t_default_allocator::NiTDefaultAllocator;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::NiTMapItem<Key, T>`
#[repr(C)]
pub struct NiTMapItem<K, T> {
    pub next: *mut NiTMapItem<K, T>, // 00
    pub first: K,                    // 08
    pub second: T,                   // ??
}

const _: () = assert!(core::mem::size_of::<NiTMapItem<*const c_char, *mut c_void>>() == 0x18);
const _: () = assert!(core::mem::offset_of!(NiTMapItem<*const c_char, *mut c_void>, next) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiTMapItem<*const c_char, *mut c_void>, first) == 0x08);
const _: () =
    assert!(core::mem::offset_of!(NiTMapItem<*const c_char, *mut c_void>, second) == 0x10);

/// C++ `RE::NiTMapBase<Allocator, Key, T>::AntiBloatAllocator`
#[repr(C)]
pub struct NiTMapAntiBloatAllocator<Allocator> {
    pub base: Allocator, // 00
    pub size: u32,       // ??
    pub pad04: u32,      // ??
}

const _: () = assert!(
    core::mem::size_of::<NiTMapAntiBloatAllocator<NiTDefaultAllocator<*mut c_void>>>() == 0x8
);
const _: () = assert!(
    core::mem::offset_of!(
        NiTMapAntiBloatAllocator<NiTDefaultAllocator<*mut c_void>>,
        size
    ) == 0x00
);
const _: () = assert!(
    core::mem::offset_of!(
        NiTMapAntiBloatAllocator<NiTDefaultAllocator<*mut c_void>>,
        pad04
    ) == 0x04
);

/// C++ `RE::NiTMapBase<Allocator, Key, T>`
#[repr(C)]
pub struct NiTMapBase<Allocator, K, T> {
    pub vtable: *const usize,                           // 00
    pub capacity: u32,                                  // 08
    pub pad0c: u32,                                     // 0C
    pub data: *mut *mut NiTMapItem<K, T>,               // 10
    pub allocator: NiTMapAntiBloatAllocator<Allocator>, // 18
}

pub type DefaultNiTMapBase<K, T> = NiTMapBase<NiTDefaultAllocator<NiTMapItem<K, T>>, K, T>;

const _: () =
    assert!(core::mem::size_of::<DefaultNiTMapBase<*const c_char, *mut c_void>>() == 0x20);
const _: () =
    assert!(core::mem::offset_of!(DefaultNiTMapBase<*const c_char, *mut c_void>, vtable) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(DefaultNiTMapBase<*const c_char, *mut c_void>, capacity) == 0x08);
const _: () =
    assert!(core::mem::offset_of!(DefaultNiTMapBase<*const c_char, *mut c_void>, pad0c) == 0x0C);
const _: () =
    assert!(core::mem::offset_of!(DefaultNiTMapBase<*const c_char, *mut c_void>, data) == 0x10);
const _: () = assert!(
    core::mem::offset_of!(DefaultNiTMapBase<*const c_char, *mut c_void>, allocator) == 0x18
);

pub type NiTMapItemSetting = NiTMapItem<*const c_char, *mut Setting>;
pub type NiTMapBaseSetting = DefaultNiTMapBase<*const c_char, *mut Setting>;

impl RttiType for NiTMapBaseSetting {
    const RTTI: VariantID =
        RTTI_NiTMapBase_DFALL_NiTMapItem_char_const___Setting_____char_const___Setting___;
}

impl<Allocator, K, T> AsRef<NiTMapBase<Allocator, K, T>> for NiTMapBase<Allocator, K, T> {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Allocator, K, T> AsMut<NiTMapBase<Allocator, K, T>> for NiTMapBase<Allocator, K, T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl NiTMapBaseSetting {
    pub const RTTI: VariantID =
        RTTI_NiTMapBase_DFALL_NiTMapItem_char_const___Setting_____char_const___Setting___;
    pub const VTABLE: &'static [VariantID] =
        &VTABLE_NiTMapBase_DFALL_NiTMapItem_char_const___Setting_____char_const___Setting___;
}

pub struct NiTMapBaseIter<'a, K, T> {
    buckets: *mut *mut NiTMapItem<K, T>,
    capacity: u32,
    bucket_index: u32,
    current: *mut NiTMapItem<K, T>,
    _marker: PhantomData<&'a NiTMapItem<K, T>>,
}

impl<'a, K, T> Iterator for NiTMapBaseIter<'a, K, T> {
    type Item = &'a NiTMapItem<K, T>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.current.is_null() && self.bucket_index < self.capacity {
                self.current = *self.buckets.add(self.bucket_index as usize);
                self.bucket_index += 1;
            }

            if self.current.is_null() {
                return None;
            }

            let item = &*self.current;
            self.current = item.next;
            Some(item)
        }
    }
}

impl<Allocator, K, T> NiTMapBase<Allocator, K, T> {
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_HASH_FUNCTION: usize = 0x01;
        pub fn hash_function(key: K) -> u32
    }

    virtual_method! {
        pub const VFUNC_KEY_EQ: usize = 0x02;
        pub fn key_eq(lhs: K, rhs: K) -> bool
    }

    virtual_method! {
        pub const VFUNC_ASSIGN_VALUE: usize = 0x03;
        pub fn assign_value(value: *mut NiTMapItem<K, T>, key: K, mapped: T)
    }

    virtual_method! {
        pub const VFUNC_CLEAR_VALUE: usize = 0x04;
        pub fn clear_value(value: *mut NiTMapItem<K, T>)
    }

    virtual_method! {
        pub const VFUNC_MALLOC_VALUE: usize = 0x05;
        pub fn malloc_value() -> *mut NiTMapItem<K, T>
    }

    virtual_method! {
        pub const VFUNC_FREE_VALUE: usize = 0x06;
        pub fn free_value(value: *mut NiTMapItem<K, T>)
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.allocator.size == 0
    }

    #[inline(always)]
    pub const fn size(&self) -> u32 {
        self.allocator.size
    }

    #[inline(always)]
    pub fn iter(&self) -> NiTMapBaseIter<'_, K, T> {
        NiTMapBaseIter {
            buckets: self.data,
            capacity: self.capacity,
            bucket_index: 0,
            current: ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

pub trait NiTMapBaseExt<Allocator, K, T> {
    fn dtor(&self);
    fn hash_function(&self, key: K) -> u32;
    fn key_eq(&self, lhs: K, rhs: K) -> bool;
    fn assign_value(&self, value: *mut NiTMapItem<K, T>, key: K, mapped: T);
    fn clear_value(&self, value: *mut NiTMapItem<K, T>);
    fn malloc_value(&self) -> *mut NiTMapItem<K, T>;
    fn free_value(&self, value: *mut NiTMapItem<K, T>);
    fn is_empty(&self) -> bool;
    fn size(&self) -> u32;
    fn iter<'a>(&'a self) -> NiTMapBaseIter<'a, K, T>
    where
        Allocator: 'a;
}

impl<Allocator, K, T, U> NiTMapBaseExt<Allocator, K, T> for U
where
    U: AsRef<NiTMapBase<Allocator, K, T>>,
{
    fn dtor(&self) {
        self.as_ref().dtor()
    }

    fn hash_function(&self, key: K) -> u32 {
        self.as_ref().hash_function(key)
    }

    fn key_eq(&self, lhs: K, rhs: K) -> bool {
        self.as_ref().key_eq(lhs, rhs)
    }

    fn assign_value(&self, value: *mut NiTMapItem<K, T>, key: K, mapped: T) {
        self.as_ref().assign_value(value, key, mapped)
    }

    fn clear_value(&self, value: *mut NiTMapItem<K, T>) {
        self.as_ref().clear_value(value)
    }

    fn malloc_value(&self) -> *mut NiTMapItem<K, T> {
        self.as_ref().malloc_value()
    }

    fn free_value(&self, value: *mut NiTMapItem<K, T>) {
        self.as_ref().free_value(value)
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    fn size(&self) -> u32 {
        self.as_ref().size()
    }

    fn iter<'a>(&'a self) -> NiTMapBaseIter<'a, K, T>
    where
        Allocator: 'a,
    {
        self.as_ref().iter()
    }
}

#[inline]
pub fn cstr_eq_ignore_ascii_case(lhs: *const c_char, rhs: *const c_char) -> bool {
    match (lhs.is_null(), rhs.is_null()) {
        (true, true) => true,
        (true, false) | (false, true) => false,
        (false, false) => unsafe {
            CStr::from_ptr(lhs)
                .to_bytes()
                .eq_ignore_ascii_case(CStr::from_ptr(rhs).to_bytes())
        },
    }
}

#[inline]
pub fn cstr_eq_str_ignore_ascii_case(lhs: *const c_char, rhs: &str) -> bool {
    if lhs.is_null() {
        return false;
    }

    unsafe {
        CStr::from_ptr(lhs)
            .to_bytes()
            .eq_ignore_ascii_case(rhs.as_bytes())
    }
}
