use alloc::boxed::Box;
use alloc::ffi::CString;
use core::ffi::c_void;
use core::ptr::NonNull;

use crate::re::bs_core_types::VMStackID;
use crate::re::ifunction::IFunction;
use crate::re::native_function_base::NativeFunctionBase;
use crate::re::stack_frame::StackFrame;
use crate::re::type_info::TypeInfo;
use crate::re::variable::Variable;
use crate::re::virtual_machine::VirtualMachine;

/// Bridge-backed owner for CommonLib `RE::NativeFunction<...>`.
///
/// The original C++ type is a header-only template whose concrete layout
/// depends on both callback signature and `std::function`. This Rust wrapper
/// keeps the source-backed `NativeFunctionBase` ABI surface while delegating
/// the template-specific storage and marshalling entrypoint to the shared C++
/// bridge.
pub struct NativeFunction {
    raw: NonNull<NativeFunctionBase>,
}

pub trait NativeFunctionHandler {
    /// # Safety
    /// The engine-provided objects must be treated as borrowed runtime values
    /// valid only for the duration of the callback.
    unsafe fn marshall_and_dispatch(
        &self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool;
}

#[derive(Clone, Copy)]
pub struct NativeFunctionDesc<'a> {
    pub fn_name: &'a str,
    pub class_name: &'a str,
    pub is_static: bool,
    pub return_type: TypeInfo,
    pub param_types: &'a [TypeInfo],
}

impl NativeFunction {
    #[inline(always)]
    pub fn new<H>(desc: NativeFunctionDesc<'_>, handler: H) -> Option<Self>
    where
        H: NativeFunctionHandler + 'static,
    {
        Self::new_inner(desc, false, handler)
    }

    #[inline(always)]
    pub(crate) fn new_inner<H>(
        desc: NativeFunctionDesc<'_>,
        is_latent: bool,
        handler: H,
    ) -> Option<Self>
    where
        H: NativeFunctionHandler + 'static,
    {
        let fn_name = CString::new(desc.fn_name).ok()?;
        let class_name = CString::new(desc.class_name).ok()?;
        let ctx = Box::into_raw(Box::new(handler)).cast::<c_void>();
        let param_types_ptr = if desc.param_types.is_empty() {
            core::ptr::null()
        } else {
            desc.param_types.as_ptr()
        };
        let raw = unsafe {
            crate::ffi::commonlib_native_function_create(
                ctx,
                Some(bridge_native_function_marshall::<H>),
                Some(bridge_native_function_destroy::<H>),
                fn_name.as_ptr(),
                class_name.as_ptr(),
                desc.is_static,
                (&desc.return_type as *const TypeInfo).cast::<c_void>(),
                param_types_ptr.cast::<c_void>(),
                desc.param_types.len(),
                is_latent,
            )
        }
        .cast::<NativeFunctionBase>();

        let Some(raw) = NonNull::new(raw) else {
            unsafe { bridge_native_function_destroy::<H>(ctx) };
            return None;
        };

        Some(Self { raw })
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *mut NativeFunctionBase {
        self.raw.as_ptr()
    }

    #[inline(always)]
    pub fn as_ifunction_ptr(&self) -> *mut IFunction {
        self.raw.as_ptr().cast::<IFunction>()
    }

    #[inline(always)]
    pub fn into_raw(self) -> *mut NativeFunctionBase {
        let raw = self.raw.as_ptr();
        core::mem::forget(self);
        raw
    }

    #[inline(always)]
    pub fn into_ifunction_raw(self) -> *mut IFunction {
        self.into_raw().cast::<IFunction>()
    }
}

impl Drop for NativeFunction {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::commonlib_native_function_destroy(self.raw.as_ptr().cast::<c_void>());
        }
    }
}

impl AsRef<NativeFunctionBase> for NativeFunction {
    #[inline(always)]
    fn as_ref(&self) -> &NativeFunctionBase {
        unsafe { self.raw.as_ref() }
    }
}

impl AsMut<NativeFunctionBase> for NativeFunction {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut NativeFunctionBase {
        unsafe { self.raw.as_mut() }
    }
}

impl AsRef<IFunction> for NativeFunction {
    #[inline(always)]
    fn as_ref(&self) -> &IFunction {
        unsafe { &*self.raw.as_ptr().cast::<IFunction>() }
    }
}

impl AsMut<IFunction> for NativeFunction {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut IFunction {
        unsafe { &mut *self.raw.as_ptr().cast::<IFunction>() }
    }
}

unsafe extern "C" fn bridge_native_function_marshall<H>(
    ctx: *mut c_void,
    base_value: *mut c_void,
    vm: *mut c_void,
    stack_id: VMStackID,
    result_value: *mut c_void,
    frame: *const c_void,
) -> bool
where
    H: NativeFunctionHandler,
{
    crate::skse::crash::guard("Papyrus native-function marshall bridge", || {
        let handler = unsafe { &*ctx.cast::<H>() };
        let base_value = unsafe { &mut *base_value.cast::<Variable>() };
        let vm = unsafe { &mut *vm.cast::<VirtualMachine>() };
        let result_value = unsafe { &mut *result_value.cast::<Variable>() };
        let frame = unsafe { &*frame.cast::<StackFrame>() };
        unsafe { handler.marshall_and_dispatch(base_value, vm, stack_id, result_value, frame) }
    })
}

unsafe extern "C" fn bridge_native_function_destroy<H>(ctx: *mut c_void) {
    if !ctx.is_null() {
        unsafe {
            drop(Box::from_raw(ctx.cast::<H>()));
        }
    }
}
