use core::ffi::c_void;

use crate::offsets::offsets_rtti::{RTTI_BSNavmeshInfoMap, RTTI_BSNavmeshInfoMap__IVisitor};
use crate::offsets::offsets_vtable::{VTABLE_BSNavmeshInfoMap, VTABLE_BSNavmeshInfoMap__IVisitor};
use crate::re::{
    BSCompressedNavmeshBounds, BSContainerForEachResult, BSNavmesh, BSPathingCell, BSTArray,
    BSTSmartPointer, FormID, NiPoint3,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSNavmeshInfo`
#[repr(C)]
pub struct BSNavmeshInfo {
    pub nav_mesh_id: FormID,       // 00
    pub approx_location: NiPoint3, // 04
    // TODO: SOURCE - keep the connected-doors array as an opaque raw pointer
    // until `BSPathingDoor` exposes the intrusive smart-pointer contract needed
    // by `BSTSmartPointer<BSPathingDoor>`; `BSNavmeshInfoMap.h` stores
    // `BSTSmartPointer<BSPathingDoor>*` here.
    pub connected_doors_array: *mut c_void,           // 10
    pub adjacent_meshes_array: *mut u32,              // 18
    pub connected_door_count: u16,                    // 20
    pub adjacent_mesh_count: u16,                     // 22
    pub preferred_percent: f32,                       // 24
    pub nav_mesh: *mut BSNavmesh,                     // 28
    pub bounds: *mut BSCompressedNavmeshBounds,       // 30
    pub pathing_cell: BSTSmartPointer<BSPathingCell>, // 38
    pub preferred_start_index: u16,                   // 40
    pub ui_flags: u8,                                 // 42
    pub pad43: u8,                                    // 43
    pub pad44: u32,                                   // 44
}

const _: () = assert!(core::mem::size_of::<BSNavmeshInfo>() == 0x48);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, nav_mesh_id) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, approx_location) == 0x04);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, connected_doors_array) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, adjacent_meshes_array) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, connected_door_count) == 0x20);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, adjacent_mesh_count) == 0x22);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, preferred_percent) == 0x24);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, nav_mesh) == 0x28);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, bounds) == 0x30);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, pathing_cell) == 0x38);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, preferred_start_index) == 0x40);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfo, ui_flags) == 0x42);

impl BSNavmeshInfo {
    #[inline(always)]
    pub fn adjacent_mesh_ids(&self) -> &[u32] {
        if self.adjacent_meshes_array.is_null() || self.adjacent_mesh_count == 0 {
            &[]
        } else {
            unsafe {
                core::slice::from_raw_parts(
                    self.adjacent_meshes_array,
                    self.adjacent_mesh_count as usize,
                )
            }
        }
    }
}

/// C++ `RE::BSNavmeshInfoMap::IVisitor`
#[repr(C)]
pub struct BSNavmeshInfoMapIVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSNavmeshInfoMapIVisitor>() == 0x08);

impl RttiType for BSNavmeshInfoMapIVisitor {
    const RTTI: VariantID = RTTI_BSNavmeshInfoMap__IVisitor;
}

impl BSNavmeshInfoMapIVisitor {
    pub const RTTI: VariantID = RTTI_BSNavmeshInfoMap__IVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSNavmeshInfoMap__IVisitor;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_VISIT: usize = 0x01;
        pub fn visit(&mut self, info: *const BSNavmeshInfo) -> BSContainerForEachResult
    }
}

/// C++ `RE::BSNavmeshInfoMap`
#[repr(C)]
pub struct BSNavmeshInfoMap {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSNavmeshInfoMap>() == 0x08);
const _: () = assert!(core::mem::offset_of!(BSNavmeshInfoMap, vtable) == 0x00);

impl RttiType for BSNavmeshInfoMap {
    const RTTI: VariantID = RTTI_BSNavmeshInfoMap;
}

impl AsRef<BSNavmeshInfoMap> for BSNavmeshInfoMap {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSNavmeshInfoMap> for BSNavmeshInfoMap {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSNavmeshInfoMap {
    pub const RTTI: VariantID = RTTI_BSNavmeshInfoMap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSNavmeshInfoMap;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_GET_NAV_MESH_INFO_FIX_ID: usize = 0x01;
        pub fn get_nav_mesh_info_fix_id(&mut self, id: u32) -> *mut BSNavmeshInfo
    }

    crate::virtual_method! {
        pub const VFUNC_GET_NAVMESH_INFO: usize = 0x02;
        pub fn get_navmesh_info(&mut self, id: u32) -> *mut BSNavmeshInfo
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ALL_NAV_MESH_INFO: usize = 0x03;
        pub fn get_all_nav_mesh_info(&mut self, results: &mut BSTArray<*mut BSNavmeshInfo>)
    }

    crate::virtual_method! {
        pub const VFUNC_BUILD_LIST_OF_CONNECTED_INFOS: usize = 0x04;
        pub fn build_list_of_connected_infos(
            &mut self,
            info: *const BSNavmeshInfo,
            results: &mut BSTArray<*mut BSNavmeshInfo>
        )
    }

    crate::virtual_method! {
        pub const VFUNC_FOR_EACH: usize = 0x05;
        pub fn for_each(&mut self, visitor: *mut BSNavmeshInfoMapIVisitor)
    }
}

pub trait BSNavmeshInfoMapExt {
    fn get_nav_mesh_info_fix_id(&mut self, id: u32) -> *mut BSNavmeshInfo;
    fn get_navmesh_info(&mut self, id: u32) -> *mut BSNavmeshInfo;
    fn get_all_nav_mesh_info(&mut self, results: &mut BSTArray<*mut BSNavmeshInfo>);
    fn build_list_of_connected_infos(
        &mut self,
        info: *const BSNavmeshInfo,
        results: &mut BSTArray<*mut BSNavmeshInfo>,
    );
    fn for_each(&mut self, visitor: *mut BSNavmeshInfoMapIVisitor);
}

impl<T: AsMut<BSNavmeshInfoMap>> BSNavmeshInfoMapExt for T {
    #[inline(always)]
    fn get_nav_mesh_info_fix_id(&mut self, id: u32) -> *mut BSNavmeshInfo {
        BSNavmeshInfoMap::get_nav_mesh_info_fix_id(self.as_mut(), id)
    }

    #[inline(always)]
    fn get_navmesh_info(&mut self, id: u32) -> *mut BSNavmeshInfo {
        BSNavmeshInfoMap::get_navmesh_info(self.as_mut(), id)
    }

    #[inline(always)]
    fn get_all_nav_mesh_info(&mut self, results: &mut BSTArray<*mut BSNavmeshInfo>) {
        BSNavmeshInfoMap::get_all_nav_mesh_info(self.as_mut(), results)
    }

    #[inline(always)]
    fn build_list_of_connected_infos(
        &mut self,
        info: *const BSNavmeshInfo,
        results: &mut BSTArray<*mut BSNavmeshInfo>,
    ) {
        BSNavmeshInfoMap::build_list_of_connected_infos(self.as_mut(), info, results)
    }

    #[inline(always)]
    fn for_each(&mut self, visitor: *mut BSNavmeshInfoMapIVisitor) {
        BSNavmeshInfoMap::for_each(self.as_mut(), visitor)
    }
}
