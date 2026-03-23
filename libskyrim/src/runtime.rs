use crate::version::Version;
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

#[inline(always)]
pub fn is_ae() -> bool {
    *CURRENT_RUNTIME == RuntimeType::AE
}
#[inline(always)]
pub fn is_se() -> bool {
    *CURRENT_RUNTIME == RuntimeType::SE
}
#[inline(always)]
pub fn is_vr() -> bool {
    *CURRENT_RUNTIME == RuntimeType::VR
}
#[inline(always)]
pub fn current_runtime() -> RuntimeType {
    *CURRENT_RUNTIME
}

#[inline(always)]
pub const fn runtime_name(runtime: RuntimeType) -> &'static str {
    match runtime {
        RuntimeType::Unknown => "Unknown",
        RuntimeType::SE => "SE",
        RuntimeType::AE => "AE",
        RuntimeType::VR => "VR",
    }
}

#[inline(always)]
pub fn current_runtime_name() -> &'static str {
    runtime_name(current_runtime())
}

#[inline(always)]
pub fn is_at_least(version: Version) -> bool {
    *CURRENT_VERSION >= version
}

#[inline(always)]
pub fn relocate<T: Copy>(se_and_vr: T, ae: T) -> T {
    if is_ae() { ae } else { se_and_vr }
}

#[inline(always)]
pub fn relocate_all<T: Copy>(se: T, ae: T, vr: T) -> T {
    if is_vr() {
        vr
    } else if is_ae() {
        ae
    } else {
        se
    }
}

#[inline(always)]
pub fn relocate_if_newer<T: Copy>(version: Version, older: T, newer: T) -> T {
    if is_at_least(version) { newer } else { older }
}

#[inline(always)]
pub fn relocate_versioned_all<T: Copy>(version: Version, se: T, ae: T, vr: T) -> T {
    if is_vr() {
        vr
    } else if is_at_least(version) {
        ae
    } else {
        se
    }
}

#[inline(always)]
#[track_caller]
pub fn require_offset(offset: usize, accessor: &str) -> usize {
    if offset == 0 {
        crate::log::fatal_runtime(format_args!(
            "{accessor} is unavailable for runtime {}",
            current_runtime_name()
        ));
    }
    offset
}

#[inline(always)]
#[track_caller]
pub fn require_vr(accessor: &str) {
    if !is_vr() {
        crate::log::fatal_runtime(format_args!(
            "{accessor} is VR-only, current runtime is {}",
            current_runtime_name()
        ));
    }
}

#[inline(always)]
#[track_caller]
pub fn require_non_vr(accessor: &str) {
    if is_vr() {
        crate::log::fatal_runtime(format_args!(
            "{accessor} is only available in SE/AE, current runtime is {}",
            current_runtime_name()
        ));
    }
}

#[inline(always)]
pub fn should_validate_layout(expected: usize) -> bool {
    expected != 0
}

#[macro_export]
macro_rules! runtime_offset {
    (se_ae: $se_ae:expr, vr: $vr:expr $(,)?) => {
        $crate::runtime::relocate_all($se_ae, $se_ae, $vr)
    };
    (se_and_vr: $se_and_vr:expr, ae: $ae:expr $(,)?) => {
        $crate::runtime::relocate($se_and_vr, $ae)
    };
    (version: $version:expr, older: $older:expr, newer: $newer:expr $(,)?) => {
        $crate::runtime::relocate_if_newer($version, $older, $newer)
    };
    (se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {
        $crate::runtime::relocate_all($se, $ae, $vr)
    };
    (version: $version:expr, se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {
        $crate::runtime::relocate_versioned_all($version, $se, $ae, $vr)
    };
}

#[macro_export]
macro_rules! runtime_data_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> &$ret_ty {
            let offset = $crate::runtime::require_offset(
                $crate::runtime_offset! { $($selector)* },
                stringify!($func_name)
            );
            unsafe { &*((self as *const _ as *const u8).add(offset) as *const $ret_ty) }
        }
    };
}

#[macro_export]
macro_rules! runtime_data_ref_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> &$ret_ty {
            let offset = $crate::runtime::require_offset(
                $crate::runtime_offset! { $($selector)* },
                stringify!($func_name)
            );
            unsafe { &*((self as *const _ as *const u8).add(offset) as *const $ret_ty) }
        }
    };
}

#[macro_export]
macro_rules! runtime_data_mut_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&mut self) -> &mut $ret_ty {
            let offset = $crate::runtime::require_offset(
                $crate::runtime_offset! { $($selector)* },
                stringify!($func_name)
            );
            unsafe { &mut *((self as *mut _ as *mut u8).add(offset) as *mut $ret_ty) }
        }
    };
}

