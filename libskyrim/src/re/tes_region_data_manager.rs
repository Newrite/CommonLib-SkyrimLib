use crate::offsets::offsets_rtti::RTTI_TESRegionDataManager;
use crate::offsets::offsets_vtable::VTABLE_TESRegionDataManager;
use crate::re::{
    TESFile, TESRegion, TESRegionData, TESRegionDataGrass, TESRegionDataLandscape,
    TESRegionDataMap, TESRegionDataObjects, TESRegionDataSound, TESRegionDataWeather,
};
use crate::relocation::{RttiType, VariantID};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionDataId {
    Objects = 2,
    Weather = 3,
    Map = 4,
    Land = 5,
    Grass = 6,
    Sound = 7,
    Imposter = 8,
}

#[repr(C)]
pub struct TESRegionDataManager {
    pub vtable: *const usize,
    pub last_loaded_region: *mut TESRegion,
}

const _: () = assert!(core::mem::size_of::<TESRegionDataManager>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESRegionDataManager, last_loaded_region) == 0x08);

impl RttiType for TESRegionDataManager {
    const RTTI: VariantID = RTTI_TESRegionDataManager;
}

impl TESRegionDataManager {
    pub const RTTI: VariantID = RTTI_TESRegionDataManager;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRegionDataManager;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_GET_LAST_LOADED_REGION: usize = 0x01;
        pub fn get_last_loaded_region() -> *mut TESRegion
    }

    crate::virtual_method! {
        pub const VFUNC_CONSTRUCT_REGION_DATA: usize = 0x02;
        pub fn construct_region_data(id: RegionDataId) -> *mut TESRegionData
    }

    crate::virtual_method! {
        pub const VFUNC_AS_REGION_DATA_OBJECTS: usize = 0x03;
        pub fn as_region_data_objects(data: *mut TESRegionData) -> *mut TESRegionDataObjects
    }

    crate::virtual_method! {
        pub const VFUNC_AS_REGION_DATA_WEATHER: usize = 0x04;
        pub fn as_region_data_weather(data: *mut TESRegionData) -> *mut TESRegionDataWeather
    }

    crate::virtual_method! {
        pub const VFUNC_AS_REGION_DATA_MAP: usize = 0x05;
        pub fn as_region_data_map(data: *mut TESRegionData) -> *mut TESRegionDataMap
    }

    crate::virtual_method! {
        pub const VFUNC_AS_REGION_DATA_LANDSCAPE: usize = 0x06;
        pub fn as_region_data_landscape(data: *mut TESRegionData) -> *mut TESRegionDataLandscape
    }

    crate::virtual_method! {
        pub const VFUNC_AS_REGION_DATA_GRASS: usize = 0x07;
        pub fn as_region_data_grass(data: *mut TESRegionData) -> *mut TESRegionDataGrass
    }

    crate::virtual_method! {
        pub const VFUNC_AS_REGION_DATA_SOUND: usize = 0x08;
        pub fn as_region_data_sound(data: *mut TESRegionData) -> *mut TESRegionDataSound
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_REGION_DATA: usize = 0x09;
        pub fn load_region_data(file: *mut TESFile, region: *mut TESRegion) -> bool
    }
}
