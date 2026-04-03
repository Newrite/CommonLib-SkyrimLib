//! External plugin API request and publication helpers.
//!
//! This module focuses on the recurring Skyrim plugin pattern:
//!
//! - find a loaded plugin DLL
//! - locate an exported API request symbol
//! - request a versioned interface pointer
//! - publish a small versioned C ABI service table from a Rust plugin
//! - keep the low-level calling convention honest while removing repeated
//!   Windows boilerplate from plugin code
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::interop::external_api;
//! use libskyrim::sdk::interop::messaging::ApiVersion;
//!
//! #[repr(C)]
//! pub struct ExampleApi {
//!     pub set_enabled: unsafe extern "system" fn(bool),
//!     pub current_version: unsafe extern "system" fn() -> u32,
//! }
//!
//! fn query_dependency_api() -> Result<(), external_api::RequestApiError> {
//!     let api = unsafe {
//!         external_api::request_plugin_api_for_plugin::<ExampleApi, ApiVersion>(
//!             "ExampleDependency",
//!             ApiVersion::new(1, 0),
//!         )?
//!     };
//!
//!     let api = unsafe { api.as_ref() };
//!     unsafe { (api.set_enabled)(true) };
//!     Ok(())
//! }
//! ```
//!
//! Matching provider:
//!
//! ```rust,ignore
//! use libskyrim::sdk::interop::external_api;
//! use libskyrim::sdk::interop::messaging::ApiVersion;
//!
//! #[repr(C)]
//! pub struct ExampleApi {
//!     pub set_enabled: unsafe extern "system" fn(bool),
//!     pub current_version: unsafe extern "system" fn() -> u32,
//! }
//!
//! static EXAMPLE_API_V1: ExampleApi = ExampleApi {
//!     set_enabled: example_set_enabled,
//!     current_version: example_current_version,
//! };
//!
//! unsafe extern "system" fn example_set_enabled(_enabled: bool) {}
//! unsafe extern "system" fn example_current_version() -> u32 { 1 }
//!
//! libskyrim::sdk::interop::external_api::export_plugin_api! {
//!     pub fn RequestPluginAPI(version: ApiVersion) -> ExampleApi {
//!         external_api::select_api_for_version(version, ApiVersion::new(1, 0), &EXAMPLE_API_V1)
//!     }
//! }
//!
//! libskyrim::sdk::interop::external_api::export_plugin_symbol! {
//!     pub as "GetExampleApi" fn get_example_api() -> ExampleApi {
//!         Some(&EXAMPLE_API_V1)
//!     }
//! }
//! ```

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::{CStr, c_void};
use core::fmt;
use core::mem::{size_of, transmute_copy};
use core::ptr::NonNull;

use crate::rex::W32::{GetModuleHandleW, GetProcAddress};

/// ABI used by `RequestPluginAPI`-style versioned exports.
pub type RequestPluginApiFn<V> = unsafe extern "system" fn(V) -> *mut c_void;

/// ABI used by getter-style custom exported service symbols.
pub type ExportedSymbolFn = unsafe extern "system" fn() -> *mut c_void;

/// Error returned when a target DLL is not currently loaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleError {
    NotLoaded,
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotLoaded => write!(f, "target module is not loaded"),
        }
    }
}

impl core::error::Error for ModuleError {}

/// Error returned when an exported symbol cannot be found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolError {
    Module(ModuleError),
    NotFound,
}

impl fmt::Display for SymbolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Module(error) => write!(f, "{}", error),
            Self::NotFound => write!(f, "requested export was not found"),
        }
    }
}

impl core::error::Error for SymbolError {}

impl From<ModuleError> for SymbolError {
    #[inline(always)]
    fn from(value: ModuleError) -> Self {
        Self::Module(value)
    }
}

/// Error returned when a `RequestPluginAPI` style call fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestApiError {
    Symbol(SymbolError),
    NullInterface,
}

impl fmt::Display for RequestApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol(error) => write!(f, "{}", error),
            Self::NullInterface => write!(f, "plugin API request returned a null interface"),
        }
    }
}

impl core::error::Error for RequestApiError {}

impl From<SymbolError> for RequestApiError {
    #[inline(always)]
    fn from(value: SymbolError) -> Self {
        Self::Symbol(value)
    }
}

