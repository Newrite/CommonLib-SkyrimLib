use core::ffi::c_void;

use crate::offsets::offsets_rtti::{RTTI_BSPortalGraph, RTTI_BSPortalGraphEntry};
use crate::offsets::offsets_vtable::{VTABLE_BSPortalGraph, VTABLE_BSPortalGraphEntry};
use crate::re::bs_core_types::FormID;
use crate::re::bs_multi_bound_room::BSMultiBoundRoom;
use crate::re::bs_occlusion_shape::BSOcclusionShape;
use crate::re::bs_portal::BSPortal;
use crate::re::bs_portal_shared_node::BSPortalSharedNode;
use crate::re::bst_array::BSTArray;
use crate::re::ni_av_object::NiAVObject;
use crate::re::ni_ref_object::{NiRef, NiRefObject};
use crate::re::ni_smart_pointer::NiPointer;
use crate::re::ni_t_pointer_list::NiTPointerList;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

core_util::abstract_type! { pub type BSPortalGraphEntry; }

impl RttiType for BSPortalGraphEntry {
    const RTTI: VariantID = RTTI_BSPortalGraphEntry;
}

impl BSPortalGraphEntry {
    pub const RTTI: VariantID = RTTI_BSPortalGraphEntry;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPortalGraphEntry;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ListEntry<T> {
    pub next: *mut ListEntry<T>, // 00
    pub prev: *mut ListEntry<T>, // 08
    pub value: *mut T,           // 10
}

const _: () = assert!(core::mem::size_of::<ListEntry<c_void>>() == 0x18);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct List<T> {
    pub first: *mut ListEntry<T>, // 00
    pub last: *mut ListEntry<T>,  // 08
    pub count: u32,               // 10
    pub pad14: u32,               // 14
}

const _: () = assert!(core::mem::size_of::<List<c_void>>() == 0x18);

#[repr(C)]
struct BSPortalGraphEntryPrefix {
    base: NiRefObject,                // 000
    portal_graph: *mut BSPortalGraph, // 010
    rest: [u8; 0x120],                // 018
    cell_id: FormID,                  // 138
}

const _: () = assert!(core::mem::offset_of!(BSPortalGraphEntryPrefix, portal_graph) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSPortalGraphEntryPrefix, cell_id) == 0x138);

/// C++ `RE::BSPortalGraph`
#[repr(C)]
pub struct BSPortalGraph {
    pub base: NiRefObject,                                       // 00
    pub occlusion_shapes: NiTPointerList<BSOcclusionShape>,      // 10
    pub portals: NiTPointerList<BSPortal>,                       // 28
    pub rooms: BSTArray<NiPointer<BSMultiBoundRoom>>,            // 40
    pub always_render_children: BSTArray<NiPointer<NiAVObject>>, // 58
    pub portal_shared_node: NiPointer<BSPortalSharedNode>,       // 70
    pub unk78: BSTArray<NiPointer<NiAVObject>>,                  // 78
    pub unk90: BSTArray<NiPointer<NiAVObject>>,                  // 90
    pub unk_a8: BSTArray<*mut c_void>,                           // A8
    pub cell_id: FormID,                                         // C0
    pub pad_c4: u32,                                             // C4
}

const _: () = assert!(core::mem::size_of::<BSPortalGraph>() == 0xC8);
const _: () = assert!(core::mem::offset_of!(BSPortalGraph, occlusion_shapes) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSPortalGraph, portals) == 0x28);
const _: () = assert!(core::mem::offset_of!(BSPortalGraph, rooms) == 0x40);
const _: () = assert!(core::mem::offset_of!(BSPortalGraph, cell_id) == 0xC0);

impl RttiType for BSPortalGraph {
    const RTTI: VariantID = RTTI_BSPortalGraph;
}

impl NiRef for BSPortalGraph {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(BSPortalGraph : NiRefObject);

impl BSPortalGraph {
    pub const RTTI: VariantID = RTTI_BSPortalGraph;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPortalGraph;

    pub fn is_compatible_entry(&self, entry: *const BSPortalGraphEntry) -> bool {
        if entry.is_null() {
            return false;
        }

        let entry = unsafe { &*(entry as *const BSPortalGraphEntryPrefix) };
        core::ptr::eq(self, entry.portal_graph) && self.cell_id == entry.cell_id
    }
}
