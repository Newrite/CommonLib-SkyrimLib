use crate::offsets::offsets_rtti::RTTI_BSWin32KeyboardDevice;
use crate::offsets::offsets_vtable::VTABLE_BSWin32KeyboardDevice;
use crate::re::{BSKeyboardDevice, BSKeyboardDeviceKey};
use crate::relocation::{RttiType, VariantID};
use crate::rex::W32;

/// C++ `RE::BSWin32KeyboardDevice`
#[repr(C)]
pub struct BSWin32KeyboardDevice {
    pub base: BSKeyboardDevice,                     // 000
    pub d_input_device: *mut W32::IDirectInput8A,   // 070
    pub di_obj_data: [W32::DIDEVICEOBJECTDATA; 10], // 078
    pub prev_state: [u8; 0x100],                    // 168
    pub cur_state: [u8; 0x100],                     // 268
    pub caps_lock_on: bool,                         // 368
}

const _: () = assert!(core::mem::size_of::<BSWin32KeyboardDevice>() == 0x370);
const _: () = assert!(core::mem::offset_of!(BSWin32KeyboardDevice, d_input_device) == 0x070);
const _: () = assert!(core::mem::offset_of!(BSWin32KeyboardDevice, prev_state) == 0x168);
const _: () = assert!(core::mem::offset_of!(BSWin32KeyboardDevice, cur_state) == 0x268);
const _: () = assert!(core::mem::offset_of!(BSWin32KeyboardDevice, caps_lock_on) == 0x368);

impl RttiType for BSWin32KeyboardDevice {
    const RTTI: VariantID = RTTI_BSWin32KeyboardDevice;
}

core_util::inherit!(BSWin32KeyboardDevice : BSKeyboardDevice, base);

impl BSWin32KeyboardDevice {
    pub const RTTI: VariantID = RTTI_BSWin32KeyboardDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSWin32KeyboardDevice;

    #[inline(always)]
    pub fn is_pressed(&self, key_code: u32) -> bool {
        (key_code as usize) < self.cur_state.len()
            && (self.cur_state[key_code as usize] & 0x80) != 0
    }

    #[inline(always)]
    pub fn remap_numpad_key(&self, key: W32::DIK) -> BSKeyboardDeviceKey {
        if unsafe { W32::GetKeyState(W32::VK::VK_NUMLOCK) } != 1 {
            return BSKeyboardDeviceKey::kNone;
        }

        match key {
            W32::DIK::DIK_NUMPAD7 => BSKeyboardDeviceKey::kKP_Multiply,
            W32::DIK::DIK_NUMPAD8 => BSKeyboardDeviceKey::kLeftAlt,
            W32::DIK::DIK_NUMPAD9 => BSKeyboardDeviceKey::kSpacebar,
            W32::DIK::DIK_NUMPAD4 => BSKeyboardDeviceKey::kPeriod,
            W32::DIK::DIK_NUMPAD5 => BSKeyboardDeviceKey::kSlash,
            W32::DIK::DIK_NUMPAD6 => BSKeyboardDeviceKey::kRightShift,
            W32::DIK::DIK_NUMPAD1 => BSKeyboardDeviceKey::kN,
            W32::DIK::DIK_NUMPAD2 => BSKeyboardDeviceKey::kM,
            W32::DIK::DIK_NUMPAD3 => BSKeyboardDeviceKey::kComma,
            W32::DIK::DIK_NUMPAD0 => BSKeyboardDeviceKey::kB,
            W32::DIK::DIK_DECIMAL => BSKeyboardDeviceKey::kC,
            W32::DIK::DIK_DIVIDE => BSKeyboardDeviceKey::kV,
            _ => BSKeyboardDeviceKey::kNone,
        }
    }
}
