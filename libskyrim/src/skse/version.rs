use core::ffi::c_char;

use crate::version::Version;

/// Plugin version info exported to SKSE.
#[repr(C)]
pub struct SksePluginVersionData {
    pub data_version: u32,
    pub plugin_version: u32,
    pub name: [c_char; 256],
    pub author: [c_char; 256],
    pub support_email: [c_char; 252],
    pub version_indep_ex: u32,
    pub version_indep: u32,
    pub compat_versions: [u32; 16],
    pub se_version_required: u32,
}

pub type PluginVersionData = SksePluginVersionData;

impl SksePluginVersionData {
    pub const VERSION: u32 = 1;

    pub const VINDEP_ADDRESS_LIBRARY_POST_AE: u32 = 1 << 0;
    pub const VINDEP_SIGNATURES: u32 = 1 << 1;
    pub const VINDEP_STRUCTS_POST_629: u32 = 1 << 2;

    pub const VINDEPEX_NO_STRUCT_USE: u32 = 1 << 0;
}

pub const RUNTIME_SSE_1_1_47: Version = Version::new(1, 1, 47, 0);
pub const RUNTIME_SSE_1_1_51: Version = Version::new(1, 1, 51, 0);
pub const RUNTIME_SSE_1_2_36: Version = Version::new(1, 2, 36, 0);
pub const RUNTIME_SSE_1_2_39: Version = Version::new(1, 2, 39, 0);
pub const RUNTIME_SSE_1_3_5: Version = Version::new(1, 3, 5, 0);
pub const RUNTIME_SSE_1_3_9: Version = Version::new(1, 3, 9, 0);
pub const RUNTIME_SSE_1_4_2: Version = Version::new(1, 4, 2, 0);
pub const RUNTIME_SSE_1_5_3: Version = Version::new(1, 5, 3, 0);
pub const RUNTIME_SSE_1_5_16: Version = Version::new(1, 5, 16, 0);
pub const RUNTIME_SSE_1_5_23: Version = Version::new(1, 5, 23, 0);
pub const RUNTIME_SSE_1_5_39: Version = Version::new(1, 5, 39, 0);
pub const RUNTIME_SSE_1_5_50: Version = Version::new(1, 5, 50, 0);
pub const RUNTIME_SSE_1_5_53: Version = Version::new(1, 5, 53, 0);
pub const RUNTIME_SSE_1_5_62: Version = Version::new(1, 5, 62, 0);
pub const RUNTIME_SSE_1_5_73: Version = Version::new(1, 5, 73, 0);
pub const RUNTIME_SSE_1_5_80: Version = Version::new(1, 5, 80, 0);
pub const RUNTIME_SSE_1_5_97: Version = Version::new(1, 5, 97, 0);
pub const RUNTIME_SSE_1_6_317: Version = Version::new(1, 6, 317, 0);
pub const RUNTIME_SSE_1_6_318: Version = Version::new(1, 6, 318, 0);
pub const RUNTIME_SSE_1_6_323: Version = Version::new(1, 6, 323, 0);
pub const RUNTIME_SSE_1_6_342: Version = Version::new(1, 6, 342, 0);
pub const RUNTIME_SSE_1_6_353: Version = Version::new(1, 6, 353, 0);
pub const RUNTIME_SSE_1_6_629: Version = Version::new(1, 6, 629, 0);
pub const RUNTIME_SSE_1_6_640: Version = Version::new(1, 6, 640, 0);
pub const RUNTIME_SSE_1_6_659: Version = Version::new(1, 6, 659, 0);
pub const RUNTIME_SSE_1_6_678: Version = Version::new(1, 6, 678, 0);
pub const RUNTIME_SSE_1_6_1130: Version = Version::new(1, 6, 1130, 0);
pub const RUNTIME_SSE_1_6_1170: Version = Version::new(1, 6, 1170, 0);
pub const RUNTIME_1_6_1179: Version = Version::new(1, 6, 1179, 0);

pub const RUNTIME_SSE_LATEST_AE: Version = RUNTIME_SSE_1_6_1170;
pub const RUNTIME_SSE_LATEST_SE: Version = RUNTIME_SSE_1_5_97;
pub const RUNTIME_SSE_LATEST: Version = RUNTIME_SSE_LATEST_AE;

pub const RUNTIME_VR_1_4_15: Version = Version::new(1, 4, 15, 0);
pub const RUNTIME_LATEST_VR: Version = RUNTIME_VR_1_4_15;
