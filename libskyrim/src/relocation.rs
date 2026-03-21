use core::ffi::c_void;
use crate::ffi;
use crate::version::{current_runtime, RUNTIME_VERSION_1_5_97};

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

/// Хранит ID Address Library для разных версий игры.
#[derive(Copy, Clone, Debug)]
pub struct VariantID {
    pub se: usize,
    pub ae: usize,
    pub vr: usize,
}

impl VariantID {
    pub const fn new(se: usize, ae: usize, vr: usize) -> Self {
        Self { se, ae, vr }
    }

    /// Возвращает нужный ID в зависимости от запущенной версии игры.
    pub fn id(&self) -> usize {
        // Упрощенная проверка: если версия <= 1.5.97, то это SE. Иначе AE.
        // (Для VR потребуется отдельная проверка, если вы его поддерживаете)
        if current_runtime() <= RUNTIME_VERSION_1_5_97 {
            self.se
        } else {
            self.ae
        }
    }

    /// Сразу возвращает абсолютный адрес в памяти игры по этому ID.
    pub fn address(&self) -> usize {
        let id = self.id();
        if id == 0 {
            return 0; // Защита от нулевых ID
        }
        unsafe { ffi::commonlib_id_to_address(id) }
    }
}

/// Трейт, который должны реализовывать все классы Skyrim, чтобы работать со `skyrim_cast`.
pub trait RttiType {
    const RTTI: VariantID;
}

/// Динамическое приведение типов Skyrim (аналог dynamic_cast).
/// Использует внутреннюю функцию движка RTDynamicCast.
pub unsafe fn skyrim_cast<T: RttiType, U: RttiType>(from: *mut T) -> *mut U {
    if from.is_null() {
        return core::ptr::null_mut();
    }

    // ID функции RTDynamicCast: SE = 102238, AE = 109689
    let rtdc_id = VariantID::new(102238, 109689, 0);
    let rtdc_addr = rtdc_id.address();

    if rtdc_addr == 0 {
        panic!("Failed to find RTDynamicCast address!");
    }

    // Сигнатура функции RTDynamicCast в движке Skyrim
    type RTDynamicCastFn = extern "C" fn(
        inptr: *mut c_void,
        vf_delta: i32,
        src_type: *const c_void,
        target_type: *const c_void,
        is_reference: i32,
    ) -> *mut c_void;

    let rtdc: RTDynamicCastFn = core::mem::transmute(rtdc_addr);

    let from_rtti = T::RTTI.address() as *const c_void;
    let to_rtti = U::RTTI.address() as *const c_void;

    if from_rtti.is_null() || to_rtti.is_null() {
        return core::ptr::null_mut(); // RTTI не найден
    }

    // Вызываем оригинальный каст движка!
    let result = rtdc(from as *mut c_void, 0, from_rtti, to_rtti, 0);

    result as *mut U
}