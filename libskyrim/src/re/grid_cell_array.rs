use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_GridCellArray;
use crate::offsets::offsets_vtable::VTABLE_GridCellArray;
use crate::re::{GridArray, NiPoint3, TESObjectCELL};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::GridCellArray`
#[repr(C)]
pub struct GridCellArray {
    pub base: GridArray,                // 00
    pub cells: *mut *mut TESObjectCELL, // 18
    pub world_center: NiPoint3,         // 20
    pub land_3d_attached: bool,         // 2C
    pub pad2d: u8,                      // 2D
    pub pad2e: u16,                     // 2E
}

const _: () = assert!(core::mem::size_of::<GridCellArray>() == 0x30);
const _: () = assert!(core::mem::offset_of!(GridCellArray, cells) == 0x18);
const _: () = assert!(core::mem::offset_of!(GridCellArray, world_center) == 0x20);
const _: () = assert!(core::mem::offset_of!(GridCellArray, land_3d_attached) == 0x2C);

inherit!(GridCellArray : GridArray);

impl RttiType for GridCellArray {
    const RTTI: VariantID = RTTI_GridCellArray;
}

impl GridCellArray {
    pub const RTTI: VariantID = RTTI_GridCellArray;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GridCellArray;

    #[inline(always)]
    pub fn get_cell(&self, x: u32, y: u32) -> *mut TESObjectCELL {
        if x < self.base.length && y < self.base.length && !self.cells.is_null() {
            unsafe { *self.cells.add((x * self.base.length + y) as usize) }
        } else {
            core::ptr::null_mut()
        }
    }

    // override (GridArray)
    // ~GridCellArray() override;                                       // 00
    // void KillAll() override;                                         // 02
    // bool SetCenter(std::int32_t, std::int32_t) override;             // 03
    // void Detach(std::uint32_t, std::uint32_t) override;              // 05
    // void ClearItem(std::uint32_t, std::uint32_t) override;           // 06
    // void MoveItem(std::uint32_t, std::uint32_t, std::uint32_t, u32); // 07
    // void SwapItem(std::uint32_t, std::uint32_t, std::uint32_t, u32); // 08
}
