//! Forward declaration for `RE::ScrapHeap`.
//!
//! The actual engine calls are routed through the C++ bridge
//! (`commonlib_scrap_heap_allocate` / `commonlib_scrap_heap_deallocate`).
//! This module only provides the opaque type marker for type safety.

core_util::abstract_type! {
    /// Opaque type representing the Skyrim engine's `ScrapHeap`.
    ///
    /// Used by `BSScrapArrayAllocator` for per-thread temporary allocations.
    /// Engine calls go through the C++ bridge in `bridge.cpp`.
    pub type ScrapHeap;
}
