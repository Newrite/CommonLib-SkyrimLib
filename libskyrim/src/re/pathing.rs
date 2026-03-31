use core_util::inherit;

use crate::offsets::offsets_rtti::{RTTI_BSPathingCellManager, RTTI_MovementManager, RTTI_Pathing};
use crate::offsets::offsets_vtable::{VTABLE_BSPathingCellManager, VTABLE_Pathing};
use crate::re::{
    BSNavmesh, BSNavmeshInfo, BSPathing, BSPathingCell, BSPathingLocation, BSPathingSolution,
    BSScrapArray, BSSpinLock, BSTArray, BSTEventSink, BSTHashMap, BSTSingletonSDM, BSTSmartPointer,
    BSTTuple, CellAttachDetachEvent, FindTriangleForLocationFilter, NiPoint3, PathingCell,
    PathingCellInfo, TESObjectCELL, TESWorldSpace,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

crate::core_util::abstract_type! { pub type IMovementPlayIdleResult; }
crate::core_util::abstract_type! { pub type MovementArbitrationAverageFloat; }
crate::core_util::abstract_type! { pub type MovementArbitrationMaxWeightParameters; }
crate::core_util::abstract_type! { pub type MovementArbitrationMaxWeightPoint; }
crate::core_util::abstract_type! { pub type MovementArbitrationVectorAdd; }
crate::core_util::abstract_type! { pub type MovementMessageBlocked; }

/// C++ `RE::BSPathingCellManager`
///
/// The empty `BSTSingletonExplicit<BSPathingCellManager>` base overlaps the
/// primary vptr and is not modeled as separate storage here.
#[repr(C)]
pub struct BSPathingCellManager {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSPathingCellManager>() == 0x08);
const _: () = assert!(core::mem::offset_of!(BSPathingCellManager, vtable) == 0x00);

impl RttiType for BSPathingCellManager {
    const RTTI: VariantID = RTTI_BSPathingCellManager;
}

impl AsRef<BSPathingCellManager> for BSPathingCellManager {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSPathingCellManager> for BSPathingCellManager {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSPathingCellManager {
    pub const RTTI: VariantID = RTTI_BSPathingCellManager;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPathingCellManager;

    crate::virtual_method! {
        pub const VFUNC_GET_EXTERIOR_CELL_WIDTH: usize = 0x00;
        pub fn get_exterior_cell_width(&self) -> f32
    }

    crate::virtual_method! {
        pub const VFUNC_GET_PATHING_CELL: usize = 0x01;
        pub fn get_pathing_cell(
            &mut self,
            location: &NiPoint3,
            hint_cell: &BSTSmartPointer<BSPathingCell>,
            cell_out: &mut BSTSmartPointer<BSPathingCell>
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_IS_CLOSE_TO: usize = 0x02;
        pub fn is_close_to(
            &self,
            loc1: &NiPoint3,
            cell1: &BSTSmartPointer<BSPathingCell>,
            loc2: &NiPoint3,
            cell2: &BSTSmartPointer<BSPathingCell>,
            tolerance: f32
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_FIND_POTENTIAL_NAVMESHES_FOR_LOCATION1: usize = 0x03;
        pub fn find_potential_navmeshes_for_location1(
            &mut self,
            location: &mut BSPathingLocation,
            radius: f32,
            nav_meshes_out: &mut BSScrapArray<BSTSmartPointer<BSNavmesh>>
        )
    }

    crate::virtual_method! {
        pub const VFUNC_FIND_POTENTIAL_NAVMESHES_FOR_LOCATION2: usize = 0x04;
        pub fn find_potential_navmeshes_for_location2(
            &mut self,
            location: &mut BSPathingLocation,
            radius: f32,
            nav_meshes_out: &mut BSTArray<BSTSmartPointer<BSNavmesh>>
        )
    }

    crate::virtual_method! {
        pub const VFUNC_FIND_CONNECTED_NAVMESHES_FOR_LOCATION3: usize = 0x05;
        pub fn find_connected_navmeshes_for_location3(
            &mut self,
            location: &mut BSPathingLocation,
            radius: f32,
            nav_meshes_out: &mut BSScrapArray<*const BSNavmeshInfo>
        )
    }

    crate::virtual_method! {
        pub const VFUNC_FIND_CONNECTED_NAVMESHES_FOR_LOCATION4: usize = 0x06;
        pub fn find_connected_navmeshes_for_location4(
            &mut self,
            location: &mut BSPathingLocation,
            radius: f32,
            nav_meshes_out: &mut BSTArray<*const BSNavmeshInfo>
        )
    }

    crate::virtual_method! {
        pub const VFUNC_FIND_CONNECTED_NAVMESHES_FOR_LOCATION5: usize = 0x07;
        pub fn find_connected_navmeshes_for_location5(
            &mut self,
            location: &mut BSPathingLocation,
            radius: f32,
            nav_meshes_out: &mut BSScrapArray<BSTSmartPointer<BSNavmesh>>
        )
    }

    crate::virtual_method! {
        pub const VFUNC_FIND_CONNECTED_NAVMESHES_FOR_LOCATION6: usize = 0x08;
        pub fn find_connected_navmeshes_for_location6(
            &mut self,
            location: &mut BSPathingLocation,
            radius: f32,
            nav_meshes_out: &mut BSTArray<BSTSmartPointer<BSNavmesh>>
        )
    }

    crate::virtual_method! {
        pub const VFUNC_GET_WATER_HEIGHT: usize = 0x09;
        pub fn get_water_height(&self, location: *const BSPathingLocation) -> f32
    }

    // 0A ~BSPathingCellManager
}

pub trait BSPathingCellManagerExt {
    fn get_exterior_cell_width(&self) -> f32;
    fn get_pathing_cell(
        &mut self,
        location: &NiPoint3,
        hint_cell: &BSTSmartPointer<BSPathingCell>,
        cell_out: &mut BSTSmartPointer<BSPathingCell>,
    ) -> bool;
    fn is_close_to(
        &self,
        loc1: &NiPoint3,
        cell1: &BSTSmartPointer<BSPathingCell>,
        loc2: &NiPoint3,
        cell2: &BSTSmartPointer<BSPathingCell>,
        tolerance: f32,
    ) -> bool;
    fn find_potential_navmeshes_for_location1(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSScrapArray<BSTSmartPointer<BSNavmesh>>,
    );
    fn find_potential_navmeshes_for_location2(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSTArray<BSTSmartPointer<BSNavmesh>>,
    );
    fn find_connected_navmeshes_for_location3(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSScrapArray<*const BSNavmeshInfo>,
    );
    fn find_connected_navmeshes_for_location4(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSTArray<*const BSNavmeshInfo>,
    );
    fn find_connected_navmeshes_for_location5(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSScrapArray<BSTSmartPointer<BSNavmesh>>,
    );
    fn find_connected_navmeshes_for_location6(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSTArray<BSTSmartPointer<BSNavmesh>>,
    );
    fn get_water_height(&self, location: *const BSPathingLocation) -> f32;
}

impl<T> BSPathingCellManagerExt for T
where
    T: AsRef<BSPathingCellManager> + AsMut<BSPathingCellManager>,
{
    #[inline(always)]
    fn get_exterior_cell_width(&self) -> f32 {
        BSPathingCellManager::get_exterior_cell_width(self.as_ref())
    }

    #[inline(always)]
    fn get_pathing_cell(
        &mut self,
        location: &NiPoint3,
        hint_cell: &BSTSmartPointer<BSPathingCell>,
        cell_out: &mut BSTSmartPointer<BSPathingCell>,
    ) -> bool {
        BSPathingCellManager::get_pathing_cell(self.as_mut(), location, hint_cell, cell_out)
    }

    #[inline(always)]
    fn is_close_to(
        &self,
        loc1: &NiPoint3,
        cell1: &BSTSmartPointer<BSPathingCell>,
        loc2: &NiPoint3,
        cell2: &BSTSmartPointer<BSPathingCell>,
        tolerance: f32,
    ) -> bool {
        BSPathingCellManager::is_close_to(self.as_ref(), loc1, cell1, loc2, cell2, tolerance)
    }

    #[inline(always)]
    fn find_potential_navmeshes_for_location1(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSScrapArray<BSTSmartPointer<BSNavmesh>>,
    ) {
        BSPathingCellManager::find_potential_navmeshes_for_location1(
            self.as_mut(),
            location,
            radius,
            nav_meshes_out,
        )
    }

    #[inline(always)]
    fn find_potential_navmeshes_for_location2(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSTArray<BSTSmartPointer<BSNavmesh>>,
    ) {
        BSPathingCellManager::find_potential_navmeshes_for_location2(
            self.as_mut(),
            location,
            radius,
            nav_meshes_out,
        )
    }

    #[inline(always)]
    fn find_connected_navmeshes_for_location3(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSScrapArray<*const BSNavmeshInfo>,
    ) {
        BSPathingCellManager::find_connected_navmeshes_for_location3(
            self.as_mut(),
            location,
            radius,
            nav_meshes_out,
        )
    }

    #[inline(always)]
    fn find_connected_navmeshes_for_location4(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSTArray<*const BSNavmeshInfo>,
    ) {
        BSPathingCellManager::find_connected_navmeshes_for_location4(
            self.as_mut(),
            location,
            radius,
            nav_meshes_out,
        )
    }

    #[inline(always)]
    fn find_connected_navmeshes_for_location5(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSScrapArray<BSTSmartPointer<BSNavmesh>>,
    ) {
        BSPathingCellManager::find_connected_navmeshes_for_location5(
            self.as_mut(),
            location,
            radius,
            nav_meshes_out,
        )
    }

    #[inline(always)]
    fn find_connected_navmeshes_for_location6(
        &mut self,
        location: &mut BSPathingLocation,
        radius: f32,
        nav_meshes_out: &mut BSTArray<BSTSmartPointer<BSNavmesh>>,
    ) {
        BSPathingCellManager::find_connected_navmeshes_for_location6(
            self.as_mut(),
            location,
            radius,
            nav_meshes_out,
        )
    }

    #[inline(always)]
    fn get_water_height(&self, location: *const BSPathingLocation) -> f32 {
        BSPathingCellManager::get_water_height(self.as_ref(), location)
    }
}

/// C++ `RE::MovementManager`
///
/// The empty `BSTSingletonExplicit<MovementManager>` base is not modeled as
/// separate storage here; the source-backed data surface begins at offset `0x00`.
#[repr(C)]
pub struct MovementManager {
    // TODO: SOURCE - `Pathing.h` stores these as `BSTSmartPointer<...>` fields.
    // Keep raw pointers until the intrusive smart-pointer contracts and delete
    // paths for the movement arbitration, blocked-message, and idle-result
    // types are translated honestly.
    pub vector_add: *mut MovementArbitrationVectorAdd, // 00
    pub float_average: *mut MovementArbitrationAverageFloat, // 08
    pub vector_max_weight: *mut MovementArbitrationMaxWeightPoint, // 10
    pub parameters_max_weight: *mut MovementArbitrationMaxWeightParameters, // 18
    pub movement_blocked_message: *mut MovementMessageBlocked, // 20
    pub null_play_idle_result: *mut IMovementPlayIdleResult, // 28
    pub empty_solution: BSTSmartPointer<BSPathingSolution>, // 30
}

const _: () = assert!(core::mem::size_of::<MovementManager>() == 0x38);
const _: () = assert!(core::mem::offset_of!(MovementManager, vector_add) == 0x00);
const _: () = assert!(core::mem::offset_of!(MovementManager, float_average) == 0x08);
const _: () = assert!(core::mem::offset_of!(MovementManager, vector_max_weight) == 0x10);
const _: () = assert!(core::mem::offset_of!(MovementManager, parameters_max_weight) == 0x18);
const _: () = assert!(core::mem::offset_of!(MovementManager, movement_blocked_message) == 0x20);
const _: () = assert!(core::mem::offset_of!(MovementManager, null_play_idle_result) == 0x28);
const _: () = assert!(core::mem::offset_of!(MovementManager, empty_solution) == 0x30);

impl AsRef<MovementManager> for MovementManager {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<MovementManager> for MovementManager {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl RttiType for MovementManager {
    const RTTI: VariantID = RTTI_MovementManager;
}

impl MovementManager {
    pub const RTTI: VariantID = RTTI_MovementManager;
}

/// C++ `RE::Pathing`
#[repr(C)]
pub struct Pathing {
    pub base: BSPathing,                                                         // 00
    pub cell_manager: BSPathingCellManager,                                      // 18
    pub cell_attach_detach_event_sink: BSTEventSink<CellAttachDetachEvent>,      // 20
    pub singleton: BSTSingletonSDM<Pathing>,                                     // 28
    pub pad29: [u8; 7],                                                          // 29
    pub movement_manager: MovementManager,                                       // 30
    pub cell_cache_lock: BSSpinLock,                                             // 68
    pub loaded_cells: BSTHashMap<PathingCellInfo, BSTSmartPointer<PathingCell>>, // 70
    pub recent_cells: BSTArray<BSTTuple<u64, BSTSmartPointer<PathingCell>>>,     // A0
}

const _: () = assert!(core::mem::size_of::<Pathing>() == 0xB8);
const _: () = assert!(core::mem::offset_of!(Pathing, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(Pathing, cell_manager) == 0x18);
const _: () = assert!(core::mem::offset_of!(Pathing, cell_attach_detach_event_sink) == 0x20);
const _: () = assert!(core::mem::offset_of!(Pathing, singleton) == 0x28);
const _: () = assert!(core::mem::offset_of!(Pathing, movement_manager) == 0x30);
const _: () = assert!(core::mem::offset_of!(Pathing, cell_cache_lock) == 0x68);
const _: () = assert!(core::mem::offset_of!(Pathing, loaded_cells) == 0x70);
const _: () = assert!(core::mem::offset_of!(Pathing, recent_cells) == 0xA0);

impl RttiType for Pathing {
    const RTTI: VariantID = RTTI_Pathing;
}

impl AsRef<Pathing> for Pathing {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Pathing> for Pathing {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(Pathing : BSPathing, base);
inherit!(Pathing => BSPathingCellManager, cell_manager);
inherit!(
    Pathing => BSTEventSink<CellAttachDetachEvent>,
    cell_attach_detach_event_sink
);
inherit!(Pathing => BSTSingletonSDM<Pathing>, singleton);
inherit!(Pathing => MovementManager, movement_manager);

impl Pathing {
    pub const RTTI: VariantID = RTTI_Pathing;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Pathing;

    // override (BSTEventSink<CellAttachDetachEvent>)
    // BSEventNotifyControl ProcessEvent(...) override;  // 01

    crate::relocation_variable! {
        fn singleton_ptr() -> *mut Pathing => RelocationID::new(514893, 401037), is_ptr
    }

    crate::relocation_func! {
        pub fn get_pathing_cell(
            &mut self,
            location: &NiPoint3,
            cell: *mut TESObjectCELL,
            world_space: *mut TESWorldSpace,
            cell_out: &mut BSTSmartPointer<BSPathingCell>
        ) -> bool => RelocationID::new(29866, 30682)
    }

    crate::relocation_func! {
        pub fn find_closest_point_on_navmesh_with_filter(
            &mut self,
            cell: &BSTSmartPointer<BSPathingCell>,
            location: &NiPoint3,
            filter: &mut FindTriangleForLocationFilter,
            point_out: &mut NiPoint3
        ) -> bool => RelocationID::new(29832, 30648)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut Pathing {
        Self::singleton_ptr()
    }

    #[inline(always)]
    pub fn find_closest_point_on_navmesh(
        &mut self,
        cell: &BSTSmartPointer<BSPathingCell>,
        location: &NiPoint3,
        point_out: &mut NiPoint3,
    ) -> bool {
        let filter =
            self.base.default_triangle_filter.as_mut() as *mut FindTriangleForLocationFilter;
        unsafe {
            Self::find_closest_point_on_navmesh_with_filter(
                self,
                cell,
                location,
                &mut *filter,
                point_out,
            )
        }
    }
}
