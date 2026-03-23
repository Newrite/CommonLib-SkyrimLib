//!
//! @file lib.rs
//! @author Andrew Spaulding (Kasplat)
//! @brief Keywords/blocks/types which should be in Rust, but aren't.
//!
//! This file declares a number of macros which provides features that Rust probably *should*
//! provide by default. This includes:
//!   - A method of scoping the question mark operator.
//!   - A method of initializing static arrays where the size of the array has no real meaning and
//!     thus can't easily be defined.
//!   - A method of declaring abstract types, where the internal data layout is unknown.
//!
//! Additionally, two types of cells are provided to ease the declaration of global variables. The
//! first is called Later<T>, and acts as a once_cell that simply panics on reinitialization. This
//! allows the implementation to use a simple atomic, rather than a mutex. The second is an unsafe
//! cell wrapper that declares the underlying object as sync, which allows global variables to be
//! used in a way similar to C, and is slightly safer than using static mut.
//!

#![no_std]

// For macros.
pub use core;
pub mod enum_set;
pub mod enum_value;
pub use enum_set::*;
pub use enum_value::*;

use core::cell::UnsafeCell;
use core::ffi::{CStr, c_char};
use core::fmt;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::sync::atomic::{AtomicBool, Ordering};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Macros
////////////////////////////////////////////////////////////////////////////////////////////////////

///
/// Scopes the question mark operator within each of its blocks.
///
/// This macro can either be used as:
///     attempt! {{ /* stuff */ }}
/// or:
///     attempt! {{ /* stuff */ } catch(e) { /* more stuff */ }},
/// where the catch block is a call to map_err() on the original try block.
///
#[macro_export]
macro_rules! attempt {
    ( $try:block ) => {
        (|| $try)()
    };

    ( $try:block catch($arg:ident) $catch:block ) => {
        $crate::core::result::Result::map_err((|| $try)(), |$arg| $catch)
    };
}

