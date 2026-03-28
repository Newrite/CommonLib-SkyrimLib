use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSRefAlias;
use crate::offsets::offsets_vtable::VTABLE_BGSRefAlias;
use crate::re::{
    Actor, BGSBaseAlias, BGSLocationRefType, BSFixedString, ObjectRefHandle, QuestEvent,
    TESBoundObject, TESCondition, TESFile, TESForm, TESNPC, TESObjectREFR, TESQuest, VMTypeID,
};
use crate::relocation::{RttiType, VariantID, skyrim_cast};
use crate::virtual_method;

/// C++ `RE::BGSRefAlias::ForcedFillData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSRefAliasForcedFillData {
    pub forced_ref: ObjectRefHandle, // 00
}

const _: () = assert!(core::mem::size_of::<BGSRefAliasForcedFillData>() == 0x4);

/// C++ `RE::BGSRefAlias::FromAliasFillData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSRefAliasFromAliasFillData {
    pub forced_from_alias: u32,                   // 00
    pub pad04: u32,                               // 04
    pub forced_ref_type: *mut BGSLocationRefType, // 08
}

const _: () = assert!(core::mem::size_of::<BGSRefAliasFromAliasFillData>() == 0x10);

/// C++ `RE::BGSRefAlias::FromEventFillData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSRefAliasFromEventFillData {
    pub forced_from_event: QuestEvent, // 00
    pub forced_event_data: u32,        // 04
}

const _: () = assert!(core::mem::size_of::<BGSRefAliasFromEventFillData>() == 0x8);

/// C++ `RE::BGSRefAlias::CreatedFillData::Level`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSRefAliasCreatedFillDataLevel {
    Easy = 0,
    Medium = 1,
    Hard = 2,
    VeryHard = 3,
    None = 4,
}

core_util::impl_enumset_type!(BGSRefAliasCreatedFillDataLevel => u16);

/// C++ `RE::BGSRefAlias::CreatedFillData::Alias::Create`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSRefAliasCreatedFillDataCreate {
    At = 0x0000,
    In = 0x8000,
}

core_util::impl_enumset_type!(BGSRefAliasCreatedFillDataCreate => u16);

/// C++ `RE::BGSRefAlias::CreatedFillData::Alias`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSRefAliasCreatedFillDataAlias {
    pub alias: u16,                                             // 00
    pub create: EnumSet<BGSRefAliasCreatedFillDataCreate, u16>, // 02
}

const _: () = assert!(core::mem::size_of::<BGSRefAliasCreatedFillDataAlias>() == 0x4);

/// C++ `RE::BGSRefAlias::CreatedFillData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSRefAliasCreatedFillData {
    pub object: *mut TESBoundObject,                          // 00
    pub alias: BGSRefAliasCreatedFillDataAlias,               // 08
    pub level: EnumSet<BGSRefAliasCreatedFillDataLevel, u16>, // 0C
    pub pad0e: u16,                                           // 0E
}

const _: () = assert!(core::mem::size_of::<BGSRefAliasCreatedFillData>() == 0x10);

/// C++ `RE::BGSRefAlias::FromExternalFillData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSRefAliasFromExternalFillData {
    pub external_quest: *mut TESQuest, // 00
    pub external_alias: u32,           // 08
    pub pad0c: u32,                    // 0C
}

const _: () = assert!(core::mem::size_of::<BGSRefAliasFromExternalFillData>() == 0x10);

/// C++ `RE::BGSRefAlias::UniqueActorFillData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSRefAliasUniqueActorFillData {
    pub unique_actor: *mut TESNPC, // 00
}

const _: () = assert!(core::mem::size_of::<BGSRefAliasUniqueActorFillData>() == 0x8);

/// C++ `RE::BGSRefAlias::NearAliasFillData::NEARFILLTYPE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSRefAliasNearAliasFillType {
    LinkedChildren = 0,
    LinkedParents = 1,
}

core_util::impl_enumset_type!(BGSRefAliasNearAliasFillType => u32);

/// C++ `RE::BGSRefAlias::NearAliasFillData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSRefAliasNearAliasFillData {
    pub near_alias: u32,                                            // 00
    pub near_fill_type: EnumSet<BGSRefAliasNearAliasFillType, u32>, // 04
}

