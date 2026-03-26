use core::ffi::c_void;
use core::mem::MaybeUninit;

pub type CommonlibBstEventSinkProcessCallback =
    unsafe extern "C" fn(ctx: *mut c_void, event: *const c_void, event_source: *mut c_void) -> i32;

pub type CommonlibDestroyCallback = unsafe extern "C" fn(ctx: *mut c_void);

unsafe extern "C" {
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
    pub fn commonlib_scrap_heap_allocate(
        heap: *mut c_void,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
    /// `heap->Deallocate(mem)` – deallocates from a ScrapHeap.
    pub fn commonlib_scrap_heap_deallocate(heap: *mut c_void, mem: *mut c_void);

    // TODO: Remove When RE-Implement actoe
    // Actor helpers
    /// `actor->GetGoldAmount(no_init)` for cases where CommonLib exposes only inline wrappers.
    pub fn commonlib_actor_get_gold_amount(actor: *mut c_void, no_init: bool) -> i32;

    // Smart-pointer out-param construction helpers.
    pub fn commonlib_make_hkref_hk_referenced_object(out: *mut c_void) -> bool;
    pub fn commonlib_make_nismart_ni_ref_object(out: *mut c_void) -> bool;

    // Input device destruction helper.
    pub fn commonlib_destroy_bsi_input_device(device: *mut c_void);

    // Input event factories.
    pub fn commonlib_button_event_create(
        input_device: i32,
        user_event: *const c_void,
        id_code: u32,
        value: f32,
        held_down_secs: f32,
    ) -> *mut c_void;

    // Generic BSTEventSink bridge.
    pub fn commonlib_bst_event_sink_create(
        ctx: *mut c_void,
        process: Option<CommonlibBstEventSinkProcessCallback>,
        destroy: Option<CommonlibDestroyCallback>,
    ) -> *mut c_void;
    pub fn commonlib_bst_event_sink_destroy(sink: *mut c_void);

    // SKSE event source getters.
    pub fn commonlib_skse_get_serialization_interface() -> *mut c_void;
    pub fn commonlib_skse_get_trampoline_interface() -> *mut c_void;
    pub fn commonlib_skse_get_mod_callback_event_source() -> *mut c_void;
    pub fn commonlib_skse_get_camera_event_source() -> *mut c_void;
    pub fn commonlib_skse_get_crosshair_ref_event_source() -> *mut c_void;
    pub fn commonlib_skse_get_action_event_source() -> *mut c_void;
    pub fn commonlib_skse_get_ni_node_update_event_source() -> *mut c_void;
}

/// Constructs a C++ object directly into caller-provided out storage and returns the
/// initialized value on success.
///
/// This is the Rust-side half of the ABI-safe out-param bridge pattern used for
/// smart-pointer helpers such as `make_hkref`, `make_nismart`, and `make_smart`.
#[inline(always)]
pub unsafe fn try_construct_out_param<T>(construct: impl FnOnce(*mut T) -> bool) -> Option<T> {
    let mut out = MaybeUninit::<T>::uninit();
    if construct(out.as_mut_ptr()) {
        Some(unsafe { out.assume_init() })
    } else {
        None
    }
}
