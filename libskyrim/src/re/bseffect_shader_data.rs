use core::ffi::c_void;

use crate::re::{
    BSFixedString, BSGraphicsDepthStencilDepthMode, BSGraphicsTextureAddressMode,
    BSIntrusiveRefCounted, NiAlphaProperty, NiColorA, NiPointer, NiTexture,
};

pub type BSEffectShaderDataNodeFilterFunction = unsafe extern "C" fn(*const BSFixedString) -> bool;

/// C++ `RE::BSEffectShaderData`
#[repr(C)]
pub struct BSEffectShaderData {
    pub base: BSIntrusiveRefCounted, // 00
    pub pad04: u32,                  // 04
    pub node_filter_function: Option<BSEffectShaderDataNodeFilterFunction>, // 08
    pub base_texture: NiPointer<NiTexture>, // 10
    pub palette_texture: NiPointer<NiTexture>, // 18
    pub block_out_texture: NiPointer<NiTexture>, // 20
    pub texture_clamp_mode: BSGraphicsTextureAddressMode, // 28
    pub fill_color: NiColorA,        // 2C
    pub rim_color: NiColorA,         // 3C
    pub base_fill_scale: f32,        // 4C
    pub base_fill_alpha: f32,        // 50
    pub base_rim_alpha: f32,         // 54
    pub u_offset: f32,               // 58
    pub v_offset: f32,               // 5C
    pub u_scale: f32,                // 60
    pub v_scale: f32,                // 64
    pub edge_exponent: f32,          // 68
    pub bound_diameter: f32,         // 6C
    pub src_blend: NiAlphaPropertyAlphaFunction, // 70
    pub dest_blend: NiAlphaPropertyAlphaFunction, // 74
    pub z_test_func: BSGraphicsDepthStencilDepthMode, // 78
    pub alpha_test_ref: i8,          // 7C
    pub grayscale_to_color: bool,    // 7D
    pub grayscale_to_alpha: bool,    // 7E
    pub ignore_texture_alpha: bool,  // 7F
    pub base_texture_projected_uvs: bool, // 80
    pub ignore_base_geom_tex_alpha: bool, // 81
    pub lighting: bool,              // 82
    pub alpha: bool,                 // 83
    pub pad84: u32,                  // 84
}

use crate::re::NiAlphaPropertyAlphaFunction;

const _: () = assert!(core::mem::size_of::<BSEffectShaderData>() == 0x88);
const _: () = assert!(core::mem::offset_of!(BSEffectShaderData, base_texture) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSEffectShaderData, src_blend) == 0x70);

impl Default for BSEffectShaderData {
    #[inline(always)]
    fn default() -> Self {
        Self {
            base: BSIntrusiveRefCounted::default(),
            pad04: 0,
            node_filter_function: None,
            base_texture: NiPointer::default(),
            palette_texture: NiPointer::default(),
            block_out_texture: NiPointer::default(),
            texture_clamp_mode: BSGraphicsTextureAddressMode::WrapSWrapT,
            fill_color: NiColorA::default(),
            rim_color: NiColorA::default(),
            base_fill_scale: 1.0,
            base_fill_alpha: 1.0,
            base_rim_alpha: 1.0,
            u_offset: 0.0,
            v_offset: 0.0,
            u_scale: 1.0,
            v_scale: 1.0,
            edge_exponent: 1.0,
            bound_diameter: 0.0,
            src_blend: NiAlphaPropertyAlphaFunction::SrcAlpha,
            dest_blend: NiAlphaPropertyAlphaFunction::InvSrcAlpha,
            z_test_func: BSGraphicsDepthStencilDepthMode::Test,
            alpha_test_ref: 0,
            grayscale_to_color: false,
            grayscale_to_alpha: false,
            ignore_texture_alpha: false,
            base_texture_projected_uvs: false,
            ignore_base_geom_tex_alpha: false,
            lighting: false,
            alpha: false,
            pad84: 0,
        }
    }
}

impl crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable for BSEffectShaderData {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let ptr = self as *const Self as *mut Self;
        unsafe {
            core::ptr::drop_in_place(ptr);
            crate::ffi::commonlib_free(ptr.cast::<c_void>());
        }
    }
}
