use core::ffi::{c_char, c_void};

use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESObjectCELL;
use crate::offsets::offsets_vtable::VTABLE_TESObjectCELL;
use crate::re::bs_atomic::{BSSpinLock, BSSpinLockGuard};
use crate::re::bs_container::BSContainerForEachResult;
use crate::re::bssimple_list::BSSimpleList;
use crate::re::bst_array::BSTArray;
use crate::re::bst_hash_map::BSTSet;
use crate::re::bst_smart_pointer::BSTSmartPointer;
use crate::re::extra_data_list::ExtraDataList;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::ni_av_object::NiAVObject;
use crate::re::ni_matrix3::NiMatrix3;
use crate::re::ni_point3::NiPoint3;
use crate::re::ni_smart_pointer::NiPointer;
use crate::re::ni_t_map::NiTMap;
use crate::re::{
    BGSEncounterZone, BGSLightingTemplate, BGSLocation, BSPortalGraph, BSTempEffectParticle, Color,
    ExtraNorthRotation, INTERIOR_DATA, NavMesh, ObjectRefHandle, TESFaction, TESForm, TESFullName,
    TESNPC, TESObjectLAND, TESObjectREFR, TESRegionList, TESWorldSpace, bhkWorld,
};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset};

crate::core_util::abstract_type! { type BGSWaterUpdateI; }

/// C++ `RE::BGSTerrainVisibilityData`
#[repr(C)]
pub struct BGSTerrainVisibilityData {
    pub vis_data: *mut c_void, // 00 - BSBitField<>*
}

const _: () = assert!(core::mem::size_of::<BGSTerrainVisibilityData>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BGSTerrainVisibilityData, vis_data) == 0x00);

/// C++ `RE::EXTERIOR_DATA::LandHideFlag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExteriorDataLandHideFlag {
    None = 0,
    Quad1 = 1 << 0,
    Quad2 = 1 << 1,
    Quad3 = 1 << 2,
    Quad4 = 1 << 3,
}

core_util::impl_enumset_type!(ExteriorDataLandHideFlag => u8);

/// C++ `RE::EXTERIOR_DATA`
#[repr(C)]
pub struct EXTERIOR_DATA {
    pub cell_x: i32,                                            // 00
    pub cell_y: i32,                                            // 04
    pub max_height_data: *mut i8,                               // 08
    pub lod_vis_data: *mut BGSTerrainVisibilityData,            // 10
    pub world_x: f32,                                           // 18
    pub world_y: f32,                                           // 1C
    pub land_hide_flags: EnumSet<ExteriorDataLandHideFlag, u8>, // 20
    pub pad21: u8,                                              // 21
    pub pad22: u16,                                             // 22
    pub pad24: u32,                                             // 24
}

const _: () = assert!(core::mem::size_of::<EXTERIOR_DATA>() == 0x28);
const _: () = assert!(core::mem::offset_of!(EXTERIOR_DATA, cell_x) == 0x00);
const _: () = assert!(core::mem::offset_of!(EXTERIOR_DATA, cell_y) == 0x04);
const _: () = assert!(core::mem::offset_of!(EXTERIOR_DATA, max_height_data) == 0x08);
const _: () = assert!(core::mem::offset_of!(EXTERIOR_DATA, lod_vis_data) == 0x10);
const _: () = assert!(core::mem::offset_of!(EXTERIOR_DATA, world_x) == 0x18);
const _: () = assert!(core::mem::offset_of!(EXTERIOR_DATA, world_y) == 0x1C);
const _: () = assert!(core::mem::offset_of!(EXTERIOR_DATA, land_hide_flags) == 0x20);
const _: () = assert!(core::mem::offset_of!(EXTERIOR_DATA, pad21) == 0x21);
const _: () = assert!(core::mem::offset_of!(EXTERIOR_DATA, pad22) == 0x22);
const _: () = assert!(core::mem::offset_of!(EXTERIOR_DATA, pad24) == 0x24);

