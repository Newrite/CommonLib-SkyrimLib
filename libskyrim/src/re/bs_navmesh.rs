#![allow(non_camel_case_types)]

use core::ffi::c_void;
use core::sync::atomic::Ordering;

use crate::core_util::{EnumSet, inherit};
use crate::offsets::offsets_rtti::RTTI_BSNavmesh;
use crate::offsets::offsets_vtable::VTABLE_BSNavmesh;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::{
    BSIntrusiveRefCounted, BSPathingCell, BSPathingDoor, BSTArray, BSTHashMap, FormID, NiPoint3,
    SimpleArray,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::EDGE_EXTRA_INFO_TYPE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSNavmeshEdgeExtraInfoType {
    Invalid = -1,
    Portal = 0,
    LedgeUp = 1,
    LedgeDown = 2,
    EnableDisablePortal = 3,
}

core_util::impl_enumset_type!(BSNavmeshEdgeExtraInfoType => u32);

/// C++ `RE::BSNavmeshVertex`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSNavmeshVertex {
    pub location: NiPoint3, // 00
}

const _: () = assert!(core::mem::size_of::<BSNavmeshVertex>() == 0x0C);
const _: () = assert!(core::mem::offset_of!(BSNavmeshVertex, location) == 0x00);

/// C++ `RE::BSNavmeshTriangle::TriangleFlag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSNavmeshTriangleFlag {
    None = 0,
    Edge0Link = 1 << 0,
    Edge1Link = 1 << 1,
    Edge2Link = 1 << 2,
    Deleted = 1 << 3,
    NoLargeCreatures = 1 << 4,
    Overlapping = 1 << 5,
    Preferred = 1 << 6,
}

core_util::impl_enumset_type!(BSNavmeshTriangleFlag => u16);

/// C++ `RE::BSNavmeshTriangle::TraversalFlag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSNavmeshTraversalFlag {
    None = 0,
    Edge0CoverValueQuarter = 1 << 0,
    Edge0CoverValueHalf = 1 << 1,
    Edge0CoverValueTri = 1 << 2,
    Edge0CoverValueFull = 1 << 3,
    Edge0Left = 1 << 4,
    Edge0Right = 1 << 5,
    Edge1CoverValueQuarter = 1 << 6,
    Edge1CoverValueHalf = 1 << 7,
    Edge1CoverValueTri = 1 << 8,
    Edge1CoverValueFull = 1 << 9,
    Edge1Left = 1 << 10,
    Edge1Right = 1 << 11,
}

core_util::impl_enumset_type!(BSNavmeshTraversalFlag => u16);

/// C++ `RE::BSNavmeshTriangle`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSNavmeshTriangle {
    pub vertices: [u16; 3],                                    // 00
    pub triangles: [u16; 3],                                   // 06
    pub triangle_flags: EnumSet<BSNavmeshTriangleFlag, u16>,   // 0C
    pub traversal_flags: EnumSet<BSNavmeshTraversalFlag, u16>, // 0E
}

const _: () = assert!(core::mem::size_of::<BSNavmeshTriangle>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSNavmeshTriangle, vertices) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSNavmeshTriangle, triangles) == 0x06);
const _: () = assert!(core::mem::offset_of!(BSNavmeshTriangle, triangle_flags) == 0x0C);
const _: () = assert!(core::mem::offset_of!(BSNavmeshTriangle, traversal_flags) == 0x0E);

/// C++ `RE::BSNavmeshTriangleEdgePortal`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSNavmeshTriangleEdgePortal {
    pub other_mesh_id: FormID, // 00
    pub triangle: u16,         // 04
    pub edge_index: i8,        // 06
    pub pad07: u8,             // 07
}

const _: () = assert!(core::mem::size_of::<BSNavmeshTriangleEdgePortal>() == 0x08);
const _: () = assert!(core::mem::offset_of!(BSNavmeshTriangleEdgePortal, other_mesh_id) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSNavmeshTriangleEdgePortal, triangle) == 0x04);
const _: () = assert!(core::mem::offset_of!(BSNavmeshTriangleEdgePortal, edge_index) == 0x06);

/// C++ `RE::BSNavmeshEdgeExtraInfo`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSNavmeshEdgeExtraInfo {
    pub type_: EnumSet<BSNavmeshEdgeExtraInfoType, u32>, // 00
    pub portal: BSNavmeshTriangleEdgePortal,             // 04
}

const _: () = assert!(core::mem::size_of::<BSNavmeshEdgeExtraInfo>() == 0x0C);
const _: () = assert!(core::mem::offset_of!(BSNavmeshEdgeExtraInfo, type_) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSNavmeshEdgeExtraInfo, portal) == 0x04);

