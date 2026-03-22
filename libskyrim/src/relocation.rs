use core::ffi::c_void;
use crate::ffi;
use crate::runtime;

/// Трейт для удобной конвертации VariantID или usize в абсолютный адрес
pub trait IntoAddress { fn into_address(self) -> usize; }
impl IntoAddress for usize { fn into_address(self) -> usize { self } }
impl IntoAddress for VariantID { fn into_address(self) -> usize { self.address() } }

/// Трейт для удобной конвертации VariantOffset или usize в смещение/индекс
pub trait IntoOffset { fn into_offset(self) -> usize; }
impl IntoOffset for usize { fn into_offset(self) -> usize { self } }
impl IntoOffset for VariantOffset { fn into_offset(self) -> usize { self.offset() } }

/// Адресация в памяти Скайрима
pub struct Relocation;

impl Relocation {
    pub fn from_id(id: usize, offset: usize) -> usize {
        let base_addr = unsafe { ffi::commonlib_id_to_address(id) };
        if base_addr == 0 { panic!("Address Library failed to resolve ID: {}", id); }
        base_addr + offset
    }

    pub fn from_offset(offset: usize) -> usize {
        let base_addr = unsafe { ffi::commonlib_offset_to_address(offset) };
        if base_addr == 0 { panic!("Failed to resolve offset: {:#X}", offset); }
        base_addr
    }

    pub fn write_bytes(addr: usize, bytes: &[u8]) {
        unsafe { ffi::commonlib_safe_write(addr, bytes.as_ptr(), bytes.len()); }
    }

    pub fn fill_bytes(addr: usize, value: u8, count: usize) {
        unsafe { ffi::commonlib_safe_fill(addr, value, count); }
    }

    pub fn write_vfunc(vtable_addr: usize, index: usize, new_func: usize) -> usize {
        unsafe { ffi::commonlib_write_vfunc(vtable_addr, index, new_func) }
    }

    /// Записывает инструкцию CALL (5 или 6 байт)
    pub fn write_call(src: usize, dst: usize, size: usize) -> usize {
        unsafe {
            match size {
                5 => ffi::commonlib_write_call5(src, dst),
                6 => ffi::commonlib_write_call6(src, dst),
                _ => panic!("Unsupported call hook size: {}. Only 5 or 6 are supported.", size),
            }
        }
    }