impl From<ModuleError> for RequestApiError {
    #[inline(always)]
    fn from(value: ModuleError) -> Self {
        Self::Symbol(value.into())
    }
}

/// Handle to a loaded module discovered through `GetModuleHandleW`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadedModule {
    handle: NonNull<c_void>,
}

impl LoadedModule {
    /// Look up an already-loaded module by DLL name.
    pub fn get(dll_name: &str) -> Result<Self, ModuleError> {
        let wide_name = utf16_null_terminated(dll_name);
        let handle = unsafe { GetModuleHandleW(wide_name.as_ptr()) };
        let handle = NonNull::new(handle).ok_or(ModuleError::NotLoaded)?;
        Ok(Self { handle })
    }

    /// Look up a loaded module by plugin name, automatically appending `.dll`
    /// when the name does not already include the extension.
    #[inline(always)]
    pub fn get_plugin(plugin_name: &str) -> Result<Self, ModuleError> {
        Self::get(&plugin_dll_name(plugin_name))
    }

    #[inline(always)]
    pub const fn as_non_null(self) -> NonNull<c_void> {
        self.handle
    }

    #[inline(always)]
    pub const fn as_ptr(self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Resolve a raw exported symbol from this loaded module.
    pub fn symbol_raw(self, symbol_name: &CStr) -> Result<NonNull<c_void>, SymbolError> {
        let proc = unsafe { GetProcAddress(self.as_ptr().cast(), symbol_name.as_ptr().cast()) }
            .ok_or(SymbolError::NotFound)?;
        let raw = proc as *const ();
        let raw = NonNull::new(raw.cast_mut().cast::<c_void>()).ok_or(SymbolError::NotFound)?;
        Ok(raw)
    }

    /// Resolve an exported symbol and reinterpret it as `T`.
    ///
    /// # Safety
    /// The caller must ensure that the symbol really has the ABI and layout of
    /// `T`. This is primarily intended for function-pointer exports.
    pub unsafe fn symbol<T>(self, symbol_name: &CStr) -> Result<T, SymbolError>
    where
        T: Copy,
    {
        let raw = self.symbol_raw(symbol_name)?;
        assert!(
            size_of::<T>() == size_of::<*mut c_void>(),
            "exported symbol casts require pointer-sized target types"
        );
        Ok(unsafe { transmute_copy(&raw.as_ptr()) })
    }

    /// Resolve a `RequestPluginAPI`-style export and request a typed
    /// interface pointer.
    ///
    /// # Safety
    /// The caller must ensure that:
    ///
    /// - the exported symbol really has the signature
    ///   `unsafe extern "system" fn(V) -> *mut c_void`
    /// - the returned pointer, when non-null, points to a live `T`
    pub unsafe fn request_plugin_api<T, V>(self, version: V) -> Result<NonNull<T>, RequestApiError>
    where
        V: Copy,
    {
        unsafe { self.request_plugin_api_with_symbol(request_plugin_api_symbol(), version) }
    }

    /// Resolve a custom API request symbol and request a typed interface
    /// pointer.
    ///
    /// # Safety
    /// The caller must ensure that:
    ///
    /// - `symbol_name` points to an export with signature
    ///   `unsafe extern "system" fn(V) -> *mut c_void`
    /// - the returned pointer, when non-null, points to a live `T`
    pub unsafe fn request_plugin_api_with_symbol<T, V>(
        self,
        symbol_name: &CStr,
        version: V,
    ) -> Result<NonNull<T>, RequestApiError>
    where
        V: Copy,
    {
        let request: RequestPluginApiFn<V> = unsafe { self.symbol(symbol_name)? };
        let raw = unsafe { request(version) };
        NonNull::new(raw.cast::<T>()).ok_or(RequestApiError::NullInterface)
    }
}

impl fmt::Debug for LoadedModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("LoadedModule")
            .field(&format_args!("{:p}", self.as_ptr()))
            .finish()
    }
}

/// Resolve a loaded module by DLL name.
#[inline(always)]
pub fn loaded_module(dll_name: &str) -> Result<LoadedModule, ModuleError> {
    LoadedModule::get(dll_name)
}

/// Resolve a loaded module by plugin name, automatically appending `.dll`
/// when needed.
#[inline(always)]
pub fn loaded_plugin_module(plugin_name: &str) -> Result<LoadedModule, ModuleError> {
    LoadedModule::get_plugin(plugin_name)
}

