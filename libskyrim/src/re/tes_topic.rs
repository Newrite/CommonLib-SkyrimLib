use core::ffi::c_char;

use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESTopic;
use crate::offsets::offsets_vtable::VTABLE_TESTopic;
use crate::re::bgs_dialogue_branch::BGSDialogueBranch;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::dialogue_types::DIALOGUE_TYPE;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_form::TESForm;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_quest::TESQuest;
use crate::re::tes_topic_info::TESTopicInfo;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::DIALOGUE_DATA::TopicFlag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESTopicFlag {
    None = 0,
    DoAllBeforeRepeating = 1 << 0,
}

/// C++ `RE::DIALOGUE_DATA::Subtype`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESTopicSubtype {
    Custom = 0,
    ForceGreet = 1,
    Rumors = 2,
    Unk3 = 3,
    Intimidate = 4,
    Flatter = 5,
    Bribe = 6,
    AskGift = 7,
    Gift = 8,
    AskFavor = 9,
    Favor = 10,
    ShowRelationships = 11,
    Follow = 12,
    Reject = 13,
    Scene = 14,
    Show = 15,
    Agree = 16,
    Refuse = 17,
    ExitFavorState = 18,
    MoralRefusal = 19,
    FlyingMountLand = 20,
    FlyingMountCancelLand = 21,
    FlyingMountAcceptTarget = 22,
    FlyingMountRejectTarget = 23,
    FlyingMountNoTarget = 24,
    FlyingMountDestinationReached = 25,
    Attack = 26,
    PowerAttack = 27,
    Bash = 28,
    Hit = 29,
    Flee = 30,
    Bleedout = 31,
    AvoidThreat = 32,
    Death = 33,
    GroupStrategy = 34,
    Block = 35,
    Taunt = 36,
    AllyKilled = 37,
    Steal = 38,
    Yield = 39,
    AcceptYield = 40,
    PickpocketCombat = 41,
    Assault = 42,
    Murder = 43,
    AssaultNPC = 44,
    MurderNPC = 45,
    PickpocketNPC = 46,
    StealFromNPC = 47,
    TrespassAgainstNPC = 48,
    Trespass = 49,
    WereTransformCrime = 50,
    VoicePowerStartShort = 51,
    VoicePowerStartLong = 52,
    VoicePowerEndShort = 53,
    VoicePowerEndLong = 54,
    AlertIdle = 55,
    LostIdle = 56,
    NormalToAlert = 57,
    AlertToCombat = 58,
    NormalToCombat = 59,
    AlertToNormal = 60,
    CombatToNormal = 61,
    CombatToLost = 62,
    LostToNormal = 63,
    LostToCombat = 64,
    DetectFriendDie = 65,
    ServiceRefusal = 66,
    Repair = 67,
    Travel = 68,
    Training = 69,
    BarterExit = 70,
    RepairExit = 71,
    Recharge = 72,
    RechargeExit = 73,
    TrainingExit = 74,
    ObserveCombat = 75,
    NoticeCorpse = 76,
    TimeToGo = 77,
    GoodBye = 78,
    Hello = 79,
    SwingMeleeWeapon = 80,
    ShootBow = 81,
    ZKeyObject = 82,
    Jump = 83,
    KnockOverObject = 84,
    DestroyObject = 85,
    StandonFurniture = 86,
    LockedObject = 87,
    PickpocketTopic = 88,
    PursueIdleTopic = 89,
    SharedInfo = 90,
    PlayerCastProjectileSpell = 91,
    PlayerCastSelfSpell = 92,
    PlayerShout = 93,
    Idle = 94,
    EnterSprintBreath = 95,
    EnterBowZoomBreath = 96,
    ExitBowZoomBreath = 97,
    ActorCollidewithActor = 98,
    PlayerinIronSights = 99,
    OutofBreath = 100,
    CombatGrunt = 101,
    LeaveWaterBreath = 102,
}

core_util::impl_enumset_type!(TESTopicFlag => u8);
core_util::impl_enumset_type!(TESTopicSubtype => u16);

