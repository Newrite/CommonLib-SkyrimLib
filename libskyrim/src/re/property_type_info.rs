use crate::re::{BSFixedString, BSTSmartPointer, IFunction, TypeInfo};

/// C++ `RE::BSScript::PropertyTypeInfo::Permissions`
///
/// CommonLibVR declares this as an empty `enum class` with `std::uint32_t`
/// storage. Rust cannot represent a zero-variant `repr(u32)` enum, so this
/// keeps the same ABI as a transparent flag wrapper.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Permissions(pub u32);

impl core_util::EnumSetType<u32> for Permissions {
    #[inline(always)]
    fn to_underlying(self) -> u32 {
        self.0
    }
}

/// C++ `RE::BSScript::PropertyTypeInfo`
#[repr(C)]
#[derive(Clone, Default)]
pub struct PropertyTypeInfo {
    pub parent_obj_name: BSFixedString,                    // 00
    pub property_name: BSFixedString,                      // 08
    pub type_: TypeInfo,                                   // 10
    pub permissions: core_util::EnumSet<Permissions, u32>, // 18
    pub pad1c: u32,                                        // 1C
    // TODO: Replace the current pointer-only `IFunction` stub with the full
    // `RE::BSScript::IFunction` translation once the Papyrus function ABI
    // surface lands; these smart pointers are layout-correct today, but their
    // helper/ownership semantics are limited by that missing dependency.
    pub get_function: BSTSmartPointer<IFunction>, // 20
    pub set_function: BSTSmartPointer<IFunction>, // 28
    pub auto_var_index: u32,                      // 30
    pub user_flags: u32,                          // 34
    pub doc_string: BSFixedString,                // 38
}

const _: () = assert!(core::mem::size_of::<PropertyTypeInfo>() == 0x40);
const _: () = assert!(core::mem::offset_of!(PropertyTypeInfo, parent_obj_name) == 0x00);
const _: () = assert!(core::mem::offset_of!(PropertyTypeInfo, property_name) == 0x08);
const _: () = assert!(core::mem::offset_of!(PropertyTypeInfo, type_) == 0x10);
const _: () = assert!(core::mem::offset_of!(PropertyTypeInfo, permissions) == 0x18);
const _: () = assert!(core::mem::offset_of!(PropertyTypeInfo, pad1c) == 0x1C);
const _: () = assert!(core::mem::offset_of!(PropertyTypeInfo, get_function) == 0x20);
const _: () = assert!(core::mem::offset_of!(PropertyTypeInfo, set_function) == 0x28);
const _: () = assert!(core::mem::offset_of!(PropertyTypeInfo, auto_var_index) == 0x30);
const _: () = assert!(core::mem::offset_of!(PropertyTypeInfo, user_flags) == 0x34);
const _: () = assert!(core::mem::offset_of!(PropertyTypeInfo, doc_string) == 0x38);
