use crate::re::{TESGlobal, TESWeather};

#[repr(C)]
pub struct WeatherType {
    pub weather: *mut TESWeather,
    pub chance: u32,
    pub unk0c: u32,
    pub global: *mut TESGlobal,
}

const _: () = assert!(core::mem::size_of::<WeatherType>() == 0x18);
const _: () = assert!(core::mem::offset_of!(WeatherType, weather) == 0x00);
const _: () = assert!(core::mem::offset_of!(WeatherType, chance) == 0x08);
const _: () = assert!(core::mem::offset_of!(WeatherType, unk0c) == 0x0C);
const _: () = assert!(core::mem::offset_of!(WeatherType, global) == 0x10);
