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

// ─── МАКРОСЫ ДЛЯ ДОСТУПА К СЪЕХАВШИМ ПОЛЯМ (RUNTIME DATA ACCESSORS) ──────────

/// Макрос для доступа к вложенным структурам и значениям по смещению.
/// Аналог RUNTIME_DATA_ACCESSOR_VERSIONED и RUNTIME_DATA_ACCESSOR из C++.
#[macro_export]
macro_rules! runtime_data_accessor {
    // Вариант 1: Отличаются только SE и AE (как в 1.6.629+)
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            version: $version:expr,
            se: $se_offset:expr,
            ae: $ae_offset:expr
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> *mut $ret_ty {
            use $crate::relocation::IntoOffset;
            let offset = if $crate::runtime::is_at_least($version) {
                $ae_offset
            } else {
                $se_offset
            };
            unsafe { (self as *const _ as *const u8).add(offset.into_offset()) as *mut $ret_ty }
        }
    };

    // Вариант 2: Отличаются базовые версии (SE/AE одинаково) и VR
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            se_ae: $se_ae_offset:expr,
            vr: $vr_offset:expr
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> *mut $ret_ty {
            use $crate::relocation::IntoOffset;
            let offset = if $crate::runtime::is_vr() {
                $vr_offset
            } else {
                $se_ae_offset
            };
            unsafe { (self as *const _ as *const u8).add(offset.into_offset()) as *mut $ret_ty }
        }
    };
}

/// Макрос для доступа к указателям (когда поле само является указателем `*mut T`).
/// Аналог RUNTIME_DATA_POINTER_ACCESSOR_EX.
#[macro_export]
macro_rules! runtime_pointer_accessor {
    // Вариант 1: Указатель отличается в SE и AE
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            version: $version:expr,
            se: $se_offset:expr,
            ae: $ae_offset:expr
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> $ret_ty {
            use $crate::relocation::IntoOffset;
            let offset = if $crate::runtime::is_at_least($version) {
                $ae_offset
            } else {
                $se_offset
            };
            unsafe {
                let ptr = (self as *const _ as *const u8).add(offset.into_offset()) as *const $ret_ty;
                *ptr
            }
        }
    };
}