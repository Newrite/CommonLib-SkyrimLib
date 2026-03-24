use bitflags::bitflags;
use core::ffi::c_char;

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESQuest;
use crate::offsets::offsets_vtable::VTABLE_TESQuest;
use crate::re::BGSBaseAlias;
use crate::re::BGSDialogueBranch;
use crate::re::BGSScene;
use crate::re::BGSStoryManagerTreeForm;
use crate::re::BGSStoryTeller;
use crate::re::BSFixedString;
use crate::re::BSSimpleList;
use crate::re::BSString;
use crate::re::BSTArray;
use crate::re::BSTHashMap;
use crate::re::DIALOGUE_TYPE_BRANCHED_TOTAL;
use crate::re::DIALOGUE_TYPE_TOTAL;
use crate::re::FormCastable;
use crate::re::FormType;
use crate::re::ObjectRefHandle;
use crate::re::QUEST_OBJECTIVE_STATE;
use crate::re::QuestEvent;
use crate::re::QueuedPromoteQuestTask;
use crate::re::TESCondition;
use crate::re::TESFullName;
use crate::re::TESGlobal;
use crate::re::TESObjectREFR;
use crate::re::TESTopic;
use crate::re::TeleportPath;
use crate::re::UnkKey;
use crate::re::UnkValue;
use crate::re::bs_atomic::{BSReadLockGuard, BSReadWriteLock};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::QuestFlag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestFlag {
    StopStart = -1,
    None = 0,
    Enabled = 1 << 0,
    Completed = 1 << 1,
    AddIdleToHello = 1 << 2,
    AllowRepeatStages = 1 << 3,
    StartsEnabled = 1 << 4,
    DisplayedInHUD = 1 << 5,
    Failed = 1 << 6,
    StageWait = 1 << 7,
    RunOnce = 1 << 8,
    ExcludeFromExport = 1 << 9,
    WarnOnAliasFillFailure = 1 << 10,
    Active = 1 << 11,
    RepeatsConditions = 1 << 12,
    KeepInstance = 1 << 13,
    WantDormant = 1 << 14,
    HasDialogueData = 1 << 15,
}

core_util::impl_enumset_type!(QuestFlag => u16);

/// C++ `RE::QUEST_OBJECTIVE_FLAGS`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestObjectiveFlag {
    None = 0,
    ORWithPrevious = 1 << 0,
    NoStatsTracking = 1 << 1,
}

core_util::impl_enumset_type!(QuestObjectiveFlag => u32);

/// C++ `RE::BGSQuestInstanceText::StringData`
#[repr(C)]
pub struct BGSQuestInstanceStringData {
    pub alias_id: u32,          // 00
    pub full_name_form_id: u32, // 04
}

const _: () = assert!(core::mem::size_of::<BGSQuestInstanceStringData>() == 0x8);

/// C++ `RE::BGSQuestInstanceText::GlobalValueData`
#[repr(C)]
pub struct BGSQuestInstanceGlobalValueData {
    pub global: *const TESGlobal, // 00
    pub value: f32,               // 08
    pub pad0c: u32,               // 0C
}

const _: () = assert!(core::mem::size_of::<BGSQuestInstanceGlobalValueData>() == 0x10);

/// C++ `RE::BGSQuestInstanceText`
#[repr(C)]
pub struct BGSQuestInstanceText {
    pub id: u32,                                               // 00
    pub pad04: u32,                                            // 04
    pub string_data: BSTArray<BGSQuestInstanceStringData>,     // 08
    pub value_data: BSTArray<BGSQuestInstanceGlobalValueData>, // 20
    pub journal_stage: u16,                                    // 38
    pub journal_stage_item: i8,                                // 3A
    pub pad3b: u8,                                             // 3B
    pub pad3c: u32,                                            // 3C
}

const _: () = assert!(core::mem::size_of::<BGSQuestInstanceText>() == 0x40);
const _: () = assert!(core::mem::offset_of!(BGSQuestInstanceText, string_data) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSQuestInstanceText, value_data) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSQuestInstanceText, journal_stage) == 0x38);

