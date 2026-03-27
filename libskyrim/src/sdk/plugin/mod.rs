//! Plugin bootstrap and runtime services.

pub mod config;
pub mod entry;
pub mod lifecycle;
pub mod log;
pub mod messaging;
pub mod serialization;
pub mod task;

pub use config::{Ini, load_ini, write_ini};
pub use entry::{LoadInterface, PluginHandle, PluginInfo, alloc_trampoline, init, init_with_log};
pub use log::{
    LogLevel, LogType, enabled, level, set_level, set_level_from_ini, set_level_from_str,
};
pub use messaging::{Message, plugin_handle, register_listener};
pub use task::{add_task, add_ui_task};
