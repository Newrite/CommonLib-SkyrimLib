use core_util::{EnumSet, inherit};
use cstd::time::tm;

use crate::offsets::offsets_rtti::RTTI_TESClimate;
use crate::offsets::offsets_vtable::VTABLE_TESClimate;
use crate::re::{FormCastable, FormType, TESFile, TESForm, TESModel, TESTexture, WeatherType};
use crate::relocation::{RttiType, VariantID};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESClimateSkyObject {
    Sun = 0,
    SunGlare = 1,
}

pub const TES_CLIMATE_SKY_OBJECT_TOTAL: usize = 2;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESClimateRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESClimateMoonPhaseLength {
    PhaseLengthMask = 0x3F,
    None = 0,
    Masser = 1 << 6,
    Secunda = 1 << 7,
}

core_util::impl_enumset_type!(TESClimateMoonPhaseLength => u8);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TESClimateTimingInterval {
    pub begin: u8,
    pub end: u8,
}

const _: () = assert!(core::mem::size_of::<TESClimateTimingInterval>() == 0x2);
const _: () = assert!(core::mem::offset_of!(TESClimateTimingInterval, begin) == 0x0);
const _: () = assert!(core::mem::offset_of!(TESClimateTimingInterval, end) == 0x1);

impl TESClimateTimingInterval {
    #[inline(always)]
    pub fn convert_interval(time: u8) -> tm {
        tm {
            tm_sec: 0,
            tm_min: ((time as i32) * 10) % 60,
            tm_hour: ((time as i32) * 10) / 60,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
        }
    }

    #[inline(always)]
    pub fn get_begin_time(&self) -> tm {
        Self::convert_interval(self.begin)
    }

    #[inline(always)]
    pub fn get_end_time(&self) -> tm {
        Self::convert_interval(self.end)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TESClimateTiming {
    pub sunrise: TESClimateTimingInterval,
    pub sunset: TESClimateTimingInterval,
    pub volatility: u8,
    pub moon_phase_length: EnumSet<TESClimateMoonPhaseLength, u8>,
    pub unk6: u8,
    pub unk7: u8,
}

const _: () = assert!(core::mem::size_of::<TESClimateTiming>() == 0x8);
const _: () = assert!(core::mem::offset_of!(TESClimateTiming, sunrise) == 0x0);
const _: () = assert!(core::mem::offset_of!(TESClimateTiming, sunset) == 0x2);
const _: () = assert!(core::mem::offset_of!(TESClimateTiming, volatility) == 0x4);
const _: () = assert!(core::mem::offset_of!(TESClimateTiming, moon_phase_length) == 0x5);
const _: () = assert!(core::mem::offset_of!(TESClimateTiming, unk6) == 0x6);
const _: () = assert!(core::mem::offset_of!(TESClimateTiming, unk7) == 0x7);

impl TESClimateTiming {
    #[inline(always)]
    pub fn includes_masser(&self) -> bool {
        self.moon_phase_length
            .all(TESClimateMoonPhaseLength::Masser)
    }

    #[inline(always)]
    pub fn includes_secunda(&self) -> bool {
        self.moon_phase_length
            .all(TESClimateMoonPhaseLength::Secunda)
    }

    #[inline(always)]
    pub fn get_phase_length(&self) -> u8 {
        self.moon_phase_length.underlying() & (TESClimateMoonPhaseLength::PhaseLengthMask as u8)
    }
}

#[repr(C)]
pub struct TESClimate {
    pub base: TESForm,
    pub night_sky: TESModel,
    pub weather_list: crate::re::BSSimpleList<*mut WeatherType>,
    pub sky_objects: [TESTexture; TES_CLIMATE_SKY_OBJECT_TOTAL],
    pub timing: TESClimateTiming,
}

const _: () = assert!(core::mem::size_of::<TESClimate>() == 0x80);
const _: () = assert!(core::mem::offset_of!(TESClimate, night_sky) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESClimate, weather_list) == 0x48);
const _: () = assert!(core::mem::offset_of!(TESClimate, sky_objects) == 0x58);
const _: () = assert!(core::mem::offset_of!(TESClimate, timing) == 0x78);

impl RttiType for TESClimate {
    const RTTI: VariantID = RTTI_TESClimate;
}

impl FormCastable for TESClimate {
    const TARGET_FORM_TYPE: FormType = FormType::Climate;
}

inherit!(TESClimate : TESForm);

impl TESClimate {
    pub const RTTI: VariantID = RTTI_TESClimate;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESClimate;
    pub const FORMTYPE: FormType = FormType::Climate;

    // override (TESForm)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_INITIALIZE_DATA: usize = 0x04;
        pub fn initialize_data(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_: *mut TESFile) -> bool
    }
}
