//! Reusable high-level hook patterns for consumer code.
//!
//! Local C++ SKSE projects repeatedly use a small set of manual hook helpers:
//!
//! - `AllocTrampoline(14)`
//! - `write_thunk_call`
//! - `write_thunk_jump`
//! - `write_vfunc`
//! - `T::func = ...` to retain the original function pointer
//!
//! This module keeps those patterns explicit while making them reusable for
//! Rust-side manual hooks that do not fit the attribute-driven surface.

use crate::relocation::{TryIntoAddress, TryIntoOffset};

use super::patch;
use super::runtime::HookInstallError;
use super::trampoline;

pub use super::trampoline::{DEFAULT_THUNK_STUB_RESERVE, default_thunk_reserve_size};

/// Reserve the common 14-byte trampoline storage for one thunk call/jump.
#[inline(always)]
pub fn reserve_thunk_stub() {
    trampoline::reserve_default_thunk();
}

/// Reserve storage for `count` ordinary thunk calls/jumps.
#[inline(always)]
pub fn reserve_thunk_stubs(count: usize) {
    trampoline::reserve_default_thunks(count);
}

/// Install one C++-style thunk call patch and return the original function address.
#[inline(always)]
pub fn write_thunk_call<const N: usize, A: TryIntoAddress>(
    src: A,
    thunk: usize,
) -> Result<usize, HookInstallError> {
    trampoline::write_call::<N, _>(src, thunk)
}

/// Install one C++-style thunk branch patch and return the original function address.
#[inline(always)]
pub fn write_thunk_branch<const N: usize, A: TryIntoAddress>(
    src: A,
    thunk: usize,
) -> Result<usize, HookInstallError> {
    trampoline::write_branch::<N, _>(src, thunk)
}

/// Install one C++-style thunk vfunc patch and return the original function address.
#[inline(always)]
pub fn write_thunk_vfunc<A: TryIntoAddress, I: TryIntoOffset>(
    vtable_addr: A,
    index: I,
    thunk: usize,
) -> Result<usize, HookInstallError> {
    patch::write_vfunc(vtable_addr, index, thunk)
}

#[inline(always)]
pub fn write_thunk_call5<A: TryIntoAddress>(
    src: A,
    thunk: usize,
) -> Result<usize, HookInstallError> {
    write_thunk_call::<5, _>(src, thunk)
}

#[inline(always)]
pub fn write_thunk_call6<A: TryIntoAddress>(
    src: A,
    thunk: usize,
) -> Result<usize, HookInstallError> {
    write_thunk_call::<6, _>(src, thunk)
}

#[inline(always)]
pub fn write_thunk_branch5<A: TryIntoAddress>(
    src: A,
    thunk: usize,
) -> Result<usize, HookInstallError> {
    write_thunk_branch::<5, _>(src, thunk)
}

#[inline(always)]
pub fn write_thunk_branch6<A: TryIntoAddress>(
    src: A,
    thunk: usize,
) -> Result<usize, HookInstallError> {
    write_thunk_branch::<6, _>(src, thunk)
}

/// Store an original function pointer returned by a thunk install helper.
///
/// # Safety
///
/// `F` must be a function-pointer-like ABI type whose size matches `usize`.
#[inline(always)]
pub unsafe fn store_original_fn<F: Copy>(slot: &mut F, address: usize) {
    debug_assert_eq!(core::mem::size_of::<F>(), core::mem::size_of::<usize>());
    *slot = unsafe { core::mem::transmute_copy(&address) };
}

/// Install one thunk call and immediately store the original function pointer.
///
/// # Safety
///
/// `original` must be a function-pointer-like ABI type whose size matches `usize`.
pub unsafe fn install_thunk_call<const N: usize, A: TryIntoAddress, F: Copy>(
    src: A,
    thunk: usize,
    original: &mut F,
) -> Result<(), HookInstallError> {
    let address = write_thunk_call::<N, _>(src, thunk)?;
    unsafe { store_original_fn(original, address) };
    Ok(())
}

