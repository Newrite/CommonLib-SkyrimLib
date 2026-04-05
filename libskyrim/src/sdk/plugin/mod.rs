//! Plugin bootstrap and runtime services.
//!
//! This domain is the SDK-facing home for the kinds of code that usually live
//! in a plugin's `main.cpp` or bootstrap module:
//!
//! - initialization and SKSE entry wiring
//! - log setup and log-level control
//! - lifecycle/message registration
//! - config/INI parsing
//! - co-save schema/model installation
//! - queued task handoff into safer execution phases
//!
//! If you are starting a new plugin, this is usually the first SDK module to
//! read alongside [`crate::sdk::papyrus`] or [`crate::sdk::events`], depending
//! on how the plugin exposes itself.
//!
//! Typical flow:
//!
//! 1. initialize through [`entry`] helpers
//! 2. install lifecycle/message listeners
//! 3. register serialization models
//! 4. queue deferred work through [`task`]
//!
//! Decision guide:
//!
//! - start with [`entry`] inside `SKSEPluginLoad` when the plugin is wiring the
//!   initial `LoadInterface` and optional trampoline allocation
//! - use [`lifecycle`] when code should be keyed to the familiar
//!   `PostLoad`/`DataLoaded`/save-load phases instead of generic message kinds
//! - use [`messaging`] when the plugin genuinely cares about sender filtering,
//!   raw message payloads, or lower-level `SKSE::MessagingInterface` details
//! - use [`config`] during bootstrap or reload boundaries, before gameplay
//!   installation fans out across the rest of the plugin
//! - use [`serialization`] when the plugin owns co-save state or versioned
//!   model registration
//! - use [`task`] when lifecycle/Papyrus/event callbacks should hand work off
//!   into a safer queued execution point
//!
//! Common bootstrap recipe:
//!
//! 1. call [`init`] or [`init_with_log`] in `SKSEPluginLoad`
//! 2. allocate trampoline space if the plugin installs manual hooks
//! 3. attach `PostLoad`/`DataLoaded` listeners through [`lifecycle`]
//! 4. resolve config/forms and register serialization models
//! 5. hand heavier gameplay/UI work off through [`task`]
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::plugin;
//!
//! fn bootstrap() {
//!     let _ = plugin::on_post_load(|_message| {
//!         let _ = plugin::load_ini("Data/SKSE/Plugins/Example.ini");
//!     });
//!
//!     let _ = plugin::on_data_loaded(|_message| {
//!         plugin::queue_gameplay_task(|| {
//!             // late initialization that should not run directly in the message callback
//!         });
//!     });
//! }
//! ```

pub mod config;
pub mod entry;
pub mod lifecycle;
pub mod log;
pub mod messaging;
pub mod serialization;
pub mod task;

/// Config/INI loading, typed field access, and hotkey parsing helpers.
pub use config::{
    Config, ConfigValueError, HotkeyCombo, HotkeyParseError, Ini, bool_value, config, f32_value,
    form_value, form_value_typed, has_field, has_section, hotkey_value, i32_value, load_ini,
    raw_value, u32_value, write_ini,
};

/// SKSE entry/bootstrap helpers for `SKSEPluginLoad`.
pub use entry::{LoadInterface, PluginHandle, PluginInfo, alloc_trampoline, init, init_with_log};

/// Lifecycle-oriented messaging helpers keyed to common plugin/game phases.
pub use lifecycle::{
    on_data_loaded, on_delete_game, on_game_lifecycle, on_input_loaded, on_lifecycle, on_new_game,
    on_plugin_phase, on_post_load, on_post_load_game, on_post_post_load, on_pre_load_game,
    on_save_game,
};

/// Logging setup and level-control helpers for plugin bootstrap/runtime code.
pub use log::{
    LogLevel, LogType, enabled, level, set_level, set_level_from_ini, set_level_from_str,
};

/// Lower-level SKSE messaging helpers and typed message wrappers.
pub use messaging::{
    GameLifecyclePhase, LifecyclePhase, Message, MessageKind, MessageListener, MessageRef,
    PluginLifecyclePhase, TypedMessageRef, TypedMessageSliceRef, on, on_filtered,
    on_game_lifecycle as on_message_game_lifecycle, on_game_lifecycle_filtered,
    on_game_lifecycle_raw, on_game_lifecycle_sender, on_game_lifecycle_sender_str,
    on_lifecycle as on_message_lifecycle, on_lifecycle_filtered, on_lifecycle_raw,
    on_lifecycle_sender, on_lifecycle_sender_str, on_plugin_phase as on_message_plugin_phase,
    on_plugin_phase_filtered, on_plugin_phase_raw, on_plugin_phase_sender,
    on_plugin_phase_sender_str, on_raw, on_sender, on_sender_str, plugin_handle, register_listener,
    register_listener_dyn,
};

/// Registered model/schema serialization API plus the underlying typed cosave
/// building blocks.
pub use serialization::{
    BoundedVec, BoundedVecError, Cosave, CosaveDecode, CosaveEncode, LoadContext, LoadError,
    LoadStatus, LoadedRecord, MigratingRecordBuilder, Model, ModelAccessError, OwnedRecord,
    RecordHeader, RecordId, RecordReader, RecordWriter, RegistrationError, ResolvedFormId,
    ResolvedVmHandle, RuntimeError, SaveError, Schema, SchemaBuildError, UniqueId, fourcc_from_str,
    has_registered_model, last_runtime_error, read_next_record, record_id, register_model,
    registered_model_unique_id, schema_fields, take_last_runtime_error, unique_id,
    unregister_model, with_registered_model, with_registered_model_mut, write_value_record,
};

/// Deferred task handoff helpers for lifecycle, event, Papyrus, and UI flows.
pub use task::{
    TaskHandoff, TaskHandoffOrigin, TaskQueueKind, add_task, add_ui_task, background_handoff,
    event_handoff, gameplay_event_handoff, gameplay_handoff, gameplay_papyrus_handoff,
    papyrus_handoff, queue_event_task, queue_gameplay_event_task, queue_gameplay_papyrus_task,
    queue_gameplay_task, queue_gameplay_task_resolving_handle,
    queue_gameplay_task_resolving_target, queue_papyrus_task, queue_task_resolving_handle,
    queue_task_resolving_target, queue_ui_event_task, queue_ui_papyrus_task,
    queue_ui_task_resolving_handle, queue_ui_task_resolving_target, ui_event_handoff, ui_handoff,
    ui_papyrus_handoff,
};