/// C++ `RE::QUEST_DATA::Type`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestType {
    None = 0,
    MainQuest = 1,
    MagesGuild = 2,
    ThievesGuild = 3,
    DarkBrotherhood = 4,
    CompanionsQuest = 5,
    Miscellaneous = 6,
    Daedric = 7,
    SideQuest = 8,
    CivilWar = 9,
    DLC01Vampire = 10,
    DLC02Dragonborn = 11,
}

core_util::impl_enumset_type!(QuestType => u8);

impl TryFrom<u8> for QuestType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::MainQuest),
            2 => Ok(Self::MagesGuild),
            3 => Ok(Self::ThievesGuild),
            4 => Ok(Self::DarkBrotherhood),
            5 => Ok(Self::CompanionsQuest),
            6 => Ok(Self::Miscellaneous),
            7 => Ok(Self::Daedric),
            8 => Ok(Self::SideQuest),
            9 => Ok(Self::CivilWar),
            10 => Ok(Self::DLC01Vampire),
            11 => Ok(Self::DLC02Dragonborn),
            _ => Err(()),
        }
    }
}

/// C++ `RE::QUEST_DATA`
#[repr(C)]
pub struct QuestData {
    pub quest_delay_time: f32,              // 00
    pub flags: EnumSet<QuestFlag, u16>,     // 04
    pub priority: i8,                       // 06
    pub quest_type: EnumSet<QuestType, u8>, // 07
}

const _: () = assert!(core::mem::size_of::<QuestData>() == 0x8);
const _: () = assert!(core::mem::offset_of!(QuestData, quest_delay_time) == 0x00);
const _: () = assert!(core::mem::offset_of!(QuestData, flags) == 0x04);
const _: () = assert!(core::mem::offset_of!(QuestData, priority) == 0x06);
const _: () = assert!(core::mem::offset_of!(QuestData, quest_type) == 0x07);

/// C++ `RE::QUEST_STAGE_DATA::Flag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestStageFlag {
    None = 0,
    StartUpStage = 1 << 1,
    ShutDownStage = 1 << 2,
    KeepInstanceDataFromHereOn = 1 << 3,
}

core_util::impl_enumset_type!(QuestStageFlag => u8);

/// C++ `RE::QUEST_STAGE_DATA`
#[repr(C)]
pub struct QuestStageData {
    pub index: u16,                         // 00
    pub flags: EnumSet<QuestStageFlag, u8>, // 02
    pub pad3: u8,                           // 03
    pub pad4: u32,                          // 04
}

const _: () = assert!(core::mem::size_of::<QuestStageData>() == 0x8);

/// C++ `RE::TESQuestStage`
#[repr(C)]
pub struct TESQuestStage {
    pub data: QuestStageData, // 00
}

const _: () = assert!(core::mem::size_of::<TESQuestStage>() == 0x8);

impl TESQuestStage {
    #[inline]
    pub fn as_bool(&self) -> bool {
        unsafe { *(self as *const Self as *const usize) != 0 }
    }
}

/// C++ `RE::TESQuestTarget::Flag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESQuestTargetFlag {
    None = 0,
    CompassMarkerIgnoresLocks = 1 << 0,
}

core_util::impl_enumset_type!(TESQuestTargetFlag => u8);

/// C++ `RE::TESQuestTarget`
#[repr(C)]
pub struct TESQuestTarget {
    pub flags: EnumSet<TESQuestTargetFlag, u8>, // 00
    pub pad01: [u8; 7],                         // 01
    pub conditions: TESCondition,               // 08
    pub alias: u32,                             // 10
    pub pad14: u32,                             // 14
    pub teleport_path: TeleportPath,            // 18
}

const _: () = assert!(core::mem::size_of::<TESQuestTarget>() == 0x60);
const _: () = assert!(core::mem::offset_of!(TESQuestTarget, flags) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESQuestTarget, conditions) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESQuestTarget, alias) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESQuestTarget, teleport_path) == 0x18);

impl TESQuestTarget {
    crate::relocation_func! {
        fn get_target_ref_impl(&self, out: *mut ObjectRefHandle, allow_pick_up_actor: bool, quest: *const TESQuest) -> *mut ObjectRefHandle => RelocationID::new(24815, 25284)
    }

    crate::relocation_func! {
        fn get_tracking_ref_impl(&self, out: *mut ObjectRefHandle, quest: *const TESQuest) -> *mut ObjectRefHandle => RelocationID::new(24816, 25285)
    }

