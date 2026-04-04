/// Generate a versioned exported API request function that returns a typed
/// service table pointer as `*mut c_void`.
///
/// The body must evaluate to `Option<&'static ApiType>`.
///
/// Example:
///
/// ```ignore
/// static API_V1: ExampleApi = ExampleApi { ... };
///
/// libskyrim::sdk::interop::external_api::export_plugin_api! {
///     pub fn RequestPluginAPI(version: ApiVersion) -> ExampleApi {
///         libskyrim::sdk::interop::external_api::select_api_for_version(
///             version,
///             ApiVersion::new(1, 0),
///             &API_V1,
///         )
///     }
/// }
/// ```
#[macro_export]
macro_rules! export_plugin_api {
    (
        $(#[$meta:meta])*
        $vis:vis fn $name:ident($version:ident : $version_ty:ty) -> $api_ty:ty $body:block
    ) => {
        $(#[$meta])*
        #[unsafe(no_mangle)]
        $vis extern "system" fn $name($version: $version_ty) -> *mut core::ffi::c_void {
            let api: Option<&'static $api_ty> = { $body };
            $crate::sdk::interop::external_api::export_api_option(api)
        }
    };
    (
        $(#[$meta:meta])*
        $vis:vis as $symbol:literal fn $name:ident($version:ident : $version_ty:ty) -> $api_ty:ty $body:block
    ) => {
        $(#[$meta])*
        #[unsafe(export_name = $symbol)]
        $vis extern "system" fn $name($version: $version_ty) -> *mut core::ffi::c_void {
            let api: Option<&'static $api_ty> = { $body };
            $crate::sdk::interop::external_api::export_api_option(api)
        }
    };
}

/// Generate a getter-style exported service symbol that returns a typed service
/// table pointer as `*mut c_void`.
///
/// The body must evaluate to `Option<&'static ApiType>`.
#[macro_export]
macro_rules! export_plugin_symbol {
    (
        $(#[$meta:meta])*
        $vis:vis fn $name:ident() -> $api_ty:ty $body:block
    ) => {
        $(#[$meta])*
        #[unsafe(no_mangle)]
        $vis extern "system" fn $name() -> *mut core::ffi::c_void {
            let api: Option<&'static $api_ty> = { $body };
            $crate::sdk::interop::external_api::export_api_option(api)
        }
    };
    (
        $(#[$meta:meta])*
        $vis:vis as $symbol:literal fn $name:ident() -> $api_ty:ty $body:block
    ) => {
        $(#[$meta])*
        #[unsafe(export_name = $symbol)]
        $vis extern "system" fn $name() -> *mut core::ffi::c_void {
            let api: Option<&'static $api_ty> = { $body };
            $crate::sdk::interop::external_api::export_api_option(api)
        }
    };
}

/// Define a typed client-side flat symbol table loader for one plugin/module.
///
/// Example:
///
/// ```ignore
/// libskyrim::sdk::interop::external_api::define_plugin_symbols! {
///     pub struct MenuFrameworkApi {
///         add_window: unsafe extern "system" fn(RenderFn) -> *mut Window = "AddWindow";
///         register_hud_element: unsafe extern "system" fn(HudCallback) -> i64 = "RegisterHudElement";
///         unregister_hud_element: unsafe extern "system" fn(i64) = "UnregisterHudElement";
///         optional maybe_get_api_version: unsafe extern "system" fn() -> u32 = "GetApiVersion";
///     }
/// }
///
/// let api = unsafe { MenuFrameworkApi::load_plugin("SKSEMenuFramework")? };
/// let api_version = api.maybe_get_api_version.map(|version| unsafe { version() });
/// ```
#[macro_export]
macro_rules! define_plugin_symbols {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($entries:tt)*
        }
    ) => {
        $crate::define_plugin_symbols!(
            @parse
            [$(#[$meta])*]
            [$vis]
            [$name]
            [module]
            []
            []
            $($entries)*
        );
    };
    (
        @parse
        [$($meta:meta)*]
        [$vis:vis]
        [$name:ident]
        [$module:ident]
        [$($fields:tt)*]
        [$($inits:tt)*]
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $($fields)*
        }

        #[allow(dead_code)]
        impl $name {
            /// # Safety
            /// The caller must ensure that every symbol in this table has the
            /// expected ABI and signature.
            $vis unsafe fn load(
                $module: $crate::sdk::interop::external_api::LoadedModule,
            ) -> Result<Self, $crate::sdk::interop::external_api::SymbolError> {
                Ok(Self {
                    $($inits)*
                })
            }

            /// # Safety
            /// The caller must ensure that every symbol in this table has the
            /// expected ABI and signature.
            $vis unsafe fn load_dll(
                dll_name: &str,
            ) -> Result<Self, $crate::sdk::interop::external_api::SymbolError> {
                unsafe {
                    Self::load($crate::sdk::interop::external_api::loaded_module(dll_name)?)
                }
            }

            /// # Safety
            /// The caller must ensure that every symbol in this table has the
            /// expected ABI and signature.
            $vis unsafe fn load_plugin(
                plugin_name: &str,
            ) -> Result<Self, $crate::sdk::interop::external_api::SymbolError> {
                unsafe {
                    Self::load($crate::sdk::interop::external_api::loaded_plugin_module(
                        plugin_name,
                    )?)
                }
            }

            /// # Safety
            /// The caller must ensure that every symbol in this table has the
            /// expected ABI and signature.
            $vis unsafe fn load_current(
            ) -> Result<Self, $crate::sdk::interop::external_api::SymbolError> {
                unsafe { Self::load($crate::sdk::interop::external_api::current_module()?) }
            }
        }
    };
    (
        @parse
        [$($meta:meta)*]
        [$vis:vis]
        [$name:ident]
        [$module:ident]
        [$($fields:tt)*]
        [$($inits:tt)*]
        optional $field:ident : $ty:ty = $symbol:literal;
        $($rest:tt)*
    ) => {
        $crate::define_plugin_symbols!(
            @parse
            [$($meta)*]
            [$vis]
            [$name]
            [$module]
            [
                $($fields)*
                $vis $field: Option<$ty>,
            ]
            [
                $($inits)*
                $field: {
                    let symbol_name = unsafe {
                        core::ffi::CStr::from_bytes_with_nul_unchecked(
                            concat!($symbol, "\0").as_bytes()
                        )
                    };
                    unsafe { $module.symbol_optional::<$ty>(symbol_name)? }
                },
            ]
            $($rest)*
        );
    };
    (
        @parse
        [$($meta:meta)*]
        [$vis:vis]
        [$name:ident]
        [$module:ident]
        [$($fields:tt)*]
        [$($inits:tt)*]
        $field:ident : $ty:ty = $symbol:literal;
        $($rest:tt)*
    ) => {
        $crate::define_plugin_symbols!(
            @parse
            [$($meta)*]
            [$vis]
            [$name]
            [$module]
            [
                $($fields)*
                $vis $field: $ty,
            ]
            [
                $($inits)*
                $field: {
                    let symbol_name = unsafe {
                        core::ffi::CStr::from_bytes_with_nul_unchecked(
                            concat!($symbol, "\0").as_bytes()
                        )
                    };
                    unsafe { $module.symbol::<$ty>(symbol_name)? }
                },
            ]
            $($rest)*
        );
    };
}

pub use crate::{define_plugin_symbols, export_plugin_api, export_plugin_symbol};
