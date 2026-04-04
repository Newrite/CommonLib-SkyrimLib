//! Patch-oriented helper layer for common hook installation patterns.
//!
//! This module intentionally stays small and explicit:
//!
//! - direct byte/value patching through `REL::safe_write`
//! - fill-style patching for NOP/INT3 regions
//! - vfunc patch helpers
//! - IAT patch helpers for the few plugins that need import-table detours

use alloc::ffi::CString;
use core::fmt;

use crate::relocation::{IntoAddress, TryIntoAddress, TryIntoOffset, safe_fill, safe_write};

use super::runtime::HookInstallError;

pub const NOP: u8 = 0x90;
pub const INT3: u8 = 0xCC;

/// Failure reported by higher-level patch helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatchInstallError {
    InvalidDllName,
    InvalidFunctionName,
    IatPatchFailed,
}

impl fmt::Display for PatchInstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDllName => f.write_str("DLL name contained an interior NUL byte"),
            Self::InvalidFunctionName => {
                f.write_str("function name contained an interior NUL byte")
            }
            Self::IatPatchFailed => f.write_str("IAT patch failed"),
        }
    }
}

impl core::error::Error for PatchInstallError {}

/// Write one raw byte sequence to process memory.
#[inline(always)]
pub fn write_bytes<A: IntoAddress>(address: A, bytes: &[u8]) {
    safe_write(address, bytes);
}

/// Write one `Copy` value to process memory.
pub fn write_value<A: IntoAddress, T: Copy>(address: A, value: T) {
    let bytes = unsafe {
        core::slice::from_raw_parts((&value as *const T).cast::<u8>(), core::mem::size_of::<T>())
    };
    safe_write(address, bytes);
}

/// Fill one memory range with one repeated byte.
#[inline(always)]
pub fn fill_bytes<A: IntoAddress>(address: A, value: u8, count: usize) {
    safe_fill(address, value, count);
}

/// Fill one memory range with x86/x64 NOP instructions.
#[inline(always)]
pub fn write_nops<A: IntoAddress>(address: A, count: usize) {
    fill_bytes(address, NOP, count);
}

/// Fill one memory range with x86/x64 INT3 instructions.
#[inline(always)]
pub fn write_int3<A: IntoAddress>(address: A, count: usize) {
    fill_bytes(address, INT3, count);
}

/// Patch one vtable slot and return the original function address.
pub fn write_vfunc<A: TryIntoAddress, I: TryIntoOffset>(
    vtable_addr: A,
    index: I,
    new_func: usize,
) -> Result<usize, HookInstallError> {
    crate::relocation::try_write_vfunc(vtable_addr, index, new_func).map_err(Into::into)
}

/// Look up one imported function in the current module.
#[inline(always)]
pub fn iat_addr(dll: &str, function: &str) -> usize {
    crate::skse::iat::get_addr(dll, function)
}

/// Patch one imported function in the current module and return the previous pointer.
pub fn patch_iat(dll: &str, function: &str, new_func: usize) -> Result<usize, PatchInstallError> {
    let _ = CString::new(dll).map_err(|_| PatchInstallError::InvalidDllName)?;
    let _ = CString::new(function).map_err(|_| PatchInstallError::InvalidFunctionName)?;

    let old = unsafe { crate::skse::iat::patch(new_func, dll, function) };
    if old == 0 {
        Err(PatchInstallError::IatPatchFailed)
    } else {
        Ok(old)
    }
}

#[cfg(test)]
mod tests {
    use super::{INT3, NOP, iat_addr, patch_iat, write_bytes, write_int3, write_nops, write_value};
    use crate::sdk::hooks::{HookInstallError, patch};

    #[test]
    fn patch_constants_match_common_instruction_bytes() {
        assert_eq!(NOP, 0x90);
        assert_eq!(INT3, 0xCC);
    }

    #[test]
    fn patch_helpers_expose_expected_signatures() {
        let _ = write_bytes::<usize> as fn(usize, &[u8]);
        let _ = write_nops::<usize> as fn(usize, usize);
        let _ = write_int3::<usize> as fn(usize, usize);
        let _ = write_value::<usize, u32> as fn(usize, u32);
        let _ = patch::write_vfunc::<usize, usize>
            as fn(usize, usize, usize) -> Result<usize, HookInstallError>;
        let _ = iat_addr as fn(&str, &str) -> usize;
        let _ = patch_iat as fn(&str, &str, usize) -> Result<usize, super::PatchInstallError>;
    }
}