/// Resolve a raw exported symbol from a loaded module.
#[inline(always)]
pub fn symbol_raw(dll_name: &str, symbol_name: &CStr) -> Result<NonNull<c_void>, SymbolError> {
    loaded_module(dll_name)?.symbol_raw(symbol_name)
}

/// Resolve a raw exported symbol from a loaded plugin module.
#[inline(always)]
pub fn plugin_symbol_raw(
    plugin_name: &str,
    symbol_name: &CStr,
) -> Result<NonNull<c_void>, SymbolError> {
    loaded_plugin_module(plugin_name)?.symbol_raw(symbol_name)
}

/// Resolve an exported symbol from a loaded module and reinterpret it as `T`.
///
/// # Safety
/// The caller must ensure that the export really has the ABI and layout of
/// `T`.
#[inline(always)]
pub unsafe fn symbol<T>(dll_name: &str, symbol_name: &CStr) -> Result<T, SymbolError>
where
    T: Copy,
{
    unsafe { loaded_module(dll_name)?.symbol(symbol_name) }
}

/// Request a typed plugin API from a loaded DLL using the conventional
/// `RequestPluginAPI` export.
///
/// # Safety
/// The caller must ensure that the exported function has the expected ABI and
/// that the returned interface really is a live `T`.
#[inline(always)]
pub unsafe fn request_plugin_api<T, V>(
    dll_name: &str,
    version: V,
) -> Result<NonNull<T>, RequestApiError>
where
    V: Copy,
{
    unsafe { loaded_module(dll_name)?.request_plugin_api(version) }
}

/// Request a typed plugin API from a loaded plugin by plugin name.
///
/// This helper automatically appends `.dll` when needed.
///
/// # Safety
/// The caller must ensure that the exported function has the expected ABI and
/// that the returned interface really is a live `T`.
#[inline(always)]
pub unsafe fn request_plugin_api_for_plugin<T, V>(
    plugin_name: &str,
    version: V,
) -> Result<NonNull<T>, RequestApiError>
where
    V: Copy,
{
    unsafe { loaded_plugin_module(plugin_name)?.request_plugin_api(version) }
}

/// Request a typed plugin API from a loaded module using a custom request
/// symbol.
///
/// # Safety
/// The caller must ensure that the exported function has the expected ABI and
/// that the returned interface really is a live `T`.
#[inline(always)]
pub unsafe fn request_plugin_api_with_symbol<T, V>(
    dll_name: &str,
    symbol_name: &CStr,
    version: V,
) -> Result<NonNull<T>, RequestApiError>
where
    V: Copy,
{
    unsafe { loaded_module(dll_name)?.request_plugin_api_with_symbol(symbol_name, version) }
}

/// Convert a plugin-owned `'static` API table into an exported C ABI pointer.
#[inline(always)]
pub fn export_api<T>(api: &'static T) -> *mut c_void {
    (api as *const T).cast_mut().cast::<c_void>()
}

/// Convert an optional plugin-owned API table into an exported C ABI pointer.
///
/// `None` maps to a null pointer, matching the common `RequestPluginAPI`
/// convention for unsupported versions or unavailable services.
#[inline(always)]
pub fn export_api_option<T>(api: Option<&'static T>) -> *mut c_void {
    match api {
        Some(api) => export_api(api),
        None => core::ptr::null_mut(),
    }
}

/// Return the API table only when the requested version matches exactly.
#[inline(always)]
pub fn select_api_for_version<T, V>(
    requested: V,
    supported: V,
    api: &'static T,
) -> Option<&'static T>
where
    V: PartialEq,
{
    (requested == supported).then_some(api)
}

