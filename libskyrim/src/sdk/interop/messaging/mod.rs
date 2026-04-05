//! Plugin-to-plugin messaging helpers built on `SKSE::MessagingInterface`.
//!
//! This module focuses on two recurring patterns:
//!
//! - install-once listeners for non-SKSE plugin messages
//! - synchronous request / response protocols built on message payloads
//!
//! Reach for [`crate::sdk::interop::external_api`] when another plugin exposes
//! exports or service getters. Reach for `sdk::interop::messaging` when the
//! contract is driven by `SKSE::MessagingInterface`, listener installation, or
//! request/response packets sent through message payloads.
//!
//! Decision guide:
//!
//! - use this module when the contract is fundamentally message-driven
//! - use [`crate::sdk::interop::external_api`] when the dependency already
//!   exports a stable callable surface
//! - use `sdk::plugin::messaging` when the plugin only cares about its own
//!   SKSE lifecycle/custom messages rather than a reusable inter-plugin
//!   protocol
//!
//! Typical protocol flow:
//!
//! 1. install a listener or `RequestServer`
//! 2. negotiate compatibility through [`ApiVersion`] and
//!    [`VersionHandshake`] when needed
//! 3. exchange typed payloads through dispatch/query helpers
//! 4. keep any exported API loading in `external_api`, not here
//!
//! Smaller fire-and-forget protocols can stay at the listener/dispatch layer:
//!
//! ```rust,ignore
//! use libskyrim::sdk::interop::messaging::{dispatch_value, listen_type_sender};
//!
//! const MSG_RELOAD: u32 = 0xCAFE;
//!
//! fn install_reload_listener() {
//!     let _ = listen_type_sender(
//!         MSG_RELOAD,
//!         c\"ExamplePlugin\",
//!         |_message| {
//!             // reload internal caches
//!         },
//!     );
//! }
//!
//! fn request_reload() {
//!     let _ = dispatch_value(c\"ExamplePlugin\", MSG_RELOAD, &1u32);
//! }
//! ```
//!
//! Example server:
//!
//! ```rust,ignore
//! use libskyrim::sdk::interop::messaging::{
//!     ApiVersion, QueryResponse, RequestMessageIds, RequestServer,
//! };
//!
//! #[repr(C)]
//! #[derive(Clone, Copy)]
//! struct PingRequest {
//!     value: u32,
//! }
//!
//! #[repr(C)]
//! #[derive(Clone, Copy)]
//! struct PongResponse {
//!     doubled: u32,
//! }
//!
//! const MSG_VERSION: u32 = 0x9000;
//! const MSG_QUERY: u32 = 0x9001;
//! const REQ_PING: u32 = 1;
//!
//! fn install_server(
//! ) -> Result<(), libskyrim::sdk::interop::messaging::ListenerInstallError> {
//!     let _server = RequestServer::builder(
//!         ApiVersion::new(1, 0),
//!         RequestMessageIds::new(MSG_VERSION, MSG_QUERY),
//!     )
//!     .handle_value(REQ_PING, |_context, request: &PingRequest| {
//!         Ok(PongResponse {
//!             doubled: request.value * 2,
//!         })
//!     })
//!     .install()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Matching client:
//!
//! ```rust,ignore
//! use libskyrim::sdk::interop::messaging::{ApiVersion, ClientError, RequestClient, RequestMessageIds};
//!
//! # #[repr(C)]
//! # #[derive(Clone, Copy)]
//! # struct PingRequest {
//! #     value: u32,
//! # }
//! # #[repr(C)]
//! # #[derive(Clone, Copy)]
//! # struct PongResponse {
//! #     doubled: u32,
//! # }
//! # const MSG_VERSION: u32 = 0x9000;
//! # const MSG_QUERY: u32 = 0x9001;
//! # const REQ_PING: u32 = 1;
//! fn ping_server() -> Result<u32, ClientError> {
//!     let client = RequestClient::new_str(
//!         "ExampleServer",
//!         ApiVersion::new(1, 0),
//!         RequestMessageIds::new(MSG_VERSION, MSG_QUERY),
//!     )
//!     .expect("plugin names used for SKSE messaging cannot contain NUL");
//!
//!     client.check_version()?;
//!     let response: PongResponse = client.query(REQ_PING, &PingRequest { value: 21 })?;
//!     Ok(response.doubled)
//! }
//! ```

mod client;
mod listener;
mod payload;
mod server;
mod types;

pub use client::*;
pub use listener::*;
pub use payload::*;
pub use server::*;
pub use types::*;
