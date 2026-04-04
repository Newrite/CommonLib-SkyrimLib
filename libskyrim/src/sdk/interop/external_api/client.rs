use core::ffi::{CStr, c_void};
use core::ptr::NonNull;

use super::callbacks::{ExportedCallbackRegistrar, ExportedSubscriber};
use super::module::LoadedModule;
use super::types::{ModuleError, RequestApiError, SymbolError};

/// Resolve a loaded module by DLL name.
#[inline(always)]
pub fn loaded_module(dll_name: &str) -> Result<LoadedModule, ModuleError> {
    LoadedModule::get(dll_name)
}

/// Resolve the current process module.
#[inline(always)]
pub fn current_module() -> Result<LoadedModule, ModuleError> {
    LoadedModule::current()
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

/// Resolve a raw exported symbol from a loaded module when present.
#[inline(always)]
pub fn symbol_optional_raw(
    dll_name: &str,
    symbol_name: &CStr,
) -> Result<Option<NonNull<c_void>>, SymbolError> {
    Ok(loaded_module(dll_name)?.symbol_raw_optional(symbol_name))
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

/// Resolve an exported symbol from a loaded module when present and reinterpret
/// it as `T`.
///
/// # Safety
/// The caller must ensure that the export, when present, really has the ABI
/// and layout of `T`.
#[inline(always)]
pub unsafe fn symbol_optional<T>(
    dll_name: &str,
    symbol_name: &CStr,
) -> Result<Option<T>, SymbolError>
where
    T: Copy,
{
    unsafe { loaded_module(dll_name)?.symbol_optional(symbol_name) }
}

/// Resolve an exported symbol from a loaded plugin module by plugin name when
/// present and reinterpret it as `T`.
///
/// This helper automatically appends `.dll` when needed.
///
/// # Safety
/// The caller must ensure that the export, when present, really has the ABI
/// and layout of `T`.
#[inline(always)]
pub unsafe fn plugin_symbol_optional<T>(
    plugin_name: &str,
    symbol_name: &CStr,
) -> Result<Option<T>, SymbolError>
where
    T: Copy,
{
    unsafe { loaded_plugin_module(plugin_name)?.symbol_optional(symbol_name) }
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

/// Request a typed plugin service from a loaded DLL using a getter-style
/// exported symbol.
///
/// # Safety
/// The caller must ensure that the exported symbol has the expected ABI and
/// that the returned interface really is a live `T`.
#[inline(always)]
pub unsafe fn request_service<T>(
    dll_name: &str,
    symbol_name: &CStr,
) -> Result<NonNull<T>, RequestApiError> {
    unsafe { loaded_module(dll_name)?.request_service(symbol_name) }
}

/// Request a typed plugin service from a loaded plugin module by plugin name
/// using a getter-style exported symbol.
///
/// This helper automatically appends `.dll` when needed.
///
/// # Safety
/// The caller must ensure that the exported symbol has the expected ABI and
/// that the returned interface really is a live `T`.
#[inline(always)]
pub unsafe fn request_service_for_plugin<T>(
    plugin_name: &str,
    symbol_name: &CStr,
) -> Result<NonNull<T>, RequestApiError> {
    unsafe { loaded_plugin_module(plugin_name)?.request_service(symbol_name) }
}

/// Request one optional typed plugin service from a loaded DLL using a
/// getter-style exported symbol.
///
/// Missing symbols and null service pointers both map to `Ok(None)`.
///
/// # Safety
/// The caller must ensure that the exported symbol, when present, has the
/// expected ABI and that the returned interface really is a live `T`.
#[inline(always)]
pub unsafe fn request_service_optional<T>(
    dll_name: &str,
    symbol_name: &CStr,
) -> Result<Option<NonNull<T>>, SymbolError> {
    unsafe { loaded_module(dll_name)?.request_service_optional(symbol_name) }
}

/// Request one optional typed plugin service from a loaded plugin module by
/// plugin name using a getter-style exported symbol.
///
/// This helper automatically appends `.dll` when needed.
///
/// Missing symbols and null service pointers both map to `Ok(None)`.
///
/// # Safety
/// The caller must ensure that the exported symbol, when present, has the
/// expected ABI and that the returned interface really is a live `T`.
#[inline(always)]
pub unsafe fn request_service_optional_for_plugin<T>(
    plugin_name: &str,
    symbol_name: &CStr,
) -> Result<Option<NonNull<T>>, SymbolError> {
    unsafe { loaded_plugin_module(plugin_name)?.request_service_optional(symbol_name) }
}

/// Resolve one exported register/unregister callback pair from a loaded DLL.
///
/// # Safety
/// The caller must ensure that both exports have the expected ABI and that the
/// registration ids belong to the matching unregister export.
#[inline(always)]
pub unsafe fn callback_registrar<Callback, Id>(
    dll_name: &str,
    register_symbol: &CStr,
    unregister_symbol: &CStr,
) -> Result<ExportedCallbackRegistrar<Callback, Id>, SymbolError> {
    unsafe { loaded_module(dll_name)?.callback_registrar(register_symbol, unregister_symbol) }
}

/// Resolve one exported register/unregister callback pair from a loaded plugin
/// module by plugin name.
///
/// This helper automatically appends `.dll` when needed.
///
/// # Safety
/// The caller must ensure that both exports have the expected ABI and that the
/// registration ids belong to the matching unregister export.
#[inline(always)]
pub unsafe fn callback_registrar_for_plugin<Callback, Id>(
    plugin_name: &str,
    register_symbol: &CStr,
    unregister_symbol: &CStr,
) -> Result<ExportedCallbackRegistrar<Callback, Id>, SymbolError> {
    unsafe {
        loaded_plugin_module(plugin_name)?.callback_registrar(register_symbol, unregister_symbol)
    }
}

/// Resolve one flat exported subscriber-style symbol from a loaded DLL.
///
/// # Safety
/// The caller must ensure that the export has the expected ABI and that the
/// argument type matches the remote plugin's ownership contract.
#[inline(always)]
pub unsafe fn subscriber<Arg>(
    dll_name: &str,
    symbol_name: &CStr,
) -> Result<ExportedSubscriber<Arg>, SymbolError> {
    unsafe { loaded_module(dll_name)?.subscriber(symbol_name) }
}

/// Resolve one flat exported subscriber-style symbol from a loaded plugin
/// module by plugin name.
///
/// This helper automatically appends `.dll` when needed.
///
/// # Safety
/// The caller must ensure that the export has the expected ABI and that the
/// argument type matches the remote plugin's ownership contract.
#[inline(always)]
pub unsafe fn subscriber_for_plugin<Arg>(
    plugin_name: &str,
    symbol_name: &CStr,
) -> Result<ExportedSubscriber<Arg>, SymbolError> {
    unsafe { loaded_plugin_module(plugin_name)?.subscriber(symbol_name) }
}
