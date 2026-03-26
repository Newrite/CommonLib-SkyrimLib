use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraPersistentCell;
use crate::offsets::offsets_vtable::VTABLE_ExtraPersistentCell;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, TESObjectCELL};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraPersistentCell`
#[repr(C)]
pub struct ExtraPersistentCell {
    pub base: BSExtraData,                   // 00
    pub persistent_cell: *mut TESObjectCELL, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraPersistentCell>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraPersistentCell, persistent_cell) == 0x10);

impl RttiType for ExtraPersistentCell {
    const RTTI: VariantID = RTTI_ExtraPersistentCell;
}

impl ExtraDataTyped for ExtraPersistentCell {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::PersistentCell;
}

inherit!(ExtraPersistentCell : BSExtraData);

impl ExtraPersistentCell {
    pub const RTTI: VariantID = RTTI_ExtraPersistentCell;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraPersistentCell;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::PersistentCell;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kPersistentCell; }
}
