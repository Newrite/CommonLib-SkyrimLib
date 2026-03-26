use crate::offsets::offsets_rtti::RTTI_TESChildCell;
use crate::offsets::offsets_vtable::VTABLE_TESChildCell;
use crate::re::TESObjectCELL;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::TESChildCell`
#[repr(C)]
pub struct TESChildCell {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<TESChildCell>() == 0x08);
const _: () = assert!(core::mem::offset_of!(TESChildCell, vtable) == 0x00);

impl RttiType for TESChildCell {
    const RTTI: VariantID = RTTI_TESChildCell;
}

impl AsRef<TESChildCell> for TESChildCell {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TESChildCell> for TESChildCell {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl TESChildCell {
    pub const RTTI: VariantID = RTTI_TESChildCell;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESChildCell;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_GET_SAVE_PARENT_CELL: usize = 0x01;
        pub fn get_save_parent_cell() -> *mut TESObjectCELL
    }
}
