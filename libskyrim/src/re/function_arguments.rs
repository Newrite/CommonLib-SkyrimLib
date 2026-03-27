use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::ptr::NonNull;

use crate::re::bst_array::BSScrapArray;
use crate::re::ifunction_arguments::IFunctionArguments;
use crate::re::variable::Variable;

/// Rust-side source for bridge-backed `RE::FunctionArguments<Args...>`.
///
/// The CommonLib type is a header-only variadic template whose concrete layout
/// depends on `std::tuple<std::decay_t<Args>...>`. Rust cannot express that as
/// a single honest `repr(C)` type, so this wrapper owns a bridge-backed
/// `RE::BSScript::IFunctionArguments` implementation instead.
pub struct FunctionArguments {
    raw: NonNull<IFunctionArguments>,
}

pub trait FunctionArgumentsSource {
    fn collect_args(&self, dst: &mut BSScrapArray<Variable>) -> bool;
}

impl FunctionArguments {
    #[inline(always)]
    pub fn empty() -> Option<Self> {
        let raw = unsafe { crate::ffi::commonlib_function_arguments_create_zero() }
            .cast::<IFunctionArguments>();
        NonNull::new(raw).map(|raw| Self { raw })
    }

    #[inline(always)]
    pub fn new<S>(source: S) -> Option<Self>
    where
        S: FunctionArgumentsSource + 'static,
    {
        let ctx = Box::into_raw(Box::new(source)).cast::<c_void>();
        let raw = unsafe {
            crate::ffi::commonlib_function_arguments_create(
                ctx,
                Some(bridge_function_arguments_collect::<S>),
                Some(bridge_function_arguments_destroy::<S>),
            )
        }
        .cast::<IFunctionArguments>();

        let Some(raw) = NonNull::new(raw) else {
            unsafe { bridge_function_arguments_destroy::<S>(ctx) };
            return None;
        };

        Some(Self { raw })
    }

    #[inline(always)]
    pub fn from_variables(values: impl IntoIterator<Item = Variable>) -> Option<Self> {
        let values: Vec<_> = values.into_iter().collect();
        if values.is_empty() {
            Self::empty()
        } else {
            Self::new(VariableArguments { values })
        }
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *mut IFunctionArguments {
        self.raw.as_ptr()
    }
}

impl Drop for FunctionArguments {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::commonlib_function_arguments_destroy(self.raw.as_ptr().cast::<c_void>());
        }
    }
}

impl AsRef<IFunctionArguments> for FunctionArguments {
    #[inline(always)]
    fn as_ref(&self) -> &IFunctionArguments {
        unsafe { self.raw.as_ref() }
    }
}

impl AsMut<IFunctionArguments> for FunctionArguments {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut IFunctionArguments {
        unsafe { self.raw.as_mut() }
    }
}

struct VariableArguments {
    values: Vec<Variable>,
}

impl FunctionArgumentsSource for VariableArguments {
    fn collect_args(&self, dst: &mut BSScrapArray<Variable>) -> bool {
        unsafe {
            dst.clear();
            for value in &self.values {
                dst.push(value.clone());
            }
        }
        true
    }
}

unsafe extern "C" fn bridge_function_arguments_collect<S>(
    ctx: *mut c_void,
    dst: *mut c_void,
) -> bool
where
    S: FunctionArgumentsSource,
{
    let source = unsafe { &*ctx.cast::<S>() };
    let dst = unsafe { &mut *dst.cast::<BSScrapArray<Variable>>() };
    source.collect_args(dst)
}

unsafe extern "C" fn bridge_function_arguments_destroy<S>(ctx: *mut c_void) {
    if !ctx.is_null() {
        unsafe {
            drop(Box::from_raw(ctx.cast::<S>()));
        }
    }
}
