use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_RegSettingCollection;
use crate::offsets::offsets_vtable::VTABLE_RegSettingCollection;
use crate::re::SettingCollectionListSetting;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::RegSettingCollection`
#[repr(C)]
pub struct RegSettingCollection {
    pub base: SettingCollectionListSetting, // 00
}

const _: () = assert!(core::mem::size_of::<RegSettingCollection>() == 0x128);
const _: () = assert!(core::mem::offset_of!(RegSettingCollection, base) == 0x00);

impl RttiType for RegSettingCollection {
    const RTTI: VariantID = RTTI_RegSettingCollection;
}

inherit!(RegSettingCollection : SettingCollectionListSetting);

impl RegSettingCollection {
    pub const RTTI: VariantID = RTTI_RegSettingCollection;
    pub const VTABLE: &'static [VariantID] = &VTABLE_RegSettingCollection;

    // override (SettingCollectionList<Setting>)
    // bool WriteSetting(Setting* a_setting) override;  // 03
    // bool ReadSetting(Setting* a_setting) override;   // 04
    // bool OpenHandle(bool a_create) override;         // 05
    // bool CloseHandle() override;                     // 06
}
