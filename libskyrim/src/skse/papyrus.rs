use core::marker::PhantomData;

use crate::re::{
    IVirtualMachine, LatentStatus, NativeFunction, NativeFunctionDesc, NativeFunctionHandler,
    NativeLatentFunction, NativeLatentFunctionDesc, PapyrusBase, PapyrusFunctionSignature,
    PapyrusLatentFunctionSignature, PapyrusLongFunctionSignature, PapyrusParameter,
    PapyrusParameterConvertible, PapyrusReturn, PapyrusReturnConvertible, PapyrusValidBase,
    StackFrame, TypeInfo, VMStackID, Variable, VirtualMachine,
};
use crate::relocation::RttiType;
use crate::sdk::core::{
    CanonicalHandle, ConstRttiCastSource, GameRef, GameRefMut, HandleFamilyTarget,
    MutRttiCastSource, Resolved, ResolvedHandle, sealed as sdk_core_sealed,
};

use super::api;
use super::interfaces::PapyrusRegFunction2;

#[inline(always)]
pub fn register(callback: PapyrusRegFunction2) -> bool {
    let interface = api::get_papyrus_interface();
    if interface.is_null() {
        return false;
    }

    unsafe { (*interface).register_vm(callback) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FunctionOptions {
    pub callable_from_tasklets: bool,
}

impl FunctionOptions {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            callable_from_tasklets: false,
        }
    }

    #[inline(always)]
    pub const fn callable_from_tasklets(mut self, value: bool) -> Self {
        self.callable_from_tasklets = value;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PapyrusRef<T> {
    raw: *mut T,
}

impl<T> PapyrusRef<T> {
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            raw: core::ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub const fn from_raw(raw: *mut T) -> Self {
        Self { raw }
    }

    #[inline(always)]
    pub const fn raw(self) -> *mut T {
        self.raw
    }

    #[inline(always)]
    pub const fn is_null(self) -> bool {
        self.raw.is_null()
    }

    #[inline(always)]
    pub fn as_ref(&self) -> Option<&T> {
        unsafe { self.raw.as_ref() }
    }

    #[inline(always)]
    pub fn as_game_ref(&self) -> GameRef<'_, T> {
        unsafe { GameRef::from_raw(self.raw) }
    }

    #[inline(always)]
    pub unsafe fn as_mut(&mut self) -> Option<&mut T> {
        unsafe { self.raw.as_mut() }
    }

    #[inline(always)]
    pub fn as_game_ref_mut(&mut self) -> GameRefMut<'_, T> {
        unsafe { GameRefMut::from_raw(self.raw) }
    }

    #[inline(always)]
    pub const fn cast<U>(self) -> PapyrusRef<U> {
        PapyrusRef::from_raw(self.raw.cast())
    }

    #[inline(always)]
    pub fn try_cast<U>(self) -> Option<PapyrusRef<U>>
    where
        T: RttiType,
        U: RttiType,
    {
        self.as_game_ref().try_cast::<U>().map(PapyrusRef::from)
    }
}

impl<T> Default for PapyrusRef<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}

impl<T> From<*mut T> for PapyrusRef<T> {
    #[inline(always)]
    fn from(value: *mut T) -> Self {
        Self::from_raw(value)
    }
}

impl<T> From<GameRef<'_, T>> for PapyrusRef<T> {
    #[inline(always)]
    fn from(value: GameRef<'_, T>) -> Self {
        Self::from_raw(value.as_ptr())
    }
}

impl<T> From<GameRefMut<'_, T>> for PapyrusRef<T> {
    #[inline(always)]
    fn from(value: GameRefMut<'_, T>) -> Self {
        Self::from_raw(value.as_ptr())
    }
}

impl<'a, T> From<&'a PapyrusRef<T>> for GameRef<'a, T> {
    #[inline(always)]
    fn from(value: &'a PapyrusRef<T>) -> Self {
        value.as_game_ref()
    }
}

impl<'a, T> From<&'a mut PapyrusRef<T>> for GameRefMut<'a, T> {
    #[inline(always)]
    fn from(value: &'a mut PapyrusRef<T>) -> Self {
        value.as_game_ref_mut()
    }
}

impl<T> sdk_core_sealed::Sealed for PapyrusRef<T> where T: RttiType {}

impl<T> ConstRttiCastSource for PapyrusRef<T>
where
    T: RttiType,
{
    type Source = T;

    #[inline(always)]
    fn raw_const_source_ptr(&self) -> *const Self::Source {
        self.raw.cast_const()
    }
}

