//! Rust-side Papyrus type traits derived from `RE/T/TypeTraits.h` and
//! `RE/P/PackUnpack.h`.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::re::pack_unpack::{get_type_info_from_vm_type_vm, pack_handle, unpack_handle};
use crate::re::static_function_tag::StaticFunctionTag;
use crate::re::{
    ActiveEffect, Array, BGSBaseAlias, BGSLocAlias, BGSRefAlias, BSFixedString, BSTSmartPointer,
    FormCastable, IVirtualMachine, RawType, ReferenceArray, TypeInfo, VMTypeID, Variable,
    VirtualMachine,
};

fn array_type_info_from_element(element_type: TypeInfo) -> TypeInfo {
    let raw = element_type.get_raw_type().underlying();
    match element_type.get_unmangled_raw_type() {
        RawType::None
        | RawType::Object
        | RawType::String
        | RawType::Int
        | RawType::Float
        | RawType::Bool => TypeInfo::from_underlying(raw + RawType::NoneArray as usize),
        _ => TypeInfo::from_underlying(raw + RawType::Object as usize),
    }
}

/// Rust-side equivalent of the source-backed `reference_array<T>` classifier in
/// `TypeTraits.h`.
pub trait PapyrusReferenceWrapper {}

/// Rust-side equivalent of the `value_type`-driven container detection used by
/// `CommonTypeTraits.h` and `TypeTraits.h`.
pub trait PapyrusArrayLike {
    type ValueType;
}

/// Rust-side equivalent of `unwrapped_type_t<T>` from `TypeTraits.h`.
pub trait PapyrusUnwrappedType {
    type Type;
}

/// Rust-side equivalent of `vm_type<T>` / `vm_type_v<T>`.
pub trait PapyrusVmType {
    fn vm_type_info(vm: &IVirtualMachine) -> Option<TypeInfo>;
}

/// Rust-side equivalent of `is_valid_base_v<T>`.
pub trait PapyrusValidBase: PapyrusBase {}

/// Rust-side equivalent of `is_parameter_convertible_v<T>`.
pub trait PapyrusParameterConvertible: PapyrusParameter {}

/// Rust-side equivalent of `is_valid_parameter_v<T>`.
pub trait PapyrusValidParameter: PapyrusParameterConvertible {}

/// Rust-side equivalent of `is_return_convertible_v<T>`.
pub trait PapyrusReturnConvertible: PapyrusReturn {}

/// Rust-side equivalent of `is_valid_return_v<T>`.
pub trait PapyrusValidReturn: PapyrusReturnConvertible {}

pub type UnwrappedType<T> = <T as PapyrusUnwrappedType>::Type;

#[inline(always)]
pub fn get_vm_type<T>(vm: &IVirtualMachine) -> Option<TypeInfo>
where
    T: PapyrusVmType,
{
    T::vm_type_info(vm)
}

#[inline(always)]
pub fn get_unwrapped_vm_type<T>(vm: &IVirtualMachine) -> Option<TypeInfo>
where
    T: PapyrusUnwrappedType,
    UnwrappedType<T>: PapyrusVmType,
{
    <UnwrappedType<T> as PapyrusVmType>::vm_type_info(vm)
}

macro_rules! impl_unwrapped_self {
    ($($ty:ty),* $(,)?) => {
        $(
            impl PapyrusUnwrappedType for $ty {
                type Type = Self;
            }
        )*
    };
}

macro_rules! impl_vm_builtin {
    ($ty:ty, $raw:expr) => {
        impl PapyrusVmType for $ty {
            #[inline(always)]
            fn vm_type_info(_vm: &IVirtualMachine) -> Option<TypeInfo> {
                Some(TypeInfo::new($raw))
            }
        }
    };
}

impl<T> PapyrusArrayLike for Vec<T> {
    type ValueType = T;
}

impl<T> PapyrusArrayLike for ReferenceArray<T>
where
    T: crate::re::reference_array::PapyrusReferenceArrayElement,
{
    type ValueType = T;
}

impl<T> PapyrusReferenceWrapper for ReferenceArray<T> where
    T: crate::re::reference_array::PapyrusReferenceArrayElement
{
}

impl_unwrapped_self!(
    (),
    bool,
    i32,
    u32,
    f32,
    BSFixedString,
    String,
    *mut StaticFunctionTag,
    *mut BGSBaseAlias,
    *mut BGSRefAlias,
    *mut BGSLocAlias,
    *mut ActiveEffect,
);

