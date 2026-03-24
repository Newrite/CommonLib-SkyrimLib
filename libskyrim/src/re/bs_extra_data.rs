use crate::offsets::offsets_rtti::RTTI_BSExtraData;
use crate::offsets::offsets_vtable::VTABLE_BSExtraData;
use crate::re::ExtraDataType;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// Marker trait for concrete `BSExtraData` descendants with a fixed `EXTRADATATYPE`.
pub trait ExtraDataTyped {
    const EXTRADATATYPE: ExtraDataType;
}

/// C++ `RE::BSExtraData`
#[repr(C)]
pub struct BSExtraData {
    pub vtable: *const usize,   // 00
    pub next: *mut BSExtraData, // 08
}

const _: () = assert!(core::mem::size_of::<BSExtraData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSExtraData, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSExtraData, next) == 0x08);

impl RttiType for BSExtraData {
    const RTTI: VariantID = RTTI_BSExtraData;
}

impl AsRef<BSExtraData> for BSExtraData {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSExtraData> for BSExtraData {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSExtraData {
    pub const RTTI: VariantID = RTTI_BSExtraData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSExtraData;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::None;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x01;
        pub fn get_type() -> ExtraDataType
    }

    virtual_method! {
        pub const VFUNC_IS_NOT_EQUAL: usize = 0x02;
        pub fn is_not_equal(rhs: *const BSExtraData) -> bool
    }

    #[inline(always)]
    pub fn create(size: usize, vtable: usize) -> *mut BSExtraData {
        unsafe {
            let memory = crate::ffi::commonlib_malloc(size);
            assert!(!memory.is_null(), "BSExtraData::Create allocation failed");
            core::ptr::write_bytes(memory, 0, size);
            *(memory as *mut usize) = vtable;
            memory.cast()
        }
    }

    #[inline(always)]
    pub fn create_typed<T>(vtable: usize) -> *mut T {
        Self::create(core::mem::size_of::<T>(), vtable).cast::<T>()
    }
}

impl PartialEq for BSExtraData {
    fn eq(&self, other: &Self) -> bool {
        !self.ne(other)
    }
}

impl Eq for BSExtraData {}

impl core::cmp::PartialOrd for BSExtraData {
    fn partial_cmp(&self, _other: &Self) -> Option<core::cmp::Ordering> {
        None
    }
}

impl BSExtraData {
    #[inline(always)]
    pub fn ne(&self, rhs: &BSExtraData) -> bool {
        if self.get_type() != rhs.get_type() {
            true
        } else {
            self.is_not_equal(rhs as *const BSExtraData)
        }
    }
}

pub trait BSExtraDataExt {
    fn get_type(&self) -> ExtraDataType;
    fn is_not_equal(&self, rhs: *const BSExtraData) -> bool;
}

impl<T: AsRef<BSExtraData>> BSExtraDataExt for T {
    fn get_type(&self) -> ExtraDataType {
        self.as_ref().get_type()
    }

    fn is_not_equal(&self, rhs: *const BSExtraData) -> bool {
        self.as_ref().is_not_equal(rhs)
    }
}
