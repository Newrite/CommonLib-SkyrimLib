use core::sync::atomic::Ordering;

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_PathingCell;
use crate::offsets::offsets_vtable::VTABLE_PathingCell;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::crc::{BSTHash, generate_crc32};
use crate::re::{BSPathingCell, CellID, FormID};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::PathingCellInfo::CellFormID`
#[repr(C)]
#[derive(Clone, Copy)]
pub union PathingCellInfoCellFormID {
    pub form_id: FormID,     // 00
    pub coordinates: CellID, // 00
}

const _: () = assert!(core::mem::size_of::<PathingCellInfoCellFormID>() == 0x4);

impl Default for PathingCellInfoCellFormID {
    #[inline(always)]
    fn default() -> Self {
        Self { form_id: 0 }
    }
}

impl PartialEq for PathingCellInfoCellFormID {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.form_id == other.form_id && self.coordinates == other.coordinates }
    }
}

impl Eq for PathingCellInfoCellFormID {}

/// C++ `RE::PathingCellInfo`
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct PathingCellInfo {
    pub world_space_id: FormID,             // 00
    pub cell_id: PathingCellInfoCellFormID, // 04
}

const _: () = assert!(core::mem::size_of::<PathingCellInfo>() == 0x8);
const _: () = assert!(core::mem::offset_of!(PathingCellInfo, world_space_id) == 0x00);
const _: () = assert!(core::mem::offset_of!(PathingCellInfo, cell_id) == 0x04);

impl BSTHash for PathingCellInfo {
    #[inline]
    fn bst_hash(&self) -> u32 {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        };
        generate_crc32(bytes)
    }
}

/// C++ `RE::PathingCell`
#[repr(C)]
pub struct PathingCell {
    pub base: BSPathingCell,                // 00
    pub pathing_cell_info: PathingCellInfo, // 10
}

const _: () = assert!(core::mem::size_of::<PathingCell>() == 0x18);
const _: () = assert!(core::mem::offset_of!(PathingCell, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(PathingCell, pathing_cell_info) == 0x10);

impl RttiType for PathingCell {
    const RTTI: VariantID = RTTI_PathingCell;
}

inherit!(PathingCell : BSPathingCell, base);

impl BSTSmartPointerIntrusiveRefCountable for PathingCell {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.base.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.base.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let func: extern "C" fn(*mut Self) =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0usize) };
        func(self as *const Self as *mut Self);
    }
}

impl PathingCell {
    pub const RTTI: VariantID = RTTI_PathingCell;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PathingCell;
}
