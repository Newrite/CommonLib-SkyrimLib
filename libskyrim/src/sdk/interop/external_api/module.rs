use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::{CStr, c_void};
use core::fmt;
use core::mem::{size_of, transmute_copy};
use core::ptr::NonNull;

use crate::rex::W32::{GetModuleHandleW, GetProcAddress};

use super::callbacks::{
    ExportedCallbackRegistrar, ExportedSubscriber, ExportedSubscriberFn, RegisterCallbackFn,
    UnregisterCallbackFn,
};
use super::types::{
    ExportedSymbolFn, ModuleError, RequestApiError, RequestPluginApiFn, SymbolError,
};

/// Handle to a loaded module discovered through `GetModuleHandleW`.
///
/// `LoadedModule` is the low-level starting point for most `external_api`
/// workflows:
///
/// - request one versioned table through `RequestPluginAPI`
/// - resolve one getter-style exported service
/// - probe one optional symbol/capability
/// - build callback/subscriber helpers over flat exported functions
///
/// It intentionally stays close to the Windows export model while removing the
/// repeated `GetModuleHandleW` / `GetProcAddress` boilerplate from plugin code.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadedModule {
    handle: NonNull<c_void>,
}

impl LoadedModule {
    /// Look up the current process module.
    pub fn current() -> Result<Self, ModuleError> {
        let handle = unsafe { GetModuleHandleW(core::ptr::null()) };
        let handle = NonNull::new(handle).ok_or(ModuleError::NotLoaded)?;
        Ok(Self { handle })
    }

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

    /// Resolve a raw exported symbol from this loaded module when present.
    #[inline(always)]
    pub fn symbol_raw_optional(self, symbol_name: &CStr) -> Option<NonNull<c_void>> {
        let proc = unsafe { GetProcAddress(self.as_ptr().cast(), symbol_name.as_ptr().cast()) }?;
        let raw = proc as *const ();
        NonNull::new(raw.cast_mut().cast::<c_void>())
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

    /// Resolve an exported symbol when present and reinterpret it as `T`.
    ///
    /// # Safety
    /// The caller must ensure that the symbol, when present, really has the
    /// ABI and layout of `T`.
    pub unsafe fn symbol_optional<T>(self, symbol_name: &CStr) -> Result<Option<T>, SymbolError>
    where
        T: Copy,
    {
        let Some(raw) = self.symbol_raw_optional(symbol_name) else {
            return Ok(None);
        };
        assert!(
            size_of::<T>() == size_of::<*mut c_void>(),
            "exported symbol casts require pointer-sized target types"
        );
        Ok(Some(unsafe { transmute_copy(&raw.as_ptr()) }))
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

    /// Resolve a getter-style exported service symbol and request a typed
    /// interface pointer.
    ///
    /// # Safety
    /// The caller must ensure that:
    ///
    /// - `symbol_name` points to an export with signature
    ///   `unsafe extern "system" fn() -> *mut c_void`
    /// - the returned pointer, when non-null, points to a live `T`
    pub unsafe fn request_service<T>(
        self,
        symbol_name: &CStr,
    ) -> Result<NonNull<T>, RequestApiError> {
        let request: ExportedSymbolFn = unsafe { self.symbol(symbol_name)? };
        let raw = unsafe { request() };
        NonNull::new(raw.cast::<T>()).ok_or(RequestApiError::NullInterface)
    }

    /// Resolve a getter-style exported service symbol when present and request
    /// a typed interface pointer.
    ///
    /// Missing symbols and null service pointers both map to `Ok(None)`,
    /// matching the common "optional capability" pattern used by flat plugin
    /// exports.
    ///
    /// # Safety
    /// The caller must ensure that:
    ///
    /// - `symbol_name` points to an export with signature
    ///   `unsafe extern "system" fn() -> *mut c_void`
    /// - the returned pointer, when non-null, points to a live `T`
    pub unsafe fn request_service_optional<T>(
        self,
        symbol_name: &CStr,
    ) -> Result<Option<NonNull<T>>, SymbolError> {
        let Some(request): Option<ExportedSymbolFn> =
            (unsafe { self.symbol_optional(symbol_name)? })
        else {
            return Ok(None);
        };
        let raw = unsafe { request() };
        Ok(NonNull::new(raw.cast::<T>()))
    }

    /// Resolve one exported register/unregister callback pair as a typed
    /// registrar.
    ///
    /// # Safety
    /// The caller must ensure that both exports have the expected ABI and
    /// that the returned registration ids belong to the matching unregister
    /// export.
    pub unsafe fn callback_registrar<Callback, Id>(
        self,
        register_symbol: &CStr,
        unregister_symbol: &CStr,
    ) -> Result<ExportedCallbackRegistrar<Callback, Id>, SymbolError> {
        let register: RegisterCallbackFn<Callback, Id> = unsafe { self.symbol(register_symbol)? };
        let unregister: UnregisterCallbackFn<Id> = unsafe { self.symbol(unregister_symbol)? };
        Ok(ExportedCallbackRegistrar::new(register, unregister))
    }

    /// Resolve one flat exported subscriber-style symbol as a typed helper.
    ///
    /// # Safety
    /// The caller must ensure that the export has the expected ABI and that
    /// the argument type matches the remote plugin's ownership contract.
    pub unsafe fn subscriber<Arg>(
        self,
        symbol_name: &CStr,
    ) -> Result<ExportedSubscriber<Arg>, SymbolError> {
        let subscribe: ExportedSubscriberFn<Arg> = unsafe { self.symbol(symbol_name)? };
        Ok(ExportedSubscriber::new(subscribe))
    }
}

impl fmt::Debug for LoadedModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("LoadedModule")
            .field(&format_args!("{:p}", self.as_ptr()))
            .finish()
    }
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
