#![allow(non_camel_case_types)]

use core_util::Enum;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SkyrimScript__DelayFunctor;
use crate::offsets::offsets_vtable::VTABLE_SkyrimScript__DelayFunctor;
use crate::re::{
    BSIntrusiveRefCounted, BSStorage, BSTSmartPointerIntrusiveRefCountable, VMStackID, Variable,
};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::SkyrimScript::DelayFunctor::FunctorType`
#[libskyrim_macros::open_enum]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DelayFunctorType {
    kMoveTo = 0,
    kSetPosition = 6,
    kSetMotionType = 8,
    kDropObject = 12,
    kAttachAshPile = 14,
    kSendPlayerToJail = 19,
    kRemoveItem = 24,
}

const _: () = assert!(core::mem::size_of::<DelayFunctorType>() == 0x4);

/// C++ `RE::SkyrimScript::DelayFunctor`
#[repr(C)]
pub struct DelayFunctor {
    pub vtable: *const usize,        // 00
    pub base: BSIntrusiveRefCounted, // 08
    pub stack_id: VMStackID,         // 0C
}

const _: () = assert!(core::mem::size_of::<DelayFunctor>() == 0x10);
const _: () = assert!(core::mem::offset_of!(DelayFunctor, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(DelayFunctor, base) == 0x08);
const _: () = assert!(core::mem::offset_of!(DelayFunctor, stack_id) == 0x0C);

inherit!(DelayFunctor : BSIntrusiveRefCounted, base);

impl RttiType for DelayFunctor {
    const RTTI: VariantID = RTTI_SkyrimScript__DelayFunctor;
}

impl BSTSmartPointerIntrusiveRefCountable for DelayFunctor {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        self.dtor();
    }
}

impl DelayFunctor {
    pub const RTTI: VariantID = RTTI_SkyrimScript__DelayFunctor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SkyrimScript__DelayFunctor;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_CALL: usize = 0x01;
        pub fn call(&mut self) -> Variable
    }

    virtual_method! {
        pub const VFUNC_IS_LATENT: usize = 0x02;
        pub fn is_latent(&self) -> bool
    }

    virtual_method! {
        pub const VFUNC_WANTS_REQUEUE: usize = 0x03;
        pub fn wants_requeue(&self) -> bool
    }

    virtual_method! {
        pub const VFUNC_SAVE_IMPL: usize = 0x04;
        pub fn save_impl(&self, storage: &mut BSStorage) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x05;
        pub fn get_type_raw(&self) -> i32
    }

    virtual_method! {
        pub const VFUNC_LOAD_IMPL: usize = 0x06;
        pub fn load_impl(&mut self, storage: &BSStorage, arg2: u32, arg3: &mut bool) -> bool
    }

    #[inline(always)]
    pub fn type_storage(&self) -> Enum<DelayFunctorType, i32> {
        Enum::from_underlying(self.get_type_raw())
    }

    #[inline(always)]
    pub fn try_get_type(&self) -> Option<DelayFunctorType> {
        self.type_storage().get()
    }

    #[inline(always)]
    pub fn get_type(&self) -> DelayFunctorType {
        self.try_get_type().unwrap_or(DelayFunctorType::kMoveTo)
    }
}

pub trait DelayFunctorExt {
    fn dtor(&mut self);
    fn call(&mut self) -> Variable;
    fn is_latent(&self) -> bool;
    fn wants_requeue(&self) -> bool;
    fn save_impl(&self, storage: &mut BSStorage) -> bool;
    fn type_storage(&self) -> Enum<DelayFunctorType, i32>;
    fn try_get_type(&self) -> Option<DelayFunctorType>;
    fn get_type(&self) -> DelayFunctorType;
    fn load_impl(&mut self, storage: &BSStorage, arg2: u32, arg3: &mut bool) -> bool;
}

impl<T> DelayFunctorExt for T
where
    T: AsRef<DelayFunctor> + AsMut<DelayFunctor>,
{
    #[inline(always)]
    fn dtor(&mut self) {
        DelayFunctor::dtor(self.as_mut())
    }

    #[inline(always)]
    fn call(&mut self) -> Variable {
        DelayFunctor::call(self.as_mut())
    }

    #[inline(always)]
    fn is_latent(&self) -> bool {
        DelayFunctor::is_latent(self.as_ref())
    }

    #[inline(always)]
    fn wants_requeue(&self) -> bool {
        DelayFunctor::wants_requeue(self.as_ref())
    }

    #[inline(always)]
    fn save_impl(&self, storage: &mut BSStorage) -> bool {
        DelayFunctor::save_impl(self.as_ref(), storage)
    }

    #[inline(always)]
    fn type_storage(&self) -> Enum<DelayFunctorType, i32> {
        DelayFunctor::type_storage(self.as_ref())
    }

    #[inline(always)]
    fn try_get_type(&self) -> Option<DelayFunctorType> {
        DelayFunctor::try_get_type(self.as_ref())
    }

    #[inline(always)]
    fn get_type(&self) -> DelayFunctorType {
        DelayFunctor::get_type(self.as_ref())
    }

    #[inline(always)]
    fn load_impl(&mut self, storage: &BSStorage, arg2: u32, arg3: &mut bool) -> bool {
        DelayFunctor::load_impl(self.as_mut(), storage, arg2, arg3)
    }
}
