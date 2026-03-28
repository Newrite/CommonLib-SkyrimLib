use crate::offsets::offsets_nirtti::NiRTTI_NiCullingProcess;
use crate::offsets::offsets_rtti::RTTI_NiCullingProcess;
use crate::offsets::offsets_vtable::VTABLE_NiCullingProcess;
use crate::re::{
    BSGeometry, NiAVObject, NiCamera, NiFrustum, NiFrustumPlanes, NiRTTI, NiVisibleArray,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::NiCullingProcess`
#[repr(C)]
pub struct NiCullingProcess {
    pub vtable: *const usize,                // 00
    pub use_virtual_append: bool,            // 08
    pub pad009: u8,                          // 09
    pub pad00a: u16,                         // 0A
    pub pad00c: u32,                         // 0C
    pub visible_set: *mut NiVisibleArray,    // 10
    pub camera: *const NiCamera,             // 18
    pub frustum: NiFrustum,                  // 20
    pub planes: NiFrustumPlanes,             // 3C
    pub custom_cull_planes: NiFrustumPlanes, // AC
    pub camera_related_updates: bool,        // 11C
    pub update_accumulate_flag: bool,        // 11D
    pub ignore_preprocess: bool,             // 11E
    pub do_custom_cull_planes: bool,         // 11F
    pub enable_plane_optimization: bool,     // 120
    pub unk121: u8,                          // 121
    pub pad122: u16,                         // 122
    pub pad124: u32,                         // 124
}

const _: () = assert!(core::mem::size_of::<NiCullingProcess>() == 0x128);
const _: () = assert!(core::mem::offset_of!(NiCullingProcess, visible_set) == 0x10);
const _: () = assert!(core::mem::offset_of!(NiCullingProcess, frustum) == 0x20);
const _: () = assert!(core::mem::offset_of!(NiCullingProcess, planes) == 0x3C);
const _: () = assert!(core::mem::offset_of!(NiCullingProcess, custom_cull_planes) == 0xAC);

impl RttiType for NiCullingProcess {
    const RTTI: VariantID = RTTI_NiCullingProcess;
}

impl NiCullingProcess {
    pub const RTTI: VariantID = RTTI_NiCullingProcess;
    pub const NI_RTTI: VariantID = NiRTTI_NiCullingProcess;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiCullingProcess;

    // TODO: `NiCullingProcess.h` also declares the `GetAs*` virtual family at
    // slots `0x01..=0x14`, but this partial translation does not surface them
    // yet because most return types still lack matching source-backed Rust RE
    // files. End state: add pointer-level translations for those descendants
    // and expose the null-default virtuals here instead of leaving the vtable
    // gap undocumented.

    crate::virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x00;
        pub fn get_rtti() -> *const NiRTTI
    }

    crate::virtual_method! {
        pub const VFUNC_PROCESS1: usize = 0x16;
        pub fn process1(&mut self, object: *mut NiAVObject, arg2: i32)
    }

    crate::virtual_method! {
        pub const VFUNC_PROCESS2: usize = 0x17;
        pub fn process2(&mut self, camera: *const NiCamera, scene: *mut NiAVObject, visible_set: *mut NiVisibleArray)
    }

    crate::virtual_method! {
        pub const VFUNC_APPEND_VIRTUAL: usize = 0x18;
        pub fn append_virtual(&mut self, visible: &mut BSGeometry, arg2: i32)
    }

    crate::relocation_func! {
        pub fn set_frustum(&mut self, frustum: *const NiFrustum) => RelocationID::new(69699, 71081)
    }
}
