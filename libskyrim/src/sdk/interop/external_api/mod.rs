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