impl<T> MutRttiCastSource for PapyrusRef<T>
where
    T: RttiType,
{
    #[inline(always)]
    fn raw_mut_source_ptr(&mut self) -> *mut Self::Source {
        self.raw
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Context {
    vm: *mut IVirtualMachine,
    stack_id: VMStackID,
}

impl Context {
    #[inline(always)]
    pub(crate) const fn new(vm: *mut IVirtualMachine, stack_id: VMStackID) -> Self {
        Self { vm, stack_id }
    }

    #[inline(always)]
    pub const fn raw_vm(&self) -> *mut IVirtualMachine {
        self.vm
    }

    #[inline(always)]
    pub fn vm_ref(&self) -> GameRef<'_, IVirtualMachine> {
        unsafe { GameRef::from_raw(self.vm) }
    }

    #[inline(always)]
    pub fn vm(&self) -> Option<&IVirtualMachine> {
        self.vm_ref().as_ref()
    }

    #[inline(always)]
    pub const fn stack_id(self) -> VMStackID {
        self.stack_id
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LatentContext {
    inner: Context,
}

impl LatentContext {
    #[inline(always)]
    pub(crate) const fn new(vm: *mut IVirtualMachine, stack_id: VMStackID) -> Self {
        Self {
            inner: Context::new(vm, stack_id),
        }
    }

    #[inline(always)]
    pub const fn context(self) -> Context {
        self.inner
    }

    #[inline(always)]
    pub const fn raw_vm(&self) -> *mut IVirtualMachine {
        self.inner.raw_vm()
    }

    #[inline(always)]
    pub fn vm_ref(&self) -> GameRef<'_, IVirtualMachine> {
        self.inner.vm_ref()
    }

    #[inline(always)]
    pub fn vm(&self) -> Option<&IVirtualMachine> {
        self.inner.vm()
    }

    #[inline(always)]
    pub const fn stack_id(self) -> VMStackID {
        self.inner.stack_id()
    }

    #[inline(always)]
    pub fn return_result<R>(self, result: R) -> bool
    where
        R: PapyrusReturn + PapyrusReturnConvertible,
    {
        let Some(vm) = self.vm() else {
            return false;
        };
        vm.return_latent_result(self.stack_id(), result)
    }

    #[inline(always)]
    pub fn return_variable(self, result: &Variable) {
        if let Some(vm) = self.vm() {
            vm.return_latent_variable(self.stack_id(), result);
        }
    }
}

pub trait PapyrusClass {
    const NAME: &'static str;
}

pub trait PapyrusModule {
    fn register(module: &mut ModuleRegistry<'_>) -> bool;
}

pub struct ModuleRegistry<'vm> {
    vm: &'vm IVirtualMachine,
    ok: bool,
}

impl<'vm> ModuleRegistry<'vm> {
    #[inline(always)]
    pub fn new(vm: &'vm IVirtualMachine) -> Self {
        Self { vm, ok: true }
    }

    #[inline(always)]
    pub fn vm(&self) -> &'vm IVirtualMachine {
        self.vm
    }

    #[inline(always)]
    pub fn note(&mut self, ok: bool) -> bool {
        self.ok &= ok;
        ok
    }

    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        self.ok
    }

    #[inline(always)]
    pub fn finish(self) -> bool {
        self.ok
    }

    #[inline(always)]
    pub fn class<'a, C>(&'a mut self) -> ClassRegistry<'a, 'vm, C>
    where
        C: PapyrusClass,
    {
        ClassRegistry {
            module: self,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn class_named<'a>(&'a mut self, class_name: &'a str) -> NamedClassRegistry<'a, 'vm> {
        NamedClassRegistry {
            module: self,
            class_name,
        }
    }
}

pub struct ClassRegistry<'a, 'vm, C>
where
    C: PapyrusClass,
{
    module: &'a mut ModuleRegistry<'vm>,
    marker: PhantomData<C>,
}

