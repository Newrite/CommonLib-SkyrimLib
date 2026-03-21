use core_util::abstract_type;
use crate::offsets::offsets_vtable::VTABLE_Character;
use crate::relocation::VariantID;

abstract_type! {
    /// RE::Character
    pub type Character;
}

impl Character {
    // Берем массив VTABLE из ваших сгенерированных оффсетов
    pub const VTABLE: &'static [VariantID] = &VTABLE_Character;

    // Индекс функции Update в таблице виртуальных функций (0xAD = 173)
    pub const VFUNC_UPDATE_IDX: usize = 0xAD;
}