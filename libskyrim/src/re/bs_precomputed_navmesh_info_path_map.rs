use crate::offsets::offsets_rtti::RTTI_BSPrecomputedNavmeshInfoPathMap;
use crate::re::{BSNavmeshInfo, BSTArray, BSTHashMap};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSPrecomputedNavmeshInfoPathMap`
#[repr(C)]
pub struct BSPrecomputedNavmeshInfoPathMap {
    pub all_paths: BSTArray<*mut BSTArray<*const BSNavmeshInfo>>, // 00
    pub info_to_index_map: BSTHashMap<*const BSNavmeshInfo, u32>, // 18
}

const _: () = assert!(core::mem::size_of::<BSPrecomputedNavmeshInfoPathMap>() == 0x48);
const _: () = assert!(core::mem::offset_of!(BSPrecomputedNavmeshInfoPathMap, all_paths) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(BSPrecomputedNavmeshInfoPathMap, info_to_index_map) == 0x18);

impl RttiType for BSPrecomputedNavmeshInfoPathMap {
    const RTTI: VariantID = RTTI_BSPrecomputedNavmeshInfoPathMap;
}

impl AsRef<BSPrecomputedNavmeshInfoPathMap> for BSPrecomputedNavmeshInfoPathMap {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSPrecomputedNavmeshInfoPathMap> for BSPrecomputedNavmeshInfoPathMap {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSPrecomputedNavmeshInfoPathMap {
    pub const RTTI: VariantID = RTTI_BSPrecomputedNavmeshInfoPathMap;

    #[inline(always)]
    pub fn all_paths_slice(&self) -> &[*mut BSTArray<*const BSNavmeshInfo>] {
        unsafe { self.all_paths.as_slice() }
    }

    #[inline(always)]
    pub fn path_count(&self) -> usize {
        self.all_paths_slice().len()
    }
}

pub trait BSPrecomputedNavmeshInfoPathMapExt {
    fn all_paths_slice(&self) -> &[*mut BSTArray<*const BSNavmeshInfo>];
    fn path_count(&self) -> usize;
}

impl<T> BSPrecomputedNavmeshInfoPathMapExt for T
where
    T: AsRef<BSPrecomputedNavmeshInfoPathMap>,
{
    #[inline(always)]
    fn all_paths_slice(&self) -> &[*mut BSTArray<*const BSNavmeshInfo>] {
        BSPrecomputedNavmeshInfoPathMap::all_paths_slice(self.as_ref())
    }

    #[inline(always)]
    fn path_count(&self) -> usize {
        BSPrecomputedNavmeshInfoPathMap::path_count(self.as_ref())
    }
}
