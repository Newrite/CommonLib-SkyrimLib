use core_util::abstract_type;
use crate::offsets::offsets_vtable::VTABLE_PlayerCharacter;
use crate::relocation::{VariantID, VariantOffset};

abstract_type! {
    /// RE::PlayerCharacter
    pub type PlayerCharacter;
}

impl PlayerCharacter {
    // Массив VTABLE для игрока
    pub const VTABLE: &'static [VariantID] = &VTABLE_PlayerCharacter;

    // Индекс функции Update совпадает, так как PlayerCharacter наследуется от Character
    pub const VFUNC_UPDATE_IDX: VariantOffset = VariantOffset::new(0xAD, 0xAF);
}