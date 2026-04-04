//! Ergonomic hook helpers on top of `libskyrim::skse::trampoline`.
//!
//! Real SKSE plugins repeatedly rebuild the same small trampoline workflow:
//! reserve some trampoline storage during startup, then install `write_call` /
//! `write_branch` hooks at runtime-aware addresses. This module keeps that
//! surface explicit while avoiding raw FFI plumbing in consumer code.

use core::ffi::c_void;
use core::fmt;
use core::ptr::NonNull;

use crate::relocation::{TryIntoAddress, try_write_branch, try_write_call};

use super::runtime::HookInstallError;

/// Common trampoline reserve size for one 5-byte or 6-byte branch/call stub.
pub const DEFAULT_THUNK_STUB_RESERVE: usize = 14;

/// Trampoline allocation pools exposed by the SKSE trampoline interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrampolinePool {
    Branch,
    Local,
}

/// Failure allocating memory from the SKSE trampoline backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrampolineAllocError {
    AllocationFailed,
}

impl fmt::Display for TrampolineAllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationFailed => {
                f.write_str("failed to allocate memory from the SKSE trampoline")
            }
        }
    }
}

impl core::error::Error for TrampolineAllocError {}

#[inline(always)]
pub const fn default_thunk_reserve_size(count: usize) -> usize {
    DEFAULT_THUNK_STUB_RESERVE.saturating_mul(count)
}

#[inline(always)]
fn non_null_or_alloc_failed<T>(raw: *mut T) -> Result<NonNull<T>, TrampolineAllocError> {
    NonNull::new(raw).ok_or(TrampolineAllocError::AllocationFailed)
}

/// Reserve global trampoline storage through SKSE.
///
/// This should usually be called during plugin startup before installing a
/// batch of call/branch hooks.
#[inline(always)]
pub fn reserve(size: usize) {
    crate::skse::alloc_trampoline(size);
}

/// Reserve the common 14-byte trampoline storage used by one thunk call/jump.
#[inline(always)]
pub fn reserve_default_thunk() {
    reserve(DEFAULT_THUNK_STUB_RESERVE);
}

/// Reserve storage for `count` ordinary thunk calls/jumps.
#[inline(always)]
pub fn reserve_default_thunks(count: usize) {
    reserve(default_thunk_reserve_size(count));
}

/// Allocate bytes from the default trampoline allocation path.
pub fn allocate_bytes(size: usize) -> Result<NonNull<u8>, TrampolineAllocError> {
    non_null_or_alloc_failed(unsafe { crate::skse::allocate(size) })
}

/// Allocate one typed value from the default trampoline allocation path.
pub fn allocate_value<T>() -> Result<NonNull<T>, TrampolineAllocError> {
    allocate_bytes(core::mem::size_of::<T>()).map(NonNull::cast)
}

/// Allocate bytes from one specific trampoline pool.
pub fn allocate_bytes_from_pool(
    size: usize,
    pool: TrampolinePool,
) -> Result<NonNull<c_void>, TrampolineAllocError> {
    let raw = unsafe {
        match pool {
            TrampolinePool::Branch => crate::skse::allocate_from_branch_pool(size),
            TrampolinePool::Local => crate::skse::allocate_from_local_pool(size),
        }
    };
    non_null_or_alloc_failed(raw)
}

/// Allocate one typed value from one specific trampoline pool.
pub fn allocate_value_from_pool<T>(
    pool: TrampolinePool,
) -> Result<NonNull<T>, TrampolineAllocError> {
    allocate_bytes_from_pool(core::mem::size_of::<T>(), pool).map(NonNull::cast)
}

/// Install one trampoline-backed branch patch.
pub fn write_branch<const N: usize, A: TryIntoAddress>(
    src: A,
    dst: usize,
) -> Result<usize, HookInstallError> {
    try_write_branch(src, dst, N).map_err(Into::into)
}

/// Install one trampoline-backed call patch.
pub fn write_call<const N: usize, A: TryIntoAddress>(
    src: A,
    dst: usize,
) -> Result<usize, HookInstallError> {
    try_write_call(src, dst, N).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_THUNK_STUB_RESERVE, TrampolinePool, default_thunk_reserve_size};

    #[test]
    fn default_thunk_reserve_size_scales_with_count() {
        assert_eq!(default_thunk_reserve_size(0), 0);
        assert_eq!(default_thunk_reserve_size(1), DEFAULT_THUNK_STUB_RESERVE);
        assert_eq!(
            default_thunk_reserve_size(4),
            DEFAULT_THUNK_STUB_RESERVE * 4
        );
    }

    #[test]
    fn trampoline_pool_is_stable_for_pattern_helpers() {
        assert_eq!(TrampolinePool::Branch, TrampolinePool::Branch);
        assert_ne!(TrampolinePool::Branch, TrampolinePool::Local);
    }
}
