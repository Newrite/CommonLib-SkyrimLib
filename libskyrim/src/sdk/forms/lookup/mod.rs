//! High-level form lookup helpers.
//!
//! This module gathers the frequent `TESDataHandler`, plugin-file, editor-ID,
//! and `"Plugin.esp|0x123"` workflows that repeatedly show up in gameplay and
//! config-driven plugins.
//!
//! Decision guide:
//!
//! - use [`lookup_form_spec`] or [`lookup_form_spec_typed`] when config or user
//!   input may contain either `Plugin.esp|0x123` specs or plain editor IDs;
//! - use [`lookup_form`] / [`lookup_form_typed`] when the plugin already has a
//!   local form ID plus plugin file name and wants normal plugin-index
//!   expansion;
//! - use [`lookup_raw_form`] / [`lookup_raw_form_typed`] when the plugin
//!   intentionally wants raw form IDs without local-ID expansion;
//! - use [`lookup_persistent_form_spec`] when resolved forms should become
//!   plugin-owned long-lived state.

mod editor_ids;
mod handler;
mod shared;
mod specs;

/// Editor-ID lookup helpers and editor-ID extraction accessors.
pub use editor_ids::{editor_id, lookup_editor_id, lookup_editor_id_typed, try_editor_id};

/// `TESDataHandler` and plugin-file-based lookup helpers.
pub use handler::{
    data_handler, loaded_plugin_file, loaded_plugin_index, lookup_form, lookup_form_typed,
    lookup_raw_form, lookup_raw_form_typed, plugin_file, plugin_index, plugin_loaded,
    resolve_local_form_id, resolve_raw_form_id,
};

/// Parsed string spec helpers such as `"Plugin.esp|0x123"` plus mixed
/// editor-ID/form-ID lookup entrypoints.
pub use specs::{
    ParsePluginFormSpecError, PluginFormSpec, lookup_form_spec, lookup_form_spec_typed,
    lookup_persistent_form_spec, lookup_raw_form_spec, lookup_raw_form_spec_typed,
    parse_plugin_form_spec, require_persistent_form_spec,
};
