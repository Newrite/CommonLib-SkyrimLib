use crate::ffi;

/// Получает абсолютный адрес в памяти игры по ID из Address Library.
/// Если вы хотите сместиться от начала функции (mid-hooking), передайте offset.
pub fn get_address(id: usize, offset: usize) -> usize {
    let base_addr = unsafe { ffi::commonlib_id_to_address(id) };
    if base_addr == 0 {
        panic!("Address Library failed to resolve ID: {}", id);
    }
    base_addr + offset
}

/// Записывает массив байт напрямую в память игры, снимая защиту (PAGE_EXECUTE_READWRITE)
pub fn write_bytes(id: usize, offset: usize, bytes: &[u8]) {
    let addr = get_address(id, offset);
    unsafe {
        ffi::commonlib_safe_write(addr, bytes.as_ptr(), bytes.len());
    }
}
