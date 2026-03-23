use crate::offsets::offsets_rtti::RTTI_BSLensFlareRenderData;
use crate::re::BSLensFlareSpriteRenderData;
use crate::re::BSTArray;
use crate::re::NiPointer;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct BSLensFlareRenderData {
    pub fade_dist_radius_scale: f32,                               // 0x00
    pub color_influence: f32,                                      // 0x04
    pub sprites: BSTArray<NiPointer<BSLensFlareSpriteRenderData>>, // 0x08
}

const _: () = assert!(core::mem::size_of::<BSLensFlareRenderData>() == 0x20);

impl RttiType for BSLensFlareRenderData {
    const RTTI: VariantID = RTTI_BSLensFlareRenderData;
}

impl BSLensFlareRenderData {
    pub const RTTI: VariantID = RTTI_BSLensFlareRenderData;
}