impl<'a, 'vm, C> ClassRegistry<'a, 'vm, C>
where
    C: PapyrusClass,
{
    #[inline(always)]
    pub fn vm(&self) -> &'vm IVirtualMachine {
        self.module.vm()
    }

    #[inline(always)]
    pub fn class_name(&self) -> &'static str {
        C::NAME
    }

    #[inline(always)]
    pub fn note(&mut self, ok: bool) -> bool {
        self.module.note(ok)
    }

    #[inline(always)]
    pub fn register_static_function<F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        F: PapyrusStaticFunctionSignature,
    {
        self.note(bind_static_function(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }

    #[inline(always)]
    pub fn register_method_function<F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        F: PapyrusMethodFunctionSignature,
    {
        self.note(bind_method_function(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }

    #[inline(always)]
    pub fn register_static_long_function<F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        F: PapyrusStaticLongFunctionSignature,
    {
        self.note(bind_static_long_function(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }

    #[inline(always)]
    pub fn register_method_long_function<F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        F: PapyrusMethodLongFunctionSignature,
    {
        self.note(bind_method_long_function(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }

    #[inline(always)]
    pub fn register_static_latent_function<R, F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        R: PapyrusReturn + PapyrusReturnConvertible + 'static,
        F: PapyrusStaticLatentFunctionSignature,
    {
        self.note(bind_static_latent_function::<R, F>(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }

    #[inline(always)]
    pub fn register_method_latent_function<R, F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        R: PapyrusReturn + PapyrusReturnConvertible + 'static,
        F: PapyrusMethodLatentFunctionSignature,
    {
        self.note(bind_method_latent_function::<R, F>(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }
}

pub struct NamedClassRegistry<'a, 'vm> {
    module: &'a mut ModuleRegistry<'vm>,
    class_name: &'a str,
}

impl<'a, 'vm> NamedClassRegistry<'a, 'vm> {
    #[inline(always)]
    pub fn vm(&self) -> &'vm IVirtualMachine {
        self.module.vm()
    }

    #[inline(always)]
    pub fn class_name(&self) -> &'a str {
        self.class_name
    }

    #[inline(always)]
    pub fn note(&mut self, ok: bool) -> bool {
        self.module.note(ok)
    }

    #[inline(always)]
    pub fn register_static_function<F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        F: PapyrusStaticFunctionSignature,
    {
        self.note(bind_static_function(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }

    #[inline(always)]
    pub fn register_method_function<F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        F: PapyrusMethodFunctionSignature,
    {
        self.note(bind_method_function(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }

    #[inline(always)]
    pub fn register_static_long_function<F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        F: PapyrusStaticLongFunctionSignature,
    {
        self.note(bind_static_long_function(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }

    #[inline(always)]
    pub fn register_method_long_function<F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        F: PapyrusMethodLongFunctionSignature,
    {
        self.note(bind_method_long_function(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }

    #[inline(always)]
    pub fn register_static_latent_function<R, F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        R: PapyrusReturn + PapyrusReturnConvertible + 'static,
        F: PapyrusStaticLatentFunctionSignature,
    {
        self.note(bind_static_latent_function::<R, F>(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }

    #[inline(always)]
    pub fn register_method_latent_function<R, F>(
        &mut self,
        fn_name: &str,
        callback: F,
        options: FunctionOptions,
    ) -> bool
    where
        R: PapyrusReturn + PapyrusReturnConvertible + 'static,
        F: PapyrusMethodLatentFunctionSignature,
    {
        self.note(bind_method_latent_function::<R, F>(
            self.vm(),
            fn_name,
            self.class_name(),
            callback,
            options,
        ))
    }
}

#[inline(always)]
pub fn register_module<M>() -> bool
where
    M: PapyrusModule,
{
    unsafe extern "system" fn register_impl<M>(vm: *mut IVirtualMachine) -> bool
    where
        M: PapyrusModule,
    {
        let Some(vm) = (unsafe { vm.as_ref() }) else {
            return false;
        };

        let mut module = ModuleRegistry::new(vm);
        let ok = M::register(&mut module);
        module.note(ok);
        module.finish()
    }

    register(register_impl::<M>)
}

pub struct Registry<'a> {
    vm: &'a IVirtualMachine,
    class_name: &'a str,
    ok: bool,
}

impl<'a> Registry<'a> {
    #[inline(always)]
    pub fn new(vm: &'a IVirtualMachine, class_name: &'a str) -> Self {
        Self {
            vm,
            class_name,
            ok: true,
        }
    }

    #[inline(always)]
    pub fn vm(&self) -> &'a IVirtualMachine {
        self.vm
    }

    #[inline(always)]
    pub fn class_name(&self) -> &'a str {
        self.class_name
    }

    #[inline(always)]
    pub fn register_function<F>(
        &mut self,
        fn_name: &str,
        callback: F,
        callable_from_tasklets: bool,
    ) -> bool
    where
        F: PapyrusFunctionSignature,
    {
        self.note(self.vm.register_function(
            fn_name,
            self.class_name,
            callback,
            callable_from_tasklets,
        ))
    }

    #[inline(always)]
    pub fn register_long_function<F>(
        &mut self,
        fn_name: &str,
        callback: F,
        callable_from_tasklets: bool,
    ) -> bool
    where
        F: PapyrusLongFunctionSignature,
    {
        self.note(self.vm.register_long_function(
            fn_name,
            self.class_name,
            callback,
            callable_from_tasklets,
        ))
    }

    #[inline(always)]
    pub fn register_latent_function<R, F>(
        &mut self,
        fn_name: &str,
        callback: F,
        callable_from_tasklets: bool,
    ) -> bool
    where
        R: PapyrusReturn + PapyrusReturnConvertible + 'static,
        F: PapyrusLatentFunctionSignature,
    {
        self.note(self.vm.register_latent_function::<R, F>(
            fn_name,
            self.class_name,
            callback,
            callable_from_tasklets,
        ))
    }

    #[inline(always)]
    pub fn note(&mut self, ok: bool) -> bool {
        self.ok &= ok;
        ok
    }

    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        self.ok
    }

    #[inline(always)]
    pub fn finish(self) -> bool {
        self.ok
    }
}

#[macro_export]
macro_rules! papyrus_register_function {
    ($registry:expr, $fn_name:expr, $callback:path => fn($base:ty $(, $param:ty)*) -> $ret:ty $(, callable_from_tasklets = $callable:expr)? $(,)?) => {{
        let registry = &mut $registry;
        registry.register_function(
            $fn_name,
            $callback as fn($base $(, $param)*) -> $ret,
            $crate::papyrus_register_function!(@callable $($callable)?),
        )
    }};
    (@callable $callable:expr) => {
        $callable
    };
    (@callable) => {
        false
    };
}

#[macro_export]
macro_rules! papyrus_register_long_function {
    ($registry:expr, $fn_name:expr, $callback:path => fn($vm:ty, $stack_id:ty, $base:ty $(, $param:ty)*) -> $ret:ty $(, callable_from_tasklets = $callable:expr)? $(,)?) => {{
        let registry = &mut $registry;
        let _ = core::marker::PhantomData::<($vm, $stack_id)>;
        registry.register_long_function(
            $fn_name,
            $callback as fn($vm, $stack_id, $base $(, $param)*) -> $ret,
            $crate::papyrus_register_long_function!(@callable $($callable)?),
        )
    }};
    (@callable $callable:expr) => {
        $callable
    };
    (@callable) => {
        false
    };
}

#[macro_export]
macro_rules! papyrus_register_latent_function {
    ($registry:expr, $fn_name:expr, $callback:path => fn($vm:ty, $stack_id:ty, $base:ty $(, $param:ty)*) -> $status:ty, returns $latent:ty $(, callable_from_tasklets = $callable:expr)? $(,)?) => {{
        let registry = &mut $registry;
        let _ = core::marker::PhantomData::<($vm, $stack_id, $status, $latent)>;
        registry.register_latent_function::<$latent, _>(
            $fn_name,
            $callback as fn($vm, $stack_id, $base $(, $param)*) -> $status,
            $crate::papyrus_register_latent_function!(@callable $($callable)?),
        )
    }};
    (@callable $callable:expr) => {
        $callable
    };
    (@callable) => {
        false
    };
}

pub trait UserPapyrusBase: Sized {
    type Raw: PapyrusBase + PapyrusValidBase + 'static;

    const IS_STATIC: bool = <Self::Raw as PapyrusBase>::IS_STATIC;

    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self>;
}

pub trait UserPapyrusParameter: Sized {
    type Raw: PapyrusParameter + PapyrusParameterConvertible + 'static;

    fn parameter_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        <Self::Raw as PapyrusParameter>::parameter_type_info(vm)
    }

    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self>;
}

impl<T> UserPapyrusBase for *mut T
where
    *mut T: PapyrusBase + PapyrusValidBase + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        Some(raw)
    }
}

impl<T> UserPapyrusBase for PapyrusRef<T>
where
    *mut T: PapyrusBase + PapyrusValidBase + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        Some(PapyrusRef::from_raw(raw))
    }
}

impl<'a, T> UserPapyrusBase for GameRef<'a, T>
where
    *mut T: PapyrusBase + PapyrusValidBase + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        Some(unsafe { GameRef::from_raw(raw) })
    }
}

impl<'a, T> UserPapyrusBase for GameRefMut<'a, T>
where
    *mut T: PapyrusBase + PapyrusValidBase + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        Some(unsafe { GameRefMut::from_raw(raw) })
    }
}

impl<H> UserPapyrusBase for ResolvedHandle<H>
where
    H: CanonicalHandle + 'static,
    *mut H::Target: PapyrusBase + PapyrusValidBase + 'static,
{
    type Raw = *mut H::Target;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        H::from_target_ptr(raw).resolve()
    }
}

impl<T> UserPapyrusBase for Resolved<T>
where
    T: HandleFamilyTarget + 'static,
    *mut T: PapyrusBase + PapyrusValidBase + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        Resolved::from_ptr(raw)
    }
}

impl<H> UserPapyrusBase for Option<ResolvedHandle<H>>
where
    H: CanonicalHandle + 'static,
    *mut H::Target: PapyrusBase + PapyrusValidBase + 'static,
{
    type Raw = *mut H::Target;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        if raw.is_null() {
            Some(None)
        } else {
            Some(H::from_target_ptr(raw).resolve())
        }
    }
}

impl<T> UserPapyrusBase for Option<Resolved<T>>
where
    T: HandleFamilyTarget + 'static,
    *mut T: PapyrusBase + PapyrusValidBase + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        if raw.is_null() {
            Some(None)
        } else {
            Some(Resolved::from_ptr(raw))
        }
    }
}

