use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_INIPrefSettingCollection;
use crate::offsets::offsets_vtable::VTABLE_INIPrefSettingCollection;
use crate::re::INISettingCollection;
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::INIPrefSettingCollection`
#[repr(C)]
pub struct INIPrefSettingCollection {
    pub base: INISettingCollection, // 00
}

const _: () = assert!(core::mem::size_of::<INIPrefSettingCollection>() == 0x128);
const _: () = assert!(core::mem::offset_of!(INIPrefSettingCollection, base) == 0x00);

impl RttiType for INIPrefSettingCollection {
    const RTTI: VariantID = RTTI_INIPrefSettingCollection;
}

inherit!(INIPrefSettingCollection : INISettingCollection);

impl INIPrefSettingCollection {
    pub const RTTI: VariantID = RTTI_INIPrefSettingCollection;
    pub const VTABLE: &'static [VariantID] = &VTABLE_INIPrefSettingCollection;

    // override (INISettingCollection)
    // void Unk_07(void) override;        // 07
    // void WriteAllSettings() override;  // 08
    // void ReadAllSettings() override;   // 09

    crate::relocation_variable! {
        fn singleton_ptr() -> &'static *mut INIPrefSettingCollection => RelocationID::new(523673, 410219)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut INIPrefSettingCollection {
        *Self::singleton_ptr()
    }
}