    /// Записывает инструкцию BRANCH/JMP (5 или 6 байт)
    pub fn write_branch(src: usize, dst: usize, size: usize) -> usize {
        unsafe {
            match size {
                5 => ffi::commonlib_write_branch5(src, dst),
                6 => ffi::commonlib_write_branch6(src, dst),
                _ => panic!("Unsupported branch hook size: {}. Only 5 or 6 are supported.", size),
            }
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

    pub fn id(&self) -> usize {
        if runtime::is_ae() { self.ae } else { self.se }
    }

    pub fn address(&self) -> usize {
        let id = self.id();
        if id == 0 { return 0; }
        unsafe { ffi::commonlib_id_to_address(id) }
    }
}

/// Хранит смещения или индексы, которые могут отличаться между SE/AE и VR.
#[derive(Copy, Clone, Debug)]
pub struct VariantOffset {
    pub se_ae: usize, // Для SE и AE (Flat)
    pub vr: usize,    // Для VR
}

impl VariantOffset {
    pub const fn new(se_ae: usize, vr: usize) -> Self {
        Self { se_ae, vr }
    }

    #[inline(always)]
    pub fn offset(&self) -> usize {
        if crate::runtime::is_vr() { self.vr } else { self.se_ae }
    }
}

pub trait RttiType { const RTTI: VariantID; }

pub unsafe fn skyrim_cast<T: RttiType, U: RttiType>(from: *mut T) -> *mut U {
    if from.is_null() { return core::ptr::null_mut(); }

    let rtdc_id = VariantID::new(102238, 109689, 0);
    let rtdc_addr = rtdc_id.address();
    if rtdc_addr == 0 { panic!("Failed to find RTDynamicCast address!"); }

    type RTDynamicCastFn = extern "C" fn(
        inptr: *mut c_void, vf_delta: i32, src_type: *const c_void,
        target_type: *const c_void, is_reference: i32
    ) -> *mut c_void;

    let rtdc: RTDynamicCastFn = core::mem::transmute(rtdc_addr);
    let from_rtti = T::RTTI.address() as *const c_void;
    let to_rtti = U::RTTI.address() as *const c_void;

    if from_rtti.is_null() || to_rtti.is_null() { return core::ptr::null_mut(); }

    let result = rtdc(from as *mut c_void, 0, from_rtti, to_rtti, 0);
    result as *mut U
}

// ── МАКРОСЫ ДЛЯ ХУКОВ ────────────────────────────────────────────────────────

#[macro_export]
macro_rules! define_vtable_hook {
    (
        $vis:vis $hook_name:ident {
            vtable: $vtable:expr,
            offset: $offset:expr, // Используем слово offset (в контексте VTable это индекс)
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),*) $(-> $ret:ty)? $body:block
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $hook_name {
            use super::*;
            type Signature = extern "C" fn($($arg_name: $arg_type),*) $(-> $ret)?;
            static ORIGINAL: $crate::core_util::Later<Signature> = $crate::core_util::Later::new();

            #[inline(always)]
            pub fn original($($arg_name: $arg_type),*) $(-> $ret)? { (*ORIGINAL)($($arg_name),*) }

            extern "C" fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? { $body }

            pub fn install() {
                use $crate::relocation::{IntoAddress, IntoOffset};
                unsafe {
                    let vtable_addr = $vtable.into_address();
                    let orig_addr = $crate::relocation::Relocation::write_vfunc(
                        vtable_addr,
                        $offset.into_offset(), // Поддерживает и usize, и VariantOffset
                        $hook_func as usize
                    );
                    ORIGINAL.init(::core::mem::transmute(orig_addr));
                }
            }
        }
    };
}

/// Создает Call/Branch хук (Трамплин). Перезаписывает инструкцию в чужой функции.
#[macro_export]
macro_rules! define_call_hook {
    (
        $vis:vis $hook_name:ident {
            address: $address:expr,
            offset: $offset:expr,
            size: $size:literal, // Обычно 5 (call) или 6
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),*) $(-> $ret:ty)? $body:block
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $hook_name {
            use super::*;
            type Signature = extern "C" fn($($arg_name: $arg_type),*) $(-> $ret)?;
            static ORIGINAL: $crate::core_util::Later<Signature> = $crate::core_util::Later::new();

            #[inline(always)]
            pub fn original($($arg_name: $arg_type),*) $(-> $ret)? { (*ORIGINAL)($($arg_name),*) }

            extern "C" fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? { $body }

            pub fn install() {
                use $crate::relocation::{IntoAddress, IntoOffset};
                unsafe {
                    // Вычисляем финальный адрес: Адрес Базовой Функции + Смещение инструкции
                    let target_addr = $address.into_address() + $offset.into_offset();
                    let orig_addr = $crate::relocation::Relocation::write_call(
                        target_addr,
                        $hook_func as usize,
                        $size
                    );
                    ORIGINAL.init(::core::mem::transmute(orig_addr));
                }
            }
        }
    };
}

