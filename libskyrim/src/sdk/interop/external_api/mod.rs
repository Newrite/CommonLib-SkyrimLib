//! External plugin API request and publication helpers.
//!
//! This module focuses on the recurring Skyrim plugin pattern:
//!
//! - find a loaded plugin DLL
//! - locate an exported API request symbol
//! - request a versioned interface pointer
//! - negotiate one interface pointer through plugin messaging
//! - publish a small versioned C ABI service table from a Rust plugin
//! - keep the low-level calling convention honest while removing repeated
//!   Windows boilerplate from plugin code
//!
//! Reach for:
//!
//! - `request_plugin_api*` when the dependency exports `RequestPluginAPI`
//! - `request_service*` when the dependency exports one getter-style symbol
//! - `callback_registrar*` when the dependency exports a register/unregister
//!   callback pair
//! - `subscriber*` when the dependency exports one flat subscriber symbol
//! - `InterfaceLoader*` helpers when negotiation happens through plugin
//!   messaging instead of exported functions
//! - `export_plugin_api!` / `export_plugin_symbol!` / provider helpers when
//!   your plugin is the API provider
//!
//! Decision guide:
//!
//! - use `request_plugin_api*` for versioned `RequestPluginAPI` exports that
//!   return one service table or interface pointer
//! - use `request_service*` for one-off getter-style exports such as
//!   `GetExampleApi`
//! - use `callback_registrar*` when the dependency exposes
//!   `register/unregister` symbol pairs and the plugin wants an RAII-style
//!   registration token
//! - use `subscriber*` when the dependency exports one flat subscriber entry
//!   point instead of a register/unregister pair
//! - use the `InterfaceLoader*` messaging helpers when the dependency becomes
//!   ready only after a lifecycle message or explicit handshake
//!
//! Correct boundary:
//!
//! This module loads and organizes inter-plugin APIs, but it does not remove
//! the need for a plugin-local C++ shim when the external API itself is a
//! non-flat C++ interface using `virtual`, `std::function`, `std::vector`, or
//! overloaded methods.
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
//!
//! Flat callback export pattern:
//!
//! ```rust,ignore
//! use libskyrim::sdk::interop::external_api;
//!
//! type HudCallback = unsafe extern "system" fn(i32);
//!
//! unsafe extern "system" fn on_hud_mode_changed(_mode: i32) {}
//!
//! fn install_dependency_callback() -> Result<(), external_api::SymbolError> {
//!     let registrar = unsafe {
//!         external_api::callback_registrar_for_plugin::<HudCallback, u32>(
//!             "SomeHudPlugin",
//!             "RegisterHudCallback",
//!             "UnregisterHudCallback",
//!         )?
//!     };
//!
//!     let _registration = unsafe { registrar.register(on_hud_mode_changed) };
//!     Ok(())
//! }
//! ```

mod callbacks;
mod client;
mod macros;
mod messaging;
mod module;
mod provider;
mod types;

pub use callbacks::*;
pub use client::*;
pub use macros::*;
pub use messaging::*;
pub use module::*;
pub use provider::*;
pub use types::*;

#[cfg(test)]
mod tests;
