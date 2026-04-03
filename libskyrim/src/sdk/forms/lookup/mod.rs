//! High-level form lookup helpers.
//!
//! This module gathers the frequent `TESDataHandler`, plugin-file, editor-ID,
//! and `"Plugin.esp|0x123"` workflows that repeatedly show up in gameplay and
//! config-driven plugins.

mod editor_ids;
mod handler;
mod shared;
mod specs;

pub use editor_ids::{editor_id, lookup_editor_id, lookup_editor_id_typed, try_editor_id};
pub use handler::{
    data_handler, loaded_plugin_file, loaded_plugin_index, lookup_form, lookup_form_typed,
    lookup_raw_form, lookup_raw_form_typed, plugin_file, plugin_index, plugin_loaded,
    resolve_local_form_id, resolve_raw_form_id,
};
pub use specs::{
    ParsePluginFormSpecError, PluginFormSpec, lookup_form_spec, lookup_form_spec_typed,
    lookup_persistent_form_spec, lookup_raw_form_spec, lookup_raw_form_spec_typed,
    parse_plugin_form_spec, require_persistent_form_spec,
};
