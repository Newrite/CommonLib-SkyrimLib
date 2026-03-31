use crate::offsets::offsets_rtti::RTTI_PrecomputedNavmeshInfoPathMap;
use crate::re::BSPrecomputedNavmeshInfoPathMap;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::PrecomputedNavmeshInfoPathMap`
#[repr(C)]
pub struct PrecomputedNavmeshInfoPathMap {
    pub base: BSPrecomputedNavmeshInfoPathMap, // 00
}

const _: () = assert!(core::mem::size_of::<PrecomputedNavmeshInfoPathMap>() == 0x48);
const _: () = assert!(core::mem::offset_of!(PrecomputedNavmeshInfoPathMap, base) == 0x00);

impl RttiType for PrecomputedNavmeshInfoPathMap {
    const RTTI: VariantID = RTTI_PrecomputedNavmeshInfoPathMap;
}

impl AsRef<BSPrecomputedNavmeshInfoPathMap> for PrecomputedNavmeshInfoPathMap {
    #[inline(always)]
    fn as_ref(&self) -> &BSPrecomputedNavmeshInfoPathMap {
        &self.base
    }
}

impl AsMut<BSPrecomputedNavmeshInfoPathMap> for PrecomputedNavmeshInfoPathMap {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut BSPrecomputedNavmeshInfoPathMap {
        &mut self.base
    }
}

impl PrecomputedNavmeshInfoPathMap {
    pub const RTTI: VariantID = RTTI_PrecomputedNavmeshInfoPathMap;
}
