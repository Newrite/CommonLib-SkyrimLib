use crate::offsets::offsets_rtti::RTTI_SettingCollectionList_Setting_;
use crate::offsets::offsets_vtable::VTABLE_SettingCollectionList_Setting_;
use crate::re::{BSSimpleList, BSSimpleListIter, Setting, SettingCollection};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SettingCollectionList<T>`
#[repr(C)]
pub struct SettingCollectionList<T> {
    pub base: SettingCollection<T>,     // 00
    pub settings: BSSimpleList<*mut T>, // 118
}

const _: () = assert!(core::mem::size_of::<SettingCollectionList<Setting>>() == 0x128);
const _: () = assert!(core::mem::offset_of!(SettingCollectionList<Setting>, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(SettingCollectionList<Setting>, settings) == 0x118);

pub type SettingCollectionListSetting = SettingCollectionList<Setting>;

impl RttiType for SettingCollectionList<Setting> {
    const RTTI: VariantID = RTTI_SettingCollectionList_Setting_;
}

impl<T> AsRef<SettingCollectionList<T>> for SettingCollectionList<T> {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsMut<SettingCollectionList<T>> for SettingCollectionList<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl SettingCollectionList<Setting> {
    pub const RTTI: VariantID = RTTI_SettingCollectionList_Setting_;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SettingCollectionList_Setting_;
}

impl<T> AsRef<SettingCollection<T>> for SettingCollectionList<T> {
    #[inline(always)]
    fn as_ref(&self) -> &SettingCollection<T> {
        &self.base
    }
}

impl<T> AsMut<SettingCollection<T>> for SettingCollectionList<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut SettingCollection<T> {
        &mut self.base
    }
}

impl<T> SettingCollectionList<T> {
    // override (SettingCollection<T>)
    // void InsertSetting(T* a_setting) override;  // 01
    // void RemoveSetting(T* a_setting) override;  // 02
    // void WriteAllSettings() override;           // 08
    // void ReadAllSettings() override;            // 09

    #[inline(always)]
    pub fn settings_iter(&self) -> core::iter::Copied<BSSimpleListIter<'_, *mut T>> {
        self.settings.iter().copied()
    }
}

pub trait SettingCollectionListExt<T> {
    fn settings_iter(&self) -> core::iter::Copied<BSSimpleListIter<'_, *mut T>>;
}

impl<T, U> SettingCollectionListExt<T> for U
where
    U: AsRef<SettingCollectionList<T>>,
{
    fn settings_iter(&self) -> core::iter::Copied<BSSimpleListIter<'_, *mut T>> {
        SettingCollectionList::settings_iter(self.as_ref())
    }
}
