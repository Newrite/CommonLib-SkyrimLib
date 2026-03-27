use crate::re::native_function::{NativeFunction, NativeFunctionDesc, NativeFunctionHandler};

/// Bridge-backed owner for CommonLib `RE::NativeLatentFunction<...>`.
///
/// The wrapped callback must still write the immediate latent-start status into
/// the `result_value` `Variable`, just like CommonLib's template base does.
pub struct NativeLatentFunction {
    inner: NativeFunction,
}

/// C++ `RE::BSScript::LatentStatus`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LatentStatus(pub bool);

impl LatentStatus {
    pub const FAILED: Self = Self(false);
    pub const STARTED: Self = Self(true);
}

#[derive(Clone, Copy)]
pub struct NativeLatentFunctionDesc<'a> {
    pub fn_name: &'a str,
    pub class_name: &'a str,
    pub is_static: bool,
    pub latent_return_type: crate::re::type_info::TypeInfo,
    pub param_types: &'a [crate::re::type_info::TypeInfo],
}

impl NativeLatentFunction {
    #[inline(always)]
    pub fn new<H>(desc: NativeLatentFunctionDesc<'_>, handler: H) -> Option<Self>
    where
        H: NativeFunctionHandler + 'static,
    {
        let inner = NativeFunction::new_inner(
            NativeFunctionDesc {
                fn_name: desc.fn_name,
                class_name: desc.class_name,
                is_static: desc.is_static,
                return_type: desc.latent_return_type,
                param_types: desc.param_types,
            },
            true,
            handler,
        )?;
        Some(Self { inner })
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *mut crate::re::native_function_base::NativeFunctionBase {
        self.inner.as_ptr()
    }

    #[inline(always)]
    pub fn as_ifunction_ptr(&self) -> *mut crate::re::ifunction::IFunction {
        self.inner.as_ifunction_ptr()
    }

    #[inline(always)]
    pub fn into_raw(self) -> *mut crate::re::native_function_base::NativeFunctionBase {
        self.inner.into_raw()
    }

    #[inline(always)]
    pub fn into_ifunction_raw(self) -> *mut crate::re::ifunction::IFunction {
        self.inner.into_ifunction_raw()
    }
}

impl AsRef<crate::re::native_function_base::NativeFunctionBase> for NativeLatentFunction {
    #[inline(always)]
    fn as_ref(&self) -> &crate::re::native_function_base::NativeFunctionBase {
        self.inner.as_ref()
    }
}

impl AsMut<crate::re::native_function_base::NativeFunctionBase> for NativeLatentFunction {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut crate::re::native_function_base::NativeFunctionBase {
        self.inner.as_mut()
    }
}

impl AsRef<crate::re::ifunction::IFunction> for NativeLatentFunction {
    #[inline(always)]
    fn as_ref(&self) -> &crate::re::ifunction::IFunction {
        self.inner.as_ref()
    }
}

impl AsMut<crate::re::ifunction::IFunction> for NativeLatentFunction {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut crate::re::ifunction::IFunction {
        self.inner.as_mut()
    }
}
