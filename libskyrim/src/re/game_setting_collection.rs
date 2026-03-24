use core::ffi::c_char;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_GameSettingCollection;
use crate::offsets::offsets_vtable::VTABLE_GameSettingCollection;
use crate::re::{Setting, SettingCollectionMapSetting};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::GameSettingCollection`
#[repr(C)]
pub struct GameSettingCollection {
    pub base: SettingCollectionMapSetting, // 00
}

const _: () = assert!(core::mem::size_of::<GameSettingCollection>() == 0x140);
const _: () = assert!(core::mem::offset_of!(GameSettingCollection, base) == 0x00);

impl RttiType for GameSettingCollection {
    const RTTI: VariantID = RTTI_GameSettingCollection;
}

inherit!(GameSettingCollection : SettingCollectionMapSetting);

impl GameSettingCollection {
    pub const RTTI: VariantID = RTTI_GameSettingCollection;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GameSettingCollection;

    // override (SettingCollectionMap<Setting>)
    // bool WriteSetting(Setting* a_setting) override;  // 03
    // bool ReadSetting(Setting* a_setting) override;   // 04
    // bool OpenHandle(bool a_create) override;         // 05
    // bool CloseHandle() override;                     // 06
    // virtual void Unk_0A(void);                       // 0A

    crate::relocation_variable! {
        fn singleton_ptr() -> &'static *mut GameSettingCollection => RelocationID::new(514622, 400782)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut GameSettingCollection {
        *Self::singleton_ptr()
    }

    pub fn get_setting(&self, name: *const c_char) -> *mut Setting {
        self.base
            .settings
            .find_case_insensitive(name)
            .map(|entry| entry.second)
            .unwrap_or(core::ptr::null_mut())
    }

    #[inline(always)]
    pub fn get_setting_str(&self, name: &str) -> *mut Setting {
        self.base
            .settings
            .find_case_insensitive_str(name)
            .map(|entry| entry.second)
            .unwrap_or(core::ptr::null_mut())
    }
}