impl_vm_builtin!((), RawType::None);
impl_vm_builtin!(bool, RawType::Bool);
impl_vm_builtin!(i32, RawType::Int);
impl_vm_builtin!(u32, RawType::Int);
impl_vm_builtin!(f32, RawType::Float);
impl_vm_builtin!(BSFixedString, RawType::String);
impl_vm_builtin!(String, RawType::String);

impl<T> PapyrusUnwrappedType for *mut T
where
    T: FormCastable,
{
    type Type = Self;
}

impl<T> PapyrusVmType for *mut T
where
    T: FormCastable,
{
    #[inline(always)]
    fn vm_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        get_type_info_from_vm_type_vm(vm, T::TARGET_FORM_TYPE as VMTypeID)
    }
}

impl PapyrusVmType for *mut BGSBaseAlias {
    #[inline(always)]
    fn vm_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        get_type_info_from_vm_type_vm(vm, BGSBaseAlias::VM_TYPE_ID)
    }
}

impl PapyrusVmType for *mut BGSRefAlias {
    #[inline(always)]
    fn vm_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        get_type_info_from_vm_type_vm(vm, BGSRefAlias::VM_TYPE_ID)
    }
}

impl PapyrusVmType for *mut BGSLocAlias {
    #[inline(always)]
    fn vm_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        get_type_info_from_vm_type_vm(vm, BGSLocAlias::VM_TYPE_ID)
    }
}

impl PapyrusVmType for *mut ActiveEffect {
    #[inline(always)]
    fn vm_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        get_type_info_from_vm_type_vm(vm, ActiveEffect::VM_TYPE_ID)
    }
}

impl<T> PapyrusUnwrappedType for Vec<T>
where
    T: PapyrusScalar,
{
    type Type = T;
}

impl<T> PapyrusVmType for Vec<T>
where
    T: PapyrusScalar + PapyrusVmType,
{
    #[inline(always)]
    fn vm_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        T::vm_type_info(vm)
    }
}

impl<T> PapyrusUnwrappedType for ReferenceArray<T>
where
    T: crate::re::reference_array::PapyrusReferenceArrayElement,
{
    type Type = T;
}

impl<T> PapyrusVmType for ReferenceArray<T>
where
    T: crate::re::reference_array::PapyrusReferenceArrayElement + PapyrusVmType,
{
    #[inline(always)]
    fn vm_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        T::vm_type_info(vm)
    }
}

pub trait PapyrusBase: Sized {
    const IS_STATIC: bool;

    unsafe fn unpack_base(src: &Variable) -> Self;
}

impl PapyrusBase for *mut StaticFunctionTag {
    const IS_STATIC: bool = true;

    #[inline(always)]
    unsafe fn unpack_base(_src: &Variable) -> Self {
        core::ptr::null_mut()
    }
}

impl PapyrusValidBase for *mut StaticFunctionTag {}

impl<T> PapyrusBase for *mut T
where
    T: FormCastable,
{
    const IS_STATIC: bool = false;

    #[inline(always)]
    unsafe fn unpack_base(src: &Variable) -> Self {
        unpack_handle(src, T::TARGET_FORM_TYPE as VMTypeID).cast::<T>()
    }
}

impl<T> PapyrusValidBase for *mut T where T: FormCastable {}

impl PapyrusBase for *mut BGSBaseAlias {
    const IS_STATIC: bool = false;

    #[inline(always)]
    unsafe fn unpack_base(src: &Variable) -> Self {
        unpack_handle(src, BGSBaseAlias::VM_TYPE_ID).cast::<BGSBaseAlias>()
    }
}

impl PapyrusValidBase for *mut BGSBaseAlias {}

impl PapyrusBase for *mut BGSRefAlias {
    const IS_STATIC: bool = false;

    #[inline(always)]
    unsafe fn unpack_base(src: &Variable) -> Self {
        unpack_handle(src, BGSRefAlias::VM_TYPE_ID).cast::<BGSRefAlias>()
    }
}

impl PapyrusValidBase for *mut BGSRefAlias {}

impl PapyrusBase for *mut BGSLocAlias {
    const IS_STATIC: bool = false;

    #[inline(always)]
    unsafe fn unpack_base(src: &Variable) -> Self {
        unpack_handle(src, BGSLocAlias::VM_TYPE_ID).cast::<BGSLocAlias>()
    }
}

impl PapyrusValidBase for *mut BGSLocAlias {}

impl PapyrusBase for *mut ActiveEffect {
    const IS_STATIC: bool = false;

