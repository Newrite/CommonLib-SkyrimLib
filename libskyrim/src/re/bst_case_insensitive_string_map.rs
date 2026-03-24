use core::ffi::{c_char, c_void};

use crate::offsets::offsets_rtti::RTTI_BSTCaseInsensitiveStringMap_Setting___;
use crate::offsets::offsets_vtable::VTABLE_BSTCaseInsensitiveStringMap_Setting___;
use crate::re::Setting;
use crate::re::ni_t_map::NiTMap;
use crate::re::ni_t_map_base::{
    NiTMapBaseIter, NiTMapItem, cstr_eq_ignore_ascii_case, cstr_eq_str_ignore_ascii_case,
};
use crate::re::ni_t_string_map::{NiTStringMap, NiTStringTemplateMap};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSTCaseInsensitiveStringMap<T>`
#[repr(C)]
pub struct BSTCaseInsensitiveStringMap<T> {
    pub base: NiTStringMap<T>, // 00
}

const _: () = assert!(core::mem::size_of::<BSTCaseInsensitiveStringMap<*mut c_void>>() == 0x28);
const _: () =
    assert!(core::mem::offset_of!(BSTCaseInsensitiveStringMap<*mut c_void>, base) == 0x00);

pub type BSTCaseInsensitiveStringMapSetting = BSTCaseInsensitiveStringMap<*mut Setting>;

impl RttiType for BSTCaseInsensitiveStringMapSetting {
    const RTTI: VariantID = RTTI_BSTCaseInsensitiveStringMap_Setting___;
}

impl BSTCaseInsensitiveStringMapSetting {
    pub const RTTI: VariantID = RTTI_BSTCaseInsensitiveStringMap_Setting___;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSTCaseInsensitiveStringMap_Setting___;
}

impl<T> AsRef<BSTCaseInsensitiveStringMap<T>> for BSTCaseInsensitiveStringMap<T> {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsMut<BSTCaseInsensitiveStringMap<T>> for BSTCaseInsensitiveStringMap<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<T> AsRef<NiTStringMap<T>> for BSTCaseInsensitiveStringMap<T> {
    #[inline(always)]
    fn as_ref(&self) -> &NiTStringMap<T> {
        &self.base
    }
}

impl<T> AsMut<NiTStringMap<T>> for BSTCaseInsensitiveStringMap<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut NiTStringMap<T> {
        &mut self.base
    }
}

impl<T> AsRef<NiTStringTemplateMap<NiTMap<*const c_char, T>, T>>
    for BSTCaseInsensitiveStringMap<T>
{
    #[inline(always)]
    fn as_ref(&self) -> &NiTStringTemplateMap<NiTMap<*const c_char, T>, T> {
        &self.base.base
    }
}

impl<T> AsMut<NiTStringTemplateMap<NiTMap<*const c_char, T>, T>>
    for BSTCaseInsensitiveStringMap<T>
{
    #[inline(always)]
    fn as_mut(&mut self) -> &mut NiTStringTemplateMap<NiTMap<*const c_char, T>, T> {
        &mut self.base.base
    }
}

impl<T> BSTCaseInsensitiveStringMap<T> {
    // override (NiTStringMap<T>)
    // std::uint32_t hash_function(key_type a_key) const override;  // 01
    // bool key_eq(key_type a_lhs, key_type a_rhs) const override;  // 02

    #[inline(always)]
    pub fn iter(&self) -> NiTMapBaseIter<'_, *const c_char, T> {
        self.base.iter()
    }

    pub fn find_case_insensitive(
        &self,
        key: *const c_char,
    ) -> Option<&NiTMapItem<*const c_char, T>> {
        self.iter()
            .find(|entry| cstr_eq_ignore_ascii_case(entry.first, key))
    }

    pub fn find_case_insensitive_str(&self, key: &str) -> Option<&NiTMapItem<*const c_char, T>> {
        self.iter()
            .find(|entry| cstr_eq_str_ignore_ascii_case(entry.first, key))
    }
}

pub trait BSTCaseInsensitiveStringMapExt<T> {
    fn iter(&self) -> NiTMapBaseIter<'_, *const c_char, T>;
    fn find_case_insensitive(&self, key: *const c_char) -> Option<&NiTMapItem<*const c_char, T>>;
    fn find_case_insensitive_str(&self, key: &str) -> Option<&NiTMapItem<*const c_char, T>>;
}

impl<T, U> BSTCaseInsensitiveStringMapExt<T> for U
where
    U: AsRef<BSTCaseInsensitiveStringMap<T>>,
{
    fn iter(&self) -> NiTMapBaseIter<'_, *const c_char, T> {
        self.as_ref().iter()
    }

    fn find_case_insensitive(&self, key: *const c_char) -> Option<&NiTMapItem<*const c_char, T>> {
        self.as_ref().find_case_insensitive(key)
    }

    fn find_case_insensitive_str(&self, key: &str) -> Option<&NiTMapItem<*const c_char, T>> {
        self.as_ref().find_case_insensitive_str(key)
    }
}
