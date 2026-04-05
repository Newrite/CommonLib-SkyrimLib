//! SKSE-owned event helper domains.
//!
//! The SKSE layer exposes two distinct event systems:
//!
//! - dispatcher-backed `BSTEventSource<T>` values such as `ActionEvent`
//! - plugin messaging listeners through `MessagingInterface`
//!
//! These stay separate here because they have different ownership, lifetime,
//! and unregister semantics.
//!
//! Rule of thumb:
//!
//! - use [`dispatchers`] when the source is a true engine event dispatcher and
//!   the plugin wants RAII-style sink registration
//! - use [`messages`] when the trigger is an SKSE lifecycle/message packet such
//!   as `PostLoad`, `DataLoaded`, `SaveGame`, or `NewGame`
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::core::PluginLifecyclePhase;
//! use libskyrim::sdk::events::skse::{dispatchers, messages};
//!
//! fn install() {
//!     let _listener = messages::on_plugin_phase(PluginLifecyclePhase::DataLoaded, |_message| {
//!         // refresh caches after data load
//!     });
//!
//!     let _camera = dispatchers::subscribe(|_event: &libskyrim::re::CameraEvent| {
//!         libskyrim::sdk::events::EventFlow::Continue
//!     });
//! }
//! ```

pub mod dispatchers;
pub mod messages;
