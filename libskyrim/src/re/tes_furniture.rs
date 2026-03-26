use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESFurniture;
use crate::offsets::offsets_vtable::VTABLE_TESFurniture;
use crate::re::actor_values::ActorValue;
use crate::re::bgs_keyword::BGSKeyword;
use crate::re::bst_array::BSTArray;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::spell_item::SpellItem;
use crate::re::tes_object_acti::TESObjectACTI;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESFurnitureActiveMarker: u32 {
        const NONE = 0;
        const SIT0 = 1 << 0;
        const SIT1 = 1 << 1;
        const SIT2 = 1 << 2;
        const SIT3 = 1 << 3;
        const SIT4 = 1 << 4;
        const SIT5 = 1 << 5;
        const SIT6 = 1 << 6;
        const SIT7 = 1 << 7;
        const SIT8 = 1 << 8;
        const SIT9 = 1 << 9;
        const SIT10 = 1 << 10;
        const SIT11 = 1 << 11;
        const SIT12 = 1 << 12;
        const SIT13 = 1 << 13;
        const SIT14 = 1 << 14;
        const SIT15 = 1 << 15;
        const SIT16 = 1 << 16;
        const SIT17 = 1 << 17;
        const SIT18 = 1 << 18;
        const SIT19 = 1 << 19;
        const SIT20 = 1 << 20;
        const SIT21 = 1 << 21;
        const SIT22 = 1 << 22;
        const SIT23 = 1 << 23;
        const DISABLES_ACTIVATION = 1 << 25;
        const IS_PERCH = 1 << 26;
        const MUST_EXIT_TO_TALK = 1 << 27;
        const SIT28 = 1 << 28;
        const CAN_LEAN = 1 << 29;
        const CAN_SIT = 1 << 30;
        const CAN_SLEEP = 1 << 31;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESFurnitureRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IS_PERCH = 1 << 7;
        const IGNORED = 1 << 12;
        const HAS_DISTANT_LOD = 1 << 15;
        const RANDOM_ANIM_START = 1 << 16;
        const IS_MARKER = 1 << 23;
        const MUST_EXIT_TO_TALK = 1 << 28;
        const CHILD_CAN_USE = 1 << 29;
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESFurnitureBenchType {
    None = 0,
    CreateObject = 1,
    SmithingWeapon = 2,
    Enchanting = 3,
    EnchantingExperiment = 4,
    Alchemy = 5,
    AlchemyExperiment = 6,
    SmithingArmor = 7,
}

core_util::impl_enumset_type!(TESFurnitureBenchType => u8);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TESFurnitureWorkBenchData {
    pub bench_type: EnumSet<TESFurnitureBenchType, u8>, // 00
    pub uses_skill: EnumSet<ActorValue, u8>,            // 01
}

const _: () = assert!(core::mem::size_of::<TESFurnitureWorkBenchData>() == 0x2);

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESFurnitureDisabledPoint {
    None = 0,
    Front = 1 << 0,
    Behind = 1 << 1,
    Right = 1 << 2,
    Left = 1 << 3,
    Up = 1 << 4,
}

core_util::impl_enumset_type!(TESFurnitureDisabledPoint => u16);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TESFurnitureDisabledEntryPoint {
    pub unk0: u16,                                                // 00
    pub disabled_points: EnumSet<TESFurnitureDisabledPoint, u16>, // 02
}

const _: () = assert!(core::mem::size_of::<TESFurnitureDisabledEntryPoint>() == 0x4);

#[repr(C)]
pub struct TESFurnitureEntryPointData {
    pub entry_point: u32,                                      // 00
    pub disabled_entry_points: TESFurnitureDisabledEntryPoint, // 04
    pub keyword: *mut BGSKeyword,                              // 08
}

const _: () = assert!(core::mem::size_of::<TESFurnitureEntryPointData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESFurnitureEntryPointData, entry_point) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(TESFurnitureEntryPointData, disabled_entry_points) == 0x04);
const _: () = assert!(core::mem::offset_of!(TESFurnitureEntryPointData, keyword) == 0x08);

#[repr(C)]
pub struct TESFurniture {
    pub base: TESObjectACTI,                                          // 00
    pub entry_point_data_array: BSTArray<TESFurnitureEntryPointData>, // C8
    pub work_bench_data: TESFurnitureWorkBenchData,                   // E0
    pub pade2: u16,                                                   // E2
    pub furn_flags: TESFurnitureActiveMarker,                         // E4
    pub associated_form: *mut SpellItem,                              // E8
}

const _: () = assert!(core::mem::size_of::<TESFurniture>() == 0xF0);
const _: () = assert!(core::mem::offset_of!(TESFurniture, entry_point_data_array) == 0xC8);
const _: () = assert!(core::mem::offset_of!(TESFurniture, work_bench_data) == 0xE0);
const _: () = assert!(core::mem::offset_of!(TESFurniture, furn_flags) == 0xE4);
const _: () = assert!(core::mem::offset_of!(TESFurniture, associated_form) == 0xE8);

impl RttiType for TESFurniture {
    const RTTI: VariantID = RTTI_TESFurniture;
}

impl FormCastable for TESFurniture {
    const TARGET_FORM_TYPE: FormType = FormType::Furniture;
}

inherit!(TESFurniture : TESObjectACTI);

impl TESFurniture {
    pub const RTTI: VariantID = RTTI_TESFurniture;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESFurniture;
    pub const FORMTYPE: FormType = FormType::Furniture;

    // override (TESObjectACTI)
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    // override (TESObjectACTI)
    // void        InitializeData() override;                                                                // 04
    // void        ClearData() override;                                                                     // 05
    // bool        Load(TESFile* a_mod) override;                                                            // 06
    // void        InitItemImpl() override;                                                                  // 13
    // bool        Activate(...) override;                                                                   // 37
    // NiAVObject* Clone3D(TESObjectREFR* a_ref) override;                                                   // 4A
    // bool        GetActivateText(TESObjectREFR* a_activator, BSString& a_dst) override;                   // 4C
    // bool        CalculateDoFavor(Actor* a_activator, bool a_arg2, TESObjectREFR* a_toActivate, float) override;  // 4D
}
