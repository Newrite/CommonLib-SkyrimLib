use crate::version::{Version};
use core_util::Later;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeType {
    Unknown,
    SE, // 1.5.97 and lower
    AE, // 1.6.x
    VR, // VR
}

pub static CURRENT_VERSION: Later<Version> = Later::new();
pub static CURRENT_RUNTIME: Later<RuntimeType> = Later::new();

/// Инициализируется один раз при старте плагина
pub fn init(version: Version) {
    CURRENT_VERSION.init(version);

    let runtime = if version.minor() == 4 {
        RuntimeType::VR
    } else if version.minor() == 6 {
        RuntimeType::AE
    } else {
        RuntimeType::SE
    };

    CURRENT_RUNTIME.init(runtime);
}

#[inline(always)] pub fn is_ae() -> bool { *CURRENT_RUNTIME == RuntimeType::AE }
#[inline(always)] pub fn is_se() -> bool { *CURRENT_RUNTIME == RuntimeType::SE }
#[inline(always)] pub fn is_vr() -> bool { *CURRENT_RUNTIME == RuntimeType::VR }

#[inline(always)]
pub fn is_at_least(version: Version) -> bool {
    *CURRENT_VERSION >= version
}