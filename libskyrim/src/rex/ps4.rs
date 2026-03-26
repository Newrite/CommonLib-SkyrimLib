//! Minimal source-backed subset of `REX/PS4/**` used by RE translations.

#[allow(non_snake_case)]
pub mod PS4 {
    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum SCE_PAD_BUTTON {
        SCE_PAD_BUTTON_SHARE = 0x0000_0001,
        SCE_PAD_BUTTON_L3 = 0x0000_0002,
        SCE_PAD_BUTTON_R3 = 0x0000_0004,
        SCE_PAD_BUTTON_OPTIONS = 0x0000_0008,
        SCE_PAD_BUTTON_UP = 0x0000_0010,
        SCE_PAD_BUTTON_RIGHT = 0x0000_0020,
        SCE_PAD_BUTTON_DOWN = 0x0000_0040,
        SCE_PAD_BUTTON_LEFT = 0x0000_0080,
        SCE_PAD_BUTTON_L2 = 0x0000_0100,
        SCE_PAD_BUTTON_R2 = 0x0000_0200,
        SCE_PAD_BUTTON_L1 = 0x0000_0400,
        SCE_PAD_BUTTON_R1 = 0x0000_0800,
        SCE_PAD_BUTTON_TRIANGLE = 0x0000_1000,
        SCE_PAD_BUTTON_CIRCLE = 0x0000_2000,
        SCE_PAD_BUTTON_CROSS = 0x0000_4000,
        SCE_PAD_BUTTON_SQUARE = 0x0000_8000,
        SCE_PAD_BUTTON_PLAYSTATION = 0x0001_0000,
        SCE_PAD_BUTTON_TOUCH_PAD = 0x0010_0000,
        SCE_PAD_BUTTON_INTERCEPTED = 0x8000_0000,
    }
}
