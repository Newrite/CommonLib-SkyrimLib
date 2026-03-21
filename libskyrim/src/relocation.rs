use crate::ffi;

/// Адресация в памяти Скайрима
pub struct Relocation;

impl Relocation {
    /// Получает абсолютный адрес в памяти игры по ID (Address Library).
    pub fn from_id(id: usize, offset: usize) -> usize {
        let base_addr = unsafe { ffi::commonlib_id_to_address(id) };
        if base_addr == 0 {
            panic!("Address Library failed to resolve ID: {}", id);
        }
        base_addr + offset
    }

    /// Получает абсолютный адрес в памяти игры по жесткому оффсету от базы.
    pub fn from_offset(offset: usize) -> usize {
        let base_addr = unsafe { ffi::commonlib_offset_to_address(offset) };
        if base_addr == 0 {
            panic!("Failed to resolve offset: {:#X}", offset);
        }
        base_addr
    }

    /// Записывает массив байт напрямую в память игры. Снимает защиту (PAGE_EXECUTE_READWRITE).
    pub fn write_bytes(addr: usize, bytes: &[u8]) {
        unsafe {
            ffi::commonlib_safe_write(addr, bytes.as_ptr(), bytes.len());
        }
    }

    /// Заполняет участок памяти указанным байтом (идеально для затирания инструкций NOP-ами `0x90`).
    pub fn fill_bytes(addr: usize, value: u8, count: usize) {
        unsafe {
            ffi::commonlib_safe_fill(addr, value, count);
        }
    }

    /// Заменяет функцию в виртуальной таблице (VTable) класса.
    /// Возвращает оригинальный указатель на функцию (чтобы его можно было вызвать внутри хука).
    pub fn write_vfunc(vtable_addr: usize, index: usize, new_func: usize) -> usize {
        unsafe {
            ffi::commonlib_write_vfunc(vtable_addr, index, new_func)
        }
    }
}
