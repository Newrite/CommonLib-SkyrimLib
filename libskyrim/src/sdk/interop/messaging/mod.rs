//! Plugin-to-plugin messaging helpers built on `SKSE::MessagingInterface`.
//!
//! This module focuses on two recurring patterns:
//!
//! - install-once listeners for non-SKSE plugin messages
//! - synchronous request / response protocols built on message payloads
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
