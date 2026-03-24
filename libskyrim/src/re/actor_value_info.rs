use bitflags::bitflags;
use core::ffi::c_char;

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_ActorValueInfo;
use crate::offsets::offsets_vtable::VTABLE_ActorValueInfo;
use crate::re::actor_value_owner::ActorValueOwner;
use crate::re::bgs_skill_perk_tree_node::BGSSkillPerkTreeNode;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_description::TESDescription;
use crate::re::tes_form::TESForm;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_icon::TESIcon;
use crate::relocation::{RttiType, VariantID};

pub type ActorValueInfoDynamicBaseValueFunctor =
    unsafe extern "C" fn(*mut ActorValueOwner, u32) -> f32;

bitflags! {
    /// C++ `RE::ActorValueInfo::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ActorValueInfoRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::ActorValueInfo::ActorValueType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorValueInfoType {
    Attribute = 0,
    Skill = 1,
    AITemperament = 2,
    DamageResistance = 3,
    LimbCondition = 4,
    Status = 5,
    Miscellaneous = 6,
}

/// C++ `RE::ActorValueInfo::ActorValueFlag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorValueInfoFlag {
    HostileEffectsScaleWithDifficulty = 1 << 1,
    SpecialStatClampsAsNonZero = 1 << 2,
    ClampAsSpecialStat = 1 << 3,
    ClampAsSkill = 1 << 4,
    CanHaveModifiers = 1 << 5,
    BaseValueIsDynamicPlusCurrent = 1 << 6,
    BaseValueComputedFromActor = 1 << 7,
    Enumeration = 1 << 8,
    Inverted = 1 << 9,
    BaseValueComputedFromRace = 1 << 11,
    CannotBeAlteredByScripts = 1 << 14,
    BaseValueIsAlwaysZero = 1 << 15,
    BaseValueIsAlwaysOne = 1 << 16,
    BaseValueIsAlwaysOneHundred = 1 << 17,
    AIProcessCachesCurrentValue = 1 << 18,
    AIProcessCachesMaxValue = 1 << 19,
    ProtectedByGodMode = 1 << 20,
    DisplayedEffectMagnitudeTimesOneHundred = 1 << 21,
}

core_util::impl_enumset_type!(ActorValueInfoFlag => u32);

/// C++ `RE::ActorValueInfo::Skill` (`AVSK`)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ActorValueInfoSkill {
    pub use_mult: f32,       // 00
    pub offset_mult: f32,    // 04
    pub improve_mult: f32,   // 08
    pub improve_offset: f32, // 0C
}

const _: () = assert!(core::mem::size_of::<ActorValueInfoSkill>() == 0x10);

/// C++ `RE::ActorValueInfo`
#[repr(C)]
pub struct ActorValueInfo {
    pub base: TESForm,                                                    // 000
    pub full_name: TESFullName,                                           // 020
    pub description: TESDescription,                                      // 030
    pub icon: TESIcon,                                                    // 040
    pub enum_name: *const c_char,                                         // 050
    pub abbreviation: BSFixedString,                                      // 058 - ANAM
    pub flags: EnumSet<ActorValueInfoFlag, u32>,                          // 060
    pub actor_value_type: ActorValueInfoType,                             // 064
    pub compute_base_func: Option<ActorValueInfoDynamicBaseValueFunctor>, // 068
    pub unk070: u32,                                                      // 070
    pub unk074: u32,                                                      // 074
    pub unk078: u32,                                                      // 078
    pub unk07c: u32,                                                      // 07C
    pub unk080: u32,                                                      // 080
    pub unk084: u32,                                                      // 084
    pub unk088: u32,                                                      // 088
    pub unk08c: u32,                                                      // 08C
    pub unk090: u32,                                                      // 090
    pub unk094: u32,                                                      // 094
    pub unk098: u32,                                                      // 098
    pub unk09c: u32,                                                      // 09C
    pub unk0a0: u32,                                                      // 0A0
    pub unk0a4: u32,                                                      // 0A4
    pub unk0a8: u32,                                                      // 0A8
    pub unk0ac: u32,                                                      // 0AC
    pub enum_value_count: u64,                                            // 0B0
    pub enum_values: [*const c_char; 0xA],                                // 0B8
    pub skill: *mut ActorValueInfoSkill,                                  // 108 - AVSK
    pub is_nth_ai_cached_current_value: u32,                              // 110
    pub is_nth_ai_cached_max_value: u32,                                  // 114
    pub perk_tree: *mut BGSSkillPerkTreeNode,                             // 118
    pub perk_tree_width: u32,                                             // 120
    pub unk124: u32,                                                      // 124 - CNAM
}

const _: () = assert!(core::mem::size_of::<ActorValueInfo>() == 0x128);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, full_name) == 0x020);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, description) == 0x030);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, icon) == 0x040);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, enum_name) == 0x050);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, abbreviation) == 0x058);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, flags) == 0x060);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, actor_value_type) == 0x064);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, compute_base_func) == 0x068);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, enum_value_count) == 0x0B0);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, enum_values) == 0x0B8);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, skill) == 0x108);
const _: () =
    assert!(core::mem::offset_of!(ActorValueInfo, is_nth_ai_cached_current_value) == 0x110);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, is_nth_ai_cached_max_value) == 0x114);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, perk_tree) == 0x118);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, perk_tree_width) == 0x120);
const _: () = assert!(core::mem::offset_of!(ActorValueInfo, unk124) == 0x124);

impl RttiType for ActorValueInfo {
    const RTTI: VariantID = RTTI_ActorValueInfo;
}

impl FormCastable for ActorValueInfo {
    const TARGET_FORM_TYPE: FormType = FormType::ActorValueInfo;
}

impl AsRef<ActorValueInfo> for ActorValueInfo {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<ActorValueInfo> for ActorValueInfo {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(ActorValueInfo : TESForm);
inherit!(ActorValueInfo => TESFullName, full_name);
inherit!(ActorValueInfo => TESDescription, description);
inherit!(ActorValueInfo => TESIcon, icon);

impl ActorValueInfo {
    pub const RTTI: VariantID = RTTI_ActorValueInfo;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ActorValueInfo;
    pub const FORMTYPE: FormType = FormType::ActorValueInfo;

    // override (TESForm)
    // void ClearData() override;           // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13

    // override (TESIcon)
    // const char* GetDefaultPath() const override;  // 06

    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        self.flags.all(ActorValueInfoFlag::Inverted)
    }
}

pub trait ActorValueInfoExt {
    fn is_inverted(&self) -> bool;
}

impl<T: AsRef<ActorValueInfo>> ActorValueInfoExt for T {
    #[inline(always)]
    fn is_inverted(&self) -> bool {
        self.as_ref().is_inverted()
    }
}
