use core::ffi::{c_char, c_void};
use core::ops::{Index, IndexMut};

use core_util::EnumSet;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESWorldSpace;
use crate::offsets::offsets_vtable::VTABLE_TESWorldSpace;
use crate::re::bgs_encounter_zone::BGSEncounterZone;
use crate::re::bgs_lighting_template::BGSLightingTemplate;
use crate::re::bgs_location::BGSLocation;
use crate::re::bgs_music_type::BGSMusicType;
use crate::re::bgs_terrain_manager::BGSTerrainManager;
use crate::re::bs_portal_graph::BSPortalGraph;
use crate::re::bs_string::BSString;
use crate::re::bst_array::BSTArray;
use crate::re::bst_hash_map::{BSTHashMap, UnkKey, UnkValue};
use crate::re::crc::{BSTHash, generate_crc32};
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::ni_node::NiNode;
use crate::re::ni_point2::NiPoint2;
use crate::re::ni_point3::NiPoint3;
use crate::re::ni_smart_pointer::NiPointer;
use crate::re::ni_t_pointer_map::NiTPointerMap;
use crate::re::tes_climate::TESClimate;
use crate::re::tes_form::{FormID, TESForm};
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_model::TESModel;
use crate::re::tes_object_cell::TESObjectCELL;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::re::tes_texture::TESTexture;
use crate::re::tes_water_form::TESWaterForm;
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::WORLD_MAP_DATA::CameraData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WorldMapCameraData {
    pub min_height: f32,    // 00
    pub max_height: f32,    // 04
    pub initial_pitch: f32, // 08
}

const _: () = assert!(core::mem::size_of::<WorldMapCameraData>() == 0xC);

/// C++ `RE::WORLD_MAP_DATA` (`MNAM`)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WORLD_MAP_DATA {
    pub usable_width: u32,               // 00
    pub usable_height: u32,              // 04
    pub nw_cell_x: i16,                  // 08
    pub nw_cell_y: i16,                  // 0A
    pub se_cell_x: i16,                  // 0C
    pub se_cell_y: i16,                  // 0E
    pub camera_data: WorldMapCameraData, // 10
}

const _: () = assert!(core::mem::size_of::<WORLD_MAP_DATA>() == 0x1C);

/// C++ `RE::WORLD_MAP_OFFSET_DATA` (`ONAM`)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WORLD_MAP_OFFSET_DATA {
    pub map_scale: f32,    // 00
    pub map_offset_x: f32, // 04
    pub map_offset_y: f32, // 08
    pub map_offset_z: f32, // 0C
}

const _: () = assert!(core::mem::size_of::<WORLD_MAP_OFFSET_DATA>() == 0x10);

/// C++ `RE::CellID`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CellID {
    pub y: i16, // 00
    pub x: i16, // 02
}

const _: () = assert!(core::mem::size_of::<CellID>() == 0x4);

impl CellID {
    #[inline(always)]
    pub const fn new(y: i16, x: i16) -> Self {
        Self { y, x }
    }
}

impl Index<usize> for CellID {
    type Output = i16;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);
        if index == 0 { &self.y } else { &self.x }
    }
}

impl IndexMut<usize> for CellID {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);
        if index == 0 { &mut self.y } else { &mut self.x }
    }
}

impl BSTHash for CellID {
    #[inline]
    fn bst_hash(&self) -> u32 {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        };
        generate_crc32(bytes)
    }
}

/// C++ `RE::BGSLargeRefData` (`RNAM`)
#[repr(C)]
pub struct BGSLargeRefData {
    pub cell_form_id_map: BSTHashMap<CellID, *mut FormID>, // 00
    pub form_id_cell_map: BSTHashMap<FormID, CellID>,      // 30
    pub cell_form_id_map_filtered: BSTHashMap<CellID, *mut FormID>, // 60
}

const _: () = assert!(core::mem::size_of::<BGSLargeRefData>() == 0x90);

/// C++ `RE::TESWorldSpace::Flag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    None = 0,
    SmallWorld = 1 << 0,
    CantFastTravel = 1 << 1,
    NoLodWater = 1 << 3,
    NoLandscape = 1 << 4,
    NoSky = 1 << 5,
    FixedDimensions = 1 << 6,
    NoGrass = 1 << 7,
}

