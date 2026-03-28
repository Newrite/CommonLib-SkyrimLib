use core::ffi::c_void;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_FaderMenu;
use crate::offsets::offsets_vtable::VTABLE_FaderMenu;
use crate::re::IMenu;
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::FaderMenu::RUNTIME_DATA`
#[repr(C)]
pub struct FaderMenuRuntimeData {
    // TODO: SOURCE - replace `*mut c_void` with the concrete smart-pointer type
    // after the pointee from `FaderMenu.h` is identified source-backed; the
    // vendored header only documents this slot as `void* ... // smart ptr`.
    pub unk30: *mut c_void, // 00
    pub is_active: bool,    // 08
    pub unk39: u8,          // 09
    pub pad3a: u16,         // 0A
    pub pad3c: u32,         // 0C
}

const _: () = assert!(core::mem::size_of::<FaderMenuRuntimeData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(FaderMenuRuntimeData, unk30) == 0x00);
const _: () = assert!(core::mem::offset_of!(FaderMenuRuntimeData, is_active) == 0x08);

/// Honest common prefix of C++ `RE::FaderMenu`.
#[repr(C)]
pub struct FaderMenu {
    pub base: IMenu, // 00
}

const _: () = assert!(core::mem::size_of::<FaderMenu>() == 0x30);
const _: () = assert!(core::mem::offset_of!(FaderMenu, base) == 0x00);

inherit!(FaderMenu : IMenu);

impl RttiType for FaderMenu {
    const RTTI: VariantID = RTTI_FaderMenu;
}

impl FaderMenu {
    pub const RTTI: VariantID = RTTI_FaderMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FaderMenu;
    pub const MENU_NAME: &'static str = "Fader Menu";
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x30, 0x30, 0x40);

    // override (IMenu)
    // ~FaderMenu() override;                                         // 00
    // UI_MESSAGE_RESULTS ProcessMessage(UIMessage& a_message);        // 04
    // void AdvanceMovie(float a_interval, std::uint32_t a_time);      // 05

    crate::runtime_data_accessor! {
        pub fn runtime_data() -> FaderMenuRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn runtime_data_mut() -> FaderMenuRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }
}
