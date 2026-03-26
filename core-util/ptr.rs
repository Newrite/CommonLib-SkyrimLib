/// Dereferences a raw mutable pointer and returns `&mut T`.
///
/// In debug builds this macro asserts that the pointer is not null.
#[macro_export]
macro_rules! deref_ptr {
    ($ptr:expr) => {{
        debug_assert!(!$ptr.is_null(), "deref_ptr: null pointer");
        unsafe { &mut *$ptr }
    }};
}

/// Dereferences a raw pointer and returns `&T`.
///
/// In debug builds this macro asserts that the pointer is not null.
#[macro_export]
macro_rules! deref_ptr_ref {
    ($ptr:expr) => {{
        debug_assert!(!$ptr.is_null(), "deref_ptr_ref: null pointer");
        unsafe { &*$ptr }
    }};
}

/// Returns `Option<&mut T>` from a nullable raw pointer.
#[macro_export]
macro_rules! try_deref_ptr {
    ($ptr:expr) => {
        unsafe { $ptr.as_mut() }
    };
}

/// Returns `Option<&T>` from a nullable raw pointer.
#[macro_export]
macro_rules! try_deref_ptr_ref {
    ($ptr:expr) => {
        unsafe { $ptr.as_ref() }
    };
}

/// Calls a method on a nullable pointer and returns `Option<R>`.
#[macro_export]
macro_rules! call_ptr {
    ($ptr:expr, $method:ident ( $($arg:expr),* )) => {
        unsafe { $ptr.as_mut() }.map(|p| p.$method($($arg),*))
    };
}

/// Casts `*mut T` to `*mut U`.
#[macro_export]
macro_rules! ptr_cast {
    ($ptr:expr, $ty:ty) => {
        $ptr as *mut $ty
    };
}

/// Reads a value at a raw byte offset from a pointer.
#[macro_export]
macro_rules! read_at_offset {
    ($ptr:expr, $offset:expr, $ty:ty) => {
        unsafe { *(($ptr as *const u8).add($offset) as *const $ty) }
    };
}

/// Writes a value at a raw byte offset from a pointer.
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