    #[inline]
    pub fn get_target_ref<'a>(
        &self,
        out: &'a mut ObjectRefHandle,
        allow_pick_up_actor: bool,
        quest: *const TESQuest,
    ) -> &'a mut ObjectRefHandle {
        let _ = self.get_target_ref_impl(out as *mut _, allow_pick_up_actor, quest);
        out
    }

    #[inline]
    pub fn get_tracking_ref<'a>(
        &self,
        out: &'a mut ObjectRefHandle,
        quest: *const TESQuest,
    ) -> &'a mut ObjectRefHandle {
        let _ = self.get_tracking_ref_impl(out as *mut _, quest);
        out
    }
}

/// C++ `RE::BGSQuestObjective`
#[repr(C)]
pub struct BGSQuestObjective {
    pub display_text: BSFixedString,               // 00
    pub owner_quest: *mut TESQuest,                // 08
    pub targets: *mut *mut TESQuestTarget,         // 10
    pub num_targets: u32,                          // 18
    pub index: u16,                                // 1C
    pub initialized: bool,                         // 1E
    pub state: EnumSet<QUEST_OBJECTIVE_STATE, u8>, // 1F
    pub flags: EnumSet<QuestObjectiveFlag, u32>,   // 20
    pub pad24: u32,                                // 24
}

const _: () = assert!(core::mem::size_of::<BGSQuestObjective>() == 0x28);
const _: () = assert!(core::mem::offset_of!(BGSQuestObjective, display_text) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSQuestObjective, owner_quest) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSQuestObjective, targets) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSQuestObjective, num_targets) == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSQuestObjective, index) == 0x1C);
const _: () = assert!(core::mem::offset_of!(BGSQuestObjective, initialized) == 0x1E);
const _: () = assert!(core::mem::offset_of!(BGSQuestObjective, state) == 0x1F);
const _: () = assert!(core::mem::offset_of!(BGSQuestObjective, flags) == 0x20);

/// C++ `RE::BGSStoryEvent`
#[repr(C)]
pub struct BGSStoryEvent {
    pub id: u32,           // 00
    pub index: u32,        // 04
    pub members: [u64; 6], // 08
}

