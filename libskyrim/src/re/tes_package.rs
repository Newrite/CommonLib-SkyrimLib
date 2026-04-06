use bitflags::bitflags;
use core::ffi::c_char;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESPackage;
use crate::offsets::offsets_vtable::VTABLE_TESPackage;
use crate::re::{
    Actor, BGSIdleCollection, BGSLoadFormBuffer, BGSSaveFormBuffer, FormCastable, FormType,
    ObjectRefHandle, PackageLocation, TESCombatStyle, TESCondition, TESFile, TESForm, TESIdleForm,
    TESPackageData, TESQuest, TESTopic,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageObjectType {
    // TODO: rename remaining all-caps spell-out variants like `ACTI`/`ARMO`
    // once we decide whether to prefer strict Rust case or preserve editor IDs.
    None = 0,
    ACTI = 1,
    ARMO = 2,
    BOOK = 3,
    CONT = 4,
    DOOR = 5,
    INGR = 6,
    LIGH = 7,
    MISC = 8,
    FLOR = 9,
    FURN = 10,
    WEAP = 11,
    AMMO = 12,
    KEYM = 13,
    ALCH = 14,
    FOOD = 15,
}

core_util::impl_enumset_type!(PackageObjectType => u32);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageType {
    None = -1,
    Explore = 0,
    Follow = 1,
    Escort = 2,
    Eat = 3,
    Sleep = 4,
    Wander = 5,
    Travel = 6,
    Accompany = 7,
    UseItemAt = 8,
    Ambush = 9,
    FleeNotCombat = 10,
    CastMagic = 11,
    Sandbox = 12,
    Patrol = 13,
    Guard = 14,
    Dialogue = 15,
    UseWeapon = 16,
    Find = 17,
    Package = 18,
    PackageTemplate = 19,
    Activate = 20,
    Alarm = 21,
    Flee = 22,
    Trespass = 23,
    Spectator = 24,
    ReactToDead = 25,
    GetUpFromChairBed = 26,
    DoNothing = 27,
    InGameDialogue = 28,
    Surface = 29,
    SearchForAttacker = 30,
    AvoidPlayer = 31,
    ReactToDestroyedObject = 32,
    ReactToGrenadeOrMine = 33,
    StealWarning = 34,
    PickPocketWarning = 35,
    MovementBlocked = 36,
    VampireFeed = 37,
    Cannibal = 38,
    Landing = 39,
    Unused = 40,
    MountActor = 41,
    DismountActor = 42,
    ClearMountPosition = 43,
    Total = 44,
}

core_util::impl_enumset_type!(PackageType => u8);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageProcedureType {
    None = -1,
    ExploreTravel = 0,
    ExploreWander = 1,
    ExploreActivate = 2,
    ExploreAcquire = 3,
    Sleep = 4,
    Eat = 5,
    FollowWithEscort = 6,
    AmbushFollow = 7,
    EscortActor = 8,
    EscortObject = 9,
    Dialogue = 10,
    Alarm = 11,
    Activate = 12,
    Greet = 13,
    ObserveCombat = 14,
    ObserveDialogue = 15,
    TalkToDead = 16,
    Flee = 17,
    Trespass = 18,
    GetUpFromChairBed = 19,
    ExploreNPC = 20,
    MountActor = 21,
    DismountActor = 22,
    DoNothing = 23,
    ExploreAcquireGeneric = 24,
    Accompany = 25,
    UseItemAt = 26,
    VampireFeed = 27,
    Ambush = 28,
    Surface = 29,
    FleeNotCombat = 30,
    SearchForAttacker = 31,
    ClearMountPosition = 32,
    WaitForDialogue = 33,
    AvoidPlayer = 34,
    Sandbox = 35,
    Patrol = 36,
    ReactToDestroyedObject = 37,
    ReactToGrenadeOrMine = 38,
    Guard = 39,
    StealWarning = 40,
    PickPocketWarning = 41,
    UseWeapon = 42,
    FollowWithoutEscort = 43,
    MovementBlocked = 44,
    Cannibal = 45,
    Package = 46,
    Landing = 47,
    KeepAnEyeOn = 48,
}

core_util::impl_enumset_type!(PackageProcedureType => u32);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackEventActionType {
    Begin = 0,
    End = 1,
    Change = 2,
    Patrol = 3,
}

core_util::impl_enumset_type!(PackEventActionType => u32);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackInterruptTarget {
    None = -1,
    Spectator = 0,
    ObserveDead = 1,
    GuardWarn = 2,
    Combat = 3,
}

core_util::impl_enumset_type!(PackInterruptTarget => u8);
core_util::impl_enumset_type!(PackInterruptTarget => u32);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageDataGeneralFlag {
    None = 0,
    OffersServices = 1 << 0,
    MustComplete = 1 << 2,
    MaintainSpeedAtGoal = 1 << 3,
    UnlocksDoorsAtPackageStart = 1 << 6,
    UnlocksDoorsAtPackageEnd = 1 << 7,
    ContinueIfPCNear = 1 << 9,
    OncePerDay = 1 << 10,
    Created = 1 << 11,
    PreferredSpeed = 1 << 13,
    AlwaysSneak = 1 << 17,
    AllowSwimming = 1 << 18,
    IgnoreCombat = 1 << 20,
    WeaponsUnequipped = 1 << 21,
    WeaponDrawn = 1 << 23,
    NoCombatAlert = 1 << 27,
    WearSleepOutfit = 1 << 29,
}

core_util::impl_enumset_type!(PackageDataGeneralFlag => u32);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageDataPreferredSpeed {
    Walk = 0,
    Jog = 1,
    Run = 2,
    FastWalk = 3,
}

core_util::impl_enumset_type!(PackageDataPreferredSpeed => u8);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageDataInterruptFlag {
    None = 0,
    HellosToPlayer = 1 << 0,
    RandomConversations = 1 << 1,
    ObserveCombatBehaviour = 1 << 2,
    GreetCorpseBehaviour = 1 << 3,
    ReactionToPlayerActions = 1 << 4,
    FriendlyFireComments = 1 << 5,
    AggroRadiusBehavior = 1 << 6,
    AllowIdleChatter = 1 << 7,
    WorldInteractions = 1 << 9,
}

core_util::impl_enumset_type!(PackageDataInterruptFlag => u16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PackageData {
    pub pack_flags: EnumSet<PackageDataGeneralFlag, u32>, // 00
    pub pack_type: EnumSet<PackageType, u8>,              // 04
    pub interrupt_override_type: EnumSet<PackInterruptTarget, u8>, // 05
    pub max_speed: EnumSet<PackageDataPreferredSpeed, u8>, // 06
    pub pad07: u8,                                        // 07
    pub fo_behavior_flags: EnumSet<PackageDataInterruptFlag, u16>, // 08
    pub package_specific_flags: u16,                      // 0A
}

const _: () = assert!(core::mem::size_of::<PackageData>() == 0x0C);

#[repr(C)]
#[derive(Clone, Copy)]
pub union PackageTargetTarget {
    pub handle: ObjectRefHandle,
    pub object: *mut TESForm,
    pub ref_or_obj: *mut TESForm,
    pub obj_type: EnumSet<PackageObjectType, u32>,
    pub alias_id: u32,
    pub interrupt_targ: EnumSet<PackInterruptTarget, u32>,
}

const _: () = assert!(core::mem::size_of::<PackageTargetTarget>() == 0x08);

#[repr(C)]
pub struct PackageTarget {
    pub targ_type: i8,               // 00
    pub pad01: u8,                   // 01
    pub pad02: u16,                  // 02
    pub target: PackageTargetTarget, // 08
    pub value: i32,                  // 10
    pub pad14: u32,                  // 14
}

const _: () = assert!(core::mem::size_of::<PackageTarget>() == 0x18);
const _: () = assert!(core::mem::offset_of!(PackageTarget, target) == 0x08);

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackSchedDataDayOfWeek {
    Any = -1,
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Weekdays = 7,
    Weekends = 8,
    MondayWednesdayFriday = 9,
    TuesdayThursday = 10,
}

core_util::impl_enumset_type!(PackSchedDataDayOfWeek => i8);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PackSchedData {
    pub month: i8,                                        // 00
    pub day_of_week: EnumSet<PackSchedDataDayOfWeek, i8>, // 01
    pub date: i8,                                         // 02
    pub hour: i8,                                         // 03
    pub minute: i8,                                       // 04
    pub pad05: u8,                                        // 05
    pub pad06: u8,                                        // 06
    pub pad07: u8,                                        // 07
    pub duration: i32,                                    // 08
}

const _: () = assert!(core::mem::size_of::<PackSchedData>() == 0x0C);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PackageSchedule {
    pub ps_data: PackSchedData, // 00
}

const _: () = assert!(core::mem::size_of::<PackageSchedule>() == 0x0C);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageEventActionTopicDataType {
    TopicRef = 0,
    TopicSubtype = 1,
}

core_util::impl_enumset_type!(PackageEventActionTopicDataType => u32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PackageEventActionTopicData {
    pub type_: EnumSet<PackageEventActionTopicDataType, u32>, // 00
    pub pad04: u32,                                           // 04
    pub topic: *mut TESTopic,                                 // 08
}

const _: () = assert!(core::mem::size_of::<PackageEventActionTopicData>() == 0x10);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PackageEventAction {
    pub idle: *mut TESIdleForm,                   // 00 - INAM
    pub type_: EnumSet<PackEventActionType, u32>, // 08
    pub pad0c: u32,                               // 0C
    pub topic: PackageEventActionTopicData,       // 10 - PDTO
}

const _: () = assert!(core::mem::size_of::<PackageEventAction>() == 0x20);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESPackageChangeFlags: u32 {
        const WAITING_FLAG = 1 << 26;
        const NEVER_RUN_FLAG = 1u32 << 31;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESPackageRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct TESPackage {
    pub base: TESForm,                                      // 00
    pub pack_data: PackageData,                             // 20 - PKDT
    pub pad2c: u32,                                         // 2C
    pub data: *mut TESPackageData,                          // 30
    pub pack_loc: *mut PackageLocation,                     // 38
    pub pack_targ: *mut PackageTarget,                      // 40
    pub idle_collection: *mut BGSIdleCollection,            // 48
    pub pack_sched: PackageSchedule,                        // 50 - PSDT
    pub pad5c: u32,                                         // 5C
    pub pack_conditions: TESCondition,                      // 60
    pub combat_style: *mut TESCombatStyle,                  // 68 - CNAM
    pub owner_quest: *mut TESQuest,                         // 70 - QNAM
    pub on_begin: PackageEventAction,                       // 78
    pub on_end: PackageEventAction,                         // 98
    pub on_change: PackageEventAction,                      // B8
    pub procedure_type: EnumSet<PackageProcedureType, u32>, // D8
    pub ref_count: u32,                                     // DC
}

const _: () = assert!(core::mem::size_of::<TESPackage>() == 0xE0);
const _: () = assert!(core::mem::offset_of!(TESPackage, pack_data) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESPackage, data) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESPackage, pack_loc) == 0x38);
const _: () = assert!(core::mem::offset_of!(TESPackage, pack_targ) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESPackage, idle_collection) == 0x48);
const _: () = assert!(core::mem::offset_of!(TESPackage, pack_sched) == 0x50);
const _: () = assert!(core::mem::offset_of!(TESPackage, pack_conditions) == 0x60);
const _: () = assert!(core::mem::offset_of!(TESPackage, combat_style) == 0x68);
const _: () = assert!(core::mem::offset_of!(TESPackage, owner_quest) == 0x70);
const _: () = assert!(core::mem::offset_of!(TESPackage, on_begin) == 0x78);
const _: () = assert!(core::mem::offset_of!(TESPackage, on_end) == 0x98);
const _: () = assert!(core::mem::offset_of!(TESPackage, on_change) == 0xB8);
const _: () = assert!(core::mem::offset_of!(TESPackage, procedure_type) == 0xD8);
const _: () = assert!(core::mem::offset_of!(TESPackage, ref_count) == 0xDC);

impl RttiType for TESPackage {
    const RTTI: VariantID = RTTI_TESPackage;
}

impl FormCastable for TESPackage {
    const TARGET_FORM_TYPE: FormType = FormType::Package;
}

inherit!(TESPackage : TESForm);

impl TESPackage {
    pub const RTTI: VariantID = RTTI_TESPackage;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESPackage;
    pub const FORMTYPE: FormType = FormType::Package;

    // override (TESForm)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_INITIALIZE_DATA: usize = 0x04;
        pub fn initialize_data(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_CLEAR_DATA: usize = 0x05;
        pub fn clear_data(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_file: *mut TESFile) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x0E;
        pub fn save_game(&mut self, buf: *mut BGSSaveFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x0F;
        pub fn load_game(&mut self, buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_REVERT: usize = 0x12;
        pub fn revert(&mut self, buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_ITEM_IMPL: usize = 0x13;
        pub fn init_item_impl(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_REF_COUNT: usize = 0x2D;
        pub fn get_ref_count() -> u32
    }

    crate::virtual_method! {
        pub const VFUNC_GET_OBJECT_TYPE_NAME: usize = 0x39;
        pub fn get_object_type_name() -> *const c_char
    }

    crate::virtual_method! {
        pub const VFUNC_IS_ACTOR_AT_LOCATION: usize = 0x3B;
        pub fn is_actor_at_location(
            &self,
            actor: *mut Actor,
            arg2: bool,
            arg3: f32,
            arg4: bool
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_IS_ACTOR_AT_SECOND_LOCATION: usize = 0x3C;
        pub fn is_actor_at_second_location(
            &self,
            arg1: *mut Actor,
            arg2: *mut Actor,
            arg3: bool,
            arg4: f32,
            arg5: bool
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_IS_ACTOR_AT_REF_TARGET: usize = 0x3D;
        pub fn is_actor_at_ref_target(&self, actor: *mut Actor, arg2: i32) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_IS_TARGET_AT_LOCATION: usize = 0x3E;
        pub fn is_target_at_location(&self, actor: *mut Actor, arg2: i32) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_IS_PACKAGE_OWNER: usize = 0x3F;
        pub fn is_package_owner(&self, actor: *mut Actor) -> bool
    }

    crate::relocation_func! {
        pub fn create_package(procedure_type: PackageProcedureType) -> *mut TESPackage => RelocationID::new(28732, 29496)
    }

    crate::relocation_func! {
        pub fn set_pack_type(&mut self, procedure_type: PackageProcedureType) => RelocationID::new(28751, 29525)
    }

    #[inline]
    pub fn get_object_type_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_object_type_name())
    }

    #[inline]
    pub fn ref_count_storage(&self) -> u32 {
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!(self.ref_count)) }
    }
}

pub trait TESPackageExt {
    fn dtor(&mut self);
    fn initialize_data(&mut self);
    fn clear_data(&mut self);
    fn load(&mut self, mod_file: *mut TESFile) -> bool;
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer);
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn revert(&mut self, buf: *mut BGSLoadFormBuffer);
    fn init_item_impl(&mut self);
    fn get_ref_count(&self) -> u32;
    fn get_object_type_name(&self) -> *const c_char;
    fn get_object_type_name_as_str(&self) -> &str;
    fn ref_count_storage(&self) -> u32;
    fn is_actor_at_location(&self, actor: *mut Actor, arg2: bool, arg3: f32, arg4: bool) -> bool;
    fn is_actor_at_second_location(
        &self,
        arg1: *mut Actor,
        arg2: *mut Actor,
        arg3: bool,
        arg4: f32,
        arg5: bool,
    ) -> bool;
    fn is_actor_at_ref_target(&self, actor: *mut Actor, arg2: i32) -> bool;
    fn is_target_at_location(&self, actor: *mut Actor, arg2: i32) -> bool;
    fn is_package_owner(&self, actor: *mut Actor) -> bool;
    fn set_pack_type(&mut self, procedure_type: PackageProcedureType);
}

impl<T: AsRef<TESPackage> + AsMut<TESPackage>> TESPackageExt for T {
    fn dtor(&mut self) {
        TESPackage::dtor(self.as_mut())
    }

    fn initialize_data(&mut self) {
        TESPackage::initialize_data(self.as_mut())
    }

    fn clear_data(&mut self) {
        TESPackage::clear_data(self.as_mut())
    }

    fn load(&mut self, mod_file: *mut TESFile) -> bool {
        TESPackage::load(self.as_mut(), mod_file)
    }

    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer) {
        TESPackage::save_game(self.as_mut(), buf)
    }

    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        TESPackage::load_game(self.as_mut(), buf)
    }

    fn revert(&mut self, buf: *mut BGSLoadFormBuffer) {
        TESPackage::revert(self.as_mut(), buf)
    }

    fn init_item_impl(&mut self) {
        TESPackage::init_item_impl(self.as_mut())
    }

    fn get_ref_count(&self) -> u32 {
        self.as_ref().get_ref_count()
    }

    fn get_object_type_name(&self) -> *const c_char {
        self.as_ref().get_object_type_name()
    }

    fn get_object_type_name_as_str(&self) -> &str {
        self.as_ref().get_object_type_name_as_str()
    }

    fn ref_count_storage(&self) -> u32 {
        self.as_ref().ref_count_storage()
    }

    fn is_actor_at_location(&self, actor: *mut Actor, arg2: bool, arg3: f32, arg4: bool) -> bool {
        self.as_ref().is_actor_at_location(actor, arg2, arg3, arg4)
    }

    fn is_actor_at_second_location(
        &self,
        arg1: *mut Actor,
        arg2: *mut Actor,
        arg3: bool,
        arg4: f32,
        arg5: bool,
    ) -> bool {
        self.as_ref()
            .is_actor_at_second_location(arg1, arg2, arg3, arg4, arg5)
    }

    fn is_actor_at_ref_target(&self, actor: *mut Actor, arg2: i32) -> bool {
        self.as_ref().is_actor_at_ref_target(actor, arg2)
    }

    fn is_target_at_location(&self, actor: *mut Actor, arg2: i32) -> bool {
        self.as_ref().is_target_at_location(actor, arg2)
    }

    fn is_package_owner(&self, actor: *mut Actor) -> bool {
        self.as_ref().is_package_owner(actor)
    }

    fn set_pack_type(&mut self, procedure_type: PackageProcedureType) {
        TESPackage::set_pack_type(self.as_mut(), procedure_type)
    }
}