/// C++ `RE::TESWorldSpace::ParentUseFlag`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentUseFlag {
    None = 0,
    UseLandData = 1 << 0,
    UseLodData = 1 << 1,
    UseMapData = 1 << 2,
    UseWaterData = 1 << 3,
    UseClimateData = 1 << 4,
    UseImageSpaceData = 1 << 5,
    UseSkyCell = 1 << 6,
}

core_util::impl_enumset_type!(Flag => u8);
core_util::impl_enumset_type!(ParentUseFlag => u16);

/// C++ `RE::TESWorldSpace::RecordFlags`
pub struct RecordFlags;

impl RecordFlags {
    pub const DELETED: u32 = 1 << 5;
    pub const IGNORED: u32 = 1 << 12;
    pub const CANT_WAIT: u32 = 1 << 19;
}

/// C++ `RE::TESWorldSpace::ShortPoint`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ShortPoint {
    pub x: i16, // 00
    pub y: i16, // 02
}

const _: () = assert!(core::mem::size_of::<ShortPoint>() == 0x4);

/// C++ `RE::TESWorldSpace`
#[repr(C)]
pub struct TESWorldSpace {
    pub base: TESForm,                                    // 000
    pub full_name: TESFullName,                           // 020
    pub model: TESModel,                                  // 030
    pub cell_map: BSTHashMap<CellID, *mut TESObjectCELL>, // 058
    pub persistent_cell: *mut TESObjectCELL,              // 088
    pub terrain_manager: *mut BGSTerrainManager,          // 090
    pub climate: *mut TESClimate,                         // 098 - CNAM
    pub flags: EnumSet<Flag, u8>,                         // 0A0 - DATA
    pub unk0a1: u8,                                       // 0A1
    pub parent_use_flags: EnumSet<ParentUseFlag, u16>,    // 0A2 - PNAM
    pub fixed_center: ShortPoint,                         // 0A4 - WCTR
    pub fixed_persistent_ref_map: BSTHashMap<u32, BSTArray<NiPointer<TESObjectREFR>>>, // 0A8
    pub mobile_persistent_refs: BSTArray<NiPointer<TESObjectREFR>>, // 0D8
    pub overlapped_multibound_map: *mut NiTPointerMap,    // 0F0
    pub sky_cell: *mut TESObjectCELL,                     // 0F8
    pub location_map: BSTHashMap<FormID, *mut BGSLocation>, // 100
    pub portal_graph: NiPointer<BSPortalGraph>,           // 130
    pub unk138: *mut c_void,                              // 138
    pub unk140: *mut c_void,                              // 140
    pub multi_bound_node: NiPointer<NiNode>,              // 148
    pub portal_shared_node: NiPointer<NiNode>,            // 150
    pub parent_world: *mut TESWorldSpace,                 // 158 - WNAM
    pub lighting_template: *mut BGSLightingTemplate,      // 160 - LTMP
    pub world_water: *mut TESWaterForm,                   // 168 - NAM2
    pub lod_water: *mut TESWaterForm,                     // 170 - NAM3
    pub lod_water_height: f32,                            // 178 - NAM4
    pub pad17c: u32,                                      // 17C
    pub unk180: u64,                                      // 180
    pub world_map_data: WORLD_MAP_DATA,                   // 188 - MNAM
    pub world_map_offset_data: WORLD_MAP_OFFSET_DATA,     // 1A4 - ONAM
    pub pad1b4: u32,                                      // 1B4
    pub music_type: *mut BGSMusicType,                    // 1B8 - ZNAM
    pub minimum_coords: NiPoint2,                         // 1C0
    pub maximum_coords: NiPoint2,                         // 1C8
    pub unk1d0: BSTHashMap<UnkKey, UnkValue>,             // 1D0
    pub editor_id: BSString,                              // 200 - EDID
    pub default_land_height: f32,                         // 210
    pub default_water_height: f32,                        // 214
    pub distant_lod_mult: f32,                            // 218 - NAMA
    pub pad21c: u32,                                      // 21C
    pub encounter_zone: *mut BGSEncounterZone,            // 220 - XEZN
    pub location: *mut BGSLocation,                       // 228 - XLCN
    pub canopy_shadow_texture: TESTexture,                // 230 - TNAM
    pub water_env_map: TESTexture,                        // 240 - UNAM
    pub large_ref_data: BGSLargeRefData,                  // 250 - RNAM
    pub unk2e0: u64,                                      // 2E0
    pub unk2e8: BSTHashMap<UnkKey, UnkValue>,             // 2E8
    pub unk318: BSTHashMap<UnkKey, UnkValue>,             // 318
    pub north_rotation: f32,                              // 348
    pub pad34c: u32,                                      // 34C
    pub max_height_data: *mut i8,                         // 350 - MHDT
}