/// Install one thunk branch and immediately store the original function pointer.
///
/// # Safety
///
/// `original` must be a function-pointer-like ABI type whose size matches `usize`.
pub unsafe fn install_thunk_branch<const N: usize, A: TryIntoAddress, F: Copy>(
    src: A,
    thunk: usize,
    original: &mut F,
) -> Result<(), HookInstallError> {
    let address = write_thunk_branch::<N, _>(src, thunk)?;
    unsafe { store_original_fn(original, address) };
    Ok(())
}

/// Install one thunk vfunc patch and immediately store the original function pointer.
///
/// # Safety
///
/// `original` must be a function-pointer-like ABI type whose size matches `usize`.
pub unsafe fn install_thunk_vfunc<A: TryIntoAddress, I: TryIntoOffset, F: Copy>(
    vtable_addr: A,
    index: I,
    thunk: usize,
    original: &mut F,
) -> Result<(), HookInstallError> {
    let address = write_thunk_vfunc(vtable_addr, index, thunk)?;
    unsafe { store_original_fn(original, address) };
    Ok(())
}

#[inline(always)]
pub unsafe fn install_thunk_call5<A: TryIntoAddress, F: Copy>(
    src: A,
    thunk: usize,
    original: &mut F,
) -> Result<(), HookInstallError> {
    unsafe { install_thunk_call::<5, _, _>(src, thunk, original) }
}

#[inline(always)]
pub unsafe fn install_thunk_call6<A: TryIntoAddress, F: Copy>(
    src: A,
    thunk: usize,
    original: &mut F,
) -> Result<(), HookInstallError> {
    unsafe { install_thunk_call::<6, _, _>(src, thunk, original) }
}

#[inline(always)]
pub unsafe fn install_thunk_branch5<A: TryIntoAddress, F: Copy>(
    src: A,
    thunk: usize,
    original: &mut F,
) -> Result<(), HookInstallError> {
    unsafe { install_thunk_branch::<5, _, _>(src, thunk, original) }
}

#[inline(always)]
pub unsafe fn install_thunk_branch6<A: TryIntoAddress, F: Copy>(
    src: A,
    thunk: usize,
    original: &mut F,
) -> Result<(), HookInstallError> {
    unsafe { install_thunk_branch::<6, _, _>(src, thunk, original) }
}

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_THUNK_STUB_RESERVE, default_thunk_reserve_size, install_thunk_branch5,
        install_thunk_call5, reserve_thunk_stub, reserve_thunk_stubs, store_original_fn,
        write_thunk_branch5, write_thunk_call5,
    };
    use crate::sdk::hooks::HookInstallError;

    fn sample_original(value: u32) -> u32 {
        value + 1
    }

    #[test]
    fn store_original_fn_round_trips_function_pointer() {
        let mut slot: fn(u32) -> u32 = |_| 0;
        unsafe { store_original_fn(&mut slot, sample_original as *const () as usize) };
        assert_eq!(slot(41), 42);
    }

    #[test]
    fn thunk_pattern_helpers_expose_expected_signatures() {
        let _ = reserve_thunk_stub as fn();
        let _ = reserve_thunk_stubs as fn(usize);
        let _ = write_thunk_call5::<usize> as fn(usize, usize) -> Result<usize, HookInstallError>;
        let _ = write_thunk_branch5::<usize> as fn(usize, usize) -> Result<usize, HookInstallError>;
        let _ = install_thunk_call5::<usize, fn(u32) -> u32>
            as unsafe fn(usize, usize, &mut fn(u32) -> u32) -> Result<(), HookInstallError>;
        let _ = install_thunk_branch5::<usize, fn(u32) -> u32>
            as unsafe fn(usize, usize, &mut fn(u32) -> u32) -> Result<(), HookInstallError>;
    }

    #[test]
    fn thunk_reserve_reexports_match_trampoline_defaults() {
        assert_eq!(DEFAULT_THUNK_STUB_RESERVE, 14);
        assert_eq!(default_thunk_reserve_size(3), 42);
    }
}
