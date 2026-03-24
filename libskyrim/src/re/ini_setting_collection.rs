use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_INISettingCollection;
use crate::offsets::offsets_vtable::VTABLE_INISettingCollection;
use crate::re::{Setting, SettingCollectionListSetting};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::INISettingCollection`
#[repr(C)]
pub struct INISettingCollection {
    pub base: SettingCollectionListSetting, // 00
}

const _: () = assert!(core::mem::size_of::<INISettingCollection>() == 0x128);
const _: () = assert!(core::mem::offset_of!(INISettingCollection, base) == 0x00);

impl RttiType for INISettingCollection {
    const RTTI: VariantID = RTTI_INISettingCollection;
}

inherit!(INISettingCollection : SettingCollectionListSetting);

impl INISettingCollection {
    pub const RTTI: VariantID = RTTI_INISettingCollection;
    pub const VTABLE: &'static [VariantID] = &VTABLE_INISettingCollection;

    // override (SettingCollectionList<Setting>)
    // bool WriteSetting(Setting* a_setting) override;  // 03
    // bool ReadSetting(Setting* a_setting) override;   // 04
    // bool OpenHandle(bool a_create) override;         // 05
    // bool CloseHandle() override;                     // 06

    crate::relocation_variable! {
        fn singleton_ptr() -> &'static *mut INISettingCollection => RelocationID::new(524557, 411155)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut INISettingCollection {
        *Self::singleton_ptr()
    }

    pub fn get_setting(&self, name: &str) -> *mut Setting {
        for setting in self.base.settings_iter() {
            if !setting.is_null() {
                let current = unsafe { &*setting };
                if current.get_name_as_str().len() == name.len()
                    && current.get_name_as_str().eq_ignore_ascii_case(name)
                {
                    return setting;
                }
            }
        }

        core::ptr::null_mut()
    }
}