impl<T> UserPapyrusParameter for T
where
    T: PapyrusParameter + PapyrusParameterConvertible + 'static,
{
    type Raw = T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        Some(raw)
    }
}

impl<T> UserPapyrusParameter for PapyrusRef<T>
where
    *mut T: PapyrusParameter + PapyrusParameterConvertible + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        Some(PapyrusRef::from_raw(raw))
    }
}

impl<'a, T> UserPapyrusParameter for GameRef<'a, T>
where
    *mut T: PapyrusParameter + PapyrusParameterConvertible + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        Some(unsafe { GameRef::from_raw(raw) })
    }
}

impl<'a, T> UserPapyrusParameter for GameRefMut<'a, T>
where
    *mut T: PapyrusParameter + PapyrusParameterConvertible + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        Some(unsafe { GameRefMut::from_raw(raw) })
    }
}

impl<H> UserPapyrusParameter for ResolvedHandle<H>
where
    H: CanonicalHandle + 'static,
    *mut H::Target: PapyrusParameter + PapyrusParameterConvertible + 'static,
{
    type Raw = *mut H::Target;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        H::from_target_ptr(raw).resolve()
    }
}

impl<T> UserPapyrusParameter for Resolved<T>
where
    T: HandleFamilyTarget + 'static,
    *mut T: PapyrusParameter + PapyrusParameterConvertible + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        Resolved::from_ptr(raw)
    }
}

impl<H> UserPapyrusParameter for Option<ResolvedHandle<H>>
where
    H: CanonicalHandle + 'static,
    *mut H::Target: PapyrusParameter + PapyrusParameterConvertible + 'static,
{
    type Raw = *mut H::Target;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        if raw.is_null() {
            Some(None)
        } else {
            Some(H::from_target_ptr(raw).resolve())
        }
    }
}

impl<T> UserPapyrusParameter for Option<Resolved<T>>
where
    T: HandleFamilyTarget + 'static,
    *mut T: PapyrusParameter + PapyrusParameterConvertible + 'static,
{
    type Raw = *mut T;

    #[inline(always)]
    unsafe fn try_from_raw(raw: Self::Raw) -> Option<Self> {
        if raw.is_null() {
            Some(None)
        } else {
            Some(Resolved::from_ptr(raw))
        }
    }
}

pub trait PapyrusStaticFunctionSignature: Copy + 'static {
    fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo>;

    fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>>;

    unsafe fn dispatch(
        self,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool;
}

pub trait PapyrusMethodFunctionSignature: Copy + 'static {
    fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo>;

    fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>>;

    unsafe fn dispatch(
        self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool;
}

pub trait PapyrusStaticLongFunctionSignature: Copy + 'static {
    fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo>;

    fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>>;

    unsafe fn dispatch(
        self,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool;
}

pub trait PapyrusMethodLongFunctionSignature: Copy + 'static {
    fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo>;

    fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>>;

    unsafe fn dispatch(
        self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool;
}

pub trait PapyrusStaticLatentFunctionSignature: Copy + 'static {
    fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>>;

    unsafe fn dispatch(
        self,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool;
}

pub trait PapyrusMethodLatentFunctionSignature: Copy + 'static {
    fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>>;

    unsafe fn dispatch(
        self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool;
}

struct StaticFunctionHandler<F> {
    callback: F,
}

