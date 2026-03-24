use crate::offsets::offsets_rtti::RTTI_SettingCollectionMap_Setting_;
use crate::offsets::offsets_vtable::VTABLE_SettingCollectionMap_Setting_;
use crate::re::ni_t_map_base::NiTMapBaseIter;
use crate::re::{BSTCaseInsensitiveStringMap, Setting, SettingCollection};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SettingCollectionMap<T>`
#[repr(C)]
pub struct SettingCollectionMap<T> {
    pub base: SettingCollection<T>,                    // 00
    pub settings: BSTCaseInsensitiveStringMap<*mut T>, // 118
}

const _: () = assert!(core::mem::size_of::<SettingCollectionMap<Setting>>() == 0x140);
const _: () = assert!(core::mem::offset_of!(SettingCollectionMap<Setting>, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(SettingCollectionMap<Setting>, settings) == 0x118);

pub type SettingCollectionMapSetting = SettingCollectionMap<Setting>;

impl RttiType for SettingCollectionMap<Setting> {
    const RTTI: VariantID = RTTI_SettingCollectionMap_Setting_;
}

impl<T> AsRef<SettingCollectionMap<T>> for SettingCollectionMap<T> {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsMut<SettingCollectionMap<T>> for SettingCollectionMap<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl SettingCollectionMap<Setting> {
    pub const RTTI: VariantID = RTTI_SettingCollectionMap_Setting_;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SettingCollectionMap_Setting_;
}

impl<T> AsRef<SettingCollection<T>> for SettingCollectionMap<T> {
    #[inline(always)]
    fn as_ref(&self) -> &SettingCollection<T> {
        &self.base
    }
}

impl<T> AsMut<SettingCollection<T>> for SettingCollectionMap<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut SettingCollection<T> {
        &mut self.base
    }
}

impl<T> SettingCollectionMap<T> {
    // override (SettingCollection<T>)
    // void InsertSetting(T* a_setting) override;  // 01
    // void RemoveSetting(T* a_setting) override;  // 02
    // void WriteAllSettings() override;           // 08
    // void ReadAllSettings() override;            // 09

    #[inline(always)]
    pub fn settings_iter(&self) -> NiTMapBaseIter<'_, *const core::ffi::c_char, *mut T> {
        self.settings.iter()
    }
}

pub trait SettingCollectionMapExt<T> {
    fn settings_iter(&self) -> NiTMapBaseIter<'_, *const core::ffi::c_char, *mut T>;
}

impl<T, U> SettingCollectionMapExt<T> for U
where
    U: AsRef<SettingCollectionMap<T>>,
{
    fn settings_iter(&self) -> NiTMapBaseIter<'_, *const core::ffi::c_char, *mut T> {
        SettingCollectionMap::settings_iter(self.as_ref())
    }
}