/// Return the first API table whose version matches the requested value.
#[inline(always)]
pub fn select_api_for_versions<T, V, const N: usize>(
    requested: V,
    supported: &[(V, &'static T); N],
) -> Option<&'static T>
where
    V: Copy + PartialEq,
{
    supported
        .iter()
        .find_map(|(version, api)| (*version == requested).then_some(*api))
}

/// Normalize a plugin name into the corresponding DLL file name.
pub fn plugin_dll_name(plugin_name: &str) -> String {
    if plugin_name.to_ascii_lowercase().ends_with(".dll") {
        plugin_name.into()
    } else {
        format!("{}.dll", plugin_name)
    }
}

/// Conventional exported request symbol used by many SKSE plugin APIs.
#[inline(always)]
pub fn request_plugin_api_symbol() -> &'static CStr {
    unsafe { CStr::from_bytes_with_nul_unchecked(b"RequestPluginAPI\0") }
}

fn utf16_null_terminated(value: &str) -> Vec<u16> {
    let mut wide = value.encode_utf16().collect::<Vec<_>>();
    wide.push(0);
    wide
}

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

pub use crate::{export_plugin_api, export_plugin_symbol};

#[cfg(test)]
mod tests {
    use super::{
        export_api, export_api_option, plugin_dll_name, select_api_for_version,
        select_api_for_versions,
    };

    #[repr(C)]
    #[derive(Debug, PartialEq, Eq)]
    struct TestApi {
        version: u32,
    }

    static API_V1: TestApi = TestApi { version: 1 };
    static API_V2: TestApi = TestApi { version: 2 };

    crate::export_plugin_api! {
        fn sdk_external_api_test_request(version: u32) -> TestApi {
            select_api_for_versions(version, &[(1u32, &API_V1), (2u32, &API_V2)])
        }
    }

    crate::export_plugin_api! {
        as "SdkExternalApiTestCustomRequest"
        fn sdk_external_api_test_custom_request(version: u32) -> TestApi {
            select_api_for_version(version, 7u32, &API_V2)
        }
    }

    crate::export_plugin_symbol! {
        fn sdk_external_api_test_symbol() -> TestApi {
            Some(&API_V1)
        }
    }

    crate::export_plugin_symbol! {
        as "SdkExternalApiTestCustomSymbol"
        fn sdk_external_api_test_custom_symbol() -> TestApi {
            Some(&API_V2)
        }
    }

    crate::export_plugin_symbol! {
        fn sdk_external_api_test_missing_symbol() -> TestApi {
            None
        }
    }

    #[test]
    fn plugin_dll_name_adds_extension_once() {
        assert_eq!(plugin_dll_name("ExamplePlugin"), "ExamplePlugin.dll");
        assert_eq!(plugin_dll_name("ExamplePlugin.dll"), "ExamplePlugin.dll");
    }

    #[test]
    fn export_api_option_maps_none_to_null() {
        assert!(export_api_option::<TestApi>(None).is_null());
        assert_eq!(
            export_api(&API_V1).cast::<TestApi>(),
            &API_V1 as *const _ as *mut _
        );
    }

    #[test]
    fn select_api_helpers_match_requested_versions() {
        assert_eq!(
            select_api_for_version(1u32, 1u32, &API_V1).map(|api| api.version),
            Some(1)
        );
        assert!(select_api_for_version(1u32, 2u32, &API_V1).is_none());

        assert_eq!(
            select_api_for_versions(2u32, &[(1u32, &API_V1), (2u32, &API_V2)])
                .map(|api| api.version),
            Some(2)
        );
        assert!(select_api_for_versions(3u32, &[(1u32, &API_V1), (2u32, &API_V2)]).is_none());
    }

    #[test]
    fn export_plugin_api_macro_returns_versioned_tables() {
        assert_eq!(
            sdk_external_api_test_request(1).cast::<TestApi>(),
            &API_V1 as *const _ as *mut _
        );
        assert_eq!(
            sdk_external_api_test_request(2).cast::<TestApi>(),
            &API_V2 as *const _ as *mut _
        );
        assert!(sdk_external_api_test_request(3).is_null());
        assert_eq!(
            sdk_external_api_test_custom_request(7).cast::<TestApi>(),
            &API_V2 as *const _ as *mut _
        );
        assert!(sdk_external_api_test_custom_request(8).is_null());
    }

    #[test]
    fn export_plugin_symbol_macro_returns_named_tables() {
        assert_eq!(
            sdk_external_api_test_symbol().cast::<TestApi>(),
            &API_V1 as *const _ as *mut _
        );
        assert_eq!(
            sdk_external_api_test_custom_symbol().cast::<TestApi>(),
            &API_V2 as *const _ as *mut _
        );
        assert!(sdk_external_api_test_missing_symbol().is_null());
    }
}
