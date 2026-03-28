use crate::offsets::offsets_rtti::RTTI_BSShader;
use crate::offsets::offsets_vtable::VTABLE_BSShader;
use crate::re::{BSGeometry, BSLight, BSRenderPass, BSShaderProperty};
use crate::relocation::{RelocationID, RttiType, VariantID};

crate::core_util::abstract_type! { pub type BSShader; }

impl RttiType for BSShader {
    const RTTI: VariantID = RTTI_BSShader;
}

impl BSShader {
    pub const RTTI: VariantID = RTTI_BSShader;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSShader;

    crate::relocation_func! {
        pub fn make_render_pass(
            &mut self,
            property: *mut BSShaderProperty,
            geometry: *mut BSGeometry,
            technique: u32,
            num_lights: u8,
            lights: *mut *mut BSLight,
        ) -> *mut BSRenderPass => RelocationID::new(100717, 107497)
    }
}

// TODO: This is currently a pointer-compatible partial translation of `RE::BSShader` backed by
// `BSShader.h`. Replace the opaque stand-in with the full multi-base layout when Rust needs
// shader fields or virtual setup/restore surface.