/// C++ `RE::BSNavmeshTriangleDoorPortal`
#[repr(C)]
pub struct BSNavmeshTriangleDoorPortal {
    // TODO: SOURCE - replace this raw pointer stand-in with
    // `BSTSmartPointer<BSPathingDoor>` after `BSPathingDoor`'s intrusive
    // refcount/delete contract is translated; `BSNavmesh.h` stores an owning
    // smart pointer here.
    pub door: *mut BSPathingDoor,   // 00
    pub owning_triangle_index: u16, // 08
    pub pad0a: u16,                 // 0A
    pub pad0c: u32,                 // 0C
}

const _: () = assert!(core::mem::size_of::<BSNavmeshTriangleDoorPortal>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSNavmeshTriangleDoorPortal, door) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(BSNavmeshTriangleDoorPortal, owning_triangle_index) == 0x08);

/// C++ `RE::BSNavmeshClosedDoorInfo`
#[repr(C)]
pub struct BSNavmeshClosedDoorInfo {
    // TODO: SOURCE - replace this raw pointer stand-in with
    // `BSTSmartPointer<BSPathingDoor>` after `BSPathingDoor`'s intrusive
    // refcount/delete contract is translated; `BSNavmesh.h` stores an owning
    // smart pointer here.
    pub door: *mut BSPathingDoor, // 00
    pub triangle_index: u16,      // 08
    pub pad0a: u16,               // 0A
    pub pad0c: u32,               // 0C
}

const _: () = assert!(core::mem::size_of::<BSNavmeshClosedDoorInfo>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSNavmeshClosedDoorInfo, door) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSNavmeshClosedDoorInfo, triangle_index) == 0x08);

/// C++ `RE::BSNavmeshCoverEdge`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSNavmeshCoverEdge {
    pub vertices: [u16; 2], // 00
    pub data: u32,          // 04
}

const _: () = assert!(core::mem::size_of::<BSNavmeshCoverEdge>() == 0x08);
const _: () = assert!(core::mem::offset_of!(BSNavmeshCoverEdge, vertices) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSNavmeshCoverEdge, data) == 0x04);

/// C++ `RE::BSNavmeshGrid`
#[repr(C)]
pub struct BSNavmeshGrid {
    pub grid_size: u32,                        // 00
    pub column_section_len: f32,               // 04
    pub row_section_len: f32,                  // 08
    pub grid_bounds_min: NiPoint3,             // 0C
    pub grid_bounds_max: NiPoint3,             // 18
    pub pad24: u32,                            // 24
    pub grid_data: SimpleArray<BSTArray<u16>>, // 28
}

const _: () = assert!(core::mem::size_of::<BSNavmeshGrid>() == 0x30);
const _: () = assert!(core::mem::offset_of!(BSNavmeshGrid, grid_size) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSNavmeshGrid, grid_bounds_min) == 0x0C);
const _: () = assert!(core::mem::offset_of!(BSNavmeshGrid, grid_bounds_max) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSNavmeshGrid, grid_data) == 0x28);

crate::core_util::abstract_type! {
    pub type BSNavmeshObstacleData;
    pub type BSNavmeshObstacleUndoData;
}

/// C++ `RE::BSNavmesh`
#[repr(C)]
pub struct BSNavmesh {
    pub vtable: *const usize,                                // 000
    pub base: BSIntrusiveRefCounted,                         // 008
    pub pad00c: u32,                                         // 00C
    pub vertices: BSTArray<BSNavmeshVertex>,                 // 010
    pub triangles: BSTArray<BSNavmeshTriangle>,              // 028
    pub extra_edge_info: BSTArray<BSNavmeshEdgeExtraInfo>,   // 040
    pub door_portals: BSTArray<BSNavmeshTriangleDoorPortal>, // 058
    pub closed_doors: BSTArray<BSNavmeshClosedDoorInfo>,     // 070
    pub cover_array: BSTArray<BSNavmeshCoverEdge>,           // 088
    pub mesh_grid: BSNavmeshGrid,                            // 0A0
    // TODO: SOURCE - replace this raw-pointer array stand-in with
    // `BSTArray<NiPointer<BSNavmeshObstacleUndoData>>` after the source-backed
    // obstacle undo pointee family is translated and proven `NiPointer`-safe.
    pub obstacles: BSTArray<*mut BSNavmeshObstacleUndoData>, // 0D0
    // TODO: SOURCE - replace the raw pointer values in this map with
    // `NiPointer<BSNavmeshObstacleData>` after the obstacle data family is
    // translated with its actual intrusive ownership contract from
    // `BSNavmesh.h`.
    pub triangle_to_obstacle_map: *mut BSTHashMap<u16, *mut BSNavmeshObstacleData>, // 0E8
    pub unk0f0: BSTArray<*mut c_void>,                                              // 0F0
    pub parent_cell: crate::re::BSTSmartPointer<BSPathingCell>,                     // 108
    pub unk110: *mut c_void,                                                        // 110
}

