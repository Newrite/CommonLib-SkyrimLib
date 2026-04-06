use crate::offsets::offsets_rtti::RTTI_TESCustomPackageData;
use crate::offsets::offsets_vtable::VTABLE_TESCustomPackageData;
use crate::re::BGSLoadFormBuffer;
use crate::re::BGSSaveFormBuffer;
use crate::re::BSFixedString;
use crate::re::BSIntrusiveRefCounted;
use crate::re::BSTArray;
use crate::re::IPackageData;
use crate::re::IProcedureTreeItem;
use crate::re::TESForm;
use crate::re::TESPackage;
use crate::re::TESPackageData;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

/// C++ `RE::BGSPackageDataList`
#[repr(C)]
pub struct BGSPackageDataList {
    pub data: *mut *mut IPackageData, // 00
    pub uids: *mut i8,                // 08
    pub data_size: u16,               // 10
    pub next_uid: i8,                 // 12
    pub pad13: u8,                    // 13
    pub pad14: u32,                   // 14
}

const _: () = assert!(core::mem::size_of::<BGSPackageDataList>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSPackageDataList, data) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSPackageDataList, uids) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSPackageDataList, data_size) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSPackageDataList, next_uid) == 0x12);

/// C++ `RE::BGSPackageDataNameMap::NameMapData`
#[repr(C)]
pub struct NameMapData {
    pub name: BSFixedString, // 00
    pub uid: i8,             // 08
    pub is_public: bool,     // 09
    pub pad0a: u8,           // 0A
    pub pad0b: u8,           // 0B
    pub pad0c: u32,          // 0C
}

const _: () = assert!(core::mem::size_of::<NameMapData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(NameMapData, name) == 0x00);
const _: () = assert!(core::mem::offset_of!(NameMapData, uid) == 0x08);
const _: () = assert!(core::mem::offset_of!(NameMapData, is_public) == 0x09);

/// C++ `RE::BGSPackageDataNameMap`
#[repr(C)]
pub struct BGSPackageDataNameMap {
    pub base: BSIntrusiveRefCounted,     // 00
    pub pad04: u32,                      // 04
    pub name_map: BSTArray<NameMapData>, // 08
}

const _: () = assert!(core::mem::size_of::<BGSPackageDataNameMap>() == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSPackageDataNameMap, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSPackageDataNameMap, name_map) == 0x08);

inherit!(BGSPackageDataNameMap : BSIntrusiveRefCounted);

/// C++ `RE::TESCustomPackageData`
#[repr(C)]
pub struct TESCustomPackageData {
    pub base: TESPackageData,                    // 00
    pub data: BGSPackageDataList,                // 08
    pub procedure_tree: *mut IProcedureTreeItem, // 20
    pub name_map: *mut BGSPackageDataNameMap,    // 28 - BSTSmartPointer
    pub template_parent: *mut TESPackage,        // 30
    pub version: u16,                            // 38
    pub always_recheck_conditions: bool,         // 3A
    pub pad3b: u8,                               // 3B
    pub pad3c: u32,                              // 3C
}

const _: () = assert!(core::mem::size_of::<TESCustomPackageData>() == 0x40);
const _: () = assert!(core::mem::offset_of!(TESCustomPackageData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESCustomPackageData, data) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESCustomPackageData, procedure_tree) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESCustomPackageData, name_map) == 0x28);
const _: () = assert!(core::mem::offset_of!(TESCustomPackageData, template_parent) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESCustomPackageData, version) == 0x38);
const _: () =
    assert!(core::mem::offset_of!(TESCustomPackageData, always_recheck_conditions) == 0x3A);

impl RttiType for TESCustomPackageData {
    const RTTI: VariantID = RTTI_TESCustomPackageData;
}

inherit!(TESCustomPackageData : TESPackageData);

impl TESCustomPackageData {
    pub const RTTI: VariantID = RTTI_TESCustomPackageData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESCustomPackageData;

    // override (TESPackageData)
    // void Copy(TESPackageData* a_package, TESForm* a_form) override;  // 01
    // void InitItem(TESForm* a_form) override;  // 03
    // void SaveGame(BGSSaveFormBuffer* a_buf) override;  // 04 - { return; }
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;  // 05 - { return; }

    virtual_method! {
        pub const VFUNC_COPY: usize = 0x01;
        pub fn copy(&mut self, package: *mut TESPackageData, form: *mut TESForm)
    }

    virtual_method! {
        pub const VFUNC_INIT_ITEM: usize = 0x03;
        pub fn init_item(&mut self, form: *mut TESForm)
    }

    virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x04;
        pub fn save_game(&mut self, buf: *mut BGSSaveFormBuffer)
    }

    virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x05;
        pub fn load_game(&mut self, buf: *mut BGSLoadFormBuffer)
    }
}