/// C++ `RE::DIALOGUE_DATA`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DIALOGUE_DATA {
    pub topic_flags: EnumSet<TESTopicFlag, u8>,    // 00
    pub dialogue_type: EnumSet<DIALOGUE_TYPE, u8>, // 01
    pub subtype: EnumSet<TESTopicSubtype, u16>,    // 02
}

const _: () = assert!(core::mem::size_of::<DIALOGUE_DATA>() == 0x4);

bitflags! {
    /// C++ `RE::TESTopic::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESTopicRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::TESTopic`
#[repr(C)]
pub struct TESTopic {
    pub base: TESForm,                        // 00
    pub full_name: TESFullName,               // 20
    pub data: DIALOGUE_DATA,                  // 30 - DATA
    pub priority_and_journal_index: u32,      // 34 - PNAM
    pub owner_branch: *mut BGSDialogueBranch, // 38 - BNAM
    pub owner_quest: *mut TESQuest,           // 40 - QNAM
    pub topic_infos: *mut *mut TESTopicInfo,  // 48
    pub num_topic_infos: u32,                 // 50 - TIFC
    pub first_file_offset: u32,               // 54
    pub form_editor_id: BSFixedString,        // 58
}

const _: () = assert!(core::mem::size_of::<TESTopic>() == 0x60);
const _: () = assert!(core::mem::offset_of!(TESTopic, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESTopic, data) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESTopic, priority_and_journal_index) == 0x34);
const _: () = assert!(core::mem::offset_of!(TESTopic, owner_branch) == 0x38);
const _: () = assert!(core::mem::offset_of!(TESTopic, owner_quest) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESTopic, topic_infos) == 0x48);
const _: () = assert!(core::mem::offset_of!(TESTopic, num_topic_infos) == 0x50);
const _: () = assert!(core::mem::offset_of!(TESTopic, first_file_offset) == 0x54);
const _: () = assert!(core::mem::offset_of!(TESTopic, form_editor_id) == 0x58);

impl RttiType for TESTopic {
    const RTTI: VariantID = RTTI_TESTopic;
}

impl FormCastable for TESTopic {
    const TARGET_FORM_TYPE: FormType = FormType::Dialogue;
}

inherit!(TESTopic : TESForm);
inherit!(TESTopic => TESFullName, full_name);

impl TESTopic {
    pub const RTTI: VariantID = RTTI_TESTopic;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESTopic;
    pub const FORMTYPE: FormType = FormType::Dialogue;

    // override (TESForm)
    // bool        Load(TESFile* a_mod) override;                // 06
    // void        InitItemImpl() override;                      // 13
    // const char* GetFormEditorID() const override;             // 32
    // bool        SetFormEditorID(const char* a_str) override;  // 33
    // bool        IsParentForm() override;                      // 34
    // bool        IsFormTypeChild(FormType a_type) override;    // 36

    // override (TESFullName)
    // std::uint32_t GetFullNameLength() const override;  // 04
    // const char*   GetFullName() const override;        // 05

    #[inline]
    pub fn get_priority(&self) -> f32 {
        (self.priority_and_journal_index >> 24) as f32
    }

    #[inline(always)]
    pub fn get_form_editor_id_local(&self) -> *const c_char {
        self.form_editor_id.as_ptr()
    }

    #[inline(always)]
    pub fn get_form_editor_id_local_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_form_editor_id_local())
    }

    #[inline]
    pub fn set_form_editor_id_local(&mut self, editor_id: *const c_char) -> bool {
        let result = self
            .form_editor_id
            .as_c_str()
            .and_then(|s| s.to_str().ok())
            .unwrap_or("<null>")
            == core_util::ptr_to_str(editor_id);
        self.form_editor_id = BSFixedString::new(editor_id);
        result
    }

    #[inline(always)]
    pub const fn is_parent_form_local(&self) -> bool {
        true
    }

    #[inline(always)]
    pub const fn is_form_type_child_local(&self, form_type: FormType) -> bool {
        matches!(form_type, FormType::Info)
    }
}