/// Макрос для генерации константы индекса и метода вызова виртуальной функции.
#[macro_export]
macro_rules! virtual_method {
    (
        // Описание константы: видимость, имя, тип и значение
        $const_vis:vis const $const_name:ident: $const_ty:ty = $const_expr:expr;

        // Описание функции: видимость, имя, аргументы и (опционально) возвращаемый тип
        $fn_vis:vis fn $func_name:ident($($arg_name:ident: $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        // 1. Генерируем саму константу (чтобы к ней можно было обратиться из define_vtable_hook!)
        $const_vis const $const_name: $const_ty = $const_expr;

        // 2. Генерируем удобный метод для вызова этой функции
        #[inline(always)]
        $fn_vis fn $func_name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            use $crate::relocation::IntoOffset;
            unsafe {
                let vtable = *(self as *const _ as *const *const usize);
                let func_ptr = vtable.add(Self::$const_name.into_offset());
                let func: extern "C" fn(*const Self $(, $arg_ty)*) $(-> $ret)? = core::mem::transmute(*func_ptr);
                func(self $(, $arg_name)*)
            }
        }
    };
}

/// Макрос для доступа к глобальным переменным игры по ID из Address Library.
#[macro_export]
macro_rules! relocation_variable {
    // --- ВАРИАНТ 1: Ссылка на значение (&'static T) ---

    // Без инлайна (через префикс @no_inline)
    ( @no_inline $vis:vis fn $name:ident() -> &'static $ty:ty => $id:expr ) => {
        $vis fn $name() -> &'static $ty {
            $crate::relocation_variable!(@body $name, $ty, $id)
        }
    };

    // По стандарту (с #[inline])
    ( $vis:vis fn $name:ident() -> &'static $ty:ty => $id:expr ) => {
        #[inline]
        $vis fn $name() -> &'static $ty {
            $crate::relocation_variable!(@body $name, $ty, $id)
        }
    };

    // --- ВАРИАНТ 2: Указатель на объект (*mut T) ---

    // Без инлайна (через префикс @no_inline)
    ( @no_inline $vis:vis fn $name:ident() -> *mut $ty:ty => $id:expr, is_ptr ) => {
        $vis fn $name() -> *mut $ty {
            $crate::relocation_variable!(@body_ptr $name, $ty, $id)
        }
    };

    // По стандарту (с #[inline])
    ( $vis:vis fn $name:ident() -> *mut $ty:ty => $id:expr, is_ptr ) => {
        #[inline]
        $vis fn $name() -> *mut $ty {
            $crate::relocation_variable!(@body_ptr $name, $ty, $id)
        }
    };

    // --- Внутренние помощники, чтобы не дублировать логику ---
    (@body $name:ident, $ty:ty, $id:expr) => {{
        use $crate::relocation::IntoAddress;
        let addr = $id.into_address();
        if addr == 0 {
            panic!(concat!("Failed to resolve address for: ", stringify!($name)));
        }
        unsafe { &*(addr as *const $ty) }
    }};

    (@body_ptr $name:ident, $ty:ty, $id:expr) => {{
        use $crate::relocation::IntoAddress;
        let addr = $id.into_address();
        if addr == 0 {
            return core::ptr::null_mut();
        }
        unsafe { *(addr as *const *mut $ty) }
    }};
}

#[macro_export]
macro_rules! relocation_func {
    // --- NON-SELF VARIANTS ---

    // No inline
    ( @no_inline $vis:vis fn $name:ident($($arg_name:ident: $arg_ty:ty),* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        $vis fn $name($($arg_name: $arg_ty),*) $(-> $ret)? {
            $crate::relocation_func!(@body $name, ($($arg_ty),*), ($($arg_name),*), $(-> $ret)?, $id)
        }
    };

    // Default (inline)
    ( $vis:vis fn $name:ident($($arg_name:ident: $arg_ty:ty),* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        #[inline]
        $vis fn $name($($arg_name: $arg_ty),*) $(-> $ret)? {
            $crate::relocation_func!(@body $name, ($($arg_ty),*), ($($arg_name),*), $(-> $ret)?, $id)
        }
    };

    // --- &SELF VARIANTS ---

    // No inline
    ( @no_inline $vis:vis fn $name:ident(&self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        $vis fn $name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::relocation_func!(@body $name, (*const Self $(, $arg_ty)*), (self as *const Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    // Default (inline)
    ( $vis:vis fn $name:ident(&self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        #[inline]
        $vis fn $name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::relocation_func!(@body $name, (*const Self $(, $arg_ty)*), (self as *const Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    // --- &MUT SELF VARIANTS ---

    // No inline
    ( @no_inline $vis:vis fn $name:ident(&mut self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        $vis fn $name(&mut self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::relocation_func!(@body $name, (*mut Self $(, $arg_ty)*), (self as *mut Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    // Default (inline)
    ( $vis:vis fn $name:ident(&mut self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        #[inline]
        $vis fn $name(&mut self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::relocation_func!(@body $name, (*mut Self $(, $arg_ty)*), (self as *mut Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    // --- INTERNAL HELPER ---
    (@body $name:ident, ($($arg_types:ty),*), ($($arg_names:expr),*), $(-> $ret:ty)?, $id:expr) => {{
        use $crate::relocation::IntoAddress;

        let addr = $id.into_address();
        if addr == 0 {
            panic!(concat!("Failed to resolve function address for: ", stringify!($name)));
        }

        unsafe {
            let func: unsafe extern "C-unwind" fn($($arg_types),*) $(-> $ret)? = core::mem::transmute(addr);
            func($($arg_names),*)
        }
    }};
}