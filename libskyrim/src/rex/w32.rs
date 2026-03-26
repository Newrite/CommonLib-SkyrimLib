//! Minimal source-backed subset of `REX/W32/**` used by RE translations.
//!
//! Keep this module scoped to the CommonLib-backed ABI surface actually needed
//! by translated RE types instead of mirroring the full Windows SDK.

#[allow(non_snake_case)]
pub mod W32 {
    pub use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
    pub use windows_sys::Win32::System::Threading::{CRITICAL_SECTION, CRITICAL_SECTION_DEBUG};

    core_util::abstract_type! {
        pub type IDirectInput8A;
        pub type IDirectInputDevice8A;
        pub type IDirectInputEffect;
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DIK {
        DIK_ESCAPE = 0x1,
        DIK_1 = 0x2,
        DIK_2 = 0x3,
        DIK_3 = 0x4,
        DIK_4 = 0x5,
        DIK_5 = 0x6,
        DIK_6 = 0x7,
        DIK_7 = 0x8,
        DIK_8 = 0x9,
        DIK_9 = 0xA,
        DIK_0 = 0xB,
        DIK_MINUS = 0xC,
        DIK_EQUALS = 0xD,
        DIK_BACK = 0xE,
        DIK_TAB = 0xF,
        DIK_Q = 0x10,
        DIK_W = 0x11,
        DIK_E = 0x12,
        DIK_R = 0x13,
        DIK_T = 0x14,
        DIK_Y = 0x15,
        DIK_U = 0x16,
        DIK_I = 0x17,
        DIK_O = 0x18,
        DIK_P = 0x19,
        DIK_LBRACKET = 0x1A,
        DIK_RBRACKET = 0x1B,
        DIK_RETURN = 0x1C,
        DIK_LCONTROL = 0x1D,
        DIK_A = 0x1E,
        DIK_S = 0x1F,
        DIK_D = 0x20,
        DIK_F = 0x21,
        DIK_G = 0x22,
        DIK_H = 0x23,
        DIK_J = 0x24,
        DIK_K = 0x25,
        DIK_L = 0x26,
        DIK_SEMICOLON = 0x27,
        DIK_APOSTROPHE = 0x28,
        DIK_GRAVE = 0x29,
        DIK_LSHIFT = 0x2A,
        DIK_BACKSLASH = 0x2B,
        DIK_Z = 0x2C,
        DIK_X = 0x2D,
        DIK_C = 0x2E,
        DIK_V = 0x2F,
        DIK_B = 0x30,
        DIK_N = 0x31,
        DIK_M = 0x32,
        DIK_COMMA = 0x33,
        DIK_PERIOD = 0x34,
        DIK_SLASH = 0x35,
        DIK_RSHIFT = 0x36,
        DIK_MULTIPLY = 0x37,
        DIK_LMENU = 0x38,
        DIK_SPACE = 0x39,
        DIK_CAPITAL = 0x3A,
        DIK_F1 = 0x3B,
        DIK_F2 = 0x3C,
        DIK_F3 = 0x3D,
        DIK_F4 = 0x3E,
        DIK_F5 = 0x3F,
        DIK_F6 = 0x40,
        DIK_F7 = 0x41,
        DIK_F8 = 0x42,
        DIK_F9 = 0x43,
        DIK_F10 = 0x44,
        DIK_NUMLOCK = 0x45,
        DIK_SCROLL = 0x46,
        DIK_NUMPAD7 = 0x47,
        DIK_NUMPAD8 = 0x48,
        DIK_NUMPAD9 = 0x49,
        DIK_SUBTRACT = 0x4A,
        DIK_NUMPAD4 = 0x4B,
        DIK_NUMPAD5 = 0x4C,
        DIK_NUMPAD6 = 0x4D,
        DIK_ADD = 0x4E,
        DIK_NUMPAD1 = 0x4F,
        DIK_NUMPAD2 = 0x50,
        DIK_NUMPAD3 = 0x51,
        DIK_NUMPAD0 = 0x52,
        DIK_DECIMAL = 0x53,
        DIK_F11 = 0x57,
        DIK_F12 = 0x58,
        DIK_NUMPADENTER = 0x9C,
        DIK_RCONTROL = 0x9D,
        DIK_DIVIDE = 0xB5,
        DIK_SYSRQ = 0xB7,
        DIK_RMENU = 0xB8,
        DIK_PAUSE = 0xC5,
        DIK_HOME = 0xC7,
        DIK_UP = 0xC8,
        DIK_PRIOR = 0xC9,
        DIK_LEFT = 0xCB,
        DIK_RIGHT = 0xCD,
        DIK_END = 0xCF,
        DIK_DOWN = 0xD0,
        DIK_NEXT = 0xD1,
        DIK_INSERT = 0xD2,
        DIK_DELETE = 0xD3,
        DIK_LWIN = 0xDB,
        DIK_RWIN = 0xDC,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct DIDEVICEOBJECTDATA {
        pub ofs: u32,
        pub data: u32,
        pub time_stamp: u32,
        pub sequence: u32,
        pub app_data: usize,
    }

    const _: () = assert!(core::mem::size_of::<DIDEVICEOBJECTDATA>() == 0x18);

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct DIMOUSESTATE2 {
        pub x: i32,
        pub y: i32,
        pub z: i32,
        pub rgb_buttons: [u8; 8],
    }

    const _: () = assert!(core::mem::size_of::<DIMOUSESTATE2>() == 0x14);

    pub const XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE;
    pub const XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE;
    pub const XINPUT_GAMEPAD_TRIGGER_THRESHOLD: u8 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_TRIGGER_THRESHOLD as u8;

    pub const XINPUT_GAMEPAD_DPAD_UP: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_UP;
    pub const XINPUT_GAMEPAD_DPAD_DOWN: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_DOWN;
    pub const XINPUT_GAMEPAD_DPAD_LEFT: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_LEFT;
    pub const XINPUT_GAMEPAD_DPAD_RIGHT: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_RIGHT;
    pub const XINPUT_GAMEPAD_START: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_START;
    pub const XINPUT_GAMEPAD_BACK: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_BACK;
    pub const XINPUT_GAMEPAD_LEFT_THUMB: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_LEFT_THUMB;
    pub const XINPUT_GAMEPAD_RIGHT_THUMB: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_THUMB;
    pub const XINPUT_GAMEPAD_LEFT_SHOULDER: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_LEFT_SHOULDER;
    pub const XINPUT_GAMEPAD_RIGHT_SHOULDER: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_SHOULDER;
    pub const XINPUT_GAMEPAD_A: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_A;
    pub const XINPUT_GAMEPAD_B: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_B;
    pub const XINPUT_GAMEPAD_X: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_X;
    pub const XINPUT_GAMEPAD_Y: u16 =
        windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_Y;

    pub const XINPUT_GAMEPAD_BUTTON_MASK: u16 = XINPUT_GAMEPAD_DPAD_UP
        | XINPUT_GAMEPAD_DPAD_DOWN
        | XINPUT_GAMEPAD_DPAD_LEFT
        | XINPUT_GAMEPAD_DPAD_RIGHT
        | XINPUT_GAMEPAD_START
        | XINPUT_GAMEPAD_BACK
        | XINPUT_GAMEPAD_LEFT_THUMB
        | XINPUT_GAMEPAD_RIGHT_THUMB
        | XINPUT_GAMEPAD_LEFT_SHOULDER
        | XINPUT_GAMEPAD_RIGHT_SHOULDER
        | XINPUT_GAMEPAD_A
        | XINPUT_GAMEPAD_B
        | XINPUT_GAMEPAD_X
        | XINPUT_GAMEPAD_Y;

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct XINPUT_GAMEPAD {
        pub buttons: u16,
        pub left_trigger: u8,
        pub right_trigger: u8,
        pub thumb_lx: i16,
        pub thumb_ly: i16,
        pub thumb_rx: i16,
        pub thumb_ry: i16,
    }

    const _: () = assert!(core::mem::size_of::<XINPUT_GAMEPAD>() == 0x0C);

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct XINPUT_STATE {
        pub packet_number: u32,
        pub gamepad: XINPUT_GAMEPAD,
    }

    const _: () = assert!(core::mem::size_of::<XINPUT_STATE>() == 0x10);

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct XINPUT_VIBRATION {
        pub left_motor_speed: u16,
        pub right_motor_speed: u16,
    }

    const _: () = assert!(core::mem::size_of::<XINPUT_VIBRATION>() == 0x04);

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct XINPUT_CAPABILITIES {
        pub type_: u8,
        pub sub_type: u8,
        pub flags: u16,
        pub gamepad: XINPUT_GAMEPAD,
        pub vibration: XINPUT_VIBRATION,
    }

    const _: () = assert!(core::mem::size_of::<XINPUT_CAPABILITIES>() == 0x14);

    #[allow(non_snake_case)]
    pub mod VK {
        pub const VK_NUMLOCK: i32 =
            windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMLOCK as i32;
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub unsafe fn GetKeyState(virtual_key: i32) -> i16 {
        unsafe { windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyState(virtual_key) }
    }
}