/// C++ `RE::TESObjectCELL::NavMeshArray`
#[repr(C)]
pub struct NavMeshArray {
    pub nav_meshes: BSTArray<BSTSmartPointer<NavMesh>>, // 00
}

const _: () = assert!(core::mem::size_of::<NavMeshArray>() == 0x18);
const _: () = assert!(core::mem::offset_of!(NavMeshArray, nav_meshes) == 0x00);

/// C++ `RE::TESObjectCELL::LOADED_CELL_DATA`
#[repr(C)]
struct LOADED_CELL_DATA {
    portal_graph: NiPointer<BSPortalGraph>,          // 000
    cell3d: NiPointer<crate::re::NiNode>,            // 008
    light_marker_node: NiPointer<crate::re::NiNode>, // 010
    sound_marker_node: NiPointer<crate::re::NiNode>, // 018
    multi_bound_node: NiPointer<crate::re::NiNode>,  // 020
    unk028: u64,                                     // 028
    unk030: u64,                                     // 030
    unk038: u64,                                     // 038
    unk040: BSTArray<ObjectRefHandle>,               // 040
    flickering_lights: BSTArray<ObjectRefHandle>,    // 058
    emittance_source_ref_map: NiTMap<*mut TESForm, ObjectRefHandle>, // 070
    emittance_light_ref_map: NiTMap<ObjectRefHandle, *mut crate::re::NiNode>, // 090
    multibound_ref_map: NiTMap<ObjectRefHandle, NiPointer<crate::re::BSMultiBoundNode>>, // 0B0
    ref_multibound_map: NiTMap<*mut crate::re::BSMultiBoundNode, ObjectRefHandle>, // 0D0
    activating_refs: BSSimpleList<ObjectRefHandle>,  // 0F0
    unk100: BSSimpleList<ObjectRefHandle>,           // 100
    unk110: u64,                                     // 110
    unk118: BSTArray<*mut c_void>,                   // 118
    decal_refs: BSTArray<ObjectRefHandle>,           // 130
    sky_actors: BSTArray<ObjectRefHandle>,           // 148
    encounter_zone: *mut BGSEncounterZone,           // 160
    decals_queued: bool,                             // 168
    pad169: [u8; 0x03],                              // 169
    critical_queued_ref_count: i32,                  // 16C
    queued_ref_count: i32,                           // 170
    queued_distant_ref_count: i32,                   // 174
    unk178: i32,                                     // 178
    refs_fully_loaded: bool,                         // 17C
    pad17d: [u8; 0x03],                              // 17D
}

const _: () = assert!(core::mem::size_of::<LOADED_CELL_DATA>() == 0x180);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, portal_graph) == 0x000);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, cell3d) == 0x008);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, light_marker_node) == 0x010);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, sound_marker_node) == 0x018);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, multi_bound_node) == 0x020);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, unk040) == 0x040);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, flickering_lights) == 0x058);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, emittance_source_ref_map) == 0x070);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, emittance_light_ref_map) == 0x090);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, multibound_ref_map) == 0x0B0);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, ref_multibound_map) == 0x0D0);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, activating_refs) == 0x0F0);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, unk100) == 0x100);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, unk118) == 0x118);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, decal_refs) == 0x130);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, sky_actors) == 0x148);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, encounter_zone) == 0x160);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, decals_queued) == 0x168);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, critical_queued_ref_count) == 0x16C);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, queued_ref_count) == 0x170);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, queued_distant_ref_count) == 0x174);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, unk178) == 0x178);
const _: () = assert!(core::mem::offset_of!(LOADED_CELL_DATA, refs_fully_loaded) == 0x17C);