    #[inline(always)]
    unsafe fn unpack_base(src: &Variable) -> Self {
        unpack_handle(src, ActiveEffect::VM_TYPE_ID).cast::<ActiveEffect>()
    }
}

impl PapyrusValidBase for *mut ActiveEffect {}

pub trait PapyrusScalar: Sized {
    fn scalar_type_info(vm: &IVirtualMachine) -> Option<TypeInfo>;

    unsafe fn unpack_scalar(src: &Variable) -> Self;

    fn pack_scalar(self, dst: &mut Variable, vm: &mut VirtualMachine) -> bool;
}

impl PapyrusScalar for bool {
    #[inline(always)]
    fn scalar_type_info(_vm: &IVirtualMachine) -> Option<TypeInfo> {
        Some(TypeInfo::new(RawType::Bool))
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        src.get_bool()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        dst.set_bool(self);
        true
    }
}

impl PapyrusScalar for i32 {
    #[inline(always)]
    fn scalar_type_info(_vm: &IVirtualMachine) -> Option<TypeInfo> {
        Some(TypeInfo::new(RawType::Int))
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        src.get_sint()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        dst.set_sint(self);
        true
    }
}

impl PapyrusScalar for u32 {
    #[inline(always)]
    fn scalar_type_info(_vm: &IVirtualMachine) -> Option<TypeInfo> {
        Some(TypeInfo::new(RawType::Int))
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        src.get_uint()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        dst.set_uint(self);
        true
    }
}

impl PapyrusScalar for f32 {
    #[inline(always)]
    fn scalar_type_info(_vm: &IVirtualMachine) -> Option<TypeInfo> {
        Some(TypeInfo::new(RawType::Float))
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        src.get_float()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        dst.set_float(self);
        true
    }
}

impl PapyrusScalar for BSFixedString {
    #[inline(always)]
    fn scalar_type_info(_vm: &IVirtualMachine) -> Option<TypeInfo> {
        Some(TypeInfo::new(RawType::String))
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        src.get_string()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        let text = self
            .as_c_str()
            .map(|value| value.to_string_lossy())
            .unwrap_or_default();
        dst.set_string(&text);
        true
    }
}

impl PapyrusScalar for String {
    #[inline(always)]
    fn scalar_type_info(_vm: &IVirtualMachine) -> Option<TypeInfo> {
        Some(TypeInfo::new(RawType::String))
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        src.get_string().to_string()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        dst.set_string(&self);
        true
    }
}

impl<T> PapyrusScalar for *mut T
where
    T: FormCastable,
{
    #[inline(always)]
    fn scalar_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        get_type_info_from_vm_type_vm(vm, T::TARGET_FORM_TYPE as VMTypeID)
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        unpack_handle(src, T::TARGET_FORM_TYPE as VMTypeID).cast::<T>()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        pack_handle(dst, self.cast(), T::TARGET_FORM_TYPE as VMTypeID);
        true
    }
}

impl PapyrusScalar for *mut BGSBaseAlias {
    #[inline(always)]
    fn scalar_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        get_type_info_from_vm_type_vm(vm, BGSBaseAlias::VM_TYPE_ID)
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        unpack_handle(src, BGSBaseAlias::VM_TYPE_ID).cast::<BGSBaseAlias>()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        pack_handle(dst, self.cast(), BGSBaseAlias::VM_TYPE_ID);
        true
    }
}

impl PapyrusScalar for *mut BGSRefAlias {
    #[inline(always)]
    fn scalar_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        get_type_info_from_vm_type_vm(vm, BGSRefAlias::VM_TYPE_ID)
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        unpack_handle(src, BGSRefAlias::VM_TYPE_ID).cast::<BGSRefAlias>()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        pack_handle(dst, self.cast(), BGSRefAlias::VM_TYPE_ID);
        true
    }
}

impl PapyrusScalar for *mut BGSLocAlias {
    #[inline(always)]
    fn scalar_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        get_type_info_from_vm_type_vm(vm, BGSLocAlias::VM_TYPE_ID)
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        unpack_handle(src, BGSLocAlias::VM_TYPE_ID).cast::<BGSLocAlias>()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        pack_handle(dst, self.cast(), BGSLocAlias::VM_TYPE_ID);
        true
    }
}

