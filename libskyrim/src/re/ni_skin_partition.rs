use crate::offsets::offsets_nirtti::NiRTTI_NiSkinPartition;
use crate::offsets::offsets_rtti::RTTI_NiSkinPartition;
use crate::offsets::offsets_vtable::VTABLE_NiSkinPartition;
use crate::re::{BSGraphicsVertexDesc, NiObject, NiRef};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSGraphics::TriShape`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BSGraphicsTriShape {
    pub vertex_buffer: *mut core::ffi::c_void, // 00
    pub index_buffer: *mut core::ffi::c_void,  // 08
    pub vertex_desc: BSGraphicsVertexDesc,     // 10
    pub ref_count: u32,                        // 18
    pub pad1c: u32,                            // 1C
    pub raw_vertex_data: *mut u8,              // 20
    pub raw_index_data: *mut u16,              // 28
}

const _: () = assert!(core::mem::size_of::<BSGraphicsTriShape>() == 0x30);
const _: () = assert!(core::mem::offset_of!(BSGraphicsTriShape, vertex_desc) == 0x10);

crate::core_util::abstract_type! { pub type NiSkinPartition; }

impl RttiType for NiSkinPartition {
    const RTTI: VariantID = RTTI_NiSkinPartition;
}

impl NiRef for NiSkinPartition {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const Self as *const NiObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const Self as *const NiObject)).dec_ref() }
    }
}

impl NiSkinPartition {
    pub const RTTI: VariantID = RTTI_NiSkinPartition;
    pub const NI_RTTI: VariantID = NiRTTI_NiSkinPartition;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiSkinPartition;
}

// TODO: This is currently a pointer-compatible partial translation of `RE::NiSkinPartition`
// backed by `NiSkinPartition.h`. Replace the opaque parent type with its real layout if
// `Partition` data or virtual surface becomes needed by Rust callers.
