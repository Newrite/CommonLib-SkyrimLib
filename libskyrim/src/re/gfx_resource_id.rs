#![allow(non_camel_case_types)]

/// C++ `RE::GFxResourceID::IDTypeConstants::IDTypeConstant`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxResourceIDTypeConstant {
    kIndexMask = 0x0000FFFF,
    kTypeMask = 0x0FFF0000,
    kGenMask = 0x00030000,
    kSWF = 0,
    kStatic = 1 << 16,
    kExport = 1 << 17,
    kTypeShift = 18,
    kInvalidID = 1 << 18,
}

/// C++ `RE::GFxResourceID::IDTypes::IDType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxResourceIDType {
    kNone = 0,
    kInternalConstant = (0 << 18) | (1 << 16),
    kGradientImage = (1 << 18) | (1 << 16),
    kDynFontImage = (2 << 18) | (1 << 16),
    kFontImage = (1 << 18) | (1 << 17),
}

/// C++ `RE::GFxResourceID`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GFxResourceID {
    pub id: u32, // 00
}

const _: () = assert!(core::mem::size_of::<GFxResourceID>() == 0x4);
const _: () = assert!(core::mem::offset_of!(GFxResourceID, id) == 0x0);