///
/// Allows for a dynamically sized initialization of an array, capturing its size
/// in the identifier specified in the array type.
///
#[macro_export]
macro_rules! disarray {
    // Size capturing.
    ( $(#[$meta:meta])* $scope:vis static $arr:ident: [$type:ty; $size:ident] = [
        $($items:expr),*
    ]; ) => {
        $scope const $size: usize = $crate::disarray!(@maybe_count $($items),*);
        $(#[$meta])* $scope static $arr: [$type; $size] = [ $($items),* ];
    };

    // Non-capturing.
    ( $(#[$meta:meta])* $scope:vis static $arr:ident: [$type:ty] = [
        $($items:expr),*
    ]; ) => {
        $(#[$meta])* $scope static $arr: [$type; $crate::disarray!(@maybe_count $($items),*)] = [
            $($items),*
        ];
    };

    // Empty array len angers the compiler (idk).
    ( @maybe_count ) => { 0 };
    ( @maybe_count $($items:expr),+ ) => { [ $($crate::disarray!(@count $items)),* ].len() };

    // Make sure items are const.
    ( @count $item:expr ) => { 0 };
}

///
/// Allows the definition of any number of abstract types, with layouts unknown to Rust.
///
/// Types declared here will automatically have repr(C) applied.
///
#[macro_export]
macro_rules! abstract_type {
    ( $( $(#[$meta:meta])* $scope:vis type $name:ident );+; ) => {
        $($(#[$meta])* #[repr(C)] $scope struct $name {
            // Stop construction - without this anyone can construct.
            _private: [u8; 0],

            // Prevent the compiler from marking as Send, Sync, or Unpin.
            _marker: $crate::core::marker::PhantomData<
                (*mut u8, $crate::core::marker::PhantomPinned)
            >
        })*
    };
}

#[macro_export]
macro_rules! inherit {
    // Вариант 1: Одинарное наследование (Главный родитель)
    // Пример: inherit!(NiObject : NiRefObject);
    ($derived:ident : $base:ident) => {
        impl core::ops::Deref for $derived {
            type Target = $base;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                // Предполагается, что первое поле всегда называется `base`
                &self.base
            }
        }

        impl core::ops::DerefMut for $derived {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }

        impl AsRef<$base> for $derived {
            #[inline(always)]
            fn as_ref(&self) -> &$base {
                &self.base
            }
        }
    };

    // Вариант 2: Множественное наследование (Боковые родители)
    // Пример: inherit!(TESObjectREFR => BSHandleRefObject, handle_ref_obj);
    ($derived:ident => $base:ident, $field:ident) => {
        impl AsRef<$base> for $derived {
            #[inline(always)]
            fn as_ref(&self) -> &$base {
                &self.$field
            }
        }

        impl AsMut<$base> for $derived {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $base {
                &mut self.$field
            }
        }
    };
}

// core_util/src/ptr.rs

/// Deref raw pointer → &mut T (panics in debug if null)
#[macro_export]
macro_rules! deref_ptr {
    ($ptr:expr) => {{
        debug_assert!(!$ptr.is_null(), "deref_ptr: null pointer");
        unsafe { &mut *$ptr }
    }};
}

/// Deref raw pointer → &T (panics in debug if null)
#[macro_export]
macro_rules! deref_ptr_ref {
    ($ptr:expr) => {{
        debug_assert!(!$ptr.is_null(), "deref_ptr_ref: null pointer");
        unsafe { &*$ptr }
    }};
}

/// Safe deref → Option<&mut T>  (None if null)
/// Use when pointer CAN be null by engine contract
#[macro_export]
macro_rules! try_deref_ptr {
    ($ptr:expr) => {
        unsafe { $ptr.as_mut() }
    };
}

/// Safe deref → Option<&T>  (None if null)
#[macro_export]
macro_rules! try_deref_ptr_ref {
    ($ptr:expr) => {
        unsafe { $ptr.as_ref() }
    };
}

/// Call a method on a nullable pointer, return Option<R>
/// Usage: call_ptr!(actor_ptr, get_base_object())
#[macro_export]
macro_rules! call_ptr {
    ($ptr:expr, $method:ident ( $($arg:expr),* )) => {
        unsafe { $ptr.as_mut() }.map(|p| p.$method($($arg),*))
    };
}

/// Cast *mut T → *mut U via transmute offset
/// Usage: ptr_cast!(raw_ptr, ActorBase)
#[macro_export]
macro_rules! ptr_cast {
    ($ptr:expr, $ty:ty) => {
        $ptr as *mut $ty
    };
}

/// Read a field at a raw byte offset from a pointer
/// Usage: read_at_offset!(ptr, 0x10, u32)
#[macro_export]
macro_rules! read_at_offset {
    ($ptr:expr, $offset:expr, $ty:ty) => {
        unsafe { *(($ptr as *const u8).add($offset) as *const $ty) }
    };
}

/// Write a value at a raw byte offset from a pointer
/// Usage: write_at_offset!(ptr, 0x10, value, u32)
#[macro_export]
macro_rules! write_at_offset {
    ($ptr:expr, $offset:expr, $val:expr, $ty:ty) => {
        unsafe { *(($ptr as *mut u8).add($offset) as *mut $ty) = $val }
    };
}

pub trait SafePtrExt {
    type Target;
    fn get_ref<'a>(self) -> Option<&'a Self::Target>;
    fn get_mut<'a>(self) -> Option<&'a mut Self::Target>;
}

impl<T> SafePtrExt for *const T {
    type Target = T;
    #[inline(always)]
    fn get_ref<'a>(self) -> Option<&'a T> {
        unsafe { self.as_ref() }
    }
    #[inline(always)]
    fn get_mut<'a>(self) -> Option<&'a mut T> {
        None
    }
}

impl<T> SafePtrExt for *mut T {
    type Target = T;
    #[inline(always)]
    fn get_ref<'a>(self) -> Option<&'a T> {
        unsafe { self.as_ref() }
    }
    #[inline(always)]
    fn get_mut<'a>(self) -> Option<&'a mut T> {
        unsafe { self.as_mut() }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// C string FFI
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Creates a CStr literal from a string literal.
#[macro_export]
macro_rules! cstr {
    ( $str:literal ) => {
        $crate::core::ffi::CStr::from_bytes_until_nul($crate::core::concat!($str, "\0").as_bytes())
            .unwrap()
    };
}

/// Creates a wide CStr literal from a string literal.
#[macro_export]
macro_rules! wcstr {
    ( $str:literal ) => {{
        const SIZE: usize = $crate::get_utf16_len($str) + 1;
        $crate::WideStr::from_slice(&$crate::create_utf16_string::<SIZE>($str))
    }};
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A wide string struct, for C FFI. Must be NUL-terminated.
#[repr(transparent)]
pub struct WideStr([u16]);

impl WideStr {
    /// Creates a wide char from a slice. The slice must only contain NUL as its last byte, and must
    /// be null terminated.
    pub const fn from_slice<'a>(s: &'a [u16]) -> &'a Self {
        assert!(s[s.len() - 1] == 0);

        // Must not contain NULL.
        let mut i = 0;
        while i < s.len() - 1 {
            assert!(s[i] != 0);
            i += 1;
        }

        // SAFETY: WideStr is declared as transparent.
        unsafe { &*(s as *const [u16] as *const Self) }
    }

    /// Creates a wide char from a pointer. The given string must be NUL terminated.
    pub unsafe fn from_ptr<'a>(s: *const u16) -> &'a Self {
        let mut wchars = 0;
        while unsafe { *s.add(wchars) } != 0 {
            wchars += 1
        }
        Self::from_slice(unsafe { core::slice::from_raw_parts::<'a, u16>(s, wchars + 1) })
    }

    /// Returns the wide char string as a pointer.
    pub const fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

///
/// A structure for formatting wide strings, and automatically converting UTF-8 to UTF-16.
/// Necessary for FFI to Windows API functions that use unicode.
///
/// The buffer is always null terminated.
///
pub struct WideStringBuffer<const SIZE: usize> {
    buf: [u16; SIZE],
    len: usize,
}

impl<const SIZE: usize> WideStringBuffer<SIZE> {
    /// Creates an empty wide string buffer.
    pub const fn new() -> Self {
        Self {
            buf: [0; SIZE],
            len: 0,
        }
    }

    /// Converts the contents of the buffer to a WideStr.
    pub fn as_w_str(&self) -> &WideStr {
        WideStr::from_slice(self.buf.split_at(self.len + 1).0)
    }

    /// Writes a wide string to the buffer, if there is room for it.
    pub fn write_w_str(&mut self, s: &WideStr) -> Result<(), fmt::Error> {
        if s.0.len() + self.len > SIZE - 1 {
            return Err(fmt::Error);
        }

        // Copy, including NUL.
        self.buf
            .split_at_mut(self.len)
            .1
            .split_at_mut(s.0.len())
            .0
            .copy_from_slice(&s.0);
        self.len += s.0.len() - 1;
        Ok(())
    }
}

impl<const SIZE: usize> fmt::Write for WideStringBuffer<SIZE> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if s.encode_utf16().count() >= SIZE - self.len {
            return Err(fmt::Error);
        }

        for w in s.encode_utf16() {
            self.buf[self.len] = w;
            self.len += 1;
        }
        self.buf[self.len] = 0;

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

///
/// A structure for formatting strings into pre-allocated storage, which allows for text to be
/// calculated at runtime without causing a heap allocation.
///
/// The buffer is always ended with a null terminator, to allow for usage with FFI.
///
pub struct StringBuffer<const SIZE: usize> {
    buf: [u8; SIZE],
    len: usize,
}

impl<const SIZE: usize> StringBuffer<SIZE> {
    /// Creates a new, empty, string buffer.
    pub const fn new() -> Self {
        Self {
            buf: [0; SIZE],
            len: 0,
        }
    }

    /// Converts the contents of the buffer to a CStr.
    pub fn as_c_str(&self) -> &CStr {
        CStr::from_bytes_with_nul(self.buf.split_at(self.len + 1).0).unwrap()
    }
}

impl<const SIZE: usize> fmt::Write for StringBuffer<SIZE> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if s.len() + self.len > SIZE - 1 {
            return Err(fmt::Error);
        }

        self.buf
            .split_at_mut(self.len)
            .1
            .split_at_mut(s.len())
            .0
            .copy_from_slice(s.as_bytes());
        self.len += s.len();
        self.buf[self.len] = 0; // Always null terminate.
        Ok(())
    }
}

#[inline]
pub fn ptr_to_str<'a>(ptr: *const c_char) -> &'a str {
    if ptr.is_null() {
        "<null>"
    } else {
        unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .unwrap_or("<invalid utf8>")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// Converts a UTF-8 string to a statically sized array of UTF-16.
#[doc(hidden)]
pub const fn create_utf16_string<const DIM: usize>(s: &'static str) -> [u16; DIM] {
    let b = s.as_bytes();
    let mut ret = [0; DIM];

    let mut b_i: usize = 0;
    let mut w_i: usize = 0;
    while b_i < b.len() {
        if b[b_i] & 0x80 == 0 {
            ret[w_i] = b[b_i] as u16;
        } else if b[b_i] & 0xE0 == 0xC0 {
            assert!(b[b_i + 1] & 0xC0 == 0x80);
            ret[w_i] = (((b[b_i] & 0x1F) as u16) << 6) | ((b[b_i + 1] & 0x3F) as u16);
            b_i += 1;
        } else if b[b_i] & 0xF0 == 0xE0 {
            assert!(b[b_i + 1] & 0xC0 == 0x80);
            assert!(b[b_i + 2] & 0xC0 == 0x80);
            ret[w_i] = (((b[b_i] & 0x0F) as u16) << 12)
                | (((b[b_i + 1] & 0x3F) as u16) << 6)
                | ((b[b_i + 2] & 0x3F) as u16);
            assert!(ret[w_i] & 0xF800 != 0xD8);
            b_i += 2;
        } else {
            assert!(b[b_i] & 0xF8 == 0xF0);
            assert!(b[b_i + 1] & 0xC0 == 0x80);
            assert!(b[b_i + 2] & 0xC0 == 0x80);
            assert!(b[b_i + 3] & 0xC0 == 0x80);
            ret[w_i] = 0xD800
                | (((b[b_i] & 0x03) as u16) << 8)
                | (((b[b_i + 1] & 0x3F) as u16) << 2)
                | (((b[b_i + 2] & 0x30) as u16) >> 4);
            ret[w_i + 1] =
                0xDC00 | (((b[b_i + 2] & 0x0F) as u16) << 6) | ((b[b_i + 3] & 0x3F) as u16);
            w_i += 1;
            b_i += 3;
        }

        w_i += 1;
        b_i += 1;
    }

    assert!(w_i == DIM - 1);
    return ret;
}

// Counts the number of UTF-16 code points in a UTF-8 string.
#[doc(hidden)]
pub const fn get_utf16_len(s: &'static str) -> usize {
    let b = s.as_bytes();

    let mut code_points: usize = 0;
    let mut i: usize = 0;
    while i < b.len() {
        if b[i] & 0x80 == 0 {
            code_points += 1;
        } else if b[i] & 0xE0 == 0xC0 {
            code_points += 1;
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
        } else if b[i] & 0xF0 == 0xE0 {
            code_points += 1;
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
        } else {
            assert!(b[i] & 0xF8 == 0xF0);
            code_points += 1;
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
        }
        i += 1;
    }

    return code_points;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cells
////////////////////////////////////////////////////////////////////////////////////////////////////

/// The core later structure. It is illegal to deref it before initialization.
pub struct Later<T> {
    is_init: AtomicBool,
    pl: UnsafeCell<MaybeUninit<T>>,
}

/// An unsafe cell which implements Sync.
#[repr(transparent)]
pub struct RacyCell<T>(UnsafeCell<T>);

////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> Later<T> {
    /// Creates a new later structure.
    pub const fn new() -> Self {
        Self {
            is_init: AtomicBool::new(false),
            pl: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    /// Initializes a later structure.
    pub fn init(&self, pl: T) {
        assert!(!self.is_init.swap(true, Ordering::Relaxed));
        // SAFETY: We have ensured that we are the only object initializing the data.
        unsafe {
            (*self.pl.get()).write(pl);
        }
    }

    /// Checks if the instance has been initialized.
    pub fn is_init(&self) -> bool {
        self.is_init.load(Ordering::Relaxed)
    }
}

impl<T> Deref for Later<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        assert!(self.is_init.load(Ordering::Relaxed));
        // SAFETY: We have ensured that the object is initialized.
        unsafe { (*self.pl.get()).assume_init_ref() }
    }
}

impl<T> Drop for Later<T> {
    fn drop(&mut self) {
        if *self.is_init.get_mut() {
            // SAFETY: We have ensured that the object is initialized.
            unsafe {
                (*self.pl.get()).assume_init_drop();
            }
        }
    }
}

// Later is sync if T is.
unsafe impl<T: Sync> Sync for Later<T> {}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> RacyCell<T> {
    /// Creates a new racy cell.
    pub const fn new(pl: T) -> Self {
        Self(UnsafeCell::new(pl))
    }

    /// Gets a pointer to the cells data.
    pub fn get(&self) -> *mut T {
        self.0.get()
    }
}

unsafe impl<T> Sync for RacyCell<T> {}
