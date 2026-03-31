#![allow(non_camel_case_types)]

use core::sync::atomic::Ordering;

use crate::offsets::offsets_rtti::RTTI_BSPathing;
use crate::offsets::offsets_vtable::VTABLE_BSPathing;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::{
    BSIntrusiveRefCounted, BSNavmesh, BSNavmeshInfo, BSPathingAvoidNode, BSPathingDoor,
    BSPathingLocation, BSTArray, FindTriangleForLocationFilterCheckDeltaZ, MovementMessage,
    NiPoint3,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSVirtualPathingNode`
#[repr(C)]
pub struct BSVirtualPathingNode {
    pub location: BSPathingLocation, // 00
    // TODO: SOURCE - replace this raw pointer stand-in with
    // `BSTSmartPointer<MovementMessage>` after the pathing movement-message
    // intrusive smart-pointer contract is translated.
    pub action_at_node: *mut MovementMessage, // 30
}

const _: () = assert!(core::mem::size_of::<BSVirtualPathingNode>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BSVirtualPathingNode, location) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSVirtualPathingNode, action_at_node) == 0x30);

/// C++ `RE::BSPathingNode`
#[repr(C)]
pub struct BSPathingNode {
    pub location: BSPathingLocation, // 00
    // TODO: SOURCE - replace this raw pointer stand-in with
    // `BSTSmartPointer<MovementMessage>` after the pathing movement-message
    // intrusive smart-pointer contract is translated.
    pub action_at_node: *mut MovementMessage, // 30
    pub tangent: NiPoint3,                    // 38
}

const _: () = assert!(core::mem::size_of::<BSPathingNode>() == 0x48);
const _: () = assert!(core::mem::offset_of!(BSPathingNode, location) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSPathingNode, action_at_node) == 0x30);
const _: () = assert!(core::mem::offset_of!(BSPathingNode, tangent) == 0x38);

/// C++ `RE::BSPathingSolution`
#[repr(C)]
pub struct BSPathingSolution {
    pub base: BSIntrusiveRefCounted, // 00
    pub virtual_pathing_nodes: crate::re::BSTSmallArray<
        BSVirtualPathingNode,
        { core::mem::size_of::<BSVirtualPathingNode>() * 2 },
    >, // 08
    pub first_loaded_virtual_node_index: i32, // 88
    pub last_loaded_virtual_node_index: i32, // 8C
    pub current_pathing_nodes: BSTArray<BSPathingNode>, // 90
    pub incomplete_path: bool,       // A8
    pub request_goal_unused: bool,   // A9
    pub pad0aa: u16,                 // AA
    pub pad0ac: u32,                 // AC
}

const _: () = assert!(core::mem::size_of::<BSPathingSolution>() == 0xB0);
const _: () = assert!(core::mem::offset_of!(BSPathingSolution, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSPathingSolution, virtual_pathing_nodes) == 0x08);
const _: () =
    assert!(core::mem::offset_of!(BSPathingSolution, first_loaded_virtual_node_index) == 0x88);
const _: () =
    assert!(core::mem::offset_of!(BSPathingSolution, last_loaded_virtual_node_index) == 0x8C);
const _: () = assert!(core::mem::offset_of!(BSPathingSolution, current_pathing_nodes) == 0x90);
const _: () = assert!(core::mem::offset_of!(BSPathingSolution, incomplete_path) == 0xA8);
const _: () = assert!(core::mem::offset_of!(BSPathingSolution, request_goal_unused) == 0xA9);

impl AsRef<BSPathingSolution> for BSPathingSolution {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSPathingSolution> for BSPathingSolution {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSTSmartPointerIntrusiveRefCountable for BSPathingSolution {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        // TODO: SOURCE - `BSPathingSolution` is intrusive-refcounted but the
        // CommonLib snapshot used here does not expose a dedicated virtual
        // delete surface for this nested type. Replace this compromise once a
        // source-backed deleter path is identified.
        unsafe { crate::ffi::commonlib_free(self as *const Self as *mut core::ffi::c_void) };
    }
}

/// C++ `RE::BSPathingStart`
#[repr(C)]
pub struct BSPathingStart {
    pub location: BSPathingLocation, // 00
    pub tangent: f32,                // 30
    pub pad34: u32,                  // 34
}

const _: () = assert!(core::mem::size_of::<BSPathingStart>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BSPathingStart, location) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSPathingStart, tangent) == 0x30);

/// C++ `RE::BSPathingGoal`
#[repr(C)]
pub struct BSPathingGoal {
    pub target_point: NiPoint3,       // 00
    pub target_angle_tolerance: f32,  // 0C
    pub location: BSPathingLocation,  // 10
    pub z_delta: f32,                 // 40
    pub goal_radius: f32,             // 44
    pub normalize_speed_at_goal: f32, // 48
    pub pad4c: u32,                   // 4C
}

const _: () = assert!(core::mem::size_of::<BSPathingGoal>() == 0x50);
const _: () = assert!(core::mem::offset_of!(BSPathingGoal, target_point) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSPathingGoal, target_angle_tolerance) == 0x0C);
const _: () = assert!(core::mem::offset_of!(BSPathingGoal, location) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSPathingGoal, z_delta) == 0x40);
const _: () = assert!(core::mem::offset_of!(BSPathingGoal, goal_radius) == 0x44);
const _: () = assert!(core::mem::offset_of!(BSPathingGoal, normalize_speed_at_goal) == 0x48);

/// C++ `RE::BSPathingRestrictions::BSPathingSearchAreaRestrictions`
#[repr(C)]
pub struct BSPathingSearchAreaRestrictions {
    pub center: NiPoint3, // 00
    pub radius: f32,      // 0C
}

const _: () = assert!(core::mem::size_of::<BSPathingSearchAreaRestrictions>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSPathingSearchAreaRestrictions, center) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSPathingSearchAreaRestrictions, radius) == 0x0C);

/// C++ `RE::BSPathingRestrictions`
#[repr(C)]
pub struct BSPathingRestrictions {
    // TODO: SOURCE - replace this raw pointer stand-in with
    // `NiPointer<BSTArray<BSPathingAvoidNode>>` after the real engine-side
    // ownership contract for this unusual pointee is translated; the current
    // Rust `NiPointer<T>` requires a `NiRef` pointee, which `BSTArray<_>` does
    // not currently provide.
    pub avoid_node_array: *mut BSTArray<BSPathingAvoidNode>, // 00
    pub area_restrictions: BSPathingSearchAreaRestrictions,  // 08
}

const _: () = assert!(core::mem::size_of::<BSPathingRestrictions>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BSPathingRestrictions, avoid_node_array) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSPathingRestrictions, area_restrictions) == 0x08);

/// C++ `RE::BSPathing`
#[repr(C)]
pub struct BSPathing {
    pub vtable: *const usize,                                              // 00
    pub default_triangle_filter: FindTriangleForLocationFilterCheckDeltaZ, // 08
}

const _: () = assert!(core::mem::size_of::<BSPathing>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BSPathing, default_triangle_filter) == 0x08);

impl RttiType for BSPathing {
    const RTTI: VariantID = RTTI_BSPathing;
}

impl AsRef<BSPathing> for BSPathing {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSPathing> for BSPathing {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSPathing {
    pub const RTTI: VariantID = RTTI_BSPathing;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPathing;

    // TODO: SOURCE - `BSPathing::GetPotentialNavMeshInfoForLocation` returns
    // `BSTArray<BSNavmeshInfo*>` by value. Keep this virtual slot omitted until
    // the repository has an ABI-safe Rust wrapper for MSVC non-trivial
    // `BSTArray` return values.

    crate::virtual_method! {
        pub const VFUNC_GET_ALL_LOADED_NAVMESHES1: usize = 0x02;
        pub fn get_all_loaded_navmeshes1(
            &mut self,
            ret_nav_meshes: &mut crate::re::BSTScrapHashMap<
                u32,
                crate::re::BSTSmartPointer<BSNavmesh>,
            >
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ALL_LOADED_NAVMESHES2: usize = 0x03;
        pub fn get_all_loaded_navmeshes2(
            &mut self,
            ret_nav_meshes: &mut crate::re::BSScrapArray<*mut BSNavmeshInfo>
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ALL_LOADED_NAVMESHES3: usize = 0x04;
        pub fn get_all_loaded_navmeshes3(
            &mut self,
            ret_nav_meshes: &mut BSTArray<*mut BSNavmeshInfo>
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ALL_LOADED_NAVMESHES4: usize = 0x05;
        pub fn get_all_loaded_navmeshes4(
            &mut self,
            ret_nav_meshes: &mut crate::re::BSScrapArray<crate::re::BSTSmartPointer<BSNavmesh>>
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ALL_LOADED_NAVMESHES5: usize = 0x06;
        pub fn get_all_loaded_navmeshes5(
            &mut self,
            ret_nav_meshes: &mut BSTArray<crate::re::BSTSmartPointer<BSNavmesh>>
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_SELECTED_DEBUG_REF: usize = 0x07;
        pub fn get_selected_debug_ref(&mut self) -> crate::re::FormID
    }

    crate::virtual_method! {
        pub const VFUNC_GET_PATHING_DOOR_FROM_COLLISION: usize = 0x08;
        // TODO: SOURCE - `BSPathing::GetPathingDoorFromCollision` takes
        // `BSTSmartPointer<BSPathingDoor>&`, but `BSPathingDoor` is still an
        // opaque stand-in without a source-backed intrusive smart-pointer
        // ownership layer. Keep the raw out-pointer until that dependency is
        // translated honestly.
        pub fn get_pathing_door_from_collision(
            &mut self,
            object: *mut crate::re::NiAVObject,
            ret_door: *mut *mut BSPathingDoor
        ) -> bool
    }
}

pub trait BSPathingExt {
    fn get_all_loaded_navmeshes1(
        &mut self,
        ret_nav_meshes: &mut crate::re::BSTScrapHashMap<u32, crate::re::BSTSmartPointer<BSNavmesh>>,
    ) -> bool;
    fn get_all_loaded_navmeshes2(
        &mut self,
        ret_nav_meshes: &mut crate::re::BSScrapArray<*mut BSNavmeshInfo>,
    ) -> bool;
    fn get_all_loaded_navmeshes3(
        &mut self,
        ret_nav_meshes: &mut BSTArray<*mut BSNavmeshInfo>,
    ) -> bool;
    fn get_all_loaded_navmeshes4(
        &mut self,
        ret_nav_meshes: &mut crate::re::BSScrapArray<crate::re::BSTSmartPointer<BSNavmesh>>,
    ) -> bool;
    fn get_all_loaded_navmeshes5(
        &mut self,
        ret_nav_meshes: &mut BSTArray<crate::re::BSTSmartPointer<BSNavmesh>>,
    ) -> bool;
    fn get_selected_debug_ref(&mut self) -> crate::re::FormID;
    fn get_pathing_door_from_collision(
        &mut self,
        object: *mut crate::re::NiAVObject,
        ret_door: *mut *mut BSPathingDoor,
    ) -> bool;
}

impl<T: AsMut<BSPathing>> BSPathingExt for T {
    #[inline(always)]
    fn get_all_loaded_navmeshes1(
        &mut self,
        ret_nav_meshes: &mut crate::re::BSTScrapHashMap<u32, crate::re::BSTSmartPointer<BSNavmesh>>,
    ) -> bool {
        BSPathing::get_all_loaded_navmeshes1(self.as_mut(), ret_nav_meshes)
    }

    #[inline(always)]
    fn get_all_loaded_navmeshes2(
        &mut self,
        ret_nav_meshes: &mut crate::re::BSScrapArray<*mut BSNavmeshInfo>,
    ) -> bool {
        BSPathing::get_all_loaded_navmeshes2(self.as_mut(), ret_nav_meshes)
    }

    #[inline(always)]
    fn get_all_loaded_navmeshes3(
        &mut self,
        ret_nav_meshes: &mut BSTArray<*mut BSNavmeshInfo>,
    ) -> bool {
        BSPathing::get_all_loaded_navmeshes3(self.as_mut(), ret_nav_meshes)
    }

    #[inline(always)]
    fn get_all_loaded_navmeshes4(
        &mut self,
        ret_nav_meshes: &mut crate::re::BSScrapArray<crate::re::BSTSmartPointer<BSNavmesh>>,
    ) -> bool {
        BSPathing::get_all_loaded_navmeshes4(self.as_mut(), ret_nav_meshes)
    }

    #[inline(always)]
    fn get_all_loaded_navmeshes5(
        &mut self,
        ret_nav_meshes: &mut BSTArray<crate::re::BSTSmartPointer<BSNavmesh>>,
    ) -> bool {
        BSPathing::get_all_loaded_navmeshes5(self.as_mut(), ret_nav_meshes)
    }

    #[inline(always)]
    fn get_selected_debug_ref(&mut self) -> crate::re::FormID {
        BSPathing::get_selected_debug_ref(self.as_mut())
    }

    #[inline(always)]
    fn get_pathing_door_from_collision(
        &mut self,
        object: *mut crate::re::NiAVObject,
        ret_door: *mut *mut BSPathingDoor,
    ) -> bool {
        BSPathing::get_pathing_door_from_collision(self.as_mut(), object, ret_door)
    }
}
