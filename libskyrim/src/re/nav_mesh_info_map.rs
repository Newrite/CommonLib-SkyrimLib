use crate::offsets::offsets_rtti::RTTI_NavMeshInfoMap;
use crate::offsets::offsets_vtable::VTABLE_NavMeshInfoMap;
use crate::re::bs_atomic::BSReadWriteLock;
use crate::re::{
    BSNavmeshInfo, BSNavmeshInfoMap, BSPrecomputedNavmeshInfoPathMap, BSTArray, BSTHashMap,
    FormCastable, FormType, PrecomputedNavmeshInfoPathMap, TESForm,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};
use core_util::inherit;

/// C++ `RE::NavMeshInfo`
#[repr(C)]
pub struct NavMeshInfo {
    pub base: BSNavmeshInfo, // 00
}

const _: () = assert!(core::mem::size_of::<NavMeshInfo>() == 0x48);
const _: () = assert!(core::mem::offset_of!(NavMeshInfo, base) == 0x00);

inherit!(NavMeshInfo : BSNavmeshInfo, base);

/// Flat-only tail of C++ `RE::NavMeshInfoMap`.
///
/// `RE/N/NavMeshInfoMap.h` only asserts the full object size under
/// `EXCLUSIVE_SKYRIM_FLAT`, so this tail remains unavailable on VR until the
/// VR offsets/layout are source-backed.
#[repr(C)]
pub struct NavMeshInfoMapRuntimeData {
    pub update_all: bool,                            // 00
    pub pad01: u8,                                   // 01
    pub pad02: u16,                                  // 02
    pub pad04: u32,                                  // 04
    pub info_map: BSTHashMap<u32, *mut NavMeshInfo>, // 08
    pub ck_nav_mesh_info_map: BSTHashMap<u64, *mut BSTArray<*mut BSNavmeshInfo>>, // 38
    pub map_lock: BSReadWriteLock,                   // 68
    pub init: bool,                                  // 70
    pub pad71: u8,                                   // 71
    pub pad72: u16,                                  // 72
    pub pad74: u32,                                  // 74
}

const _: () = assert!(core::mem::size_of::<NavMeshInfoMapRuntimeData>() == 0x78);
const _: () = assert!(core::mem::offset_of!(NavMeshInfoMapRuntimeData, update_all) == 0x00);
const _: () = assert!(core::mem::offset_of!(NavMeshInfoMapRuntimeData, info_map) == 0x08);
const _: () =
    assert!(core::mem::offset_of!(NavMeshInfoMapRuntimeData, ck_nav_mesh_info_map) == 0x38);
const _: () = assert!(core::mem::offset_of!(NavMeshInfoMapRuntimeData, map_lock) == 0x68);
const _: () = assert!(core::mem::offset_of!(NavMeshInfoMapRuntimeData, init) == 0x70);

/// C++ `RE::NavMeshInfoMap`
#[repr(C)]
pub struct NavMeshInfoMap {
    pub base: TESForm,                                       // 00
    pub navmesh_info_map: BSNavmeshInfoMap,                  // 20
    pub pad28: [u8; 0x08],                                   // 28
    pub precomputed_path_map: PrecomputedNavmeshInfoPathMap, // 30
}

const _: () = assert!(core::mem::size_of::<NavMeshInfoMap>() == 0x78);
const _: () = assert!(core::mem::offset_of!(NavMeshInfoMap, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(NavMeshInfoMap, navmesh_info_map) == 0x20);
const _: () = assert!(core::mem::offset_of!(NavMeshInfoMap, precomputed_path_map) == 0x30);

impl RttiType for NavMeshInfoMap {
    const RTTI: VariantID = RTTI_NavMeshInfoMap;
}

impl FormCastable for NavMeshInfoMap {
    const TARGET_FORM_TYPE: FormType = FormType::Navigation;
}

inherit!(NavMeshInfoMap : TESForm, base);

inherit!(NavMeshInfoMap => BSNavmeshInfoMap, navmesh_info_map);
inherit!(NavMeshInfoMap => PrecomputedNavmeshInfoPathMap, precomputed_path_map);

impl AsRef<BSPrecomputedNavmeshInfoPathMap> for NavMeshInfoMap {
    #[inline(always)]
    fn as_ref(&self) -> &BSPrecomputedNavmeshInfoPathMap {
        self.precomputed_path_map.as_ref()
    }
}

impl AsMut<BSPrecomputedNavmeshInfoPathMap> for NavMeshInfoMap {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut BSPrecomputedNavmeshInfoPathMap {
        self.precomputed_path_map.as_mut()
    }
}

impl NavMeshInfoMap {
    pub const RTTI: VariantID = RTTI_NavMeshInfoMap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NavMeshInfoMap;
    pub const FORMTYPE: FormType = FormType::Navigation;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x78, 0x78, 0x0);

    crate::runtime_optional_data_accessor! {
        fn runtime_data_impl() -> NavMeshInfoMapRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        fn runtime_data_impl_mut() -> NavMeshInfoMapRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    #[inline(always)]
    pub fn get_runtime_data(&self) -> Option<&NavMeshInfoMapRuntimeData> {
        crate::runtime_assert_size!(NavMeshInfoMapRuntimeData, se_ae: 0x78, vr: 0x0);
        self.runtime_data_impl()
    }

    #[inline(always)]
    pub fn get_runtime_data_mut(&mut self) -> Option<&mut NavMeshInfoMapRuntimeData> {
        crate::runtime_assert_size!(NavMeshInfoMapRuntimeData, se_ae: 0x78, vr: 0x0);
        self.runtime_data_impl_mut()
    }

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }
}
