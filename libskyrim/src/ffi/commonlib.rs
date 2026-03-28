use core::ffi::{c_char, c_void};

pub type CommonlibBstEventSinkProcessCallback =
    unsafe extern "C" fn(ctx: *mut c_void, event: *const c_void, event_source: *mut c_void) -> i32;
pub type CommonlibFunctionArgumentsCollectCallback =
    unsafe extern "C" fn(ctx: *mut c_void, dst: *mut c_void) -> bool;
pub type CommonlibNativeFunctionMarshallCallback = unsafe extern "C" fn(
    ctx: *mut c_void,
    base_value: *mut c_void,
    vm: *mut c_void,
    stack_id: u32,
    result_value: *mut c_void,
    frame: *const c_void,
) -> bool;

pub type CommonlibDestroyCallback = unsafe extern "C" fn(ctx: *mut c_void);

unsafe extern "C" {
    // CommonLib initialization.
    pub fn init_commonlib(skse_interface: *const c_void);
    pub fn init_commonlib_with_log(skse_interface: *const c_void, log: bool);

    // Address Library helpers.
    pub fn commonlib_id_to_address(id: usize) -> usize;
    pub fn commonlib_offset_to_address(offset: usize) -> usize;

    // Memory patching helpers.
    pub fn commonlib_safe_write(addr: usize, data: *const u8, len: usize);
    pub fn commonlib_safe_fill(addr: usize, value: u8, len: usize);

    // Trampoline and patch writing helpers.
    pub fn commonlib_write_branch5(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_branch6(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_call5(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_call6(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_function_hook_universal(target: usize, dst: usize) -> usize;
    pub fn commonlib_alloc_trampoline(size: usize);
    pub fn commonlib_trampoline_allocate(size: usize) -> *mut u8;

    // VTable patching helper.
    pub fn commonlib_write_vfunc(vtable_addr: usize, idx: usize, new_func: usize) -> usize;

    // Task interface helpers.
    pub fn commonlib_add_task(cb: extern "C" fn(*mut c_void), data: *mut c_void);
    pub fn commonlib_add_ui_task(cb: extern "C" fn(*mut c_void), data: *mut c_void);

    // Engine memory wrappers.
    pub fn commonlib_malloc(size: usize) -> *mut c_void;
    pub fn commonlib_free(ptr: *mut c_void);
    pub fn commonlib_aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
    pub fn commonlib_aligned_free(ptr: *mut c_void);
    pub fn commonlib_calloc(count: usize, size: usize) -> *mut c_void;
    pub fn commonlib_realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void;

    // MemoryManager helpers.
    pub fn commonlib_memory_manager_get_singleton() -> *mut c_void;
    pub fn commonlib_memory_manager_get_thread_scrap_heap(mgr: *mut c_void) -> *mut c_void;

    // ScrapHeap helpers.
    pub fn commonlib_scrap_heap_allocate(
        heap: *mut c_void,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
    pub fn commonlib_scrap_heap_deallocate(heap: *mut c_void, mem: *mut c_void);

    // Actor helper wrappers for inline-only CommonLib API.
    pub fn commonlib_actor_get_gold_amount(actor: *mut c_void, no_init: bool) -> i32;

    // Smart-pointer and factory helpers.
    pub fn commonlib_make_hkref_hk_referenced_object(out: *mut c_void) -> bool;
    pub fn commonlib_make_nismart_ni_ref_object(out: *mut c_void) -> bool;
    pub fn commonlib_gfx_movie_view_add_ref(movie_view: *mut c_void);
    pub fn commonlib_gfx_movie_view_release(movie_view: *mut c_void);
    pub fn commonlib_fx_delegate_add_ref(delegate: *mut c_void);
    pub fn commonlib_fx_delegate_release(delegate: *mut c_void);
    pub fn commonlib_fx_delegate_handler_add_ref(handler: *mut c_void);
    pub fn commonlib_fx_delegate_handler_release(handler: *mut c_void);
    pub fn commonlib_imenu_add_ref(menu: *mut c_void);
    pub fn commonlib_imenu_release(menu: *mut c_void);
    pub fn commonlib_bgs_attack_data_create() -> *mut c_void;

    // Input device helpers.
    pub fn commonlib_destroy_bsi_input_device(device: *mut c_void);
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

    // Generic Papyrus argument bridge.
    pub fn commonlib_function_arguments_create(
        ctx: *mut c_void,
        collect: Option<CommonlibFunctionArgumentsCollectCallback>,
        destroy: Option<CommonlibDestroyCallback>,
    ) -> *mut c_void;
    pub fn commonlib_function_arguments_create_zero() -> *mut c_void;
    pub fn commonlib_function_arguments_destroy(args: *mut c_void);

    // Generic Papyrus native-function bridge.
    pub fn commonlib_native_function_create(
        ctx: *mut c_void,
        marshall: Option<CommonlibNativeFunctionMarshallCallback>,
        destroy: Option<CommonlibDestroyCallback>,
        fn_name: *const c_char,
        class_name: *const c_char,
        is_static: bool,
        return_type: *const c_void,
        param_types: *const c_void,
        param_count: usize,
        is_latent: bool,
    ) -> *mut c_void;
    pub fn commonlib_native_function_destroy(function: *mut c_void);
}
