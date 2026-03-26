use core::ffi::c_void;

unsafe extern "C" {
    // SKSE interface getters.
    pub fn commonlib_skse_get_serialization_interface() -> *mut c_void;
    pub fn commonlib_skse_get_papyrus_interface() -> *mut c_void;
    pub fn commonlib_skse_get_messaging_interface() -> *mut c_void;
    pub fn commonlib_skse_get_object_interface() -> *mut c_void;
    pub fn commonlib_skse_get_trampoline_interface() -> *mut c_void;
    pub fn commonlib_skse_get_delay_functor_manager() -> *mut c_void;
    pub fn commonlib_skse_get_object_registry() -> *mut c_void;
    pub fn commonlib_skse_get_persistent_object_storage() -> *mut c_void;

    // SKSE event source getters.
    pub fn commonlib_skse_get_mod_callback_event_source() -> *mut c_void;
    pub fn commonlib_skse_get_camera_event_source() -> *mut c_void;
    pub fn commonlib_skse_get_crosshair_ref_event_source() -> *mut c_void;
    pub fn commonlib_skse_get_action_event_source() -> *mut c_void;
    pub fn commonlib_skse_get_ni_node_update_event_source() -> *mut c_void;

    // SKSE translation helpers.
    pub fn commonlib_skse_translation_parse_translation(name: *const i8);
    pub fn commonlib_skse_translation_translate(
        key: *const i8,
        out_buf: *mut i8,
        out_buf_len: usize,
    ) -> usize;

    // SKSE IAT helpers.
    pub fn commonlib_skse_iat_get_addr(dll: *const i8, function: *const i8) -> usize;
    pub fn commonlib_skse_iat_get_addr_for_module(
        module: *mut c_void,
        dll: *const i8,
        function: *const i8,
    ) -> usize;
    pub fn commonlib_skse_iat_patch(new_func: usize, dll: *const i8, function: *const i8) -> usize;
}
