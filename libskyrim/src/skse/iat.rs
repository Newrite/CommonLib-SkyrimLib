use alloc::ffi::CString;
use core::ffi::c_void;

/// Thin Rust facade over `SKSE::IAT` helpers.
pub struct Iat;

impl Iat {
    #[inline(always)]
    pub fn get_addr(dll: &str, function: &str) -> usize {
        let (Ok(dll), Ok(function)) = (CString::new(dll), CString::new(function)) else {
            return 0;
        };

        unsafe { crate::ffi::commonlib_skse_iat_get_addr(dll.as_ptr(), function.as_ptr()) }
    }

    #[inline(always)]
    pub fn get_addr_for_module(module: *mut c_void, dll: &str, function: &str) -> usize {
        if module.is_null() {
            return 0;
        }

        let (Ok(dll), Ok(function)) = (CString::new(dll), CString::new(function)) else {
            return 0;
        };

        unsafe {
            crate::ffi::commonlib_skse_iat_get_addr_for_module(
                module,
                dll.as_ptr(),
                function.as_ptr(),
            )
        }
    }

    #[inline(always)]
    pub fn get_ptr(dll: &str, function: &str) -> *mut c_void {
        Self::get_addr(dll, function) as *mut c_void
    }

    #[inline(always)]
    pub fn get_ptr_for_module(module: *mut c_void, dll: &str, function: &str) -> *mut c_void {
        Self::get_addr_for_module(module, dll, function) as *mut c_void
    }

    #[inline(always)]
    pub unsafe fn patch(new_func: usize, dll: &str, function: &str) -> usize {
        let (Ok(dll), Ok(function)) = (CString::new(dll), CString::new(function)) else {
            return 0;
        };

        unsafe { crate::ffi::commonlib_skse_iat_patch(new_func, dll.as_ptr(), function.as_ptr()) }
    }
}

#[inline(always)]
pub fn get_addr(dll: &str, function: &str) -> usize {
    Iat::get_addr(dll, function)
}

#[inline(always)]
pub fn get_addr_for_module(module: *mut c_void, dll: &str, function: &str) -> usize {
    Iat::get_addr_for_module(module, dll, function)
}

#[inline(always)]
pub fn get_ptr(dll: &str, function: &str) -> *mut c_void {
    Iat::get_ptr(dll, function)
}

#[inline(always)]
pub fn get_ptr_for_module(module: *mut c_void, dll: &str, function: &str) -> *mut c_void {
    Iat::get_ptr_for_module(module, dll, function)
}

#[inline(always)]
pub unsafe fn patch(new_func: usize, dll: &str, function: &str) -> usize {
    unsafe { Iat::patch(new_func, dll, function) }
}