impl PapyrusScalar for *mut ActiveEffect {
    #[inline(always)]
    fn scalar_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        get_type_info_from_vm_type_vm(vm, ActiveEffect::VM_TYPE_ID)
    }

    #[inline(always)]
    unsafe fn unpack_scalar(src: &Variable) -> Self {
        unpack_handle(src, ActiveEffect::VM_TYPE_ID).cast::<ActiveEffect>()
    }

    #[inline(always)]
    fn pack_scalar(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        pack_handle(dst, self.cast(), ActiveEffect::VM_TYPE_ID);
        true
    }
}

pub trait PapyrusParameter: Sized {
    fn parameter_type_info(vm: &IVirtualMachine) -> Option<TypeInfo>;

    unsafe fn unpack_parameter(src: &Variable) -> Self;
}

impl<T> PapyrusParameter for T
where
    T: PapyrusScalar,
{
    #[inline(always)]
    fn parameter_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        T::scalar_type_info(vm)
    }

    #[inline(always)]
    unsafe fn unpack_parameter(src: &Variable) -> Self {
        unsafe { T::unpack_scalar(src) }
    }
}

impl<T> PapyrusParameterConvertible for T where T: PapyrusScalar {}
impl<T> PapyrusValidParameter for T where T: PapyrusScalar {}

impl<T> PapyrusParameter for Vec<T>
where
    T: PapyrusScalar,
{
    #[inline(always)]
    fn parameter_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        Some(array_type_info_from_element(T::scalar_type_info(vm)?))
    }

    #[inline(always)]
    unsafe fn unpack_parameter(src: &Variable) -> Self {
        if src.is_none_array() || src.is_none_object() {
            return Vec::new();
        }

        let array = src.get_array();
        if array.is_null() {
            return Vec::new();
        }

        let array = unsafe { &*array.get() };
        let mut out = Vec::with_capacity(array.size() as usize);
        for i in 0..array.size() {
            out.push(unsafe { T::unpack_scalar(array.get(i)) });
        }
        out
    }
}

impl<T> PapyrusParameterConvertible for Vec<T> where T: PapyrusScalar {}
impl<T> PapyrusValidParameter for Vec<T> where T: PapyrusScalar {}

impl<T> PapyrusParameter for ReferenceArray<T>
where
    T: crate::re::reference_array::PapyrusReferenceArrayElement,
{
    #[inline(always)]
    fn parameter_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        Some(array_type_info_from_element(T::scalar_type_info(vm)?))
    }

    #[inline(always)]
    unsafe fn unpack_parameter(src: &Variable) -> Self {
        ReferenceArray::from_variable(src)
    }
}

impl<T> PapyrusParameterConvertible for ReferenceArray<T> where
    T: crate::re::reference_array::PapyrusReferenceArrayElement
{
}

impl<T> PapyrusValidParameter for ReferenceArray<T> where
    T: crate::re::reference_array::PapyrusReferenceArrayElement
{
}

pub trait PapyrusReturn: Sized {
    fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo>;

    fn pack_return(self, dst: &mut Variable, vm: &mut VirtualMachine) -> bool;
}

impl PapyrusReturn for () {
    #[inline(always)]
    fn return_type_info(_vm: &IVirtualMachine) -> Option<TypeInfo> {
        Some(TypeInfo::new(RawType::None))
    }

    #[inline(always)]
    fn pack_return(self, dst: &mut Variable, _vm: &mut VirtualMachine) -> bool {
        dst.set_none();
        true
    }
}

impl PapyrusReturnConvertible for () {}
impl PapyrusValidReturn for () {}

impl<T> PapyrusReturn for T
where
    T: PapyrusScalar,
{
    #[inline(always)]
    fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        T::scalar_type_info(vm)
    }

    #[inline(always)]
    fn pack_return(self, dst: &mut Variable, vm: &mut VirtualMachine) -> bool {
        T::pack_scalar(self, dst, vm)
    }
}

impl<T> PapyrusReturnConvertible for T where T: PapyrusScalar {}
impl<T> PapyrusValidReturn for T where T: PapyrusScalar {}

impl<T> PapyrusReturn for Vec<T>
where
    T: PapyrusScalar,
{
    #[inline(always)]
    fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
        Some(array_type_info_from_element(T::scalar_type_info(vm)?))
    }

    #[inline(always)]
    fn pack_return(self, dst: &mut Variable, vm: &mut VirtualMachine) -> bool {
        let Some(element_type) = T::scalar_type_info(&vm.base) else {
            return false;
        };

        let mut array = BSTSmartPointer::<Array>::default();
        if !vm.create_array(&element_type, self.len() as u32, &mut array) || array.is_null() {
            return false;
        }

        let array_ptr = array.get();
        for (i, value) in self.into_iter().enumerate() {
            if !value.pack_scalar(unsafe { (*array_ptr).get_mut(i as u32) }, vm) {
                return false;
            }
        }

        dst.set_array(array);
        true
    }
}

