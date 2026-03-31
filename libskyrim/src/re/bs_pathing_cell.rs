use core::sync::atomic::Ordering;

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BSPathingCell;
use crate::offsets::offsets_vtable::VTABLE_BSPathingCell;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::{
    BSIntrusiveRefCounted, BSPathingNumericIDVisitor, BSPathingSpace, BSPathingStreamRead,
    BSPathingStreamWrite, bhkWorld,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSPathingCell`
#[repr(C)]
pub struct BSPathingCell {
    pub vtable: *const usize,        // 00
    pub base: BSIntrusiveRefCounted, // 08
    pub pad0c: u32,                  // 0C
}

const _: () = assert!(core::mem::size_of::<BSPathingCell>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSPathingCell, base) == 0x08);

impl RttiType for BSPathingCell {
    const RTTI: VariantID = RTTI_BSPathingCell;
}

inherit!(BSPathingCell : BSIntrusiveRefCounted, base);

impl AsRef<BSPathingCell> for BSPathingCell {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSPathingCell> for BSPathingCell {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSTSmartPointerIntrusiveRefCountable for BSPathingCell {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let func: extern "C" fn(*mut Self) =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0usize) };
        func(self as *const Self as *mut Self);
    }
}

impl BSPathingCell {
    pub const RTTI: VariantID = RTTI_BSPathingCell;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPathingCell;

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x01;
        pub fn get_type(&self) -> u32
    }

    crate::virtual_method! {
        pub const VFUNC_WRITE: usize = 0x02;
        pub fn write(&mut self, stream: &BSPathingStreamWrite)
    }

    crate::virtual_method! {
        pub const VFUNC_READ: usize = 0x03;
        pub fn read(&mut self, stream: &BSPathingStreamRead)
    }

    crate::virtual_method! {
        pub const VFUNC_FIXUP_NUMERIC_ID: usize = 0x04;
        pub fn fixup_numeric_id(&mut self, visitor: &mut BSPathingNumericIDVisitor)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_SPACE: usize = 0x05;
        // TODO: SOURCE - `BSPathingCell::GetSpace` takes
        // `BSTSmartPointer<BSPathingSpace>&`, but `BSPathingSpace` is still an
        // opaque stand-in without a source-backed intrusive smart-pointer
        // ownership layer. Keep the raw out-pointer until that dependency is
        // translated honestly.
        pub fn get_space(&mut self, out: *mut *mut BSPathingSpace)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_BHK_WORLD: usize = 0x06;
        pub fn get_bhk_world(&mut self) -> *mut bhkWorld
    }

    crate::virtual_method! {
        pub const VFUNC_Q_VALID: usize = 0x07;
        pub fn q_valid(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_Q_ATTACHED: usize = 0x08;
        pub fn q_attached(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_Q_LOADED: usize = 0x09;
        pub fn q_loaded(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_IS_IN_SAME_SPACE: usize = 0x0A;
        pub fn is_in_same_space(&mut self, other_cell: *mut crate::re::BSTSmartPointer<BSPathingCell>) -> bool
    }
}

pub trait BSPathingCellExt {
    fn get_type(&self) -> u32;
    fn write(&mut self, stream: &BSPathingStreamWrite);
    fn read(&mut self, stream: &BSPathingStreamRead);
    fn fixup_numeric_id(&mut self, visitor: &mut BSPathingNumericIDVisitor);
    fn get_space(&mut self, out: *mut *mut BSPathingSpace);
    fn get_bhk_world(&mut self) -> *mut bhkWorld;
    fn q_valid(&self) -> bool;
    fn q_attached(&self) -> bool;
    fn q_loaded(&self) -> bool;
    fn is_in_same_space(
        &mut self,
        other_cell: *mut crate::re::BSTSmartPointer<BSPathingCell>,
    ) -> bool;
}

impl<T> BSPathingCellExt for T
where
    T: AsRef<BSPathingCell> + AsMut<BSPathingCell>,
{
    #[inline(always)]
    fn get_type(&self) -> u32 {
        BSPathingCell::get_type(self.as_ref())
    }

    #[inline(always)]
    fn write(&mut self, stream: &BSPathingStreamWrite) {
        BSPathingCell::write(self.as_mut(), stream)
    }

    #[inline(always)]
    fn read(&mut self, stream: &BSPathingStreamRead) {
        BSPathingCell::read(self.as_mut(), stream)
    }

    #[inline(always)]
    fn fixup_numeric_id(&mut self, visitor: &mut BSPathingNumericIDVisitor) {
        BSPathingCell::fixup_numeric_id(self.as_mut(), visitor)
    }

    #[inline(always)]
    fn get_space(&mut self, out: *mut *mut BSPathingSpace) {
        BSPathingCell::get_space(self.as_mut(), out)
    }

    #[inline(always)]
    fn get_bhk_world(&mut self) -> *mut bhkWorld {
        BSPathingCell::get_bhk_world(self.as_mut())
    }

    #[inline(always)]
    fn q_valid(&self) -> bool {
        BSPathingCell::q_valid(self.as_ref())
    }

    #[inline(always)]
    fn q_attached(&self) -> bool {
        BSPathingCell::q_attached(self.as_ref())
    }

    #[inline(always)]
    fn q_loaded(&self) -> bool {
        BSPathingCell::q_loaded(self.as_ref())
    }

    #[inline(always)]
    fn is_in_same_space(
        &mut self,
        other_cell: *mut crate::re::BSTSmartPointer<BSPathingCell>,
    ) -> bool {
        BSPathingCell::is_in_same_space(self.as_mut(), other_cell)
    }
}
