use core::ffi::c_void;

extern "C" {
    pub fn init_commonlib(skse_interface: *const c_void);

    pub fn commonlib_id_to_address(id: usize) -> usize;
    pub fn commonlib_offset_to_address(offset: usize) -> usize;

    pub fn commonlib_safe_write(addr: usize, data: *const u8, len: usize);
    pub fn commonlib_safe_fill(addr: usize, value: u8, len: usize);

    pub fn commonlib_write_branch5(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_branch6(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_call5(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_call6(src: usize, dst: usize) -> usize;

    pub fn commonlib_alloc_trampoline(size: usize);
    pub fn commonlib_trampoline_allocate(size: usize) -> *mut u8;

    pub fn commonlib_write_vfunc(vtable_addr: usize, idx: usize, new_func: usize) -> usize;
    pub fn commonlib_add_task(cb: extern "C" fn(*mut c_void), data: *mut c_void);
    pub fn commonlib_add_ui_task(cb: extern "C" fn(*mut c_void), data: *mut c_void);
}
