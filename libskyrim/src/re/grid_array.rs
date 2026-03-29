use crate::offsets::offsets_rtti::RTTI_GridArray;
use crate::offsets::offsets_vtable::VTABLE_GridArray;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::GridArray`
#[repr(C)]
pub struct GridArray {
    pub vtable: *const usize, // 00
    pub unk08: u32,           // 08
    pub unk0c: u32,           // 0C
    pub length: u32,          // 10
    pub pad14: u32,           // 14
}

const _: () = assert!(core::mem::size_of::<GridArray>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GridArray, unk08) == 0x08);
const _: () = assert!(core::mem::offset_of!(GridArray, unk0c) == 0x0C);
const _: () = assert!(core::mem::offset_of!(GridArray, length) == 0x10);
const _: () = assert!(core::mem::offset_of!(GridArray, pad14) == 0x14);

impl RttiType for GridArray {
    const RTTI: VariantID = RTTI_GridArray;
}

impl GridArray {
    pub const RTTI: VariantID = RTTI_GridArray;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GridArray;

    // ~GridArray() override;                                                     // 00
    // virtual void DetachAll();                                                  // 01
    // virtual void KillAll();                                                    // 02
    // virtual bool SetCenter(std::int32_t, std::int32_t);                        // 03
    // virtual void Shift(std::int32_t, std::int32_t);                            // 04
    // virtual void Detach(std::uint32_t, std::uint32_t) = 0;                     // 05
    // virtual void ClearItem(std::uint32_t, std::uint32_t) = 0;                  // 06
    // virtual void MoveItem(std::uint32_t, std::uint32_t, std::uint32_t, u32)=0; // 07
    // virtual void SwapItem(std::uint32_t, std::uint32_t, std::uint32_t, u32)=0; // 08
}