const _: () = assert!(core::mem::size_of::<BSNavmesh>() == 0x118);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, base) == 0x008);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, vertices) == 0x010);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, triangles) == 0x028);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, extra_edge_info) == 0x040);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, door_portals) == 0x058);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, closed_doors) == 0x070);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, cover_array) == 0x088);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, mesh_grid) == 0x0A0);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, obstacles) == 0x0D0);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, triangle_to_obstacle_map) == 0x0E8);
const _: () = assert!(core::mem::offset_of!(BSNavmesh, parent_cell) == 0x108);

impl RttiType for BSNavmesh {
    const RTTI: VariantID = RTTI_BSNavmesh;
}

inherit!(BSNavmesh : BSIntrusiveRefCounted, base);

impl AsRef<BSNavmesh> for BSNavmesh {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSNavmesh> for BSNavmesh {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSTSmartPointerIntrusiveRefCountable for BSNavmesh {
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
        let func: extern "C" fn(*mut Self) =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0usize) };
        func(self as *const Self as *mut Self);
    }
}

impl BSNavmesh {
    pub const RTTI: VariantID = RTTI_BSNavmesh;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSNavmesh;

    #[inline(always)]
    pub fn vertices_slice(&self) -> &[BSNavmeshVertex] {
        unsafe { self.vertices.as_slice() }
    }

    #[inline(always)]
    pub fn triangles_slice(&self) -> &[BSNavmeshTriangle] {
        unsafe { self.triangles.as_slice() }
    }

    #[inline(always)]
    pub fn extra_edge_info_slice(&self) -> &[BSNavmeshEdgeExtraInfo] {
        unsafe { self.extra_edge_info.as_slice() }
    }

    #[inline(always)]
    pub fn door_portals_slice(&self) -> &[BSNavmeshTriangleDoorPortal] {
        unsafe { self.door_portals.as_slice() }
    }

    #[inline(always)]
    pub fn closed_doors_slice(&self) -> &[BSNavmeshClosedDoorInfo] {
        unsafe { self.closed_doors.as_slice() }
    }

    #[inline(always)]
    pub fn cover_edges_slice(&self) -> &[BSNavmeshCoverEdge] {
        unsafe { self.cover_array.as_slice() }
    }

    #[inline(always)]
    pub fn vertex_count(&self) -> usize {
        self.vertices_slice().len()
    }

    #[inline(always)]
    pub fn triangle_count(&self) -> usize {
        self.triangles_slice().len()
    }

    #[inline(always)]
    pub fn cover_edge_count(&self) -> usize {
        self.cover_edges_slice().len()
    }

    #[inline(always)]
    pub fn triangle(&self, index: usize) -> Option<&BSNavmeshTriangle> {
        self.triangles_slice().get(index)
    }

    #[inline(always)]
    pub fn vertex(&self, index: usize) -> Option<&BSNavmeshVertex> {
        self.vertices_slice().get(index)
    }

    #[inline(always)]
    pub fn triangle_vertex(&self, triangle_index: usize, vertex_index: usize) -> Option<NiPoint3> {
        let triangle = self.triangle(triangle_index)?;
        let vertex_index = *triangle.vertices.get(vertex_index)? as usize;
        self.vertex(vertex_index).map(|vertex| vertex.location)
    }

    #[inline(always)]
    pub fn triangle_vertices(&self, triangle_index: usize) -> Option<[NiPoint3; 3]> {
        Some([
            self.triangle_vertex(triangle_index, 0)?,
            self.triangle_vertex(triangle_index, 1)?,
            self.triangle_vertex(triangle_index, 2)?,
        ])
    }

    #[inline(always)]
    pub fn triangle_center(&self, triangle_index: usize) -> Option<NiPoint3> {
        let [a, b, c] = self.triangle_vertices(triangle_index)?;
        Some(NiPoint3::new(
            (a.x + b.x + c.x) / 3.0,
            (a.y + b.y + c.y) / 3.0,
            (a.z + b.z + c.z) / 3.0,
        ))
    }

    #[inline(always)]
    pub fn grid_bucket(&self, index: usize) -> Option<&BSTArray<u16>> {
        self.mesh_grid.grid_data.as_slice().get(index)
    }