impl<F> NativeFunctionHandler for StaticFunctionHandler<F>
where
    F: PapyrusStaticFunctionSignature,
{
    #[inline(always)]
    unsafe fn marshall_and_dispatch(
        &self,
        _base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool {
        unsafe { self.callback.dispatch(vm, stack_id, result_value, frame) }
    }
}

struct MethodFunctionHandler<F> {
    callback: F,
}

impl<F> NativeFunctionHandler for MethodFunctionHandler<F>
where
    F: PapyrusMethodFunctionSignature,
{
    #[inline(always)]
    unsafe fn marshall_and_dispatch(
        &self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool {
        unsafe {
            self.callback
                .dispatch(base_value, vm, stack_id, result_value, frame)
        }
    }
}

struct StaticLongFunctionHandler<F> {
    callback: F,
}

impl<F> NativeFunctionHandler for StaticLongFunctionHandler<F>
where
    F: PapyrusStaticLongFunctionSignature,
{
    #[inline(always)]
    unsafe fn marshall_and_dispatch(
        &self,
        _base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool {
        unsafe { self.callback.dispatch(vm, stack_id, result_value, frame) }
    }
}

struct MethodLongFunctionHandler<F> {
    callback: F,
}

impl<F> NativeFunctionHandler for MethodLongFunctionHandler<F>
where
    F: PapyrusMethodLongFunctionSignature,
{
    #[inline(always)]
    unsafe fn marshall_and_dispatch(
        &self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool {
        unsafe {
            self.callback
                .dispatch(base_value, vm, stack_id, result_value, frame)
        }
    }
}

struct StaticLatentFunctionHandler<F> {
    callback: F,
}

impl<F> NativeFunctionHandler for StaticLatentFunctionHandler<F>
where
    F: PapyrusStaticLatentFunctionSignature,
{
    #[inline(always)]
    unsafe fn marshall_and_dispatch(
        &self,
        _base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool {
        unsafe { self.callback.dispatch(vm, stack_id, result_value, frame) }
    }
}

struct MethodLatentFunctionHandler<F> {
    callback: F,
}

impl<F> NativeFunctionHandler for MethodLatentFunctionHandler<F>
where
    F: PapyrusMethodLatentFunctionSignature,
{
    #[inline(always)]
    unsafe fn marshall_and_dispatch(
        &self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool {
        unsafe {
            self.callback
                .dispatch(base_value, vm, stack_id, result_value, frame)
        }
    }
}

macro_rules! impl_papyrus_static_function_signature {
    ($(($param:ident, $arg:ident, $idx:expr)),* $(,)?) => {
        impl<R, $($param),*> PapyrusStaticFunctionSignature for fn($($param),*) -> R
        where
            R: PapyrusReturn + PapyrusReturnConvertible + 'static,
            $($param: UserPapyrusParameter + 'static),*
        {
            #[inline(always)]
            fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
                R::return_type_info(vm)
            }

            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                vm: &mut VirtualMachine,
                _stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                $(let Some($arg) = (unsafe {
                    <$param as UserPapyrusParameter>::try_from_raw(
                        <<$param as UserPapyrusParameter>::Raw as PapyrusParameter>::unpack_parameter(
                            frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                        ),
                    )
                }) else {
                    return false;
                };)*
                R::pack_return(self($($arg),*), result_value, vm)
            }
        }
    };
}

macro_rules! impl_papyrus_method_function_signature {
    ($(($param:ident, $arg:ident, $idx:expr)),* $(,)?) => {
        impl<R, Base, $($param),*> PapyrusMethodFunctionSignature for fn(Base, $($param),*) -> R
        where
            R: PapyrusReturn + PapyrusReturnConvertible + 'static,
            Base: UserPapyrusBase + 'static,
            $($param: UserPapyrusParameter + 'static),*
        {
            #[inline(always)]
            fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
                R::return_type_info(vm)
            }

            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                base_value: &mut Variable,
                vm: &mut VirtualMachine,
                _stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                let Some(base) = (unsafe {
                    Base::try_from_raw(
                        <Base::Raw as PapyrusBase>::unpack_base(base_value),
                    )
                }) else {
                    return false;
                };
                $(let Some($arg) = (unsafe {
                    <$param as UserPapyrusParameter>::try_from_raw(
                        <<$param as UserPapyrusParameter>::Raw as PapyrusParameter>::unpack_parameter(
                            frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                        ),
                    )
                }) else {
                    return false;
                };)*
                R::pack_return(self(base, $($arg),*), result_value, vm)
            }
        }
    };
}

macro_rules! impl_papyrus_static_long_function_signature {
    ($(($param:ident, $arg:ident, $idx:expr)),* $(,)?) => {
        impl<R, $($param),*> PapyrusStaticLongFunctionSignature for fn(Context, $($param),*) -> R
        where
            R: PapyrusReturn + PapyrusReturnConvertible + 'static,
            $($param: UserPapyrusParameter + 'static),*
        {
            #[inline(always)]
            fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
                R::return_type_info(vm)
            }

            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                vm: &mut VirtualMachine,
                stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                let ctx = Context::new(vm as *mut VirtualMachine as *mut IVirtualMachine, stack_id);
                $(let Some($arg) = (unsafe {
                    <$param as UserPapyrusParameter>::try_from_raw(
                        <<$param as UserPapyrusParameter>::Raw as PapyrusParameter>::unpack_parameter(
                            frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                        ),
                    )
                }) else {
                    return false;
                };)*
                R::pack_return(self(ctx, $($arg),*), result_value, vm)
            }
        }
    };
}

macro_rules! impl_papyrus_method_long_function_signature {
    ($(($param:ident, $arg:ident, $idx:expr)),* $(,)?) => {
        impl<R, Base, $($param),*> PapyrusMethodLongFunctionSignature
            for fn(Context, Base, $($param),*) -> R
        where
            R: PapyrusReturn + PapyrusReturnConvertible + 'static,
            Base: UserPapyrusBase + 'static,
            $($param: UserPapyrusParameter + 'static),*
        {
            #[inline(always)]
            fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
                R::return_type_info(vm)
            }

            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                base_value: &mut Variable,
                vm: &mut VirtualMachine,
                stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                let ctx = Context::new(vm as *mut VirtualMachine as *mut IVirtualMachine, stack_id);
                let Some(base) = (unsafe {
                    Base::try_from_raw(
                        <Base::Raw as PapyrusBase>::unpack_base(base_value),
                    )
                }) else {
                    return false;
                };
                $(let Some($arg) = (unsafe {
                    <$param as UserPapyrusParameter>::try_from_raw(
                        <<$param as UserPapyrusParameter>::Raw as PapyrusParameter>::unpack_parameter(
                            frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                        ),
                    )
                }) else {
                    return false;
                };)*
                R::pack_return(self(ctx, base, $($arg),*), result_value, vm)
            }
        }
    };
}

macro_rules! impl_papyrus_static_latent_function_signature {
    ($(($param:ident, $arg:ident, $idx:expr)),* $(,)?) => {
        impl<$($param),*> PapyrusStaticLatentFunctionSignature
            for fn(LatentContext, $($param),*) -> LatentStatus
        where
            $($param: UserPapyrusParameter + 'static),*
        {
            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                vm: &mut VirtualMachine,
                stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                let ctx =
                    LatentContext::new(vm as *mut VirtualMachine as *mut IVirtualMachine, stack_id);
                $(let Some($arg) = (unsafe {
                    <$param as UserPapyrusParameter>::try_from_raw(
                        <<$param as UserPapyrusParameter>::Raw as PapyrusParameter>::unpack_parameter(
                            frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                        ),
                    )
                }) else {
                    return false;
                };)*
                result_value.set_bool(self(ctx, $($arg),*).0);
                true
            }
        }
    };
}

macro_rules! impl_papyrus_method_latent_function_signature {
    ($(($param:ident, $arg:ident, $idx:expr)),* $(,)?) => {
        impl<Base, $($param),*> PapyrusMethodLatentFunctionSignature
            for fn(LatentContext, Base, $($param),*) -> LatentStatus
        where
            Base: UserPapyrusBase + 'static,
            $($param: UserPapyrusParameter + 'static),*
        {
            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<alloc::vec::Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                base_value: &mut Variable,
                vm: &mut VirtualMachine,
                stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                let ctx =
                    LatentContext::new(vm as *mut VirtualMachine as *mut IVirtualMachine, stack_id);
                let Some(base) = (unsafe {
                    Base::try_from_raw(
                        <Base::Raw as PapyrusBase>::unpack_base(base_value),
                    )
                }) else {
                    return false;
                };
                $(let Some($arg) = (unsafe {
                    <$param as UserPapyrusParameter>::try_from_raw(
                        <<$param as UserPapyrusParameter>::Raw as PapyrusParameter>::unpack_parameter(
                            frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                        ),
                    )
                }) else {
                    return false;
                };)*
                result_value.set_bool(self(ctx, base, $($arg),*).0);
                true
            }
        }
    };
}

impl_papyrus_static_function_signature!();
impl_papyrus_static_function_signature!((A0, a0, 0));
impl_papyrus_static_function_signature!((A0, a0, 0), (A1, a1, 1));
impl_papyrus_static_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2));
impl_papyrus_static_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2), (A3, a3, 3));
impl_papyrus_static_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4)
);
impl_papyrus_static_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4),
    (A5, a5, 5)
);

