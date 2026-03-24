use core::ffi::c_void;

use crate::re::BGSPackageDataList;
use crate::re::BGSStoryEvent;
use crate::re::ObjectRefHandle;
use crate::re::TESForm;
use crate::re::TESGlobal;
use crate::re::TESObjectREFR;
use crate::re::TESQuest;
use crate::relocation::RelocationID;

/// C++ `RE::CONDITIONITEMOBJECT`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionItemObject {
    Self_ = 0,
    Target = 1,
    Ref = 2,
    CombatTarget = 3,
    LinkedRef = 4,
    QuestAlias = 5,
    PackData = 6,
    EventData = 7,
    CommandTarget = 8,
}

core_util::impl_enumset_type!(ConditionItemObject => u8);

/// C++ `RE::FUNCTION_DATA::FunctionID`
pub type ConditionFunctionId = u16;

/// C++ `RE::FUNCTION_DATA`
#[repr(C)]
pub struct FunctionData {
    pub function: ConditionFunctionId, // 00
    pub pad02: u16,                    // 02
    pub pad04: u32,                    // 04
    pub params: [*mut c_void; 2],      // 08
}

const _: () = assert!(core::mem::size_of::<FunctionData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(FunctionData, function) == 0x00);
const _: () = assert!(core::mem::offset_of!(FunctionData, params) == 0x08);

impl Default for FunctionData {
    fn default() -> Self {
        Self {
            function: u16::MAX,
            pad02: 0,
            pad04: 0,
            params: [core::ptr::null_mut(); 2],
        }
    }
}

/// C++ `RE::CONDITION_ITEM_DATA::OpCode`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionItemOpCode {
    EqualTo = 0,
    NotEqualTo = 1,
    GreaterThan = 2,
    GreaterThanOrEqualTo = 3,
    LessThan = 4,
    LessThanOrEqualTo = 5,
}

/// C++ `RE::CONDITION_ITEM_DATA::GlobalOrFloat`
#[repr(C)]
pub union GlobalOrFloat {
    pub global: *mut TESGlobal,
    pub value: f32,
    pub pad: u64,
}

const _: () = assert!(core::mem::size_of::<GlobalOrFloat>() == 0x8);

impl Default for GlobalOrFloat {
    fn default() -> Self {
        Self {
            global: core::ptr::null_mut(),
        }
    }
}

/// C++ `RE::CONDITION_ITEM_DATA::Flags`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConditionItemFlags {
    pub storage: u8,
}

const _: () = assert!(core::mem::size_of::<ConditionItemFlags>() == 0x1);

impl Default for ConditionItemFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl ConditionItemFlags {
    #[inline]
    pub const fn new() -> Self {
        Self {
            storage: (ConditionItemOpCode::EqualTo as u8) << 5,
        }
    }

    #[inline]
    pub const fn is_or(self) -> bool {
        self.storage & (1 << 0) != 0
    }

    #[inline]
    pub const fn uses_aliases(self) -> bool {
        self.storage & (1 << 1) != 0
    }

    #[inline]
    pub const fn global(self) -> bool {
        self.storage & (1 << 2) != 0
    }

    #[inline]
    pub const fn use_pack_data(self) -> bool {
        self.storage & (1 << 3) != 0
    }

    #[inline]
    pub const fn swap_target(self) -> bool {
        self.storage & (1 << 4) != 0
    }

    #[inline]
    pub const fn op_code(self) -> ConditionItemOpCode {
        match (self.storage >> 5) & 0x07 {
            0 => ConditionItemOpCode::EqualTo,
            1 => ConditionItemOpCode::NotEqualTo,
            2 => ConditionItemOpCode::GreaterThan,
            3 => ConditionItemOpCode::GreaterThanOrEqualTo,
            4 => ConditionItemOpCode::LessThan,
            5 => ConditionItemOpCode::LessThanOrEqualTo,
            _ => ConditionItemOpCode::EqualTo,
        }
    }
}

/// C++ `RE::CONDITION_ITEM_DATA`
#[repr(C)]
pub struct ConditionItemData {
    pub comparison_value: GlobalOrFloat,                     // 00
    pub run_on_ref: ObjectRefHandle,                         // 08
    pub data_id: u32,                                        // 0C
    pub function_data: FunctionData,                         // 10
    pub flags: ConditionItemFlags,                           // 28
    pub object: core_util::EnumSet<ConditionItemObject, u8>, // 29
    pub pad2a: u16,                                          // 2A
    pub pad2c: u32,                                          // 2C
}

const _: () = assert!(core::mem::size_of::<ConditionItemData>() == 0x30);
const _: () = assert!(core::mem::offset_of!(ConditionItemData, comparison_value) == 0x00);
const _: () = assert!(core::mem::offset_of!(ConditionItemData, run_on_ref) == 0x08);
const _: () = assert!(core::mem::offset_of!(ConditionItemData, data_id) == 0x0C);
const _: () = assert!(core::mem::offset_of!(ConditionItemData, function_data) == 0x10);
const _: () = assert!(core::mem::offset_of!(ConditionItemData, flags) == 0x28);
const _: () = assert!(core::mem::offset_of!(ConditionItemData, object) == 0x29);
const _: () = assert!(core::mem::offset_of!(ConditionItemData, pad2a) == 0x2A);
const _: () = assert!(core::mem::offset_of!(ConditionItemData, pad2c) == 0x2C);