    crate::virtual_method! {
        pub const VFUNC_Q_NAVMESH_ID: usize = 0x01;
        pub fn q_navmesh_id(&mut self) -> u32
    }
}

pub trait BSNavmeshExt {
    fn q_navmesh_id(&mut self) -> u32;
    fn vertices_slice(&self) -> &[BSNavmeshVertex];
    fn triangles_slice(&self) -> &[BSNavmeshTriangle];
    fn extra_edge_info_slice(&self) -> &[BSNavmeshEdgeExtraInfo];
    fn door_portals_slice(&self) -> &[BSNavmeshTriangleDoorPortal];
    fn closed_doors_slice(&self) -> &[BSNavmeshClosedDoorInfo];
    fn cover_edges_slice(&self) -> &[BSNavmeshCoverEdge];
    fn vertex_count(&self) -> usize;
    fn triangle_count(&self) -> usize;
    fn cover_edge_count(&self) -> usize;
    fn triangle(&self, index: usize) -> Option<&BSNavmeshTriangle>;
    fn vertex(&self, index: usize) -> Option<&BSNavmeshVertex>;
    fn triangle_vertex(&self, triangle_index: usize, vertex_index: usize) -> Option<NiPoint3>;
    fn triangle_vertices(&self, triangle_index: usize) -> Option<[NiPoint3; 3]>;
    fn triangle_center(&self, triangle_index: usize) -> Option<NiPoint3>;
    fn grid_bucket(&self, index: usize) -> Option<&BSTArray<u16>>;
}

impl<T> BSNavmeshExt for T
where
    T: AsRef<BSNavmesh> + AsMut<BSNavmesh>,
{
    #[inline(always)]
    fn q_navmesh_id(&mut self) -> u32 {
        BSNavmesh::q_navmesh_id(self.as_mut())
    }

    #[inline(always)]
    fn vertices_slice(&self) -> &[BSNavmeshVertex] {
        BSNavmesh::vertices_slice(self.as_ref())
    }

    #[inline(always)]
    fn triangles_slice(&self) -> &[BSNavmeshTriangle] {
        BSNavmesh::triangles_slice(self.as_ref())
    }

    #[inline(always)]
    fn extra_edge_info_slice(&self) -> &[BSNavmeshEdgeExtraInfo] {
        BSNavmesh::extra_edge_info_slice(self.as_ref())
    }

    #[inline(always)]
    fn door_portals_slice(&self) -> &[BSNavmeshTriangleDoorPortal] {
        BSNavmesh::door_portals_slice(self.as_ref())
    }

    #[inline(always)]
    fn closed_doors_slice(&self) -> &[BSNavmeshClosedDoorInfo] {
        BSNavmesh::closed_doors_slice(self.as_ref())
    }

    #[inline(always)]
    fn cover_edges_slice(&self) -> &[BSNavmeshCoverEdge] {
        BSNavmesh::cover_edges_slice(self.as_ref())
    }

    #[inline(always)]
    fn vertex_count(&self) -> usize {
        BSNavmesh::vertex_count(self.as_ref())
    }

    #[inline(always)]
    fn triangle_count(&self) -> usize {
        BSNavmesh::triangle_count(self.as_ref())
    }

    #[inline(always)]
    fn cover_edge_count(&self) -> usize {
        BSNavmesh::cover_edge_count(self.as_ref())
    }

    #[inline(always)]
    fn triangle(&self, index: usize) -> Option<&BSNavmeshTriangle> {
        BSNavmesh::triangle(self.as_ref(), index)
    }

    #[inline(always)]
    fn vertex(&self, index: usize) -> Option<&BSNavmeshVertex> {
        BSNavmesh::vertex(self.as_ref(), index)
    }

    #[inline(always)]
    fn triangle_vertex(&self, triangle_index: usize, vertex_index: usize) -> Option<NiPoint3> {
        BSNavmesh::triangle_vertex(self.as_ref(), triangle_index, vertex_index)
    }

    #[inline(always)]
    fn triangle_vertices(&self, triangle_index: usize) -> Option<[NiPoint3; 3]> {
        BSNavmesh::triangle_vertices(self.as_ref(), triangle_index)
    }

    #[inline(always)]
    fn triangle_center(&self, triangle_index: usize) -> Option<NiPoint3> {
        BSNavmesh::triangle_center(self.as_ref(), triangle_index)
    }

    #[inline(always)]
    fn grid_bucket(&self, index: usize) -> Option<&BSTArray<u16>> {
        BSNavmesh::grid_bucket(self.as_ref(), index)
    }
}
