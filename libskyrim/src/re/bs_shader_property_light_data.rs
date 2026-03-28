use crate::re::{BSLight, BSTArray};

/// C++ `RE::BSShaderPropertyLightData`
#[repr(C)]
pub struct BSShaderPropertyLightData {
    pub lights: BSTArray<*mut BSLight>, // 00
    pub light_list_fence: i32,          // 18
    pub active_light_mask: u32,         // 1C
    pub light_list_changed: bool,       // 20
    pub pad21: [u8; 7],                 // 21
}

const _: () = assert!(core::mem::size_of::<BSShaderPropertyLightData>() == 0x28);
const _: () = assert!(core::mem::offset_of!(BSShaderPropertyLightData, lights) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSShaderPropertyLightData, light_list_changed) == 0x20);
