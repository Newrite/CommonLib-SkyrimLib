use crate::offsets::offsets_rtti::RTTI_FxResponseArgsBase;
use crate::offsets::offsets_vtable::VTABLE_FxResponseArgsBase;
use crate::re::GFxValue;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::FxResponseArgsBase`
#[repr(C)]
pub struct FxResponseArgsBase {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<FxResponseArgsBase>() == 0x8);
const _: () = assert!(core::mem::offset_of!(FxResponseArgsBase, vtable) == 0x0);

impl RttiType for FxResponseArgsBase {
    const RTTI: VariantID = RTTI_FxResponseArgsBase;
}

impl FxResponseArgsBase {
    pub const RTTI: VariantID = RTTI_FxResponseArgsBase;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FxResponseArgsBase;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_VALUES: usize = 0x01;
        pub fn get_values(&mut self, params: *mut *mut GFxValue) -> u32
    }
}

impl AsRef<FxResponseArgsBase> for FxResponseArgsBase {
    #[inline(always)]
    fn as_ref(&self) -> &FxResponseArgsBase {
        self
    }
}

impl AsMut<FxResponseArgsBase> for FxResponseArgsBase {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut FxResponseArgsBase {
        self
    }
}