/// C++ `RE::TESObjectCELL::Flag`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESObjectCELLFlag {
    IsInteriorCell = 1 << 0,
    HasWater = 1 << 1,
    CanTravelFromHere = 1 << 2,
    NoLodWater = 1 << 3,
    HasTempData = 1 << 4,
    PublicArea = 1 << 5,
    HandChanged = 1 << 6,
    ShowSky = 1 << 7,
    UseSkyLighting = 1 << 8,
    WarnToLeave = 1 << 9,
}

core_util::impl_enumset_type!(TESObjectCELLFlag => u16);

/// C++ `RE::TESObjectCELL::CellState`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESObjectCELLState {
    Attached = 7,
}

core_util::impl_enumset_type!(TESObjectCELLState => u8);

bitflags! {
    /// C++ `RE::TESObjectCELL::ChangeFlags::ChangeFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESObjectCELLChangeFlags: u32 {
        const FLAGS = 1 << 1;
        const FULL_NAME = 1 << 2;
        const OWNERSHIP = 1 << 3;
        const EXTERIOR_SHORT = 1 << 28;
        const EXTERIOR_CHAR = 1 << 29;
        const DETACH_TIME = 1 << 30;
        const SEEND_DATA = 1u32 << 31;
    }
}

bitflags! {
    /// C++ `RE::TESObjectCELL::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESObjectCELLRecordFlags: u32 {
        const DELETED = 1 << 5;
        const PERSISTENT = 1 << 10;
        const IGNORED = 1 << 12;
        const OFF_LIMITS = 1 << 17;
        const COMPRESSED = 1 << 18;
        const CANT_WAIT = 1 << 19;
    }
}

/// C++ `RE::TESObjectCELL::CellData`
#[repr(C)]
#[derive(Clone, Copy)]
pub union TESObjectCELLCellData {
    pub exterior: *mut EXTERIOR_DATA,
    pub interior: *mut INTERIOR_DATA,
}

const _: () = assert!(core::mem::size_of::<TESObjectCELLCellData>() == 0x8);

#[repr(C)]
struct TESObjectCELLRuntimeData {
    cell_data: TESObjectCELLCellData,              // 00
    cell_land: *mut TESObjectLAND,                 // 08
    water_height: f32,                             // 10
    pad14: u32,                                    // 14
    nav_meshes: *mut NavMeshArray,                 // 18
    references: BSTSet<NiPointer<TESObjectREFR>>,  // 20
    unk0b0: *mut TESForm,                          // 50
    object_list: BSTArray<*mut TESObjectREFR>,     // 58
    unk0d0: BSTArray<*mut c_void>,                 // 70
    water_objects: BSTArray<*mut BGSWaterUpdateI>, // 88
    unk100: BSTArray<*mut c_void>,                 // A0
    spin_lock: BSSpinLock,                         // B8
    world_space: *mut TESWorldSpace,               // C0
    loaded_data: *mut LOADED_CELL_DATA,            // C8
    lighting_template: *mut BGSLightingTemplate,   // D0
    unk138: u64,                                   // D8
}

const _: () = assert!(core::mem::size_of::<TESObjectCELLRuntimeData>() == 0xE0);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, cell_data) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, cell_land) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, water_height) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, nav_meshes) == 0x18);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, references) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, unk0b0) == 0x50);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, object_list) == 0x58);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, unk0d0) == 0x70);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, water_objects) == 0x88);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, unk100) == 0xA0);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, spin_lock) == 0xB8);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, world_space) == 0xC0);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, loaded_data) == 0xC8);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, lighting_template) == 0xD0);
const _: () = assert!(core::mem::offset_of!(TESObjectCELLRuntimeData, unk138) == 0xD8);

/// C++ `RE::TESObjectCELL`
#[repr(C)]
pub struct TESObjectCELL {
    pub base: TESForm,                               // 000
    pub full_name: TESFullName,                      // 020
    pub grass_create_lock: BSSpinLock,               // 030
    pub grass_task_lock: BSSpinLock,                 // 038
    pub cell_flags: EnumSet<TESObjectCELLFlag, u16>, // 040
    pub cell_game_flags: u16,                        // 042
    pub cell_state: EnumSet<TESObjectCELLState, u8>, // 044
    pub auto_water_loaded: bool,                     // 045
    pub cell_detached: bool,                         // 046
    pub pad047: u8,                                  // 047
    pub extra_list: ExtraDataList,                   // 048
}

