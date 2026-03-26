#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_BSKeyboardDevice;
use crate::offsets::offsets_vtable::VTABLE_BSKeyboardDevice;
use crate::re::BSInputDevice;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSKeyboardDevice::Keys::Key`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSKeyboardDeviceKey {
    kNone = 0x00,
    kEscape = 0x01,
    kNum1 = 0x02,
    kNum2 = 0x03,
    kNum3 = 0x04,
    kNum4 = 0x05,
    kNum5 = 0x06,
    kNum6 = 0x07,
    kNum7 = 0x08,
    kNum8 = 0x09,
    kNum9 = 0x0A,
    kNum0 = 0x0B,
    kMinus = 0x0C,
    kEquals = 0x0D,
    kBackspace = 0x0E,
    kTab = 0x0F,
    kQ = 0x10,
    kW = 0x11,
    kE = 0x12,
    kR = 0x13,
    kT = 0x14,
    kY = 0x15,
    kU = 0x16,
    kI = 0x17,
    kO = 0x18,
    kP = 0x19,
    kBracketLeft = 0x1A,
    kBracketRight = 0x1B,
    kEnter = 0x1C,
    kLeftControl = 0x1D,
    kA = 0x1E,
    kS = 0x1F,
    kD = 0x20,
    kF = 0x21,
    kG = 0x22,
    kH = 0x23,
    kJ = 0x24,
    kK = 0x25,
    kL = 0x26,
    kSemicolon = 0x27,
    kApostrophe = 0x28,
    kTilde = 0x29,
    kLeftShift = 0x2A,
    kBackslash = 0x2B,
    kZ = 0x2C,
    kX = 0x2D,
    kC = 0x2E,
    kV = 0x2F,
    kB = 0x30,
    kN = 0x31,
    kM = 0x32,
    kComma = 0x33,
    kPeriod = 0x34,
    kSlash = 0x35,
    kRightShift = 0x36,
    kKP_Multiply = 0x37,
    kLeftAlt = 0x38,
    kSpacebar = 0x39,
    kCapsLock = 0x3A,
    kF1 = 0x3B,
    kF2 = 0x3C,
    kF3 = 0x3D,
    kF4 = 0x3E,
    kF5 = 0x3F,
    kF6 = 0x40,
    kF7 = 0x41,
    kF8 = 0x42,
    kF9 = 0x43,
    kF10 = 0x44,
    kNumLock = 0x45,
    kScrollLock = 0x46,
    kKP_7 = 0x47,
    kKP_8 = 0x48,
    kKP_9 = 0x49,
    kKP_Subtract = 0x4A,
    kKP_4 = 0x4B,
    kKP_5 = 0x4C,
    kKP_6 = 0x4D,
    kKP_Plus = 0x4E,
    kKP_1 = 0x4F,
    kKP_2 = 0x50,
    kKP_3 = 0x51,
    kKP_0 = 0x52,
    kKP_Decimal = 0x53,
    kF11 = 0x57,
    kF12 = 0x58,
    kKP_Enter = 0x9C,
    kRightControl = 0x9D,
    kKP_Divide = 0xB5,
    kPrintScreen = 0xB7,
    kRightAlt = 0xB8,
    kPause = 0xC5,
    kHome = 0xC7,
    kUp = 0xC8,
    kPageUp = 0xC9,
    kLeft = 0xCB,
    kRight = 0xCD,
    kEnd = 0xCF,
    kDown = 0xD0,
    kPageDown = 0xD1,
    kInsert = 0xD2,
    kDelete = 0xD3,
    kLeftWin = 0xDB,
    kRightWin = 0xDC,
}

/// C++ `RE::BSKeyboardDevice`
#[repr(C)]
pub struct BSKeyboardDevice {
    pub base: BSInputDevice, // 00
}

const _: () = assert!(core::mem::size_of::<BSKeyboardDevice>() == 0x70);
const _: () = assert!(core::mem::offset_of!(BSKeyboardDevice, base) == 0x00);

impl RttiType for BSKeyboardDevice {
    const RTTI: VariantID = RTTI_BSKeyboardDevice;
}

core_util::inherit!(BSKeyboardDevice : BSInputDevice, base);

impl BSKeyboardDevice {
    pub const RTTI: VariantID = RTTI_BSKeyboardDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSKeyboardDevice;

    crate::virtual_method! {
        pub const VFUNC_UNK_09: usize = 0x09;
        pub fn unk_09(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0A: usize = 0x0A;
        pub fn unk_0a(&mut self)
    }
}

pub trait BSKeyboardDeviceExt {
    fn unk_09(&mut self);
    fn unk_0a(&mut self);
}

impl<T> BSKeyboardDeviceExt for T
where
    T: AsRef<BSKeyboardDevice> + AsMut<BSKeyboardDevice>,
{
    #[inline(always)]
    fn unk_09(&mut self) {
        BSKeyboardDevice::unk_09(self.as_mut())
    }

    #[inline(always)]
    fn unk_0a(&mut self) {
        BSKeyboardDevice::unk_0a(self.as_mut())
    }
}
