//! Advanced engine-facing SDK domains.
//!
//! These modules are intentionally separate from the early default SDK surface.
//! They are expected to wrap more volatile or engine-heavy families.
//!
//! Reach for `sdk::advanced` when the workflow is real and repeated, but still
//! too engine-heavy to belong in the early default authoring path:
//!
//! - physics/Havok queries
//! - direct VM/runtime script interaction
//! - future render/scene helpers once a stable abstraction boundary is clearer
//!
//! Compared with the rest of the SDK, these modules are more likely to expose
//! lower-level seams and to keep narrower, more source-shaped APIs.
//!
//! Rule of thumb:
//!
//! - start with [`crate::sdk::papyrus`] for authoring/registration-oriented
//!   script work
//! - move to [`vm`] when the plugin needs live runtime calls, bound-object
//!   access, or direct property reads/writes
//! - start with [`crate::sdk::gameplay`] for ordinary actor/world workflows
//! - move to [`physics`] when the plugin needs Havok-aware raycasts, line of
//!   sight, or spawn validation
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::advanced::{physics, vm};
//! use libskyrim::re::Actor;
//!
//! fn ping_target_script(target: &Actor) {
//!     if let Some(handle) = vm::form_handle(target) {
//!         let _ = vm::send_event(handle, "OnSdkPing", ());
//!     }
//!
//!     let _ = physics::has_line_of_sight(target, target);
//! }
//! ```

pub mod physics;
pub mod render;
pub mod scene;
pub mod vm;
