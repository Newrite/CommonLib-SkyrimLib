use core::ffi::c_void;

extern "C" {
    // ── Initialization ──────────────────────────────────────────────────────
    pub fn init_commonlib(skse_interface: *const c_void);

    // ── Address Library ─────────────────────────────────────────────────────
    pub fn commonlib_id_to_address(id: usize) -> usize;
    pub fn commonlib_offset_to_address(offset: usize) -> usize;

    // ── Memory patching ─────────────────────────────────────────────────────
    pub fn commonlib_safe_write(addr: usize, data: *const u8, len: usize);
    pub fn commonlib_safe_fill(addr: usize, value: u8, len: usize);

    // ── Trampolines (Hooks) ─────────────────────────────────────────────────
    pub fn commonlib_write_branch5(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_branch6(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_call5(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_call6(src: usize, dst: usize) -> usize;

    pub fn commonlib_alloc_trampoline(size: usize);
    pub fn commonlib_trampoline_allocate(size: usize) -> *mut u8;

    // ── VTable ──────────────────────────────────────────────────────────────
    pub fn commonlib_write_vfunc(vtable_addr: usize, idx: usize, new_func: usize) -> usize;

    // ── Task Interface ──────────────────────────────────────────────────────
    pub fn commonlib_add_task(cb: extern "C" fn(*mut c_void), data: *mut c_void);
    pub fn commonlib_add_ui_task(cb: extern "C" fn(*mut c_void), data: *mut c_void);

    // ── Engine Memory (RE::malloc / RE::free wrappers) ──────────────────────
    /// `RE::malloc(size)` — allocates from the engine's default heap.
    pub fn commonlib_malloc(size: usize) -> *mut c_void;
    /// `RE::free(ptr)` — frees memory allocated by `RE::malloc`.
    pub fn commonlib_free(ptr: *mut c_void);
    /// `RE::aligned_alloc(alignment, size)` — aligned allocation from the engine heap.
    pub fn commonlib_aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
    /// `RE::aligned_free(ptr)` — frees aligned memory.
    pub fn commonlib_aligned_free(ptr: *mut c_void);
    /// `RE::calloc(count, size)` — allocates zeroed memory.
    pub fn commonlib_calloc(count: usize, size: usize) -> *mut c_void;
    /// `RE::realloc(ptr, new_size)` — reallocates memory.
    pub fn commonlib_realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void;

    // ── MemoryManager ───────────────────────────────────────────────────────
    /// `RE::MemoryManager::GetSingleton()` — returns the global MemoryManager.
    pub fn commonlib_memory_manager_get_singleton() -> *mut c_void;
    /// `mgr->GetThreadScrapHeap()` — returns the current thread's ScrapHeap.
    pub fn commonlib_memory_manager_get_thread_scrap_heap(mgr: *mut c_void) -> *mut c_void;

    // ── ScrapHeap ───────────────────────────────────────────────────────────
    /// `heap->Allocate(size, alignment)` — allocates from a ScrapHeap.
    pub fn commonlib_scrap_heap_allocate(heap: *mut c_void, size: usize, alignment: usize) -> *mut c_void;
    /// `heap->Deallocate(mem)` — deallocates from a ScrapHeap.
    pub fn commonlib_scrap_heap_deallocate(heap: *mut c_void, mem: *mut c_void);
}