const _: () = assert!(core::mem::size_of::<TESWorldSpace>() == 0x358);
const _: () = assert!(core::mem::offset_of!(TESWorldSpace, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESWorldSpace, model) == 0x30);

impl RttiType for TESWorldSpace {
    const RTTI: VariantID = RTTI_TESWorldSpace;
}

impl FormCastable for TESWorldSpace {
    const TARGET_FORM_TYPE: FormType = FormType::WorldSpace;
}

inherit!(TESWorldSpace : TESForm);
inherit!(TESWorldSpace => TESFullName, full_name);
inherit!(TESWorldSpace => TESModel, model);

impl TESWorldSpace {
    pub const RTTI: VariantID = RTTI_TESWorldSpace;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESWorldSpace;
    pub const FORMTYPE: FormType = FormType::WorldSpace;

    // override (TESForm)
    // ~TESWorldSpace() override;                                                                       // 00
    // void InitializeData() override;                                                                  // 04
    // void ClearData() override;                                                                       // 05
    // bool Load(TESFile* a_mod) override;                                                              // 06
    // bool LoadPartial(TESFile* a_mod) override;                                                       // 07
    // TESForm* CreateDuplicateForm(bool a_createEditorID, NiTPointerMap<TESForm*, TESForm*>* a_map); // 09
    // bool FindInFileFast(TESFile* a_mod) override;                                                    // 0C
    // void InitItemImpl() override;                                                                    // 13
    // const char* GetFormEditorID() const override;                                                    // 32
    // bool SetFormEditorID(const char* a_str) override;                                                // 33
    // bool IsParentForm() override;                                                                    // 34
    // bool IsFormTypeChild(FormType a_type) override;                                                  // 36

    #[inline(always)]
    pub fn has_max_height_data(&self) -> bool {
        !self.max_height_data.is_null()
    }

    // RELOCATION_ID SE: 20103, AE: 20551
    crate::relocation_func! {
        pub fn get_max_height_at(&self, xy: &NiPoint3, out_height: &mut f32) -> bool => RelocationID::new(20103, 20551)
    }

    // RELOCATION_ID SE: 20095, AE: 20543
    crate::relocation_func! {
        pub fn get_sky_cell(&self) -> *mut TESObjectCELL => RelocationID::new(20095, 20543)
    }

    #[inline]
    pub fn get_terrain_manager(&self) -> *mut BGSTerrainManager {
        let mut world = self;
        while !world.parent_world.is_null() && world.parent_use_flags.any(ParentUseFlag::UseLodData)
        {
            world = unsafe { &*world.parent_world };
        }
        world.terrain_manager
    }

    #[inline]
    pub fn get_default_water_height(&self) -> f32 {
        let mut world = self;
        while !world.parent_world.is_null()
            && world.parent_use_flags.any(ParentUseFlag::UseLandData)
        {
            world = unsafe { &*world.parent_world };
        }
        world.default_water_height
    }

    #[inline(always)]
    pub fn get_form_editor_id_local(&self) -> *const c_char {
        self.editor_id.c_str()
    }

    #[inline(always)]
    pub fn get_form_editor_id_local_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_form_editor_id_local())
    }

    #[inline(always)]
    pub fn set_form_editor_id_local(&mut self, editor_id: *const c_char) -> bool {
        self.editor_id.set_c_str(editor_id, 0)
    }
}
