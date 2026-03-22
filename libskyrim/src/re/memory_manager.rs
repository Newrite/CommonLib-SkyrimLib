//! Forward declaration for `RE::MemoryManager`.
//!
//! The actual engine calls are routed through the C++ bridge
//! (`commonlib_memory_manager_get_singleton` / `commonlib_memory_manager_get_thread_scrap_heap`).
//! This module only provides the opaque type marker for type safety.

core_util::abstract_type! {
    /// Opaque type representing the Skyrim engine's `MemoryManager`.
    ///
    /// Engine calls go through the C++ bridge in `bridge.cpp`.
    pub type MemoryManager;
}
