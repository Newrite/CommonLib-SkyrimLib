use core::ffi::{c_char, c_void};

use crate::offsets::offsets_rtti::RTTI_NiTMap_char_const___Setting___;
use crate::offsets::offsets_vtable::VTABLE_NiTMap_char_const___Setting___;
use crate::re::Setting;
use crate::re::ni_t_default_allocator::NiTDefaultAllocator;
use crate::re::ni_t_map_base::{NiTMapBase, NiTMapItem};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::NiTMap<Key, T>`
#[repr(C)]
pub struct NiTMap<K, T> {
    pub base: NiTMapBase<NiTDefaultAllocator<NiTMapItem<K, T>>, K, T>, // 00
}

const _: () = assert!(core::mem::size_of::<NiTMap<*const c_char, *mut c_void>>() == 0x20);
const _: () = assert!(core::mem::offset_of!(NiTMap<*const c_char, *mut c_void>, base) == 0x00);

pub type NiTMapSetting = NiTMap<*const c_char, *mut Setting>;

impl RttiType for NiTMapSetting {
    const RTTI: VariantID = RTTI_NiTMap_char_const___Setting___;
}

impl NiTMapSetting {
    pub const RTTI: VariantID = RTTI_NiTMap_char_const___Setting___;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiTMap_char_const___Setting___;
}

impl<K, T> AsRef<NiTMap<K, T>> for NiTMap<K, T> {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<K, T> AsMut<NiTMap<K, T>> for NiTMap<K, T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<K, T> AsRef<NiTMapBase<NiTDefaultAllocator<NiTMapItem<K, T>>, K, T>> for NiTMap<K, T> {
    #[inline(always)]
    fn as_ref(&self) -> &NiTMapBase<NiTDefaultAllocator<NiTMapItem<K, T>>, K, T> {
        &self.base
    }
}

impl<K, T> AsMut<NiTMapBase<NiTDefaultAllocator<NiTMapItem<K, T>>, K, T>> for NiTMap<K, T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut NiTMapBase<NiTDefaultAllocator<NiTMapItem<K, T>>, K, T> {
        &mut self.base
    }
}

impl<K, T> NiTMap<K, T> {
    // override (NiTMapBase)
    // value_type* malloc_value() override;  // 05
    // void free_value(value_type* a_value) override;  // 06

    #[inline(always)]
    pub fn iter(&self) -> crate::re::ni_t_map_base::NiTMapBaseIter<'_, K, T> {
        self.base.iter()
    }
}

pub trait NiTMapExt<K, T> {
    fn iter(&self) -> crate::re::ni_t_map_base::NiTMapBaseIter<'_, K, T>;
}

impl<K, T, U> NiTMapExt<K, T> for U
where
    U: AsRef<NiTMap<K, T>>,
{
    fn iter(&self) -> crate::re::ni_t_map_base::NiTMapBaseIter<'_, K, T> {
        self.as_ref().iter()
    }
}