impl<T> PapyrusReturnConvertible for Vec<T> where T: PapyrusScalar {}
impl<T> PapyrusValidReturn for Vec<T> where T: PapyrusScalar {}

pub trait IsReturnConvertible: PapyrusReturnConvertible {}

impl<T> IsReturnConvertible for T where T: PapyrusReturnConvertible {}

impl crate::re::reference_array::PapyrusReferenceArrayElement for bool {
    #[inline(always)]
    unsafe fn unpack_reference_array_element(src: &Variable) -> Self {
        unsafe { <Self as PapyrusScalar>::unpack_scalar(src) }
    }

    #[inline(always)]
    fn pack_reference_array_element(self, dst: &mut Variable) -> bool {
        dst.set_bool(self);
        true
    }
}

impl crate::re::reference_array::PapyrusReferenceArrayElement for i32 {
    #[inline(always)]
    unsafe fn unpack_reference_array_element(src: &Variable) -> Self {
        unsafe { <Self as PapyrusScalar>::unpack_scalar(src) }
    }

    #[inline(always)]
    fn pack_reference_array_element(self, dst: &mut Variable) -> bool {
        dst.set_sint(self);
        true
    }
}

impl crate::re::reference_array::PapyrusReferenceArrayElement for u32 {
    #[inline(always)]
    unsafe fn unpack_reference_array_element(src: &Variable) -> Self {
        unsafe { <Self as PapyrusScalar>::unpack_scalar(src) }
    }

    #[inline(always)]
    fn pack_reference_array_element(self, dst: &mut Variable) -> bool {
        dst.set_uint(self);
        true
    }
}

impl crate::re::reference_array::PapyrusReferenceArrayElement for f32 {
    #[inline(always)]
    unsafe fn unpack_reference_array_element(src: &Variable) -> Self {
        unsafe { <Self as PapyrusScalar>::unpack_scalar(src) }
    }

    #[inline(always)]
    fn pack_reference_array_element(self, dst: &mut Variable) -> bool {
        dst.set_float(self);
        true
    }
}

impl crate::re::reference_array::PapyrusReferenceArrayElement for BSFixedString {
    #[inline(always)]
    unsafe fn unpack_reference_array_element(src: &Variable) -> Self {
        unsafe { <Self as PapyrusScalar>::unpack_scalar(src) }
    }

    #[inline(always)]
    fn pack_reference_array_element(self, dst: &mut Variable) -> bool {
        let text = self
            .as_c_str()
            .map(|value| value.to_string_lossy())
            .unwrap_or_default();
        dst.set_string(&text);
        true
    }
}

impl crate::re::reference_array::PapyrusReferenceArrayElement for String {
    #[inline(always)]
    unsafe fn unpack_reference_array_element(src: &Variable) -> Self {
        unsafe { <Self as PapyrusScalar>::unpack_scalar(src) }
    }

    #[inline(always)]
    fn pack_reference_array_element(self, dst: &mut Variable) -> bool {
        dst.set_string(&self);
        true
    }
}

impl<T> crate::re::reference_array::PapyrusReferenceArrayElement for *mut T
where
    T: FormCastable,
{
    #[inline(always)]
    unsafe fn unpack_reference_array_element(src: &Variable) -> Self {
        unsafe { <Self as PapyrusScalar>::unpack_scalar(src) }
    }

    #[inline(always)]
    fn pack_reference_array_element(self, dst: &mut Variable) -> bool {
        pack_handle(dst, self.cast(), T::TARGET_FORM_TYPE as VMTypeID);
        true
    }
}

// TODO: `ReferenceArray.h` only provides the same-name wrapper for builtin
// and TESForm pointer element types. `TypeTraits.h` / `PackUnpack.h` can still
// classify alias and active-effect wrappers generically, but libskyrim should
// not invent a broader Rust-side `reference_array<T>` surface until CommonLib
// exposes matching source-backed wrapper semantics for those element kinds.
//
// TODO: `TypeTraits.h` also models coroutine-style latent return validation via
// `LatentPromiseBase` / `is_valid_latent_return_v`. libskyrim currently exposes
// the same source-backed callback-driven latent registration path as
// `IVirtualMachine::RegisterLatentFunction`, so the coroutine promise branch
// should only be translated once there is a retained-promise ABI layer for
// Rust-side latent functions.