#[macro_export]
macro_rules! runtime_optional_data_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> Option<&$ret_ty> {
            let offset = $crate::runtime_offset! { $($selector)* };
            if offset == 0 {
                None
            } else {
                Some(unsafe { &*((self as *const _ as *const u8).add(offset) as *const $ret_ty) })
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_optional_data_mut_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&mut self) -> Option<&mut $ret_ty> {
            let offset = $crate::runtime_offset! { $($selector)* };
            if offset == 0 {
                None
            } else {
                Some(unsafe { &mut *((self as *mut _ as *mut u8).add(offset) as *mut $ret_ty) })
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_data_ptr_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> *mut $ret_ty {
            let offset = $crate::runtime::require_offset(
                $crate::runtime_offset! { $($selector)* },
                stringify!($func_name)
            );
            unsafe { (self as *const _ as *mut u8).add(offset) as *mut $ret_ty }
        }
    };
}

#[macro_export]
macro_rules! runtime_pointer_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> $ret_ty {
            let offset = $crate::runtime::require_offset(
                $crate::runtime_offset! { $($selector)* },
                stringify!($func_name)
            );
            unsafe {
                let ptr = (self as *const _ as *const u8).add(offset) as *const $ret_ty;
                *ptr
            }
        }
    };
}

#[macro_export]
macro_rules! vr_runtime_data_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            vr: $vr:expr $(,)?
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> &$ret_ty {
            $crate::runtime::require_vr(stringify!($func_name));
            unsafe { &*((self as *const _ as *const u8).add($vr) as *const $ret_ty) }
        }
    };
}

#[macro_export]
macro_rules! vr_runtime_data_mut_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            vr: $vr:expr $(,)?
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&mut self) -> &mut $ret_ty {
            $crate::runtime::require_vr(stringify!($func_name));
            unsafe { &mut *((self as *mut _ as *mut u8).add($vr) as *mut $ret_ty) }
        }
    };
}

#[macro_export]
macro_rules! se_only_pointer_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            se: $se:expr $(,)?
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> *mut $ret_ty {
            if $crate::runtime::is_vr() {
                core::ptr::null_mut()
            } else {
                unsafe { (self as *const _ as *mut u8).add($se) as *mut $ret_ty }
            }
        }
    };
}

#[macro_export]
macro_rules! vr_only_pointer_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            vr: $vr:expr $(,)?
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> *mut $ret_ty {
            if !$crate::runtime::is_vr() {
                core::ptr::null_mut()
            } else {
                unsafe { (self as *const _ as *mut u8).add($vr) as *mut $ret_ty }
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_cast_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        $crate::runtime_data_ref_accessor! {
            $vis fn $func_name() -> $ret_ty {
                $($selector)*
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_cast_mut_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        $crate::runtime_data_mut_accessor! {
            $vis fn $func_name() -> $ret_ty {
                $($selector)*
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_optional_pointer_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> Option<$ret_ty> {
            let offset = $crate::runtime_offset! { $($selector)* };
            if offset == 0 {
                None
            } else {
                unsafe {
                    let ptr = (self as *const _ as *const u8).add(offset) as *const $ret_ty;
                    Some(*ptr)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_assert_size {
    ($ty:ty, se_ae: $se_ae:expr, vr: $vr:expr $(,)?) => {
        $crate::runtime_assert_size!($ty, se: $se_ae, ae: $se_ae, vr: $vr);
    };
    ($ty:ty, version: $version:expr, older: $older:expr, newer: $newer:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_if_newer($version, $older, $newer);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::size_of::<$ty>(),
                expected,
                "size_of::<{}>() mismatch for runtime {}",
                stringify!($ty),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
    ($ty:ty, se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_all($se, $ae, $vr);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::size_of::<$ty>(),
                expected,
                "size_of::<{}>() mismatch for runtime {}",
                stringify!($ty),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
    ($ty:ty, version: $version:expr, se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_versioned_all($version, $se, $ae, $vr);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::size_of::<$ty>(),
                expected,
                "size_of::<{}>() mismatch for runtime {}",
                stringify!($ty),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
}

#[macro_export]
macro_rules! runtime_assert_offset {
    ($ty:ty, $field:tt, se_ae: $se_ae:expr, vr: $vr:expr $(,)?) => {
        $crate::runtime_assert_offset!($ty, $field, se: $se_ae, ae: $se_ae, vr: $vr);
    };
    ($ty:ty, $field:tt, version: $version:expr, older: $older:expr, newer: $newer:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_if_newer($version, $older, $newer);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::offset_of!($ty, $field),
                expected,
                "offset_of!({}, {}) mismatch for runtime {}",
                stringify!($ty),
                stringify!($field),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
    ($ty:ty, $field:tt, se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_all($se, $ae, $vr);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::offset_of!($ty, $field),
                expected,
                "offset_of!({}, {}) mismatch for runtime {}",
                stringify!($ty),
                stringify!($field),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
    ($ty:ty, $field:tt, version: $version:expr, se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_versioned_all($version, $se, $ae, $vr);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::offset_of!($ty, $field),
                expected,
                "offset_of!({}, {}) mismatch for runtime {}",
                stringify!($ty),
                stringify!($field),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
}