const _: () = assert!(core::mem::size_of::<BGSStoryEvent>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BGSStoryEvent, id) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSStoryEvent, members) == 0x08);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESQuestChangeFlags: u32 {
        const NONE = 0;
        const QUEST_FLAGS = 1 << 1;
        const QUEST_SCRIPT_DELAY = 1 << 2;
        const QUEST_ALREADY_RUN = 1 << 26;
        const QUEST_INSTANCE_DATA = 1 << 27;
        const QUEST_RUNTIME_DATA = 1 << 28;
        const QUEST_OBJECTIVES = 1 << 29;
        const QUEST_SCRIPT = 1 << 30;
        const QUEST_STAGES = 1u32 << 31;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESQuestRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::TESQuest`
#[repr(C)]
pub struct TESQuest {
    pub base: BGSStoryManagerTreeForm,                         // 000
    pub full_name: TESFullName,                                // 028
    pub instance_data: BSTArray<*mut BGSQuestInstanceText>,    // 038
    pub current_instance_id: u32,                              // 050
    pub pad054: u32,                                           // 054
    pub aliases: BSTArray<*mut BGSBaseAlias>,                  // 058
    pub ref_alias_map: BSTHashMap<u32, ObjectRefHandle>,       // 070
    pub unk0a0: BSTHashMap<UnkKey, UnkValue>,                  // 0A0
    pub alias_access_lock: BSReadWriteLock,                    // 0D0
    pub data: QuestData,                                       // 0D8
    pub event_id: QuestEvent,                                  // 0E0
    pub pad0e4: u32,                                           // 0E4
    pub executed_stages: *mut BSSimpleList<TESQuestStage>,     // 0E8
    pub waiting_stages: *mut BSSimpleList<*mut TESQuestStage>, // 0F0
    pub objectives: BSSimpleList<*mut BGSQuestObjective>,      // 0F8
    pub obj_conditions: TESCondition,                          // 108
    pub story_manager_conditions: TESCondition,                // 110
    pub branched_dialogue: [BSTHashMap<*mut BGSDialogueBranch, *mut BSTArray<*mut TESTopic>>;
        DIALOGUE_TYPE_BRANCHED_TOTAL], // 118
    pub topics: [BSTArray<*mut TESTopic>; DIALOGUE_TYPE_TOTAL - DIALOGUE_TYPE_BRANCHED_TOTAL], // 178
    pub scenes: BSTArray<*mut BGSScene>,             // 208
    pub text_globals: *mut BSTArray<*mut TESGlobal>, // 220
    pub current_stage: u16,                          // 228
    pub already_run: bool,                           // 22A
    pub pad22b: u8,                                  // 22B
    pub pad22c: u32,                                 // 22C
    pub form_editor_id: BSString,                    // 230
    pub start_event_data: *const BGSStoryEvent,      // 240
    pub promote_task: *mut QueuedPromoteQuestTask,   // 248 - NiPointer
    pub promoted_refs: BSTArray<ObjectRefHandle>,    // 250
}

const _: () = assert!(core::mem::size_of::<TESQuest>() == 0x268);
const _: () = assert!(core::mem::offset_of!(TESQuest, base) == 0x000);
const _: () = assert!(core::mem::offset_of!(TESQuest, full_name) == 0x028);
const _: () = assert!(core::mem::offset_of!(TESQuest, instance_data) == 0x038);
const _: () = assert!(core::mem::offset_of!(TESQuest, current_instance_id) == 0x050);
const _: () = assert!(core::mem::offset_of!(TESQuest, aliases) == 0x058);
const _: () = assert!(core::mem::offset_of!(TESQuest, ref_alias_map) == 0x070);
const _: () = assert!(core::mem::offset_of!(TESQuest, unk0a0) == 0x0A0);
const _: () = assert!(core::mem::offset_of!(TESQuest, alias_access_lock) == 0x0D0);
const _: () = assert!(core::mem::offset_of!(TESQuest, data) == 0x0D8);
const _: () = assert!(core::mem::offset_of!(TESQuest, event_id) == 0x0E0);
const _: () = assert!(core::mem::offset_of!(TESQuest, executed_stages) == 0x0E8);
const _: () = assert!(core::mem::offset_of!(TESQuest, waiting_stages) == 0x0F0);
const _: () = assert!(core::mem::offset_of!(TESQuest, objectives) == 0x0F8);
const _: () = assert!(core::mem::offset_of!(TESQuest, obj_conditions) == 0x108);
const _: () = assert!(core::mem::offset_of!(TESQuest, story_manager_conditions) == 0x110);
const _: () = assert!(core::mem::offset_of!(TESQuest, branched_dialogue) == 0x118);
const _: () = assert!(core::mem::offset_of!(TESQuest, topics) == 0x178);
const _: () = assert!(core::mem::offset_of!(TESQuest, scenes) == 0x208);
const _: () = assert!(core::mem::offset_of!(TESQuest, text_globals) == 0x220);
const _: () = assert!(core::mem::offset_of!(TESQuest, current_stage) == 0x228);
const _: () = assert!(core::mem::offset_of!(TESQuest, already_run) == 0x22A);
const _: () = assert!(core::mem::offset_of!(TESQuest, form_editor_id) == 0x230);
const _: () = assert!(core::mem::offset_of!(TESQuest, start_event_data) == 0x240);
const _: () = assert!(core::mem::offset_of!(TESQuest, promote_task) == 0x248);
const _: () = assert!(core::mem::offset_of!(TESQuest, promoted_refs) == 0x250);

impl RttiType for TESQuest {
    const RTTI: VariantID = RTTI_TESQuest;
}

impl FormCastable for TESQuest {
    const TARGET_FORM_TYPE: FormType = FormType::Quest;
}

inherit!(TESQuest : BGSStoryManagerTreeForm);
inherit!(TESQuest => TESFullName, full_name);

impl TESQuest {
    pub const RTTI: VariantID = RTTI_TESQuest;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESQuest;
    pub const FORMTYPE: FormType = FormType::Quest;

    // override (BGSStoryManagerTreeForm)
    // void InitializeData() override;  // 04
    // void ClearData() override;  // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void SaveGame(BGSSaveFormBuffer* a_buf) override;  // 0E
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;  // 0F
    // void FinishLoadGame(BGSLoadFormBuffer* a_buf) override;  // 11
    // void Revert(BGSLoadFormBuffer* a_buf) override;  // 12
    // void InitItemImpl() override;  // 13
    // const char* GetFormEditorID() const override;  // 32 - { return formEditorID.c_str(); }
    // bool SetFormEditorID(const char* a_str) override;  // 33
    // TESCondition* QConditions() override;  // 3D - { return &objConditions; }
    // BGSStoryManagerTreeVisitor::VisitControl AcceptVisitor(BGSStoryManagerTreeVisitor& a_visitor) override;  // 3E

    crate::relocation_func! {
        pub fn ensure_quest_started(&mut self, result: &mut bool, start_now: bool) -> bool => RelocationID::new(24481, 25003)
    }

    crate::relocation_func! {
        pub fn force_ref_into_alias(&mut self, alias_id: u32, refr: *mut TESObjectREFR) => RelocationID::new(24523, 25052)
    }

    crate::relocation_func! {
        pub fn get_journal_text_for_instance(&mut self, out: &mut BSString, instance_id: u32) => RelocationID::new(24549, 25078)
    }

    crate::relocation_func! {
        pub fn reset(&mut self) => RelocationID::new(24486, 25014)
    }

    #[inline]
    pub fn get_aliased_ref(&self, alias_id: u32) -> ObjectRefHandle {
        let _locker = BSReadLockGuard::new(&self.alias_access_lock);
        let value = self.ref_alias_map.find(&alias_id);
        if value.is_null() {
            ObjectRefHandle::new()
        } else {
            unsafe { (*value).second }
        }
    }

    #[inline]
    pub const fn get_current_stage_id(&self) -> u16 {
        self.current_stage
    }

    #[inline]
    pub fn get_type(&self) -> Option<QuestType> {
        self.data.quest_type.get()
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.data.flags.all(QuestFlag::Active)
    }

    #[inline]
    pub fn is_completed(&self) -> bool {
        self.data.flags.all(QuestFlag::Completed)
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.data.flags.all(QuestFlag::Enabled)
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        !self.is_stopping() && self.promote_task.is_null()
    }

    #[inline]
    pub fn is_starting(&self) -> bool {
        self.is_enabled()
            && (self.data.flags == QuestFlag::StopStart || !self.promote_task.is_null())
    }

    #[inline]
    pub fn is_stopped(&self) -> bool {
        self.data.flags.none(QuestFlag::Enabled) && self.data.flags.none(QuestFlag::StageWait)
    }

    #[inline]
    pub fn is_stopping(&self) -> bool {
        !self.is_enabled() && self.data.flags == QuestFlag::StopStart
    }

    #[inline]
    pub fn q_conditions(&mut self) -> *mut TESCondition {
        &mut self.obj_conditions
    }

    #[inline]
    pub fn get_form_editor_id(&self) -> *const c_char {
        self.form_editor_id.c_str()
    }

    #[inline]
    pub fn set_form_editor_id(&mut self, string: *const c_char) -> bool {
        self.form_editor_id.set_c_str(string, 0)
    }

    #[inline]
    pub fn reset_and_update(&mut self) {
        self.reset();

        let enabled = self.is_enabled();
        if enabled != self.starts_enabled() {
            let story_teller = BGSStoryTeller::get_singleton();
            if !story_teller.is_null() {
                unsafe {
                    if enabled {
                        (*story_teller).begin_start_up_quest(self);
                    } else {
                        (*story_teller).begin_shut_down_quest(self);
                    }
                }
            }
        }
    }

    #[inline]
    pub fn set_enabled(&mut self, set: bool) {
        if set {
            self.data.flags.set(QuestFlag::Enabled);
        } else {
            self.data.flags.reset(QuestFlag::Enabled);
        }
        self.add_change(TESQuestChangeFlags::QUEST_FLAGS.bits());
    }

    #[inline]
    pub fn start(&mut self) -> bool {
        if self.event_id != QuestEvent::None {
            return false;
        }

        let mut result = false;
        self.ensure_quest_started(&mut result, true)
    }

    #[inline]
    pub fn starts_enabled(&self) -> bool {
        self.data.flags.all(QuestFlag::StartsEnabled)
    }

    #[inline]
    pub fn stop(&mut self) {
        if self.is_enabled() {
            self.set_enabled(false);
        }
    }
}