const _: () = assert!(core::mem::size_of::<TESObjectCELL>() == 0x58);
const _: () = assert!(core::mem::offset_of!(TESObjectCELL, full_name) == 0x020);
const _: () = assert!(core::mem::offset_of!(TESObjectCELL, grass_create_lock) == 0x030);
const _: () = assert!(core::mem::offset_of!(TESObjectCELL, grass_task_lock) == 0x038);
const _: () = assert!(core::mem::offset_of!(TESObjectCELL, cell_flags) == 0x040);
const _: () = assert!(core::mem::offset_of!(TESObjectCELL, cell_game_flags) == 0x042);
const _: () = assert!(core::mem::offset_of!(TESObjectCELL, cell_state) == 0x044);
const _: () = assert!(core::mem::offset_of!(TESObjectCELL, auto_water_loaded) == 0x045);
const _: () = assert!(core::mem::offset_of!(TESObjectCELL, cell_detached) == 0x046);
const _: () = assert!(core::mem::offset_of!(TESObjectCELL, pad047) == 0x047);
const _: () = assert!(core::mem::offset_of!(TESObjectCELL, extra_list) == 0x048);

impl RttiType for TESObjectCELL {
    const RTTI: VariantID = RTTI_TESObjectCELL;
}

impl FormCastable for TESObjectCELL {
    const TARGET_FORM_TYPE: FormType = FormType::Cell;
}

inherit!(TESObjectCELL : TESForm);
inherit!(TESObjectCELL => TESFullName, full_name);

