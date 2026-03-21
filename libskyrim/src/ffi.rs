// libskyrim/src/ffi.rs
use core::ffi::c_void;

extern "C" {
    pub fn init_commonlib(skse_interface: *const c_void);
    pub fn commonlib_id_to_address(id: usize) -> usize;
    pub fn commonlib_safe_write(addr: usize, data: *const u8, len: usize);
    pub fn commonlib_write_branch5(src: usize, dst: usize) -> usize;
    pub fn commonlib_write_call5(src: usize, dst: usize) -> usize;
    pub fn commonlib_alloc_trampoline(size: usize);
}
