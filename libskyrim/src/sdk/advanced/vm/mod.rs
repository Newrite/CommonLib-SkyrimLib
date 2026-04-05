//! Advanced script VM and BSScript helpers.
//!
//! This layer is intentionally lower-level than `sdk::papyrus`: it focuses on
//! live VM/runtime operations such as bound-object lookup, property access, and
//! immediate method/static dispatch.
//!
//! Reach for this module when a plugin already has script names, handles, or
//! bound objects and needs to talk to the runtime VM immediately. Prefer
//! [`crate::sdk::papyrus`] for authoring new functions/classes/modules.
//!
//! Common workflows:
//!
//! - convert a form into a [`crate::re::VMHandle`] with [`form_handle`]
//! - send an event to one handle or every bound script with [`send_event`] /
//!   [`send_event_all`]
//! - resolve or create a bound object with [`find_form_bound_object`] /
//!   [`find_or_create_form_bound_object`]
//! - read or write bound state with [`form_bound_property_value`] or
//!   [`set_object_property`]
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::advanced::vm;
//! use libskyrim::re::TESForm;
//!
//! fn refresh_bound_script(form: &TESForm) {
//!     let Some(handle) = vm::form_handle(form) else {
//!         return;
//!     };
//!
//!     let _ = vm::send_event(handle, "OnSdkRefresh", ());
//!     let _ = vm::dispatch_form_method_call(form, "ExampleScript", "RefreshNow", ());
//! }
//! ```

mod access;
mod binding;
mod dispatch;
mod shared;
mod types;

pub use access::*;
pub use binding::*;
pub use dispatch::*;
pub use types::*;

#[cfg(test)]
mod tests;
