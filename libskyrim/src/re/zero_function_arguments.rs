use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSScript__ZeroFunctionArguments;
use crate::offsets::offsets_vtable::VTABLE_BSScript__ZeroFunctionArguments;
use crate::re::ifunction_arguments::IFunctionArguments;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSScript::ZeroFunctionArguments`
#[repr(C)]
pub struct ZeroFunctionArguments {
    pub base: IFunctionArguments, // 00
}

const _: () = assert!(core::mem::size_of::<ZeroFunctionArguments>() == 0x8);
const _: () = assert!(core::mem::offset_of!(ZeroFunctionArguments, base) == 0x0);

inherit!(ZeroFunctionArguments : IFunctionArguments);

impl RttiType for ZeroFunctionArguments {
    const RTTI: VariantID = RTTI_BSScript__ZeroFunctionArguments;
}

impl ZeroFunctionArguments {
    pub const RTTI: VariantID = RTTI_BSScript__ZeroFunctionArguments;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSScript__ZeroFunctionArguments;
}