impl TESObjectCELL {
    pub const RTTI: VariantID = RTTI_TESObjectCELL;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectCELL;
    pub const FORMTYPE: FormType = FormType::Cell;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x60, 0x68, 0x60);

    // override (TESForm)
    // void        ClearData() override;                                                                               // 05
    // bool        Load(TESFile* a_mod) override;                                                                      // 06
    // TESForm*    CreateDuplicateForm(bool a_createEditorID, NiTPointerMap<TESForm*, TESForm*>* a_copyMap) override; // 09
    // bool        FindInFileFast(TESFile* a_mod) override;                                                            // 0C
    // void        SaveGame(BGSSaveFormBuffer* a_buf) override;                                                        // 0E
    // void        LoadGame(BGSLoadFormBuffer* a_buf) override;                                                        // 0F
    // void        Revert(BGSLoadFormBuffer* a_buf) override;                                                          // 12
    // void        InitItemImpl() override;                                                                            // 13
    // void        GetFormDetailedString(char* a_buf, std::uint32_t a_bufLen) override;                                // 16
    // void        SetAltered(bool a_set) override;                                                                    // 24
    // bool        BelongsInGroup(FORM* a_form, bool a_allowParentGroups, bool a_currentOnly) override;                // 30
    // void        CreateGroupData(FORM* a_form, FORM_GROUP* a_group) override;                                        // 31
    // const char* GetFormEditorID() const override;                                                                   // 32
    // bool        SetFormEditorID(const char* a_str) override;                                                        // 33
    // bool        IsParentForm() override;                                                                            // 34
    // bool        IsFormTypeChild(FormType a_type) override;                                                          // 36

    crate::runtime_data_ptr_accessor! {
        fn runtime_data_ptr() -> TESObjectCELLRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    #[inline(always)]
    fn get_runtime_data(&self) -> &TESObjectCELLRuntimeData {
        unsafe { &*self.runtime_data_ptr() }
    }

    #[inline(always)]
    pub(crate) fn world_space_raw(&self) -> *mut TESWorldSpace {
        self.get_runtime_data().world_space
    }

    #[inline(always)]
    pub(crate) fn nav_meshes_raw(&self) -> *mut NavMeshArray {
        self.get_runtime_data().nav_meshes
    }

    crate::relocation_func! {
        pub fn get_bhk_world(&self) -> *mut bhkWorld => RelocationID::new(18536, 18995)
    }

    crate::relocation_func! {
        pub fn get_location(&self) -> *mut BGSLocation => RelocationID::new(18474, 18905)
    }

    crate::relocation_func! {
        pub fn get_region_list(&mut self, create_if_missing: bool) -> *mut TESRegionList => RelocationID::new(18540, 18999)
    }

    crate::relocation_func! {
        pub fn get_water_height(&mut self, pos: &NiPoint3, water_height: &mut f32) -> bool => RelocationID::new(18543, 19002)
    }

    crate::relocation_func! {
        pub fn place_particle_effect(
            &mut self,
            lifetime: f32,
            model_name: *const c_char,
            normal: &NiMatrix3,
            pos: &NiPoint3,
            scale: f32,
            flags: u32,
            target: *mut NiAVObject
        ) -> *mut BSTempEffectParticle => RelocationID::new(29219, 30072)
    }

    pub fn for_each_reference<F>(&self, mut callback: F)
    where
        F: FnMut(*mut TESObjectREFR) -> BSContainerForEachResult,
    {
        let runtime_data = unsafe { &mut *self.runtime_data_ptr() };
        let _locker = unsafe { BSSpinLockGuard::from_ptr(&mut runtime_data.spin_lock) };

        for reference in runtime_data.references.iter() {
            let reference = reference.get();
            if !reference.is_null() && callback(reference).is_stop() {
                break;
            }
        }
    }

    pub fn for_each_reference_in_range<F>(&self, origin: &NiPoint3, radius: f32, mut callback: F)
    where
        F: FnMut(*mut TESObjectREFR) -> BSContainerForEachResult,
    {
        let squared_radius = radius * radius;
        self.for_each_reference(|reference| {
            let distance = origin.get_squared_distance(Self::reference_position(reference));
            if distance <= squared_radius {
                callback(reference)
            } else {
                BSContainerForEachResult::Continue
            }
        });
    }

    #[inline]
    pub fn get_actor_owner(&mut self) -> *mut TESNPC {
        let owner = self.get_owner();
        if owner.is_null() || !unsafe { (*owner).is(FormType::NPC) } {
            core::ptr::null_mut()
        } else {
            owner.cast()
        }
    }

    #[inline]
    pub fn get_coordinates(&mut self) -> *mut EXTERIOR_DATA {
        if self.is_exterior_cell() {
            unsafe { self.get_runtime_data().cell_data.exterior }
        } else {
            core::ptr::null_mut()
        }
    }

    #[inline]
    pub fn get_faction_owner(&mut self) -> *mut TESFaction {
        let owner = self.get_owner();
        if owner.is_null() || !unsafe { (*owner).is(FormType::Faction) } {
            core::ptr::null_mut()
        } else {
            owner.cast()
        }
    }

    #[inline]
    pub fn get_lighting(&mut self) -> *mut INTERIOR_DATA {
        if self.is_interior_cell() {
            unsafe { self.get_runtime_data().cell_data.interior }
        } else {
            core::ptr::null_mut()
        }
    }

    pub fn get_north_rotation(&self) -> f32 {
        if self.is_exterior_cell() {
            unsafe { (*self.get_runtime_data().world_space).north_rotation }
        } else {
            let x_north = self.extra_list.get_by_type_typed::<ExtraNorthRotation>();
            if x_north.is_null() {
                0.0
            } else {
                unsafe { (*x_north).north_rot }
            }
        }
    }

    pub fn get_owner(&mut self) -> *mut TESForm {
        let runtime_data = self.get_runtime_data();
        let owner = self.extra_list.get_owner();
        if !owner.is_null() {
            return owner;
        }

        let zone = if !runtime_data.loaded_data.is_null() {
            unsafe { (*runtime_data.loaded_data).encounter_zone }
        } else {
            let zone = self.extra_list.get_encounter_zone();
            if zone.is_null() && self.is_exterior_cell() {
                let world_space = runtime_data.world_space;
                if world_space.is_null() {
                    core::ptr::null_mut()
                } else {
                    unsafe { (*world_space).encounter_zone }
                }
            } else {
                zone
            }
        };

        if zone.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*zone).data.zone_owner.cast() }
        }
    }

    pub fn get_exterior_water_height(&self) -> f32 {
        let runtime_data = self.get_runtime_data();
        if self.cell_flags.none(TESObjectCELLFlag::HasWater)
            || self.cell_flags.any(TESObjectCELLFlag::IsInteriorCell)
        {
            return -f32::INFINITY;
        }

        if runtime_data.water_height < 2_147_483_600.0 {
            return runtime_data.water_height;
        }

        let world_space = runtime_data.world_space;
        if world_space.is_null() {
            -f32::INFINITY
        } else {
            unsafe { (*world_space).get_default_water_height() }
        }
    }

    #[inline(always)]
    pub fn is_attached(&self) -> bool {
        self.cell_state == TESObjectCELLState::Attached
    }

    #[inline(always)]
    pub fn is_exterior_cell(&self) -> bool {
        !self.is_interior_cell()
    }

    #[inline(always)]
    pub fn is_interior_cell(&self) -> bool {
        self.cell_flags.all(TESObjectCELLFlag::IsInteriorCell)
    }

    #[inline(always)]
    pub fn set_actor_owner(&mut self, owner: *mut TESNPC) {
        self.set_owner(owner.cast());
    }

    #[inline(always)]
    pub fn set_faction_owner(&mut self, owner: *mut TESFaction) {
        self.set_owner(owner.cast());
    }

    pub fn set_fog_color(&mut self, near: Color, far: Color) {
        if self.uses_sky_lighting() {
            return;
        }

        let lighting = self.get_lighting();
        if !lighting.is_null() {
            unsafe {
                (*lighting).fog_color_near = near;
                (*lighting).fog_color_far = far;
            }
        }
    }

    pub fn set_fog_planes(&mut self, near: f32, far: f32) {
        if self.uses_sky_lighting() {
            return;
        }

        let lighting = self.get_lighting();
        if !lighting.is_null() {
            unsafe {
                (*lighting).fog_near = near;
                (*lighting).fog_far = far;
            }
        }
    }

    pub fn set_fog_power(&mut self, power: f32) {
        if self.uses_sky_lighting() {
            return;
        }

        let lighting = self.get_lighting();
        if !lighting.is_null() {
            unsafe {
                (*lighting).fog_power = power;
            }
        }
    }

    pub fn set_hand_changed(&mut self, changed: bool) {
        self.cell_flags
            .set_enabled(changed, TESObjectCELLFlag::HandChanged);
        self.base.add_change(TESObjectCELLChangeFlags::FLAGS.bits());
    }

    pub fn set_owner(&mut self, owner: *mut TESForm) {
        self.extra_list.set_owner(owner);
        self.base
            .add_change(TESObjectCELLChangeFlags::OWNERSHIP.bits());
    }

    pub fn set_public(&mut self, public: bool) {
        self.cell_flags
            .set_enabled(public, TESObjectCELLFlag::PublicArea);
        self.base.add_change(TESObjectCELLChangeFlags::FLAGS.bits());
    }

    #[inline(always)]
    pub fn uses_sky_lighting(&self) -> bool {
        self.cell_flags.all(TESObjectCELLFlag::UseSkyLighting)
    }

    #[inline(always)]
    fn reference_position(reference: *mut TESObjectREFR) -> NiPoint3 {
        unsafe { (*reference).get_position() }
    }
}