impl_papyrus_method_function_signature!();
impl_papyrus_method_function_signature!((A0, a0, 0));
impl_papyrus_method_function_signature!((A0, a0, 0), (A1, a1, 1));
impl_papyrus_method_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2));
impl_papyrus_method_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2), (A3, a3, 3));
impl_papyrus_method_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4)
);
impl_papyrus_method_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4),
    (A5, a5, 5)
);

impl_papyrus_static_long_function_signature!();
impl_papyrus_static_long_function_signature!((A0, a0, 0));
impl_papyrus_static_long_function_signature!((A0, a0, 0), (A1, a1, 1));
impl_papyrus_static_long_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2));
impl_papyrus_static_long_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2), (A3, a3, 3));
impl_papyrus_static_long_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4)
);
impl_papyrus_static_long_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4),
    (A5, a5, 5)
);

impl_papyrus_method_long_function_signature!();
impl_papyrus_method_long_function_signature!((A0, a0, 0));
impl_papyrus_method_long_function_signature!((A0, a0, 0), (A1, a1, 1));
impl_papyrus_method_long_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2));
impl_papyrus_method_long_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2), (A3, a3, 3));
impl_papyrus_method_long_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4)
);
impl_papyrus_method_long_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4),
    (A5, a5, 5)
);

impl_papyrus_static_latent_function_signature!();
impl_papyrus_static_latent_function_signature!((A0, a0, 0));
impl_papyrus_static_latent_function_signature!((A0, a0, 0), (A1, a1, 1));
impl_papyrus_static_latent_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2));
impl_papyrus_static_latent_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2), (A3, a3, 3));
impl_papyrus_static_latent_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4)
);
impl_papyrus_static_latent_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4),
    (A5, a5, 5)
);

impl_papyrus_method_latent_function_signature!();
impl_papyrus_method_latent_function_signature!((A0, a0, 0));
impl_papyrus_method_latent_function_signature!((A0, a0, 0), (A1, a1, 1));
impl_papyrus_method_latent_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2));
impl_papyrus_method_latent_function_signature!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2), (A3, a3, 3));
impl_papyrus_method_latent_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4)
);
impl_papyrus_method_latent_function_signature!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4),
    (A5, a5, 5)
);

fn bind_static_function<F>(
    vm: &IVirtualMachine,
    fn_name: &str,
    class_name: &str,
    callback: F,
    options: FunctionOptions,
) -> bool
where
    F: PapyrusStaticFunctionSignature,
{
    let Some(param_types) = F::parameter_type_infos(vm) else {
        return false;
    };
    let Some(return_type) = F::return_type_info(vm) else {
        return false;
    };
    let Some(function) = NativeFunction::new(
        NativeFunctionDesc {
            fn_name,
            class_name,
            is_static: true,
            return_type,
            param_types: &param_types,
        },
        StaticFunctionHandler { callback },
    ) else {
        return false;
    };

    vm.bind_native_function(
        fn_name,
        class_name,
        function,
        options.callable_from_tasklets,
    )
}

fn bind_method_function<F>(
    vm: &IVirtualMachine,
    fn_name: &str,
    class_name: &str,
    callback: F,
    options: FunctionOptions,
) -> bool
where
    F: PapyrusMethodFunctionSignature,
{
    let Some(param_types) = F::parameter_type_infos(vm) else {
        return false;
    };
    let Some(return_type) = F::return_type_info(vm) else {
        return false;
    };
    let Some(function) = NativeFunction::new(
        NativeFunctionDesc {
            fn_name,
            class_name,
            is_static: false,
            return_type,
            param_types: &param_types,
        },
        MethodFunctionHandler { callback },
    ) else {
        return false;
    };

    vm.bind_native_function(
        fn_name,
        class_name,
        function,
        options.callable_from_tasklets,
    )
}

fn bind_static_long_function<F>(
    vm: &IVirtualMachine,
    fn_name: &str,
    class_name: &str,
    callback: F,
    options: FunctionOptions,
) -> bool
where
    F: PapyrusStaticLongFunctionSignature,
{
    let Some(param_types) = F::parameter_type_infos(vm) else {
        return false;
    };
    let Some(return_type) = F::return_type_info(vm) else {
        return false;
    };
    let Some(function) = NativeFunction::new(
        NativeFunctionDesc {
            fn_name,
            class_name,
            is_static: true,
            return_type,
            param_types: &param_types,
        },
        StaticLongFunctionHandler { callback },
    ) else {
        return false;
    };

    vm.bind_native_function(
        fn_name,
        class_name,
        function,
        options.callable_from_tasklets,
    )
}