impl Default for ConditionItemData {
    fn default() -> Self {
        Self {
            comparison_value: GlobalOrFloat::default(),
            run_on_ref: ObjectRefHandle::new(),
            data_id: 0,
            function_data: FunctionData::default(),
            flags: ConditionItemFlags::new(),
            object: core_util::EnumSet::from_underlying(ConditionItemObject::Self_ as u8),
            pad2a: 0,
            pad2c: 0,
        }
    }
}

/// C++ `RE::ConditionCheckParams`
#[repr(C)]
pub struct ConditionCheckParams {
    pub action_ref: *mut TESObjectREFR,             // 00
    pub target_ref: *mut TESObjectREFR,             // 08
    pub quest: *mut TESQuest,                       // 10
    pub quest_start_event: *mut BGSStoryEvent,      // 18
    pub unk20: *mut c_void,                         // 20
    pub unk28: bool,                                // 28
    pub pad29: [u8; 7],                             // 29
    pub package_data_list: *mut BGSPackageDataList, // 30
}

const _: () = assert!(core::mem::size_of::<ConditionCheckParams>() == 0x38);
const _: () = assert!(core::mem::offset_of!(ConditionCheckParams, action_ref) == 0x00);
const _: () = assert!(core::mem::offset_of!(ConditionCheckParams, target_ref) == 0x08);
const _: () = assert!(core::mem::offset_of!(ConditionCheckParams, quest) == 0x10);
const _: () = assert!(core::mem::offset_of!(ConditionCheckParams, quest_start_event) == 0x18);
const _: () = assert!(core::mem::offset_of!(ConditionCheckParams, unk20) == 0x20);
const _: () = assert!(core::mem::offset_of!(ConditionCheckParams, unk28) == 0x28);
const _: () = assert!(core::mem::offset_of!(ConditionCheckParams, package_data_list) == 0x30);

impl ConditionCheckParams {
    #[inline]
    pub const fn new(action_ref: *mut TESObjectREFR, target_ref: *mut TESObjectREFR) -> Self {
        Self {
            action_ref,
            target_ref,
            quest: core::ptr::null_mut(),
            quest_start_event: core::ptr::null_mut(),
            unk20: core::ptr::null_mut(),
            unk28: false,
            pad29: [0; 7],
            package_data_list: core::ptr::null_mut(),
        }
    }
}

/// C++ `RE::TESConditionItem`
#[repr(C)]
pub struct TESConditionItem {
    pub next: *mut TESConditionItem, // 00
    pub data: ConditionItemData,     // 08
}

const _: () = assert!(core::mem::size_of::<TESConditionItem>() == 0x38);
const _: () = assert!(core::mem::offset_of!(TESConditionItem, next) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESConditionItem, data) == 0x08);

impl Default for TESConditionItem {
    fn default() -> Self {
        Self {
            next: core::ptr::null_mut(),
            data: ConditionItemData::default(),
        }
    }
}

impl TESConditionItem {
    #[inline]
    pub fn evaluate(&self, params: &mut ConditionCheckParams) -> bool {
        self.is_true(params)
    }

    crate::relocation_func! {
        pub fn copy(&mut self, other: *const TESConditionItem, arg2: *mut TESForm) => RelocationID::new(29086, 29904)
    }

    crate::relocation_func! {
        pub fn is_true(&self, params: &mut ConditionCheckParams) -> bool => RelocationID::new(29090, 29924)
    }
}

/// C++ `RE::TESCondition`
#[repr(C)]
pub struct TESCondition {
    pub head: *mut TESConditionItem, // 00
}

const _: () = assert!(core::mem::size_of::<TESCondition>() == 0x8);
const _: () = assert!(core::mem::offset_of!(TESCondition, head) == 0x00);

impl Default for TESCondition {
    fn default() -> Self {
        Self::new()
    }
}

impl TESCondition {
    #[inline]
    pub const fn new() -> Self {
        Self {
            head: core::ptr::null_mut(),
        }
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    #[inline]
    pub const fn as_bool(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    pub fn evaluate(&self, action_ref: *mut TESObjectREFR, target_ref: *mut TESObjectREFR) -> bool {
        self.is_true(action_ref, target_ref)
    }

    crate::relocation_func! {
        pub fn copy(&mut self, other: *const TESCondition, arg2: *mut TESForm) => RelocationID::new(29067, 29879)
    }

    crate::relocation_func! {
        pub fn is_true(&self, action_ref: *mut TESObjectREFR, target_ref: *mut TESObjectREFR) -> bool => RelocationID::new(29074, 29888)
    }
}
