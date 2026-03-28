use crate::re::{BSGeometry, BSLight, BSShader, BSShaderProperty};
use crate::relocation::RelocationID;

/// C++ `RE::BSRenderPass::LODMode`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BSRenderPassLODMode {
    pub value: u8, // 00
}

const _: () = assert!(core::mem::size_of::<BSRenderPassLODMode>() == 0x1);

impl BSRenderPassLODMode {
    #[inline(always)]
    pub const fn index(self) -> u8 {
        self.value & 0x7F
    }

    #[inline(always)]
    pub const fn single_level(self) -> bool {
        (self.value & 0x80) != 0
    }
}

/// C++ `RE::BSRenderPass`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BSRenderPass {
    pub shader: *mut BSShader,                  // 00
    pub shader_property: *mut BSShaderProperty, // 08
    pub geometry: *mut BSGeometry,              // 10
    pub pass_enum: u32,                         // 18
    pub accumulation_hint: u8,                  // 1C
    pub extra_param: u8,                        // 1D
    pub lod_mode: BSRenderPassLODMode,          // 1E
    pub num_lights: u8,                         // 1F
    pub num_shadow_lights: u8,                  // 20
    pub unk21: u8,                              // 21
    pub unk24: u32,                             // 24
    pub next: *mut BSRenderPass,                // 28
    pub pass_group_next: *mut BSRenderPass,     // 30
    pub scene_lights: *mut *mut BSLight,        // 38
    pub cache_pool_id: u32,                     // 40
    pub pad44: u32,                             // 44
}

const _: () = assert!(core::mem::size_of::<BSRenderPass>() == 0x48);
const _: () = assert!(core::mem::offset_of!(BSRenderPass, next) == 0x28);

impl BSRenderPass {
    crate::relocation_func! {
        pub fn clear_render_pass(&mut self) => RelocationID::new(100718, 107498)
    }
}
