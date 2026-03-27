use crate::offsets::offsets_rtti::RTTI_BSScript__IFunctionArguments;
use crate::offsets::offsets_vtable::VTABLE_BSScript__IFunctionArguments;
use crate::re::bst_array::BSScrapArray;
use crate::re::variable::Variable;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSScript::IFunctionArguments`
#[repr(C)]
pub struct IFunctionArguments {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IFunctionArguments>() == 0x8);

impl RttiType for IFunctionArguments {
    const RTTI: VariantID = RTTI_BSScript__IFunctionArguments;
}

impl IFunctionArguments {
    pub const RTTI: VariantID = RTTI_BSScript__IFunctionArguments;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSScript__IFunctionArguments;

    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor(&mut self) }
    crate::virtual_method! { pub const VFUNC_COLLECT_ARGS: usize = 0x01; pub fn collect_args(dst: &mut BSScrapArray<Variable>) -> bool }
}
