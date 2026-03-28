//! Runtime support for high-level SDK hook attributes.

use core::fmt;

use crate::relocation::RelocationError;
use crate::sdk::core::{HandleFamilyTarget, ResolvableHandle, Resolved, ResolvedHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookGuardFailure {
    Null,
    Unresolved,
    ConvertFail,
}

impl fmt::Display for HookGuardFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => f.write_str("null hook argument"),
            Self::Unresolved => f.write_str("failed to resolve hook argument"),
            Self::ConvertFail => f.write_str("failed to convert hook argument"),
        }
    }
}

impl core::error::Error for HookGuardFailure {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookInstallError {
    Relocation(RelocationError),
    AlreadyInstalled(&'static str),
}

impl HookInstallError {
    #[inline(always)]
    pub fn install_or_fatal(self, hook_name: &str) -> ! {
        match self {
            Self::Relocation(error) => crate::skse::log::fatal_runtime(format_args!(
                "failed to install hook `{hook_name}`: {error}"
            )),
            Self::AlreadyInstalled(existing) => crate::skse::log::fatal_runtime(format_args!(
                "hook `{hook_name}` is already installed ({existing})"
            )),
        }
    }
}

impl fmt::Display for HookInstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Relocation(error) => write!(f, "{error}"),
            Self::AlreadyInstalled(name) => write!(f, "hook `{name}` is already installed"),
        }
    }
}

impl From<RelocationError> for HookInstallError {
    #[inline(always)]
    fn from(value: RelocationError) -> Self {
        Self::Relocation(value)
    }
}

impl core::error::Error for HookInstallError {}

#[repr(transparent)]
pub struct Original<F>(F);

impl<F> Original<F> {
    #[inline(always)]
    pub const fn new(inner: F) -> Self {
        Self(inner)
    }

    #[inline(always)]
    pub fn into_inner(self) -> F {
        self.0
    }

    #[inline(always)]
    pub const fn as_ref(&self) -> &F {
        &self.0
    }
}

macro_rules! impl_original_call {
    ($(($($arg_ty:ident $arg_name:ident),*)),* $(,)?) => {
        $(
            impl<R, $($arg_ty),*> Original<fn($($arg_ty),*) -> R> {
                #[inline(always)]
                pub fn call(&self, $($arg_name: $arg_ty),*) -> R {
                    (self.0)($($arg_name),*)
                }
            }
        )*
    };
}

impl_original_call!(
    (),
    (A0 a0),
    (A0 a0, A1 a1),
    (A0 a0, A1 a1, A2 a2),
    (A0 a0, A1 a1, A2 a2, A3 a3),
    (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4),
    (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5),
    (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6),
    (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7),
    (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8),
    (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9),
    (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10),
    (A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11),
);

pub trait HookArg<'a, Abi>: Sized {
    fn from_abi(arg: Abi) -> Result<Self, HookGuardFailure>;

    fn into_abi(self) -> Abi;
}

impl<'a, T: Copy> HookArg<'a, T> for T {
    #[inline(always)]
    fn from_abi(arg: T) -> Result<Self, HookGuardFailure> {
        Ok(arg)
    }

    #[inline(always)]
    fn into_abi(self) -> T {
        self
    }
}

impl<'a, T> HookArg<'a, *mut T> for &'a T {
    #[inline(always)]
    fn from_abi(arg: *mut T) -> Result<Self, HookGuardFailure> {
        unsafe { arg.as_ref() }.ok_or(HookGuardFailure::Null)
    }

    #[inline(always)]
    fn into_abi(self) -> *mut T {
        self as *const T as *mut T
    }
}

impl<'a, T> HookArg<'a, *mut T> for &'a mut T {
    #[inline(always)]
    fn from_abi(arg: *mut T) -> Result<Self, HookGuardFailure> {
        unsafe { arg.as_mut() }.ok_or(HookGuardFailure::Null)
    }

    #[inline(always)]
    fn into_abi(self) -> *mut T {
        self
    }
}

impl<'a, T> HookArg<'a, *mut T> for Option<&'a T> {
    #[inline(always)]
    fn from_abi(arg: *mut T) -> Result<Self, HookGuardFailure> {
        Ok(unsafe { arg.as_ref() })
    }

    #[inline(always)]
    fn into_abi(self) -> *mut T {
        self.map_or(core::ptr::null_mut(), |value| value as *const T as *mut T)
    }
}

impl<'a, T> HookArg<'a, *mut T> for Option<&'a mut T> {
    #[inline(always)]
    fn from_abi(arg: *mut T) -> Result<Self, HookGuardFailure> {
        Ok(unsafe { arg.as_mut() })
    }

    #[inline(always)]
    fn into_abi(self) -> *mut T {
        self.map_or(core::ptr::null_mut(), |value| value as *mut T)
    }
}

impl<'a, H> HookArg<'a, H> for ResolvedHandle<H>
where
    H: ResolvableHandle,
{
    #[inline(always)]
    fn from_abi(arg: H) -> Result<Self, HookGuardFailure> {
        if arg.is_null() {
            Err(HookGuardFailure::Null)
        } else {
            ResolvedHandle::from_handle(arg).ok_or(HookGuardFailure::Unresolved)
        }
    }

    #[inline(always)]
    fn into_abi(self) -> H {
        self.handle()
    }
}

impl<'a, H> HookArg<'a, H> for Option<ResolvedHandle<H>>
where
    H: ResolvableHandle,
{
    #[inline(always)]
    fn from_abi(arg: H) -> Result<Self, HookGuardFailure> {
        if arg.is_null() {
            Ok(None)
        } else {
            ResolvedHandle::from_handle(arg)
                .map(Some)
                .ok_or(HookGuardFailure::Unresolved)
        }
    }

    #[inline(always)]
    fn into_abi(self) -> H {
        match self {
            Some(value) => value.handle(),
            None => H::from_abi_default_null(),
        }
    }
}

pub trait HookNullAbi<Abi> {
    fn from_abi_default_null() -> Abi;
}

impl<H> HookNullAbi<H> for H
where
    H: ResolvableHandle,
{
    #[inline(always)]
    fn from_abi_default_null() -> H {
        unsafe { core::mem::zeroed() }
    }
}

impl<'a, T> HookArg<'a, *mut T> for Resolved<T>
where
    T: HandleFamilyTarget,
{
    #[inline(always)]
    fn from_abi(arg: *mut T) -> Result<Self, HookGuardFailure> {
        if arg.is_null() {
            return Err(HookGuardFailure::Null);
        }

        let handle = T::canonical_handle(arg);
        let inner = ResolvedHandle::from_handle(handle).ok_or(HookGuardFailure::Unresolved)?;
        Self::from_inner(inner).ok_or(HookGuardFailure::ConvertFail)
    }

    #[inline(always)]
    fn into_abi(self) -> *mut T {
        self.as_ptr()
    }
}

impl<'a, T> HookArg<'a, *mut T> for Option<Resolved<T>>
where
    T: HandleFamilyTarget,
{
    #[inline(always)]
    fn from_abi(arg: *mut T) -> Result<Self, HookGuardFailure> {
        if arg.is_null() {
            return Ok(None);
        }

        let handle = T::canonical_handle(arg);
        let inner = ResolvedHandle::from_handle(handle).ok_or(HookGuardFailure::Unresolved)?;
        Resolved::from_inner(inner)
            .map(Some)
            .ok_or(HookGuardFailure::ConvertFail)
    }

    #[inline(always)]
    fn into_abi(self) -> *mut T {
        self.map_or(core::ptr::null_mut(), |value| value.as_ptr())
    }
}