fn bind_method_long_function<F>(
    vm: &IVirtualMachine,
    fn_name: &str,
    class_name: &str,
    callback: F,
    options: FunctionOptions,
) -> bool
where
    F: PapyrusMethodLongFunctionSignature,
{
    let Some(param_types) = F::parameter_type_infos(vm) else {
        return false;
    };
    let Some(return_type) = F::return_type_info(vm) else {
        return false;
    };
    let Some(function) = NativeFunction::new(
        NativeFunctionDesc {
            fn_name,
            class_name,
            is_static: false,
            return_type,
            param_types: &param_types,
        },
        MethodLongFunctionHandler { callback },
    ) else {
        return false;
    };

    vm.bind_native_function(
        fn_name,
        class_name,
        function,
        options.callable_from_tasklets,
    )
}

fn bind_static_latent_function<R, F>(
    vm: &IVirtualMachine,
    fn_name: &str,
    class_name: &str,
    callback: F,
    options: FunctionOptions,
) -> bool
where
    R: PapyrusReturn + PapyrusReturnConvertible + 'static,
    F: PapyrusStaticLatentFunctionSignature,
{
    let Some(param_types) = F::parameter_type_infos(vm) else {
        return false;
    };
    let Some(latent_return_type) = R::return_type_info(vm) else {
        return false;
    };
    let Some(function) = NativeLatentFunction::new(
        NativeLatentFunctionDesc {
            fn_name,
            class_name,
            is_static: true,
            latent_return_type,
            param_types: &param_types,
        },
        StaticLatentFunctionHandler { callback },
    ) else {
        return false;
    };

    vm.bind_native_latent_function(
        fn_name,
        class_name,
        function,
        options.callable_from_tasklets,
    )
}

fn bind_method_latent_function<R, F>(
    vm: &IVirtualMachine,
    fn_name: &str,
    class_name: &str,
    callback: F,
    options: FunctionOptions,
) -> bool
where
    R: PapyrusReturn + PapyrusReturnConvertible + 'static,
    F: PapyrusMethodLatentFunctionSignature,
{
    let Some(param_types) = F::parameter_type_infos(vm) else {
        return false;
    };
    let Some(latent_return_type) = R::return_type_info(vm) else {
        return false;
    };
    let Some(function) = NativeLatentFunction::new(
        NativeLatentFunctionDesc {
            fn_name,
            class_name,
            is_static: false,
            latent_return_type,
            param_types: &param_types,
        },
        MethodLatentFunctionHandler { callback },
    ) else {
        return false;
    };

    vm.bind_native_latent_function(
        fn_name,
        class_name,
        function,
        options.callable_from_tasklets,
    )
}

