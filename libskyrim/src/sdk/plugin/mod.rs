//! Plugin bootstrap and runtime services.

pub mod config;
pub mod entry;
pub mod lifecycle;
pub mod log;
pub mod messaging;
pub mod serialization;
pub mod task;

pub use config::{
    Config, ConfigValueError, HotkeyCombo, HotkeyParseError, Ini, bool_value, config, f32_value,
    form_value, form_value_typed, has_field, has_section, hotkey_value, i32_value, load_ini,
    raw_value, u32_value, write_ini,
};
pub use entry::{LoadInterface, PluginHandle, PluginInfo, alloc_trampoline, init, init_with_log};
pub use lifecycle::{
    on_data_loaded, on_delete_game, on_game_lifecycle, on_input_loaded, on_lifecycle, on_new_game,
    on_plugin_phase, on_post_load, on_post_load_game, on_post_post_load, on_pre_load_game,
    on_save_game,
};
pub use log::{
    LogLevel, LogType, enabled, level, set_level, set_level_from_ini, set_level_from_str,
};
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
pub use serialization::{
    BoundedVec, BoundedVecError, Cosave, CosaveDecode, CosaveEncode, LoadContext, LoadError,
    LoadStatus, LoadedRecord, MigratingRecordBuilder, Model, ModelAccessError, OwnedRecord,
    RecordHeader, RecordId, RecordReader, RecordWriter, RegistrationError, ResolvedFormId,
    ResolvedVmHandle, RuntimeError, SaveError, Schema, SchemaBuildError, UniqueId, fourcc_from_str,
    is_registered, last_error, read_next_record, record_id, register_model, registered_unique_id,
    schema_fields, take_last_error, unique_id, unregister_model, with_model, with_model_mut,
    write_value_record,
};
pub use task::{
    TaskHandoff, TaskHandoffOrigin, TaskQueueKind, add_event_task, add_gameplay_event_task,
    add_gameplay_papyrus_task, add_gameplay_task, add_gameplay_task_resolving_handle,
    add_gameplay_task_resolving_target, add_papyrus_task, add_task, add_task_resolving_handle,
    add_task_resolving_target, add_ui_event_task, add_ui_papyrus_task, add_ui_task,
    add_ui_task_resolving_handle, add_ui_task_resolving_target, gameplay_handoff,
    gameplay_handoff_from_event, gameplay_handoff_from_papyrus, handoff, handoff_from_event,
    handoff_from_papyrus, ui_handoff, ui_handoff_from_event, ui_handoff_from_papyrus,
};
