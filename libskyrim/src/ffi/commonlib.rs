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
    pub fn commonlib_raise_seh_exception(code: u32);

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
    pub fn commonlib_actor_drop_object(
        actor: *mut c_void,
        out: *mut c_void,
        object: *const c_void,
        extra_list: *mut c_void,
        count: i32,
        drop_loc: *const c_void,
        rotate: *const c_void,
    );
    pub fn commonlib_actor_set_last_ridden_mount(actor: *mut c_void, mount: *const c_void);
    pub fn commonlib_actor_q_last_ridden_mount(actor: *const c_void, out: *mut c_void);
    pub fn commonlib_actor_handle_get_smart_pointer_const(
        handle: *const c_void,
        out: *mut c_void,
    ) -> bool;
    pub fn commonlib_actor_handle_get_smart_pointer_mut(
        handle: *mut c_void,
        out: *mut c_void,
    ) -> bool;
    pub fn commonlib_object_ref_handle_get_smart_pointer_const(
        handle: *const c_void,
        out: *mut c_void,
    ) -> bool;
    pub fn commonlib_object_ref_handle_get_smart_pointer_mut(
        handle: *mut c_void,
        out: *mut c_void,
    ) -> bool;
    pub fn commonlib_projectile_handle_get_smart_pointer_const(
        handle: *const c_void,
        out: *mut c_void,
    ) -> bool;
    pub fn commonlib_projectile_handle_get_smart_pointer_mut(
        handle: *mut c_void,
        out: *mut c_void,
    ) -> bool;
    pub fn commonlib_tes_object_refr_remove_item(
        refr: *mut c_void,
        out: *mut c_void,
        item: *mut c_void,
        count: i32,
        reason: i32,
        extra_list: *mut c_void,
        move_to_ref: *mut c_void,
        drop_loc: *const c_void,
        rotate: *const c_void,
    );

    // Smart-pointer and factory helpers.
    pub fn commonlib_make_hkref_hk_referenced_object(out: *mut c_void) -> bool;
    pub fn commonlib_make_nismart_ni_ref_object(out: *mut c_void) -> bool;
    pub fn commonlib_gfx_movie_view_add_ref(movie_view: *mut c_void);
    pub fn commonlib_gfx_movie_view_release(movie_view: *mut c_void);
    pub fn commonlib_gfx_resource_delete(resource: *mut c_void);
    pub fn commonlib_fx_delegate_add_ref(delegate: *mut c_void);
    pub fn commonlib_fx_delegate_release(delegate: *mut c_void);
    pub fn commonlib_fx_delegate_handler_add_ref(handler: *mut c_void);
    pub fn commonlib_fx_delegate_handler_release(handler: *mut c_void);
    pub fn commonlib_imenu_add_ref(menu: *mut c_void);
    pub fn commonlib_imenu_release(menu: *mut c_void);
    pub fn commonlib_bgs_attack_data_create() -> *mut c_void;

    // BSFixedString helper wrappers.
    pub fn commonlib_bs_fixed_string_ctor8(out: *mut c_void, string: *const c_char);
    pub fn commonlib_bs_fixed_string_copy(out: *mut c_void, src: *const c_void);
    pub fn commonlib_bs_fixed_string_destroy(string: *mut c_void);
    pub fn commonlib_bs_fixed_string_size(string: *const c_void) -> u32;
    pub fn commonlib_bs_fixed_string_c_str(string: *const c_void) -> *const c_char;
    pub fn commonlib_bs_fixed_string_eq(lhs: *const c_void, rhs: *const c_void) -> bool;
    pub fn commonlib_bs_fixed_string_hash(string: *const c_void) -> u32;
    pub fn commonlib_bs_fixed_string_ctor16(out: *mut c_void, string: *const u16);
    pub fn commonlib_bs_fixed_string_w_copy(out: *mut c_void, src: *const c_void);
    pub fn commonlib_bs_fixed_string_w_destroy(string: *mut c_void);
    pub fn commonlib_bs_fixed_string_w_size(string: *const c_void) -> u32;
    pub fn commonlib_bs_fixed_string_w_c_str(string: *const c_void) -> *const u16;
    pub fn commonlib_bs_fixed_string_w_eq(lhs: *const c_void, rhs: *const c_void) -> bool;
    pub fn commonlib_bs_fixed_string_w_hash(string: *const c_void) -> u32;
    pub fn commonlib_bs_string_pool_release8(entry: *mut *const c_char);
    pub fn commonlib_bs_string_pool_release16(entry: *mut *const u16);

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

    // UI helper bridges.
    pub fn commonlib_create_ui_message_data(class_name: *const c_char) -> *mut c_void;
    pub fn commonlib_ui_get_menu(
        ui: *mut c_void,
        menu_name: *const c_char,
        out: *mut c_void,
    ) -> bool;
    pub fn commonlib_ui_is_menu_open(ui: *mut c_void, menu_name: *const c_char) -> bool;
    pub fn commonlib_ui_register_menu(
        ui: *mut c_void,
        menu_name: *const c_char,
        creator: *mut c_void,
    ) -> bool;
    pub fn commonlib_ui_message_queue_add_message(
        menu_name: *const c_char,
        message_type: i32,
        data: *mut c_void,
    ) -> bool;
    pub fn commonlib_tes_form_lookup_by_editor_id(editor_id: *const c_char) -> *mut c_void;
    pub fn commonlib_ni_controller_manager_get_sequence_by_name(
        manager: *mut c_void,
        name: *const c_char,
    ) -> *mut c_void;
}