#[macro_export]
macro_rules! papyrus_class {
    ($(#[$meta:meta])* $vis:vis $name:ident = $class_name:literal $(;)?) => {
        $(#[$meta])*
        $vis struct $name;

        impl $crate::skse::papyrus::PapyrusClass for $name {
            const NAME: &'static str = $class_name;
        }
    };
}

#[macro_export]
macro_rules! papyrus_static_function {
    ($registry:expr, $fn_name:expr, $callback:path => fn() $(-> $ret:ty)? $(, callable_from_tasklets = $callable:expr)? $(,)?) => {{
        let registry = &mut $registry;
        registry.register_static_function(
            $fn_name,
            $callback as fn() $(-> $ret)?,
            $crate::skse::papyrus::FunctionOptions::new()
                .callable_from_tasklets($crate::papyrus_static_function!(@callable $($callable)?)),
        )
    }};
    ($registry:expr, $fn_name:expr, $callback:path => fn($($arg_name:ident : $arg_ty:ty),+ $(,)?) $(-> $ret:ty)? $(, callable_from_tasklets = $callable:expr)? $(,)?) => {{
        let registry = &mut $registry;
        registry.register_static_function(
            $fn_name,
            $callback as fn($($arg_ty),+) $(-> $ret)?,
            $crate::skse::papyrus::FunctionOptions::new()
                .callable_from_tasklets($crate::papyrus_static_function!(@callable $($callable)?)),
        )
    }};
    (@callable $callable:expr) => { $callable };
    (@callable) => { false };
}

#[macro_export]
macro_rules! papyrus_method_function {
    ($registry:expr, $fn_name:expr, $callback:path => fn($base_name:ident : $base_ty:ty $(, $arg_name:ident : $arg_ty:ty)*) $(-> $ret:ty)? $(, callable_from_tasklets = $callable:expr)? $(,)?) => {{
        let registry = &mut $registry;
        registry.register_method_function(
            $fn_name,
            $callback as fn($base_ty $(, $arg_ty)*) $(-> $ret)?,
            $crate::skse::papyrus::FunctionOptions::new()
                .callable_from_tasklets($crate::papyrus_method_function!(@callable $($callable)?)),
        )
    }};
    (@callable $callable:expr) => { $callable };
    (@callable) => { false };
}

#[macro_export]
macro_rules! papyrus_static_long_function {
    ($registry:expr, $fn_name:expr, $callback:path => fn($ctx_name:ident : $ctx_ty:ty $(, $arg_name:ident : $arg_ty:ty)*) $(-> $ret:ty)? $(, callable_from_tasklets = $callable:expr)? $(,)?) => {{
        let registry = &mut $registry;
        registry.register_static_long_function(
            $fn_name,
            $callback as fn($ctx_ty $(, $arg_ty)*) $(-> $ret)?,
            $crate::skse::papyrus::FunctionOptions::new()
                .callable_from_tasklets($crate::papyrus_static_long_function!(@callable $($callable)?)),
        )
    }};
    (@callable $callable:expr) => { $callable };
    (@callable) => { false };
}

#[macro_export]
macro_rules! papyrus_method_long_function {
    ($registry:expr, $fn_name:expr, $callback:path => fn($ctx_name:ident : $ctx_ty:ty, $base_name:ident : $base_ty:ty $(, $arg_name:ident : $arg_ty:ty)*) $(-> $ret:ty)? $(, callable_from_tasklets = $callable:expr)? $(,)?) => {{
        let registry = &mut $registry;
        registry.register_method_long_function(
            $fn_name,
            $callback as fn($ctx_ty, $base_ty $(, $arg_ty)*) $(-> $ret)?,
            $crate::skse::papyrus::FunctionOptions::new()
                .callable_from_tasklets($crate::papyrus_method_long_function!(@callable $($callable)?)),
        )
    }};
    (@callable $callable:expr) => { $callable };
    (@callable) => { false };
}

#[macro_export]
macro_rules! papyrus_static_latent_function {
    ($registry:expr, $fn_name:expr, $callback:path => fn($ctx_name:ident : $ctx_ty:ty $(, $arg_name:ident : $arg_ty:ty)*) returns $latent:ty $(, callable_from_tasklets = $callable:expr)? $(,)?) => {{
        let registry = &mut $registry;
        registry.register_static_latent_function::<$latent, _>(
            $fn_name,
            $callback as fn($ctx_ty $(, $arg_ty)*) -> $crate::re::LatentStatus,
            $crate::skse::papyrus::FunctionOptions::new()
                .callable_from_tasklets($crate::papyrus_static_latent_function!(@callable $($callable)?)),
        )
    }};
    (@callable $callable:expr) => { $callable };
    (@callable) => { false };
}

#[macro_export]
macro_rules! papyrus_method_latent_function {
    ($registry:expr, $fn_name:expr, $callback:path => fn($ctx_name:ident : $ctx_ty:ty, $base_name:ident : $base_ty:ty $(, $arg_name:ident : $arg_ty:ty)*) returns $latent:ty $(, callable_from_tasklets = $callable:expr)? $(,)?) => {{
        let registry = &mut $registry;
        registry.register_method_latent_function::<$latent, _>(
            $fn_name,
            $callback as fn($ctx_ty, $base_ty $(, $arg_ty)*) -> $crate::re::LatentStatus,
            $crate::skse::papyrus::FunctionOptions::new()
                .callable_from_tasklets($crate::papyrus_method_latent_function!(@callable $($callable)?)),
        )
    }};
    (@callable $callable:expr) => { $callable };
    (@callable) => { false };
}

#[macro_export]
macro_rules! papyrus_module {
    ($(#[$meta:meta])* $vis:vis $name:ident { $($body:tt)* }) => {
        $(#[$meta])*
        $vis struct $name;

        impl $crate::skse::papyrus::PapyrusModule for $name {
            fn register(module: &mut $crate::skse::papyrus::ModuleRegistry<'_>) -> bool {
                $crate::papyrus_module!(@module module; $($body)*);
                module.is_ok()
            }
        }
    };
    (@module $module:ident;) => {};
    (@module $module:ident; class $class:ty { $($entries:tt)* } $($rest:tt)*) => {
        {
            let mut class = $module.class::<$class>();
            $crate::papyrus_module!(@class class; $($entries)*);
        }
        $crate::papyrus_module!(@module $module; $($rest)*);
    };
    (@module $module:ident; class_named $class_name:literal { $($entries:tt)* } $($rest:tt)*) => {
        {
            let mut class = $module.class_named($class_name);
            $crate::papyrus_module!(@class class; $($entries)*);
        }
        $crate::papyrus_module!(@module $module; $($rest)*);
    };
    (@class $class:ident;) => {};
    (@class $class:ident; static fn $fn_name:literal => $callback:path => fn($($sig:tt)*) $(-> $ret:ty)? $(, callable_from_tasklets = $callable:expr)? ; $($rest:tt)*) => {
        let _ = $crate::papyrus_static_function!($class, $fn_name, $callback => fn($($sig)*) $(-> $ret)? $(, callable_from_tasklets = $callable)?);
        $crate::papyrus_module!(@class $class; $($rest)*);
    };
    (@class $class:ident; method fn $fn_name:literal => $callback:path => fn($($sig:tt)*) $(-> $ret:ty)? $(, callable_from_tasklets = $callable:expr)? ; $($rest:tt)*) => {
        let _ = $crate::papyrus_method_function!($class, $fn_name, $callback => fn($($sig)*) $(-> $ret)? $(, callable_from_tasklets = $callable)?);
        $crate::papyrus_module!(@class $class; $($rest)*);
    };
    (@class $class:ident; long static fn $fn_name:literal => $callback:path => fn($($sig:tt)*) $(-> $ret:ty)? $(, callable_from_tasklets = $callable:expr)? ; $($rest:tt)*) => {
        let _ = $crate::papyrus_static_long_function!($class, $fn_name, $callback => fn($($sig)*) $(-> $ret)? $(, callable_from_tasklets = $callable)?);
        $crate::papyrus_module!(@class $class; $($rest)*);
    };
    (@class $class:ident; long method fn $fn_name:literal => $callback:path => fn($($sig:tt)*) $(-> $ret:ty)? $(, callable_from_tasklets = $callable:expr)? ; $($rest:tt)*) => {
        let _ = $crate::papyrus_method_long_function!($class, $fn_name, $callback => fn($($sig)*) $(-> $ret)? $(, callable_from_tasklets = $callable)?);
        $crate::papyrus_module!(@class $class; $($rest)*);
    };
    (@class $class:ident; latent static fn $fn_name:literal => $callback:path => fn($($sig:tt)*) returns $latent:ty $(, callable_from_tasklets = $callable:expr)? ; $($rest:tt)*) => {
        let _ = $crate::papyrus_static_latent_function!($class, $fn_name, $callback => fn($($sig)*) returns $latent $(, callable_from_tasklets = $callable)?);
        $crate::papyrus_module!(@class $class; $($rest)*);
    };
    (@class $class:ident; latent method fn $fn_name:literal => $callback:path => fn($($sig:tt)*) returns $latent:ty $(, callable_from_tasklets = $callable:expr)? ; $($rest:tt)*) => {
        let _ = $crate::papyrus_method_latent_function!($class, $fn_name, $callback => fn($($sig)*) returns $latent $(, callable_from_tasklets = $callable)?);
        $crate::papyrus_module!(@class $class; $($rest)*);
    };
}
