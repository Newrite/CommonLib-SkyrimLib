#![allow(non_camel_case_types)]

/// C++ `RE::GFxFileConstants`
#[repr(C)]
pub struct GFxFileConstants {
    pub _pad: u8,
}

const _: () = assert!(core::mem::size_of::<GFxFileConstants>() == 0x1);

/// C++ `RE::GFxFileConstants::FileFormatType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxFileConstantsFileFormatType {
    kUnopened = 0,
    kUnknown = 1,
    kSWF = 2,
    kGFX = 3,
    kJPEG = 10,
    kPNG = 11,
    kGIF = 12,
    kTGA = 13,
    kDDS = 14,
    kHDR = 15,
    kBMP = 16,
    kDIB = 17,
    kPFM = 18,
    kTIFF = 19,
    kWAVE = 20,
    kNextAvail = 21,
    kOriginal = 65535,
}
