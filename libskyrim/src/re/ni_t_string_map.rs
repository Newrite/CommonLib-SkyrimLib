use core::ffi::{c_char, c_void};

use crate::offsets::offsets_rtti::{
    RTTI_NiTStringMap_Setting___,
    RTTI_NiTStringTemplateMap_NiTMap_char_const___Setting____Setting___,
};
use crate::offsets::offsets_vtable::{
    VTABLE_NiTStringMap_Setting___,
    VTABLE_NiTStringTemplateMap_NiTMap_char_const___Setting____Setting___,
};
use crate::re::Setting;
use crate::re::ni_t_map::NiTMap;
use crate::re::ni_t_map_base::{NiTMapBase, NiTMapItem};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::NiTStringTemplateMap<Parent, T>`
#[repr(C)]
pub struct NiTStringTemplateMap<Parent, T> {
    pub base: Parent, // 00
    pub copy: bool,   // 20
    pub pad21: u8,    // 21
    pub pad22: u16,   // 22
    pub pad24: u32,   // 24
    _marker: core::marker::PhantomData<fn() -> T>,
}

const _: () = assert!(
    core::mem::size_of::<NiTStringTemplateMap<NiTMap<*const c_char, *mut c_void>, *mut c_void>>()
        == 0x28
);
const _: () = assert!(
    core::mem::offset_of!(
        NiTStringTemplateMap<NiTMap<*const c_char, *mut c_void>, *mut c_void>,
        base
    ) == 0x00
);
const _: () = assert!(
    core::mem::offset_of!(
        NiTStringTemplateMap<NiTMap<*const c_char, *mut c_void>, *mut c_void>,
        copy
    ) == 0x20
);
const _: () = assert!(
    core::mem::offset_of!(
        NiTStringTemplateMap<NiTMap<*const c_char, *mut c_void>, *mut c_void>,
        pad21
    ) == 0x21
);
const _: () = assert!(
    core::mem::offset_of!(
        NiTStringTemplateMap<NiTMap<*const c_char, *mut c_void>, *mut c_void>,
        pad22
    ) == 0x22
);
const _: () = assert!(
    core::mem::offset_of!(
        NiTStringTemplateMap<NiTMap<*const c_char, *mut c_void>, *mut c_void>,
        pad24
    ) == 0x24
);

pub type NiTStringTemplateMapSetting =
    NiTStringTemplateMap<NiTMap<*const c_char, *mut Setting>, *mut Setting>;

impl RttiType for NiTStringTemplateMapSetting {
    const RTTI: VariantID = RTTI_NiTStringTemplateMap_NiTMap_char_const___Setting____Setting___;
}

impl NiTStringTemplateMapSetting {
    pub const RTTI: VariantID = RTTI_NiTStringTemplateMap_NiTMap_char_const___Setting____Setting___;
    pub const VTABLE: &'static [VariantID] =
        &VTABLE_NiTStringTemplateMap_NiTMap_char_const___Setting____Setting___;
}

impl<Parent, T> NiTStringTemplateMap<Parent, T> {
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_HASH_FUNCTION: usize = 0x01;
        pub fn hash_function(key: *const c_char) -> u32
    }

    virtual_method! {
        pub const VFUNC_KEY_EQ: usize = 0x02;
        pub fn key_eq(lhs: *const c_char, rhs: *const c_char) -> bool
    }

    virtual_method! {
        pub const VFUNC_ASSIGN_VALUE: usize = 0x03;
        pub fn assign_value(value: *mut NiTMapItem<*const c_char, T>, key: *const c_char, mapped: T)
    }

    virtual_method! {
        pub const VFUNC_CLEAR_VALUE: usize = 0x04;
        pub fn clear_value(value: *mut NiTMapItem<*const c_char, T>)
    }
}

/// C++ `RE::NiTStringMap<T>`
#[repr(C)]
pub struct NiTStringMap<T> {
    pub base: NiTStringTemplateMap<NiTMap<*const c_char, T>, T>, // 00
}

const _: () = assert!(core::mem::size_of::<NiTStringMap<*mut c_void>>() == 0x28);
const _: () = assert!(core::mem::offset_of!(NiTStringMap<*mut c_void>, base) == 0x00);

pub type NiTStringMapSetting = NiTStringMap<*mut Setting>;

impl RttiType for NiTStringMapSetting {
    const RTTI: VariantID = RTTI_NiTStringMap_Setting___;
}

impl NiTStringMapSetting {
    pub const RTTI: VariantID = RTTI_NiTStringMap_Setting___;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiTStringMap_Setting___;
}

impl<T> AsRef<NiTStringMap<T>> for NiTStringMap<T> {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsMut<NiTStringMap<T>> for NiTStringMap<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<Parent, T> AsRef<Parent> for NiTStringTemplateMap<Parent, T> {
    #[inline(always)]
    fn as_ref(&self) -> &Parent {
        &self.base
    }
}

impl<Parent, T> AsMut<Parent> for NiTStringTemplateMap<Parent, T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Parent {
        &mut self.base
    }
}

impl<T> AsRef<NiTMap<*const c_char, T>> for NiTStringMap<T> {
    #[inline(always)]
    fn as_ref(&self) -> &NiTMap<*const c_char, T> {
        &self.base.base
    }
}

impl<T> AsMut<NiTMap<*const c_char, T>> for NiTStringMap<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut NiTMap<*const c_char, T> {
        &mut self.base.base
    }
}

impl<T>
    AsRef<
        NiTMapBase<
            crate::re::ni_t_default_allocator::NiTDefaultAllocator<NiTMapItem<*const c_char, T>>,
            *const c_char,
            T,
        >,
    > for NiTStringMap<T>
{
    #[inline(always)]
    fn as_ref(
        &self,
    ) -> &NiTMapBase<
        crate::re::ni_t_default_allocator::NiTDefaultAllocator<NiTMapItem<*const c_char, T>>,
        *const c_char,
        T,
    > {
        &self.base.base.base
    }
}

impl<T>
    AsMut<
        NiTMapBase<
            crate::re::ni_t_default_allocator::NiTDefaultAllocator<NiTMapItem<*const c_char, T>>,
            *const c_char,
            T,
        >,
    > for NiTStringMap<T>
{
    #[inline(always)]
    fn as_mut(
        &mut self,
    ) -> &mut NiTMapBase<
        crate::re::ni_t_default_allocator::NiTDefaultAllocator<NiTMapItem<*const c_char, T>>,
        *const c_char,
        T,
    > {
        &mut self.base.base.base
    }
}

impl<T> AsRef<NiTStringTemplateMap<NiTMap<*const c_char, T>, T>> for NiTStringMap<T> {
    #[inline(always)]
    fn as_ref(&self) -> &NiTStringTemplateMap<NiTMap<*const c_char, T>, T> {
        &self.base
    }
}

impl<T> AsMut<NiTStringTemplateMap<NiTMap<*const c_char, T>, T>> for NiTStringMap<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut NiTStringTemplateMap<NiTMap<*const c_char, T>, T> {
        &mut self.base
    }
}

impl<T> NiTStringMap<T> {
    // override (NiTStringTemplateMap<NiTMap<const char*, T>, T>)
    // virtual ~NiTStringMap();  // 00

    #[inline(always)]
    pub fn iter(&self) -> crate::re::ni_t_map_base::NiTMapBaseIter<'_, *const c_char, T> {
        self.base.base.iter()
    }
}

pub trait NiTStringTemplateMapExt<Parent, T> {
    fn dtor(&self);
    fn hash_function(&self, key: *const c_char) -> u32;
    fn key_eq(&self, lhs: *const c_char, rhs: *const c_char) -> bool;
    fn assign_value(&self, value: *mut NiTMapItem<*const c_char, T>, key: *const c_char, mapped: T);
    fn clear_value(&self, value: *mut NiTMapItem<*const c_char, T>);
}

impl<Parent, T, U> NiTStringTemplateMapExt<Parent, T> for U
where
    U: AsRef<NiTStringTemplateMap<Parent, T>>,
{
    fn dtor(&self) {
        self.as_ref().dtor()
    }

    fn hash_function(&self, key: *const c_char) -> u32 {
        self.as_ref().hash_function(key)
    }

    fn key_eq(&self, lhs: *const c_char, rhs: *const c_char) -> bool {
        self.as_ref().key_eq(lhs, rhs)
    }

    fn assign_value(
        &self,
        value: *mut NiTMapItem<*const c_char, T>,
        key: *const c_char,
        mapped: T,
    ) {
        self.as_ref().assign_value(value, key, mapped)
    }

    fn clear_value(&self, value: *mut NiTMapItem<*const c_char, T>) {
        self.as_ref().clear_value(value)
    }
}

pub trait NiTStringMapExt<T> {
    fn iter(&self) -> crate::re::ni_t_map_base::NiTMapBaseIter<'_, *const c_char, T>;
}

impl<T, U> NiTStringMapExt<T> for U
where
    U: AsRef<NiTStringMap<T>>,
{
    fn iter(&self) -> crate::re::ni_t_map_base::NiTMapBaseIter<'_, *const c_char, T> {
        self.as_ref().iter()
    }
}
