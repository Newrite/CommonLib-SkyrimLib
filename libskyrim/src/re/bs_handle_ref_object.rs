use core::sync::atomic::Ordering;

use crate::re::NiRefObject;
use crate::relocation::{VariantID, RttiType};
use core_util::inherit;

// Импорт оригинальных констант из оффсетов
use crate::offsets::offsets_rtti::RTTI_BSHandleRefObject;
use crate::offsets::offsets_vtable::VTABLE_BSHandleRefObject;

#[repr(C)]
pub struct BSHandleRefObject {
    // В Rust мы помещаем базовый класс как первое поле.
    // Поскольку NiRefObject имеет repr(C), его поля будут в начале.
    pub base: NiRefObject,
}

// Проверка размера: 0x10
const _: () = assert!(core::mem::size_of::<BSHandleRefObject>() == 0x10);

impl RttiType for BSHandleRefObject {
    const RTTI: VariantID = RTTI_BSHandleRefObject;
}

// ─── РЕАЛИЗАЦИЯ НАСЛЕДОВАНИЯ ─────────────────────────────────────────────────

// Автоматически генерирует Deref, DerefMut и AsRef<NiRefObject>
inherit!(BSHandleRefObject : NiRefObject);

impl BSHandleRefObject {
    pub const RTTI: VariantID = RTTI_BSHandleRefObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_BSHandleRefObject;

    // Константы масок
    pub const REF_COUNT_MASK: u32 = 0x3FF; // Первые 10 бит — это сам счетчик
    pub const HANDLE_VALID_BIT: u32 = 1 << 10; // 11-й бит — валидность хендла

    #[inline(always)]
    pub fn q_ref_count(&self) -> u32 {
        // Читаем из базы, но применяем маску
        self.base.ref_count.load(Ordering::Acquire) & Self::REF_COUNT_MASK
    }

    #[inline(always)]
    pub fn is_handle_valid(&self) -> bool {
        (self.base.ref_count.load(Ordering::Acquire) & Self::HANDLE_VALID_BIT) != 0
    }

    #[inline(always)]
    pub fn inc_ref_count(&self) {
        // Мы можем просто прибавить 1. Даже если мы затронем биты выше 10-го,
        // маска при чтении (QRefCount) это отсечет.
        // В оригинале Скайрима используется обычный инкремент.
        self.base.ref_count.fetch_add(1, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn dec_ref_count(&self) {
        // Важный момент: декрементируем всё значение,
        // но проверяем на 0 только ту часть, что под маской.
        let old_val = self.base.ref_count.fetch_sub(1, Ordering::AcqRel);
        if (old_val - 1) & Self::REF_COUNT_MASK == 0 {
            // Вызываем delete_this, который нам доступен благодаря макросу inherit!
            self.delete_this();
        }
    }
}

// ─── ПЕРЕОПРЕДЕЛЕНИЕ ТРЕЙТА NiRef ──────────────────────────────────────────

use crate::re::ni_ref_object::NiRef;

// Это критически важно: NiPointer<T> использует методы этого трейта.
// Так как логика DecRefCount здесь своя (с маской), мы должны её перегрузить.
impl NiRef for BSHandleRefObject {
    #[inline(always)]
    fn inc_ref(&self) {
        self.inc_ref_count();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.dec_ref_count();
    }
}