const _: () = assert!(core::mem::size_of::<BGSRefAliasNearAliasFillData>() == 0x8);

/// C++ `RE::BGSRefAlias::GenericFillData::Padding`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSRefAliasGenericFillDataPadding {
    pub pad1: u64, // 00
    pub pad2: u64, // 08
    pub pad3: u64, // 10
}

const _: () = assert!(core::mem::size_of::<BGSRefAliasGenericFillDataPadding>() == 0x18);

/// C++ `RE::BGSRefAlias::GenericFillData`
#[repr(C)]
#[derive(Clone, Copy)]
pub union BGSRefAliasGenericFillData {
    pub padding: BGSRefAliasGenericFillDataPadding,
    pub forced: BGSRefAliasForcedFillData,
    pub from_alias: BGSRefAliasFromAliasFillData,
    pub from_event: BGSRefAliasFromEventFillData,
    pub created: BGSRefAliasCreatedFillData,
    pub from_external: BGSRefAliasFromExternalFillData,
    pub unique_actor: BGSRefAliasUniqueActorFillData,
    pub near_alias: BGSRefAliasNearAliasFillData,
}

const _: () = assert!(core::mem::size_of::<BGSRefAliasGenericFillData>() == 0x18);

/// C++ `RE::BGSRefAlias`
#[repr(C)]
pub struct BGSRefAlias {
    pub base: BGSBaseAlias,                    // 00
    pub fill_data: BGSRefAliasGenericFillData, // 28
    pub conditions: *mut TESCondition,         // 40
}

const _: () = assert!(core::mem::size_of::<BGSRefAlias>() == 0x48);
const _: () = assert!(core::mem::offset_of!(BGSRefAlias, fill_data) == 0x28);
const _: () = assert!(core::mem::offset_of!(BGSRefAlias, conditions) == 0x40);

impl RttiType for BGSRefAlias {
    const RTTI: VariantID = RTTI_BGSRefAlias;
}

inherit!(BGSRefAlias : BGSBaseAlias);

impl BGSRefAlias {
    pub const RTTI: VariantID = RTTI_BGSRefAlias;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSRefAlias;
    pub const VM_TYPE_ID: VMTypeID = 140;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    // override (BGSBaseAlias)
    virtual_method! {
        pub const VFUNC_LOAD: usize = 0x01;
        pub fn load(&mut self, mod_file: *mut TESFile) -> bool
    }

    virtual_method! {
        pub const VFUNC_INIT_ITEM: usize = 0x02;
        pub fn init_item(&mut self, form: *mut TESForm)
    }

    virtual_method! {
        pub const VFUNC_Q_TYPE: usize = 0x03;
        fn q_type_ptr(&self) -> *const BSFixedString
    }

    #[inline(always)]
    pub fn q_type(&self) -> &BSFixedString {
        let q_type = self.q_type_ptr();
        debug_assert!(!q_type.is_null());
        unsafe { &*q_type }
    }

    #[inline(always)]
    pub fn get_reference(&self) -> *mut TESObjectREFR {
        let owner = self.base.owning_quest;
        if owner.is_null() {
            return core::ptr::null_mut();
        }

        let handle = unsafe { (*owner).get_aliased_ref(self.base.alias_id) };
        handle.get().get()
    }

    #[inline(always)]
    pub fn get_reference_ref(&self) -> Option<&TESObjectREFR> {
        unsafe { self.get_reference().as_ref() }
    }

    #[inline(always)]
    pub fn get_actor_reference(&self) -> *mut Actor {
        let refr = self.get_reference();
        if refr.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { skyrim_cast::<TESObjectREFR, Actor>(refr) }
        }
    }

    #[inline(always)]
    pub fn get_actor_reference_ref(&self) -> Option<&Actor> {
        unsafe { self.get_actor_reference().as_ref() }
    }

    #[inline(always)]
    pub fn force_ref_to(&mut self, refr: *mut TESObjectREFR) {
        if refr.is_null() {
            return;
        }

        let owner = self.base.owning_quest;
        if !owner.is_null() {
            unsafe { (*owner).force_ref_into_alias(self.base.alias_id, refr) };
        }
    }
}
