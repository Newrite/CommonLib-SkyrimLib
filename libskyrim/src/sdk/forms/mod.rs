//! Form lookup and form-centric utilities for plugin authors.
//!
//! This domain collects the repeated utility code that many plugins rebuild
//! around Skyrim forms:
//!
//! - editor-ID and `plugin:FormID` lookup
//! - persistent form wrappers for cross-save/plugin-owned state
//! - keyword and list traversal/mutation
//! - typed `Setting` access and reload/save helpers
//!
//! In practice this layer is most useful when configuration, gameplay
//! installation, and runtime lookups need to cooperate. The individual pieces
//! live in:
//!
//! - [`lookup`] for string/spec based resolution
//! - [`persistent`] for stored form wrappers
//! - [`keywords`] and [`lists`] for common form-container workflows
//! - [`settings`] for typed `Setting` access
//!
//! A common plugin-facing flow looks like this:
//!
//! 1. resolve forms from config through [`lookup`]
//! 2. convert required plugin-owned forms into [`PersistentForm`] or
//!    [`PersistentFormPtr`]
//! 3. use [`keywords`] / [`lists`] helpers during installation or runtime
//! 4. use [`settings`] when gameplay/config state needs to touch engine
//!    `Setting` collections
//!
//! Decision guide:
//!
//! - use [`lookup`] when the plugin is still dealing with strings, editor IDs,
//!   or `plugin:FormID` specs
//! - use [`PersistentForm`] / [`PersistentFormPtr`] when the resolved form
//!   becomes part of long-lived plugin state
//! - use [`keywords`] or [`lists`] when the plugin is traversing or mutating a
//!   form-owned container, not just resolving one form once
//! - use [`settings`] when config/bootstrap logic must also read or write
//!   engine `Setting` values
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::re::BGSKeyword;
//! use libskyrim::sdk::forms;
//!
//! fn require_keyword(spec: &str) -> forms::PersistentForm<BGSKeyword> {
//!     forms::lookup::require_persistent_form_spec::<BGSKeyword>(
//!         spec,
//!         "Gameplay.RequiredKeyword",
//!     )
//! }
//! ```
//!
//! Config-driven install recipe:
//!
//! 1. read the raw field through [`crate::sdk::plugin::config`]
//! 2. resolve it through [`lookup`]
//! 3. upgrade it into [`PersistentForm`] if the plugin should keep it
//! 4. use [`keywords`] / [`lists`] / gameplay helpers during installation
//!    without re-parsing the original string again

mod shared;

pub mod keywords;
pub mod lists;
pub mod lookup;
pub mod persistent;
pub mod settings;

pub use persistent::{PersistentForm, PersistentFormPtr};
