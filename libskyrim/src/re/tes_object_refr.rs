use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::ffi::{c_char, c_void};

use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESObjectREFR;
use crate::offsets::offsets_vtable::VTABLE_TESObjectREFR;
use crate::re::bgs_default_object_manager::BGSDefaultObjectManager;
use crate::re::item_remove_reason::ITEM_REMOVE_REASON;
use crate::re::magic_system::CastingSource;
use crate::re::tes_form::RecordFlag as TESFormRecordFlag;
use crate::re::{
    Actor, ActorCause, BGSAnimationSequencer, BGSArtObject, BGSDecalGroup, BGSDialogueBranch,
    BGSEncounterZone, BGSKeyword, BGSListForm, BGSLocation, BGSScene, BGSWorldLocation,
    BIPED_OBJECT, BSAnimationGraphEvent, BSAnimationUpdateData, BSContainerForEachResult,
    BSEventNotifyControl, BSFaceGenAnimationData, BSFaceGenNiNode, BSFixedString,
    BSHandleRefObject, BSTEventSink, BSTEventSource, BSTSmallArray, BSTSmartPointer, BipedAnim,
    DialogueResponse, DoorTeleportData, EnchantmentItem, Explosion, ExtraCharge,
    ExtraContainerChanges, ExtraDataList, ExtraDataType, ExtraDroppedItemList, ExtraEnchantment,
    ExtraFlags, ExtraFlagsFlag, ExtraOwnership, ExtraPersistentCell, ExtraTextDisplayData,
    FormCastable, FormType, IAnimationGraphManagerHolder, InventoryChanges, InventoryEntryData,
    LOCK_LEVEL, MagicCaster, MagicTarget, ModelReferenceEffect, NavMeshArray, NiAVObject,
    NiControllerManager, NiControllerSequence, NiNode, NiPoint3, NiPointer, NiRef, NiTransform,
    ObjectRefHandle, Projectile, REFR_LOCK, RefHandle, ShaderReferenceEffect, TESActorBase,
    TESAmmo, TESBoundObject, TESContainer, TESDataHandler, TESEffectShader, TESEnchantableForm,
    TESForm, TESNPC, TESObjectCELL, TESPackage, TESTopicInfo, TESWaterForm, TESWorldSpace,
    hkVector4, hkpCollidable, hkpMotionMotionType,
};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset, skyrim_cast};

crate::core_util::abstract_type! {
    pub type BSAnimNoteReceiver;
    pub type TargetEntry;
    pub type TrapData;
    pub type TrapEntry;
}

pub type TESObjectREFRCount = i32;
pub type TESObjectREFRInventoryCountMap = BTreeMap<*mut TESBoundObject, TESObjectREFRCount>;
pub type TESObjectREFRInventoryItemMap =
    BTreeMap<*mut TESBoundObject, (TESObjectREFRCount, Box<InventoryEntryData>)>;
pub type TESObjectREFRInventoryDropMap =
    BTreeMap<*mut TESBoundObject, (TESObjectREFRCount, Vec<ObjectRefHandle>)>;

const DEFAULT_OBJECT_OBJECTS_OFFSET: VariantOffset = VariantOffset::new(0x20, 0x20, 0x20);
const DEFAULT_OBJECT_OBJECT_INIT_OFFSET: VariantOffset = VariantOffset::new(0xB80, 0xBA8, 0xB80);
const DEFAULT_OBJECT_TOTAL: VariantOffset = VariantOffset::new(364, 364, 369);
const DEFAULT_OBJECT_KEYWORD_ACTIVATOR_FURNITURE_NO_PLAYER: u32 = 182;
const DEFAULT_OBJECT_ID_KEYWORD_HORSE: u32 = 155;
const DEFAULT_OBJECT_ID_KEYWORD_NPC: u32 = 157;
const DEFAULT_OBJECT_ID_KEYWORD_DRAGON: u32 = 201 | (207 << 16);
const DEFAULT_OBJECT_ID_KEYWORD_ANIMAL: u32 = 217 | (223 << 16);
const DEFAULT_OBJECT_ID_KEYWORD_JEWELRY: u32 = 311 | (328 << 16);

#[inline(always)]
fn tes_object_cell_world_space(cell: *mut TESObjectCELL) -> *mut TESWorldSpace {
    if cell.is_null() {
        return core::ptr::null_mut();
    }

    unsafe { (&*cell).world_space_raw() }
}

#[inline(always)]
fn tes_object_cell_nav_meshes(cell: *mut TESObjectCELL) -> *mut NavMeshArray {
    if cell.is_null() {
        return core::ptr::null_mut();
    }

    unsafe { (&*cell).nav_meshes_raw() }
}

#[inline(always)]
fn default_object_manager_map_index(keyword_type: u32) -> Option<usize> {
    if keyword_type <= DEFAULT_OBJECT_KEYWORD_ACTIVATOR_FURNITURE_NO_PLAYER {
        return Some(keyword_type as usize);
    }

    let mapped = if crate::runtime::is_vr() {
        (keyword_type & 0xFFFF_0000) >> 16
    } else {
        keyword_type & 0x0000_FFFF
    };
    (mapped != 0).then_some(mapped as usize)
}

#[inline(always)]
fn default_object_manager_get_singleton() -> *mut BGSDefaultObjectManager {
    type Func = unsafe extern "C" fn() -> *mut BGSDefaultObjectManager;
    let func: Func = unsafe { core::mem::transmute(RelocationID::new(10878, 13894).address()) };
    unsafe { func() }
}

fn tes_object_refr_default_keyword(keyword_type: u32) -> *mut BGSKeyword {
    let singleton = default_object_manager_get_singleton();
    if singleton.is_null() {
        return core::ptr::null_mut();
    }

    let Some(idx) = default_object_manager_map_index(keyword_type) else {
        return core::ptr::null_mut();
    };

    debug_assert!(idx < DEFAULT_OBJECT_TOTAL.offset());
    if idx >= DEFAULT_OBJECT_TOTAL.offset() {
        return core::ptr::null_mut();
    }

    let object_init = unsafe {
        (singleton as *const u8)
            .add(DEFAULT_OBJECT_OBJECT_INIT_OFFSET.offset())
            .cast::<bool>()
    };
    if !unsafe { *object_init.add(idx) } {
        return core::ptr::null_mut();
    }

    let objects = unsafe {
        (singleton as *mut u8)
            .add(DEFAULT_OBJECT_OBJECTS_OFFSET.offset())
            .cast::<*mut TESForm>()
    };
    let keyword = unsafe { *objects.add(idx) };
    if keyword.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { skyrim_cast::<TESForm, BGSKeyword>(keyword) }
    }
}

/// C++ `RE::OBJ_REFR`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OBJ_REFR {
    pub object_reference: *mut TESBoundObject, // 00
    pub angle: NiPoint3,                       // 08
    pub location: NiPoint3,                    // 14
}

const _: () = assert!(core::mem::size_of::<OBJ_REFR>() == 0x20);
const _: () = assert!(core::mem::offset_of!(OBJ_REFR, object_reference) == 0x00);
const _: () = assert!(core::mem::offset_of!(OBJ_REFR, angle) == 0x08);
const _: () = assert!(core::mem::offset_of!(OBJ_REFR, location) == 0x14);

/// C++ `RE::LOADED_REF_DATA`
#[repr(C)]
pub struct LOADED_REF_DATA {
    pub unk00: BSTSmallArray<*mut c_void, { core::mem::size_of::<*mut c_void>() }>, // 00
    pub current_water_type: *mut TESWaterForm,                                      // 18
    pub relevant_water_height: f32,                                                 // 20
    pub cached_radius: f32,                                                         // 24
    pub flags: u16,                                                                 // 28
    pub underwater_count: i16,                                                      // 2A
    pub pad2c: u32,                                                                 // 2C
    pub unk30: u64,                                                                 // 30
    pub unk38: u64,                                                                 // 38
    pub unk40: u64,                                                                 // 40
    pub unk48: u64,                                                                 // 48
    pub unk50: u64,                                                                 // 50
    pub unk58: u64,                                                                 // 58
    pub unk60: u64,                                                                 // 60
    pub data_3d: NiPointer<NiAVObject>,                                             // 68
    pub unk70: *mut c_void,                                                         // 70
}

const _: () = assert!(core::mem::size_of::<LOADED_REF_DATA>() == 0x78);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, unk00) == 0x00);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, current_water_type) == 0x18);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, relevant_water_height) == 0x20);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, cached_radius) == 0x24);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, flags) == 0x28);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, underwater_count) == 0x2A);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, unk30) == 0x30);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, unk38) == 0x38);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, unk40) == 0x40);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, unk48) == 0x48);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, unk50) == 0x50);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, unk58) == 0x58);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, unk60) == 0x60);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, data_3d) == 0x68);
const _: () = assert!(core::mem::offset_of!(LOADED_REF_DATA, unk70) == 0x70);

bitflags! {
    /// C++ `RE::TESObjectREFR::ChangeFlags::ChangeFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESObjectREFRChangeFlags: u32 {
        const MOVED = 1 << 1;
        const HAVOK_MOVED = 1 << 2;
        const CELL_CHANGED = 1 << 3;
        const SCALE = 1 << 4;
        const INVENTORY = 1 << 5;
        const OWNERSHIP_EXTRA = 1 << 6;
        const BASE_OBJECT = 1 << 7;
        const ITEM_EXTRA_DATA = 1 << 10;
        const AMMO_EXTRA = 1 << 11;
        const LOCK_EXTRA = 1 << 12;
        const TELEPORT_EXTRA = 1 << 17;
        const EMPTY = 1 << 21;
        const OPEN_DEFAULT_STATE = 1 << 22;
        const OPEN_STATE = 1 << 23;
        const PROMOTED = 1 << 25;
        const ACTIVATING_CHILDREN = 1 << 26;
        const LEVELED_INVENTORY = 1 << 27;
        const ANIMATION = 1 << 28;
        const ENC_ZONE_EXTRA = 1 << 29;
        const CREATED_ONLY_EXTRA = 1 << 30;
        const GAME_ONLY_EXTRA = 1u32 << 31;
    }
}

bitflags! {
    /// C++ `RE::TESObjectREFR::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESObjectREFRRecordFlags: u32 {
        const IS_GROUND_PIECE = 1 << 4;
        const COLLISIONS_DISABLED = 1 << 4;
        const DELETED = 1 << 5;
        const HIDDEN_FROM_LOCAL_MAP = 1 << 6;
        const TURN_OFF_FIRE = 1 << 7;
        const INACCESSIBLE = 1 << 8;
        const LOD_RESPECTS_ENABLE_STATE = 1 << 8;
        const STARTS_DEAD = 1 << 8;
        const DOESNT_LIGHT_WATER = 1 << 8;
        const MOTION_BLUR = 1 << 9;
        const PERSISTENT = 1 << 10;
        const INITIALLY_DISABLED = 1 << 11;
        const IGNORED = 1 << 12;
        const START_UNCONSCIOUS = 1 << 13;
        const SKY_MARKER = 1 << 13;
        const HARVESTED = 1 << 13;
        const IS_FULL_LOD = 1 << 16;
        const NEVER_FADES = 1 << 16;
        const DOESNT_LIGHT_LANDSCAPE = 1 << 17;
        const TEMP_3D = 1 << 19;
        const IGNORE_FRIENDLY_HITS = 1 << 20;
        const NO_AI_ACQUIRE = 1 << 25;
        const COLLISION_GEOMETRY_FILTER = 1 << 26;
        const COLLISION_GEOMETRY_BOUNDING_BOX = 1 << 27;
        const REFLECTED_BY_AUTO_WATER = 1 << 28;
        const DONT_HAVOK_SETTLE = 1 << 29;
        const GROUND = 1 << 30;
        const RESPAWNS = 1 << 30;
        const MULTIBOUND = 1u32 << 31;
    }
}

/// C++ `RE::TESObjectREFR::REFERENCE_RUNTIME_DATA`
#[repr(C)]
pub struct REFERENCE_RUNTIME_DATA {
    pub unk88: u64,          // 00
    pub ref_scale: u16,      // 08
    pub model_state: i8,     // 0A
    pub pre_destroyed: bool, // 0B
    pub pad94: u32,          // 0C
}

const _: () = assert!(core::mem::size_of::<REFERENCE_RUNTIME_DATA>() == 0x10);
const _: () = assert!(core::mem::offset_of!(REFERENCE_RUNTIME_DATA, unk88) == 0x00);
const _: () = assert!(core::mem::offset_of!(REFERENCE_RUNTIME_DATA, ref_scale) == 0x08);
const _: () = assert!(core::mem::offset_of!(REFERENCE_RUNTIME_DATA, model_state) == 0x0A);
const _: () = assert!(core::mem::offset_of!(REFERENCE_RUNTIME_DATA, pre_destroyed) == 0x0B);
const _: () = assert!(core::mem::offset_of!(REFERENCE_RUNTIME_DATA, pad94) == 0x0C);

/// Cross-runtime honest prefix of C++ `RE::TESObjectREFR`.
#[repr(C)]
pub struct TESObjectREFR {
    pub base: TESForm,                                                // 00
    pub ref_object: BSHandleRefObject,                                // 20
    pub event_sink: BSTEventSink<BSAnimationGraphEvent>,              // 30
    pub animation_graph_manager_holder: IAnimationGraphManagerHolder, // 38
    pub data: OBJ_REFR,                                               // 40
    pub parent_cell: *mut TESObjectCELL,                              // 60
    pub loaded_data: *mut LOADED_REF_DATA,                            // 68
    pub extra_list: ExtraDataList,                                    // 70
}

const _: () = assert!(core::mem::size_of::<TESObjectREFR>() == 0x80);
const _: () = assert!(core::mem::offset_of!(TESObjectREFR, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESObjectREFR, ref_object) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESObjectREFR, event_sink) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESObjectREFR, animation_graph_manager_holder) == 0x38);
const _: () = assert!(core::mem::offset_of!(TESObjectREFR, data) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESObjectREFR, parent_cell) == 0x60);
const _: () = assert!(core::mem::offset_of!(TESObjectREFR, loaded_data) == 0x68);
const _: () = assert!(core::mem::offset_of!(TESObjectREFR, extra_list) == 0x70);

impl RttiType for TESObjectREFR {
    const RTTI: VariantID = RTTI_TESObjectREFR;
}

impl FormCastable for TESObjectREFR {
    const TARGET_FORM_TYPE: FormType = FormType::Reference;
}

impl AsRef<TESObjectREFR> for TESObjectREFR {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TESObjectREFR> for TESObjectREFR {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl NiRef for TESObjectREFR {
    #[inline(always)]
    fn inc_ref(&self) {
        self.ref_object.inc_ref_count();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.ref_object.dec_ref_count();
    }
}

inherit!(TESObjectREFR : TESForm);
inherit!(TESObjectREFR => BSHandleRefObject, ref_object);
inherit!(TESObjectREFR => BSTEventSink<BSAnimationGraphEvent>, event_sink);
inherit!(TESObjectREFR => IAnimationGraphManagerHolder, animation_graph_manager_holder);

impl TESObjectREFR {
    pub const RTTI: VariantID = RTTI_TESObjectREFR;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectREFR;
    pub const FORMTYPE: FormType = FormType::Reference;
    pub const REFERENCE_RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x88, 0x90, 0x88);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x98, 0xA0, 0x98);

    crate::runtime_data_accessor! {
        fn reference_runtime_data() -> REFERENCE_RUNTIME_DATA {
            se_and_vr: 0x88,
            ae: 0x90
        }
    }

    crate::runtime_data_mut_accessor! {
        fn reference_runtime_data_mut() -> REFERENCE_RUNTIME_DATA {
            se_and_vr: 0x88,
            ae: 0x90
        }
    }

    crate::virtual_method! {
        pub const VFUNC_PREDESTROY: usize = 0x3B;
        pub fn predestroy(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_EDITOR_LOCATION1: usize = 0x3C;
        pub fn get_editor_location1(&self) -> *mut BGSLocation
    }

    crate::virtual_method! {
        pub const VFUNC_GET_EDITOR_LOCATION2: usize = 0x3D;
        pub fn get_editor_location2(
            &mut self,
            out_pos: &mut NiPoint3,
            out_rot: &mut NiPoint3,
            out_world_or_cell: &mut *mut TESForm,
            fallback: *mut TESObjectCELL
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_FORCE_EDITOR_LOCATION: usize = 0x3E;
        pub fn force_editor_location(&mut self, location: *mut BGSLocation)
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE_3D_POSITION: usize = 0x3F;
        pub fn update_3d_position(&mut self, warp: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE_SOUND_CALLBACK: usize = 0x40;
        pub fn update_sound_callback(&mut self, end_scene_action: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_DIALOGUE_WITH_PLAYER: usize = 0x41;
        pub fn set_dialogue_with_player(
            &mut self,
            flag: bool,
            force_greet: bool,
            topic: *mut TESTopicInfo
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_DAMAGE_OBJECT: usize = 0x42;
        pub fn damage_object(&mut self, object_health: f32, arg3: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FULL_LOD_REF: usize = 0x43;
        pub fn get_full_lod_ref(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SET_FULL_LOD_REF: usize = 0x44;
        pub fn set_full_lod_ref(&mut self, set: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_SEQUENCER: usize = 0x45;
        pub fn get_sequencer(&self) -> *mut BGSAnimationSequencer
    }

    crate::virtual_method! {
        pub const VFUNC_Q_CAN_UPDATE_SYNC: usize = 0x46;
        pub fn q_can_update_sync(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ALLOW_PROMOTE_TO_PERSISTENT: usize = 0x47;
        pub fn get_allow_promote_to_persistent(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_HAS_KEYWORD_HELPER: usize = 0x48;
        pub fn has_keyword_helper(&self, keyword: *const BGSKeyword) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_CHECK_FOR_CURRENT_ALIAS_PACKAGE: usize = 0x49;
        pub fn check_for_current_alias_package(&mut self) -> *mut TESPackage
    }

    crate::virtual_method! {
        pub const VFUNC_GET_CURRENT_SCENE: usize = 0x4A;
        pub fn get_current_scene(&self) -> *mut BGSScene
    }

    crate::virtual_method! {
        pub const VFUNC_SET_CURRENT_SCENE: usize = 0x4B;
        pub fn set_current_scene(&mut self, scene: *mut BGSScene)
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE_IN_DIALOGUE: usize = 0x4C;
        pub fn update_in_dialogue(&mut self, response: *mut DialogueResponse, unused: bool) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_EXCLUSIVE_BRANCH: usize = 0x4D;
        pub fn get_exclusive_branch(&self) -> *mut BGSDialogueBranch
    }

    crate::virtual_method! {
        pub const VFUNC_SET_EXCLUSIVE_BRANCH: usize = 0x4E;
        pub fn set_exclusive_branch(&mut self, branch: *mut BGSDialogueBranch)
    }

    crate::virtual_method! {
        pub const VFUNC_STOP_CURRENT_DIALOGUE: usize = 0x4F;
        pub fn stop_current_dialogue(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_ACTOR_CAUSE: usize = 0x50;
        pub fn set_actor_cause(&mut self, cause: *mut ActorCause)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ACTOR_CAUSE: usize = 0x51;
        pub fn get_actor_cause(&self) -> *mut ActorCause
    }

    crate::virtual_method! {
        pub const VFUNC_GET_STARTING_ANGLE: usize = 0x52;
        pub fn get_starting_angle(&self) -> NiPoint3
    }

    crate::virtual_method! {
        pub const VFUNC_GET_STARTING_LOCATION: usize = 0x53;
        pub fn get_starting_location(&self) -> NiPoint3
    }

    crate::virtual_method! {
        pub const VFUNC_SET_STARTING_POSITION: usize = 0x54;
        pub fn set_starting_position(&mut self, pos: &NiPoint3)
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE_REF_LIGHT: usize = 0x55;
        pub fn update_ref_light(&mut self)
    }

    pub const VFUNC_REMOVE_ITEM: usize = 0x56;

    pub fn remove_item(
        &mut self,
        item: *mut TESBoundObject,
        count: i32,
        reason: ITEM_REMOVE_REASON,
        extra_list: *mut ExtraDataList,
        move_to_ref: *mut TESObjectREFR,
        drop_loc: *const NiPoint3,
        rotate: *const NiPoint3,
    ) -> ObjectRefHandle {
        let mut out = ObjectRefHandle::new();
        unsafe {
            crate::ffi::commonlib_tes_object_refr_remove_item(
                self as *mut Self as *mut c_void,
                &mut out as *mut _ as *mut c_void,
                item as *mut c_void,
                count,
                reason as i32,
                extra_list as *mut c_void,
                move_to_ref as *mut c_void,
                drop_loc as *const c_void,
                rotate as *const c_void,
            );
        }
        out
    }

    crate::virtual_method! {
        pub const VFUNC_ADD_WORN_ITEM: usize = 0x57;
        pub fn add_worn_item(
            &mut self,
            item: *mut TESBoundObject,
            count: i32,
            force_equip: bool,
            arg4: u32,
            arg5: u32
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_DO_TRAP1: usize = 0x58;
        pub fn do_trap1(&mut self, data: &mut TrapData)
    }

    crate::virtual_method! {
        pub const VFUNC_DO_TRAP2: usize = 0x59;
        pub fn do_trap2(&mut self, trap: *mut TrapEntry, target: *mut TargetEntry)
    }

    crate::virtual_method! {
        pub const VFUNC_ADD_OBJECT_TO_CONTAINER: usize = 0x5A;
        pub fn add_object_to_container(
            &mut self,
            object: *mut TESBoundObject,
            extra_list: *mut ExtraDataList,
            count: i32,
            from_refr: *mut TESObjectREFR
        )
    }

    crate::virtual_method! {
        pub const VFUNC_GET_LOOKING_AT_LOCATION: usize = 0x5B;
        pub fn get_looking_at_location(&self) -> NiPoint3
    }

    crate::virtual_method! {
        pub const VFUNC_GET_MAGIC_CASTER: usize = 0x5C;
        pub fn get_magic_caster(&mut self, source: CastingSource) -> *mut MagicCaster
    }

    crate::virtual_method! {
        pub const VFUNC_GET_MAGIC_TARGET: usize = 0x5D;
        pub fn get_magic_target(&mut self) -> *mut MagicTarget
    }

    crate::virtual_method! {
        pub const VFUNC_IS_CHILD: usize = 0x5E;
        pub fn is_child(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TEMPLATE_ACTOR_BASE: usize = 0x5F;
        pub fn get_template_actor_base(&mut self) -> *mut TESActorBase
    }

    crate::virtual_method! {
        pub const VFUNC_SET_TEMPLATE_ACTOR_BASE: usize = 0x60;
        pub fn set_template_actor_base(&mut self, template: *mut TESActorBase)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FACE_NODE_SKINNED: usize = 0x61;
        pub fn get_face_node_skinned(&mut self) -> *mut BSFaceGenNiNode
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FACE_NODE: usize = 0x62;
        pub fn get_face_node(&mut self) -> *mut BSFaceGenNiNode
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FACE_GEN_ANIMATION_DATA: usize = 0x63;
        pub fn get_face_gen_animation_data(&mut self) -> *mut BSFaceGenAnimationData
    }

    crate::virtual_method! {
        pub const VFUNC_CLAMP_TO_GROUND: usize = 0x64;
        pub fn clamp_to_ground(&mut self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_DETACH_HAVOK: usize = 0x65;
        pub fn detach_havok(&mut self, obj_3d: *mut NiAVObject) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_HAVOK: usize = 0x66;
        pub fn init_havok(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_67: usize = 0x67;
        pub fn unk_67(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_68: usize = 0x68;
        pub fn unk_68(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_69: usize = 0x69;
        pub fn unk_69(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_3D: usize = 0x6A;
        pub fn load_3d(&mut self, background_loading: bool) -> *mut NiAVObject
    }

    crate::virtual_method! {
        pub const VFUNC_RELEASE_3D_RELATED_DATA: usize = 0x6B;
        pub fn release_3d_related_data(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_3D: usize = 0x6C;
        pub fn set_3d(&mut self, object: *mut NiAVObject, queue_3d_tasks: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_SHOULD_BACKGROUND_CLONE: usize = 0x6D;
        pub fn should_background_clone(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_6E: usize = 0x6E;
        pub fn unk_6e(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_3D1: usize = 0x6F;
        pub fn get_3d1(&self, first_person: bool) -> *mut NiAVObject
    }

    crate::virtual_method! {
        pub const VFUNC_GET_3D2: usize = 0x70;
        pub fn get_3d2(&self) -> *mut NiAVObject
    }

    crate::virtual_method! {
        pub const VFUNC_IS_3RD_PERSON_VISIBLE: usize = 0x71;
        pub fn is_3rd_person_visible(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_POPULATE_GRAPH_PROJECTS_TO_LOAD: usize = 0x72;
        pub fn populate_graph_projects_to_load(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_BOUND_MIN: usize = 0x73;
        pub fn get_bound_min(&self) -> NiPoint3
    }

    crate::virtual_method! {
        pub const VFUNC_GET_BOUND_MAX: usize = 0x74;
        pub fn get_bound_max(&self) -> NiPoint3
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_75: usize = 0x75;
        pub fn unk_75(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_NON_NPC_ANIMATION: usize = 0x76;
        pub fn init_non_npc_animation(&mut self, node_for_anim: &mut NiNode) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_CHECK_AND_FIX_SKIN_AND_BONE_ORDER: usize = 0x77;
        pub fn check_and_fix_skin_and_bone_order(&mut self, node_to_test: &mut NiNode) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_78: usize = 0x78;
        pub fn unk_78(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_MODIFY_ANIMATION_UPDATE_DATA: usize = 0x79;
        pub fn modify_animation_update_data(&mut self, data: &mut BSAnimationUpdateData)
    }

    crate::virtual_method! {
        pub const VFUNC_SHOULD_SAVE_ANIMATION_ON_UNLOADING: usize = 0x7A;
        pub fn should_save_animation_on_unloading(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SHOULD_SAVE_ANIMATION_ON_SAVING: usize = 0x7B;
        pub fn should_save_animation_on_saving(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SHOULD_PERFORM_REVERT: usize = 0x7C;
        pub fn should_perform_revert(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE_ANIMATION: usize = 0x7D;
        pub fn update_animation(&mut self, delta: f32)
    }

    #[inline(always)]
    unsafe fn get_biped1_raw(&self, first_person: bool) -> *const BSTSmartPointer<BipedAnim> {
        let func: extern "C" fn(*const Self, bool) -> *const BSTSmartPointer<BipedAnim> =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0x7E) };
        func(self, first_person)
    }

    #[inline(always)]
    unsafe fn get_biped2_raw(&self) -> *const BSTSmartPointer<BipedAnim> {
        let func: extern "C" fn(*const Self) -> *const BSTSmartPointer<BipedAnim> =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0x7F) };
        func(self)
    }

    #[inline(always)]
    unsafe fn get_current_biped_raw(&self) -> *const BSTSmartPointer<BipedAnim> {
        let func: extern "C" fn(*const Self) -> *const BSTSmartPointer<BipedAnim> =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0x80) };
        func(self)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_BIPED: usize = 0x81;
        pub fn set_biped(&mut self, biped: &BSTSmartPointer<BipedAnim>)
    }
}

impl TESObjectREFR {
    #[inline(always)]
    pub fn create_reference(
        handle_out: &mut ObjectRefHandle,
        form_type: FormType,
        add_actor_to_process_list: bool,
    ) -> ObjectRefHandle {
        let mut out = ObjectRefHandle::new();
        Self::create_reference_impl(&mut out, handle_out, form_type, add_actor_to_process_list);
        out
    }

    crate::relocation_func! {
        fn create_reference_impl(
            out: &mut ObjectRefHandle,
            handle_out: &mut ObjectRefHandle,
            form_type: FormType,
            add_actor_to_process_list: bool
        ) => RelocationID::new(19142, 19544)
    }

    crate::relocation_func! {
        pub fn find_reference_for_3d(object_3d: *mut NiAVObject) -> *mut TESObjectREFR
            => RelocationID::new(19323, 19750)
    }

    crate::relocation_func! {
        pub fn activate_ref(
            &self,
            activator: *mut TESObjectREFR,
            arg2: u8,
            object: *mut TESBoundObject,
            count: i32,
            default_processing_only: bool
        ) -> bool => RelocationID::new(19369, 19796)
    }

    crate::relocation_func! {
        pub fn add_lock(&self) -> *mut REFR_LOCK => RelocationID::new(19816, 20221)
    }

    crate::relocation_func! {
        pub fn add_teleport(&self) -> *mut DoorTeleportData => RelocationID::new(19809, 20214)
    }

    crate::relocation_func! {
        pub fn apply_art_object(
            &self,
            art_object: *mut BGSArtObject,
            duration: f32,
            facing_ref: *mut TESObjectREFR,
            face_target: bool,
            attach_to_camera: bool,
            attach_node: *mut NiAVObject,
            interface_effect: bool
        ) -> *mut ModelReferenceEffect => RelocationID::new(22289, 22769)
    }

    crate::relocation_func! {
        pub fn apply_effect_shader(
            &self,
            effect_shader: *mut TESEffectShader,
            duration: f32,
            facing_ref: *mut TESObjectREFR,
            face_target: bool,
            attach_to_camera: bool,
            attach_node: *mut NiAVObject,
            interface_effect: bool
        ) -> *mut ShaderReferenceEffect => RelocationID::new(19446, 19872)
    }

    crate::relocation_func! {
        pub fn can_be_moved(&self) -> bool => RelocationID::new(19244, 19670)
    }

    crate::relocation_func! {
        pub fn clear_destruction(&mut self) => RelocationID::new(14082, 14181)
    }

    crate::relocation_func! {
        pub fn enable(&mut self, reset_inventory: bool) => RelocationID::new(19373, 19800)
    }

    crate::relocation_func! {
        pub fn get_calc_level(&self, adjust_level: bool) -> u16 => RelocationID::new(19800, 20205)
    }

    crate::relocation_func! {
        pub fn get_current_location(&self) -> *mut BGSLocation => RelocationID::new(19385, 19812)
    }

    crate::relocation_func! {
        pub fn get_display_full_name(&self) -> *const c_char => RelocationID::new(19354, 19781)
    }

    #[inline(always)]
    pub fn get_display_full_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_display_full_name())
    }

    crate::relocation_func! {
        pub fn get_distance(
            &self,
            other: *mut TESObjectREFR,
            disabled_refs: bool,
            ignore_worldspace: bool
        ) -> f32 => RelocationID::new(19396, 19823)
    }

    crate::relocation_func! {
        pub fn get_encounter_zone(&self) -> *mut BGSEncounterZone => RelocationID::new(19797, 20202)
    }

    crate::relocation_func! {
        pub fn get_inventory_item_at(
            &self,
            index: i32,
            is_viewing_container: bool
        ) -> *mut InventoryEntryData => RelocationID::new(19273, 19699)
    }

    crate::relocation_func! {
        pub fn get_inventory_item_count(
            &self,
            is_viewing_container: bool,
            playable: bool
        ) -> i32 => RelocationID::new(19274, 19700)
    }

    crate::relocation_func! {
        pub fn get_lock(&self) -> *mut REFR_LOCK => RelocationID::new(19818, 20223)
    }

    crate::relocation_func! {
        pub fn get_owner(&self) -> *mut TESForm => RelocationID::new(19789, 20194)
    }

    crate::relocation_func! {
        pub fn get_scale(&self) -> f32 => RelocationID::new(19238, 19664)
    }

    crate::relocation_func! {
        pub fn get_steal_value(
            &self,
            entry_data: *const InventoryEntryData,
            num_items: u32,
            use_mult: bool
        ) -> u32 => RelocationID::new(15807, 16045)
    }

    crate::relocation_func! {
        pub fn get_transform(&self, transform: &mut NiTransform) => RelocationID::new(19326, 19753)
    }

    crate::relocation_func! {
        pub fn get_weight_in_container(&self) -> f32 => RelocationID::new(19277, 19703)
    }

    crate::relocation_func! {
        pub fn has_quest_object(&self) -> bool => RelocationID::new(19201, 19627)
    }

    crate::relocation_func! {
        pub fn init_child_activates(&mut self, action_ref: *mut TESObjectREFR)
            => RelocationID::new(19857, 20264)
    }

    crate::relocation_func! {
        pub fn init_inventory_if_required(&mut self, ignore_container_extra_data: bool) -> bool
            => RelocationID::new(15800, 16038)
    }

    crate::relocation_func! {
        pub fn is_an_owner(
            &self,
            test_owner: *const Actor,
            use_faction: bool,
            requires_owner: bool
        ) -> bool => RelocationID::new(19805, 20210)
    }

    crate::relocation_func! {
        pub fn is_crime_to_activate(&self) -> bool => RelocationID::new(19400, 19827)
    }

    crate::relocation_func! {
        pub fn set_angle(&mut self, angle: &NiPoint3) => RelocationID::new(19359, 19786)
    }

    crate::relocation_func! {
        pub fn set_owner(&mut self, owner: *mut TESForm) => RelocationID::new(19793, 20198)
    }

    crate::relocation_func! {
        pub fn set_position(&mut self, pos: &NiPoint3) => RelocationID::new(19363, 19790)
    }

    crate::relocation_func! {
        pub fn set_scale(&mut self, scale: f32) => RelocationID::new(19239, 19665)
    }

    crate::relocation_func! {
        pub fn set_temporary(&mut self) => RelocationID::new(14485, 14642)
    }

    crate::relocation_func! {
        fn make_inventory_changes(&mut self) -> *mut InventoryChanges => RelocationID::new(15802, 16040)
    }

    crate::relocation_func! {
        fn move_to_impl(
            &mut self,
            target_handle: &ObjectRefHandle,
            target_cell: *mut TESObjectCELL,
            self_world_space: *mut TESWorldSpace,
            position: &NiPoint3,
            rotation: &NiPoint3
        ) => RelocationID::new(56227, 56626)
    }

    crate::relocation_func! {
        fn play_animation_impl(
            &mut self,
            manager: *mut NiControllerManager,
            to_seq: *mut NiControllerSequence,
            from_seq: *mut NiControllerSequence,
            arg4: bool
        ) => RelocationID::new(14189, 14297)
    }

    #[inline(always)]
    pub fn get_reference_runtime_data(&self) -> &REFERENCE_RUNTIME_DATA {
        crate::runtime_assert_size!(REFERENCE_RUNTIME_DATA, se: 0x10, ae: 0x10, vr: 0x10);
        crate::runtime_assert_offset!(REFERENCE_RUNTIME_DATA, ref_scale, se: 0x08, ae: 0x08, vr: 0x08);
        crate::runtime_assert_offset!(REFERENCE_RUNTIME_DATA, model_state, se: 0x0A, ae: 0x0A, vr: 0x0A);
        self.reference_runtime_data()
    }

    #[inline(always)]
    pub fn get_reference_runtime_data_mut(&mut self) -> &mut REFERENCE_RUNTIME_DATA {
        crate::runtime_assert_size!(REFERENCE_RUNTIME_DATA, se: 0x10, ae: 0x10, vr: 0x10);
        self.reference_runtime_data_mut()
    }

    #[inline(always)]
    pub fn default_inventory_filter(_object: &TESBoundObject) -> bool {
        true
    }

    #[inline(always)]
    pub fn lookup_by_handle(ref_handle: RefHandle) -> NiPointer<TESObjectREFR> {
        ObjectRefHandle { handle: ref_handle }.get()
    }

    #[inline(always)]
    pub fn lookup_by_handle_into(
        ref_handle: RefHandle,
        refr_out: &mut NiPointer<TESObjectREFR>,
    ) -> bool {
        *refr_out = Self::lookup_by_handle(ref_handle);
        !refr_out.get().is_null()
    }

    #[inline(always)]
    pub fn create_ref_handle(&self) -> ObjectRefHandle {
        self.get_handle()
    }

    #[inline(always)]
    pub fn process_animation_graph_event(
        &mut self,
        event: *const BSAnimationGraphEvent,
        event_source: *mut BSTEventSource<BSAnimationGraphEvent>,
    ) -> BSEventNotifyControl {
        unsafe { self.event_sink.process_event(event, event_source) }
    }
}

impl TESObjectREFR {
    #[inline(always)]
    pub fn do_trap_data(&mut self, data: &mut TrapData) {
        self.do_trap1(data);
    }

    #[inline(always)]
    pub fn do_trap_entry(&mut self, trap: *mut TrapEntry, target: *mut TargetEntry) {
        self.do_trap2(trap, target);
    }

    #[inline(always)]
    pub fn get_3d(&self) -> *mut NiAVObject {
        self.get_3d2()
    }

    #[inline(always)]
    pub fn get_3d_with_view(&self, first_person: bool) -> *mut NiAVObject {
        self.get_3d1(first_person)
    }

    #[inline(always)]
    pub fn get_actor_owner(&self) -> *mut TESNPC {
        let x_ownership = self.extra_list.get_by_type_typed::<ExtraOwnership>();
        if x_ownership.is_null() {
            return core::ptr::null_mut();
        }

        let owner = unsafe { (*x_ownership).owner };
        if owner.is_null() || unsafe { !(*owner).is(FormType::NPC) } {
            core::ptr::null_mut()
        } else {
            unsafe { skyrim_cast::<TESForm, TESNPC>(owner) }
        }
    }

    #[inline(always)]
    pub const fn get_angle(&self) -> NiPoint3 {
        self.data.angle
    }

    #[inline(always)]
    pub const fn get_angle_x(&self) -> f32 {
        self.data.angle.x
    }

    #[inline(always)]
    pub const fn get_angle_y(&self) -> f32 {
        self.data.angle.y
    }

    #[inline(always)]
    pub const fn get_angle_z(&self) -> f32 {
        self.data.angle.z
    }

    #[inline(always)]
    pub fn get_base_height(&self) -> f32 {
        let mut height = f32::from(self.get_reference_runtime_data().ref_scale) / 100.0;
        let object = self.get_object_reference();
        if !object.is_null() {
            let npc = unsafe { skyrim_cast::<TESForm, TESNPC>(object.cast::<TESForm>()) };
            if !npc.is_null() {
                height *= unsafe { (*npc).get_height() };
            }
        }
        height
    }

    #[inline(always)]
    pub const fn get_base_object(&self) -> *mut TESBoundObject {
        self.data.object_reference
    }

    #[inline(always)]
    pub fn get_biped(&self) -> &BSTSmartPointer<BipedAnim> {
        let biped = unsafe { self.get_biped2_raw() };
        assert!(
            !biped.is_null(),
            "TESObjectREFR::GetBiped returned null reference"
        );
        unsafe { &*biped }
    }

    #[inline(always)]
    pub fn get_biped_with_view(&self, first_person: bool) -> &BSTSmartPointer<BipedAnim> {
        let biped = unsafe { self.get_biped1_raw(first_person) };
        assert!(
            !biped.is_null(),
            "TESObjectREFR::GetBiped(bool) returned null reference"
        );
        unsafe { &*biped }
    }

    #[inline(always)]
    pub fn get_current_biped(&self) -> &BSTSmartPointer<BipedAnim> {
        let biped = unsafe { self.get_current_biped_raw() };
        assert!(
            !biped.is_null(),
            "TESObjectREFR::GetCurrentBiped returned null reference"
        );
        unsafe { &*biped }
    }

    #[inline(always)]
    pub fn get_container(&self) -> *mut TESContainer {
        let object = self.get_object_reference();
        if object.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { skyrim_cast::<TESForm, TESContainer>(object.cast::<TESForm>()) }
        }
    }

    #[inline(always)]
    pub fn get_editor_location(&self) -> *mut BGSLocation {
        self.get_editor_location1()
    }

    #[inline(always)]
    pub fn get_editor_location_data(
        &mut self,
        out_pos: &mut NiPoint3,
        out_rot: &mut NiPoint3,
        out_world_or_cell: &mut *mut TESForm,
        fallback: *mut TESObjectCELL,
    ) -> bool {
        self.get_editor_location2(out_pos, out_rot, out_world_or_cell, fallback)
    }

    pub fn get_enchantment(&self) -> *mut EnchantmentItem {
        let x_ench = self.extra_list.get_by_type_typed::<ExtraEnchantment>();
        if !x_ench.is_null() && unsafe { !(*x_ench).enchantment.is_null() } {
            return unsafe { (*x_ench).enchantment };
        }

        let object = self.get_object_reference();
        if object.is_null() {
            return core::ptr::null_mut();
        }

        let enchantable =
            unsafe { skyrim_cast::<TESForm, TESEnchantableForm>(object.cast::<TESForm>()) };
        if enchantable.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*enchantable).form_enchanting }
        }
    }

    pub fn get_enchantment_charge(&self) -> Option<f64> {
        let mut result = None;
        let object = self.get_object_reference();
        let enchantable = if object.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { skyrim_cast::<TESForm, TESEnchantableForm>(object.cast::<TESForm>()) }
        };

        if !enchantable.is_null()
            && unsafe { !(*enchantable).form_enchanting.is_null() }
            && unsafe { (*enchantable).amount_of_enchantment != 0 }
        {
            result = Some(100.0);
        }

        let x_charge = self.extra_list.get_by_type_typed::<ExtraCharge>();
        let x_ench = self.extra_list.get_by_type_typed::<ExtraEnchantment>();
        if !x_ench.is_null()
            && unsafe { !(*x_ench).enchantment.is_null() }
            && unsafe { (*x_ench).charge != 0 }
        {
            result = Some(if x_charge.is_null() {
                100.0
            } else {
                (f64::from(unsafe { (*x_charge).charge }) / f64::from(unsafe { (*x_ench).charge }))
                    * 100.0
            });
        } else if !x_charge.is_null()
            && !enchantable.is_null()
            && unsafe { !(*enchantable).form_enchanting.is_null() }
            && unsafe { (*enchantable).amount_of_enchantment != 0 }
        {
            result = Some(
                (f64::from(unsafe { (*x_charge).charge })
                    / f64::from(unsafe { (*enchantable).amount_of_enchantment }))
                    * 100.0,
            );
        }

        result
    }

    #[inline(always)]
    pub fn get_faction_owner(&self) -> *mut crate::re::TESFaction {
        let x_ownership = self.extra_list.get_by_type_typed::<ExtraOwnership>();
        if x_ownership.is_null() {
            return core::ptr::null_mut();
        }

        let owner = unsafe { (*x_ownership).owner };
        if owner.is_null() || unsafe { !(*owner).is(FormType::Faction) } {
            core::ptr::null_mut()
        } else {
            unsafe { skyrim_cast::<TESForm, crate::re::TESFaction>(owner) }
        }
    }

    #[inline(always)]
    pub fn get_handle(&self) -> ObjectRefHandle {
        ObjectRefHandle::from_ptr(self as *const Self as *mut Self)
    }

    pub fn get_heading_angle(&self, pos: &NiPoint3, abs: bool) -> f32 {
        let theta = unsafe { atan2f(pos.x - self.get_position_x(), pos.y - self.get_position_y()) };
        let mut heading = (theta - self.get_angle_z()).to_degrees();

        if heading < -180.0 {
            heading += 360.0;
        }

        if heading > 180.0 {
            heading -= 360.0;
        }

        if abs { heading.abs() } else { heading }
    }

    #[inline(always)]
    pub fn get_height(&self) -> f32 {
        let min = self.get_bound_min();
        let max = self.get_bound_max();
        self.get_base_height() * (max.z - min.z)
    }

    #[inline(always)]
    pub fn get_inventory(&mut self) -> TESObjectREFRInventoryItemMap {
        self.get_inventory_with(Self::default_inventory_filter, false)
    }

    pub fn get_inventory_with<F>(
        &mut self,
        mut filter: F,
        no_init: bool,
    ) -> TESObjectREFRInventoryItemMap
    where
        F: FnMut(&TESBoundObject) -> bool,
    {
        let mut results = TESObjectREFRInventoryItemMap::new();

        let inv_changes = self.get_inventory_changes(no_init);
        if !inv_changes.is_null() {
            let entry_list = unsafe { (*inv_changes).entry_list };
            if !entry_list.is_null() {
                for &entry in unsafe { (*entry_list).iter() } {
                    if entry.is_null() {
                        continue;
                    }

                    let object = unsafe { (*entry).object };
                    if object.is_null() || !filter(unsafe { &*object }) {
                        continue;
                    }

                    let mut copied = Box::new(InventoryEntryData::default());
                    copied.deep_copy(unsafe { &*entry });
                    results.insert(object, (unsafe { (*entry).count_delta }, copied));
                }
            }
        }

        let container = self.get_container();
        if !container.is_null() {
            unsafe {
                (*container).for_each_container_object(|entry| {
                    let object = entry.obj;
                    if object.is_null() || !filter(&*object) {
                        return BSContainerForEachResult::Continue;
                    }

                    let should_ignore = results
                        .get(&object)
                        .map(|(_, data)| data.is_leveled())
                        .unwrap_or(false);
                    if should_ignore {
                        return BSContainerForEachResult::Continue;
                    }

                    if let Some((count, _)) = results.get_mut(&object) {
                        *count += entry.count;
                    } else {
                        results.insert(
                            object,
                            (entry.count, Box::new(InventoryEntryData::new(object, 0))),
                        );
                    }

                    BSContainerForEachResult::Continue
                })
            };
        }

        results
    }
}

impl TESObjectREFR {
    #[inline(always)]
    pub fn get_inventory_count(&mut self, no_init: bool) -> i32 {
        self.get_inventory_counts_with(Self::default_inventory_filter, no_init)
            .values()
            .copied()
            .sum()
    }

    #[inline(always)]
    pub fn get_inventory_counts(&mut self) -> TESObjectREFRInventoryCountMap {
        self.get_inventory_counts_with(Self::default_inventory_filter, false)
    }

    pub fn get_inventory_counts_with<F>(
        &mut self,
        filter: F,
        no_init: bool,
    ) -> TESObjectREFRInventoryCountMap
    where
        F: FnMut(&TESBoundObject) -> bool,
    {
        self.get_inventory_with(filter, no_init)
            .into_iter()
            .map(|(key, value)| (key, value.0))
            .collect()
    }

    #[inline(always)]
    pub fn get_dropped_inventory(&self) -> TESObjectREFRInventoryDropMap {
        self.get_dropped_inventory_with(Self::default_inventory_filter)
    }

    pub fn get_dropped_inventory_with<F>(&self, mut filter: F) -> TESObjectREFRInventoryDropMap
    where
        F: FnMut(&TESBoundObject) -> bool,
    {
        let mut results = TESObjectREFRInventoryDropMap::new();

        let x_drop = self.extra_list.get_by_type_typed::<ExtraDroppedItemList>();
        if x_drop.is_null() {
            return results;
        }

        for handle in unsafe { (*x_drop).dropped_item_list.iter() } {
            let reference = handle.get().get();
            if reference.is_null() {
                continue;
            }

            let object = unsafe { (*reference).get_object_reference() };
            if object.is_null() || !filter(unsafe { &*object }) {
                continue;
            }

            let count = unsafe { (*reference).extra_list.get_count() };
            if let Some(existing) = results.get_mut(&object) {
                existing.0 += count;
                existing.1.push(*handle);
            } else {
                results.insert(object, (count, Vec::from([*handle])));
            }
        }

        results
    }

    pub fn get_inventory_changes(&mut self, no_init: bool) -> *mut InventoryChanges {
        let x_container_changes = self
            .extra_list
            .get_by_type(ExtraDataType::ContainerChanges)
            .cast::<ExtraContainerChanges>();

        if x_container_changes.is_null() {
            if no_init {
                return core::ptr::null_mut();
            }

            if !self.init_inventory_if_required(false) {
                return self.force_init_inventory_changes();
            }
        }

        let x_container_changes = self
            .extra_list
            .get_by_type(ExtraDataType::ContainerChanges)
            .cast::<ExtraContainerChanges>();
        if x_container_changes.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*x_container_changes).changes }
        }
    }

    #[inline(always)]
    pub fn get_linked_ref(&self, keyword: *mut BGSKeyword) -> *mut TESObjectREFR {
        self.extra_list.get_linked_ref(keyword)
    }

    #[inline(always)]
    pub fn get_name(&self) -> *const c_char {
        let object = self.get_object_reference();
        if object.is_null() {
            c"".as_ptr()
        } else {
            unsafe { (&*object.cast::<TESForm>()).get_name() }
        }
    }

    #[inline(always)]
    pub fn get_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_name())
    }

    #[inline(always)]
    pub fn get_node_by_name(&self, node_name: &BSFixedString) -> *mut NiAVObject {
        let node = self.get_3d();
        if node.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (&mut *node).get_object_by_name(node_name as *const _) }
        }
    }

    #[inline(always)]
    pub const fn get_object_reference(&self) -> *mut TESBoundObject {
        self.data.object_reference
    }

    #[inline(always)]
    pub fn get_lock_level(&self) -> LOCK_LEVEL {
        let state = self.get_lock();
        if state.is_null() {
            LOCK_LEVEL::Unlocked
        } else {
            unsafe { (*state).get_lock_level(self) }
        }
    }

    #[inline(always)]
    pub const fn get_parent_cell(&self) -> *mut TESObjectCELL {
        self.parent_cell
    }

    #[inline(always)]
    pub const fn get_position(&self) -> NiPoint3 {
        self.data.location
    }

    #[inline(always)]
    pub const fn get_position_x(&self) -> f32 {
        self.data.location.x
    }

    #[inline(always)]
    pub const fn get_position_y(&self) -> f32 {
        self.data.location.y
    }

    #[inline(always)]
    pub const fn get_position_z(&self) -> f32 {
        self.data.location.z
    }

    pub fn get_sequence(&self, name: &str) -> *mut NiControllerSequence {
        let node = self.get_3d();
        if node.is_null() {
            return core::ptr::null_mut();
        }

        let controller = unsafe { (&*node).get_controllers() };
        if controller.is_null() {
            return core::ptr::null_mut();
        }

        let manager = unsafe { (&mut *controller).as_ni_controller_manager() };
        if manager.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*manager).get_sequence_by_name(name) }
        }
    }

    pub fn get_submerge_level(&self, z_pos: f32, cell: *mut TESObjectCELL) -> f32 {
        // TODO: `TESObjectREFR.h` declares `IsPointDeepUnderWater(float, TESObjectCELL*) const`,
        // but the vendored CommonLib tree does not provide a matching `.cpp` body or relocation
        // wrapper. Keep this helper aligned with the shipped `GetSubmergeLevel` behavior until
        // that dedicated engine entrypoint is source-backed.
        let mut water_height = if cell.is_null() || core::ptr::eq(cell, self.parent_cell) {
            self.get_water_height()
        } else {
            unsafe { (*cell).get_exterior_water_height() }
        };

        if water_height == f32::NEG_INFINITY && !cell.is_null() {
            water_height = unsafe { (*cell).get_exterior_water_height() };
        }

        if water_height <= z_pos {
            0.0
        } else {
            ((water_height - z_pos) / self.get_height()).min(1.0)
        }
    }

    pub fn get_water_height(&self) -> f32 {
        if !self.loaded_data.is_null() {
            let water_height = unsafe { (*self.loaded_data).relevant_water_height };
            if water_height != f32::NEG_INFINITY {
                return water_height;
            }
        }

        if self.parent_cell.is_null() {
            f32::NEG_INFINITY
        } else {
            unsafe { (*self.parent_cell).get_exterior_water_height() }
        }
    }

    #[inline(always)]
    pub fn get_weight(&self) -> f32 {
        let object = self.get_object_reference();
        if object.is_null() {
            0.0
        } else {
            unsafe { (&*object.cast::<TESForm>()).get_weight() }
        }
    }

    pub fn get_world_location(&self) -> BGSWorldLocation {
        let mut world_location = BGSWorldLocation {
            pos: self.get_position(),
            space: core::ptr::null_mut(),
        };

        if !self.parent_cell.is_null() {
            if unsafe { (*self.parent_cell).is_interior_cell() }
                || tes_object_cell_world_space(self.parent_cell).is_null()
            {
                world_location.space = self.parent_cell.cast();
            }
        } else {
            let x_persistent_cell = self.extra_list.get_by_type_typed::<ExtraPersistentCell>();
            if !x_persistent_cell.is_null() {
                let persistent_cell = unsafe { (*x_persistent_cell).persistent_cell };
                if !persistent_cell.is_null() && unsafe { (*persistent_cell).is_exterior_cell() } {
                    world_location.space = tes_object_cell_world_space(persistent_cell).cast();
                }
            }
        }

        world_location
    }

    pub fn get_worldspace(&self) -> *mut TESWorldSpace {
        let mut cell = self.parent_cell;
        if cell.is_null() {
            cell = self.get_save_parent_cell();
        }

        if cell.is_null() || unsafe { !(*cell).is_exterior_cell() } {
            core::ptr::null_mut()
        } else {
            tes_object_cell_world_space(cell)
        }
    }

    #[inline(always)]
    pub fn has_collision(&self) -> bool {
        self.base.form_flags.bits() & TESObjectREFRRecordFlags::COLLISIONS_DISABLED.bits() == 0
    }

    #[inline(always)]
    pub fn has_container(&self) -> bool {
        !self.get_container().is_null()
    }

    #[inline(always)]
    pub fn has_keyword(&self, keyword: *const BGSKeyword) -> bool {
        self.has_keyword_helper(keyword)
    }

    pub fn has_keyword_in_array(&self, keywords: &[*mut BGSKeyword], match_all: bool) -> bool {
        let mut has_keyword = false;

        for &keyword in keywords {
            has_keyword = !keyword.is_null() && self.has_keyword(keyword);
            if (match_all && !has_keyword) || has_keyword {
                break;
            }
        }

        has_keyword
    }

    pub fn has_keyword_in_list(&self, keyword_list: *mut BGSListForm, match_all: bool) -> bool {
        if keyword_list.is_null() {
            return false;
        }

        let mut has_keyword = false;
        unsafe {
            (*keyword_list).for_each_form(|form| {
                let keyword = if form.is_null() {
                    core::ptr::null_mut()
                } else {
                    skyrim_cast::<TESForm, BGSKeyword>(form)
                };
                has_keyword = !keyword.is_null() && self.has_keyword(keyword);
                if (match_all && !has_keyword) || has_keyword {
                    BSContainerForEachResult::Stop
                } else {
                    BSContainerForEachResult::Continue
                }
            })
        };
        has_keyword
    }

    #[inline(always)]
    pub fn has_keyword_with_type(&self, keyword_type: u32) -> bool {
        let keyword = tes_object_refr_default_keyword(keyword_type);
        !keyword.is_null() && self.has_keyword(keyword)
    }

    #[inline(always)]
    pub fn instantiate_hit_art(
        &self,
        art: *mut BGSArtObject,
        duration: f32,
        facing_ref: *mut TESObjectREFR,
        face_target: bool,
        attach_to_camera: bool,
        attach_node: *mut NiAVObject,
        interface_effect: bool,
    ) -> *mut ModelReferenceEffect {
        self.apply_art_object(
            art,
            duration,
            facing_ref,
            face_target,
            attach_to_camera,
            attach_node,
            interface_effect,
        )
    }

    #[inline(always)]
    pub fn instantiate_hit_shader(
        &self,
        shader: *mut TESEffectShader,
        duration: f32,
        facing_ref: *mut TESObjectREFR,
        face_target: bool,
        attach_to_camera: bool,
        attach_node: *mut NiAVObject,
        interface_effect: bool,
    ) -> *mut ShaderReferenceEffect {
        self.apply_effect_shader(
            shader,
            duration,
            facing_ref,
            face_target,
            attach_to_camera,
            attach_node,
            interface_effect,
        )
    }
}

impl TESObjectREFR {
    #[inline(always)]
    pub fn is_3d_loaded(&self) -> bool {
        !self.get_3d().is_null()
    }

    #[inline(always)]
    pub fn is_activation_blocked(&self) -> bool {
        let x_flags = self.extra_list.get_by_type_typed::<ExtraFlags>();
        !x_flags.is_null() && unsafe { (*x_flags).is_activation_blocked() }
    }

    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        self.base.form_flags.bits() & TESObjectREFRRecordFlags::INITIALLY_DISABLED.bits() != 0
    }

    #[inline(always)]
    pub fn is_enchanted(&self) -> bool {
        !self.get_enchantment().is_null()
    }

    #[inline(always)]
    pub fn is_animal(&self) -> bool {
        self.has_keyword_with_type(DEFAULT_OBJECT_ID_KEYWORD_ANIMAL)
    }

    #[inline(always)]
    pub fn is_dragon(&self) -> bool {
        self.has_keyword_with_type(DEFAULT_OBJECT_ID_KEYWORD_DRAGON)
    }

    #[inline(always)]
    pub fn is_horse(&self) -> bool {
        self.has_keyword_with_type(DEFAULT_OBJECT_ID_KEYWORD_HORSE)
    }

    #[inline(always)]
    pub fn is_humanoid(&self) -> bool {
        self.has_keyword_with_type(DEFAULT_OBJECT_ID_KEYWORD_NPC)
    }

    #[inline(always)]
    pub fn is_in_water(&self) -> bool {
        self.get_water_height() > self.get_position_z()
    }

    #[inline(always)]
    pub fn is_initially_disabled(&self) -> bool {
        self.base.form_flags.bits() & TESObjectREFRRecordFlags::INITIALLY_DISABLED.bits() != 0
    }

    #[inline(always)]
    pub fn is_jewelry(&self) -> bool {
        self.has_keyword_with_type(DEFAULT_OBJECT_ID_KEYWORD_JEWELRY)
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.get_lock_level() != LOCK_LEVEL::Unlocked
    }

    #[inline(always)]
    pub fn is_marked_for_deletion(&self) -> bool {
        self.base.form_flags.bits() & TESObjectREFRRecordFlags::DELETED.bits() != 0
    }

    #[inline(always)]
    pub fn is_off_limits(&self) -> bool {
        self.is_crime_to_activate()
    }

    #[inline(always)]
    pub fn is_persistent(&self) -> bool {
        self.base.form_flags.bits() & TESObjectREFRRecordFlags::PERSISTENT.bits() != 0
    }

    #[inline(always)]
    pub fn is_point_submerged_more_than(
        &self,
        pos: NiPoint3,
        cell: *mut TESObjectCELL,
        water_level: f32,
    ) -> bool {
        self.get_submerge_level(pos.z, cell) >= water_level
    }

    pub fn move_to(&mut self, target: *mut TESObjectREFR) {
        assert!(
            !target.is_null(),
            "TESObjectREFR::MoveTo target must not be null"
        );
        // TODO: `TESObjectREFR.h` declares `MoveRefToNewSpace(TESObjectCELL*, TESWorldSpace*)`,
        // but the vendored CommonLib tree does not provide a `.cpp` body or relocation wrapper
        // for that narrower helper. Keep using the proven `MoveToImpl(...)` path until the
        // engine entrypoint for `MoveRefToNewSpace` is source-backed.
        let target_ref = unsafe { &*target };
        let handle = target_ref.get_handle();
        self.move_to_impl(
            &handle,
            target_ref.get_parent_cell(),
            target_ref.get_worldspace(),
            &target_ref.get_position(),
            &target_ref.get_angle(),
        );
    }

    pub fn find_nearest_vertex(&self, minimum_offset: f32) -> Option<NiPoint3> {
        let nav_meshes = tes_object_cell_nav_meshes(self.get_parent_cell());
        if nav_meshes.is_null() {
            return None;
        }

        let mut shortest_distance = f32::MAX;
        let mut position = None;
        for nav_mesh in unsafe { (*nav_meshes).nav_meshes.as_slice() } {
            let nav_mesh = nav_mesh.get();
            if nav_mesh.is_null() {
                continue;
            }

            let vertices = unsafe { &(*nav_mesh).navmesh.vertices };
            for vertex in unsafe { vertices.as_slice() } {
                let linear_distance = self.get_position().get_distance(vertex.location);
                if linear_distance < shortest_distance && linear_distance >= minimum_offset {
                    shortest_distance = linear_distance;
                    position = Some(vertex.location);
                }
            }
        }

        position
    }

    pub fn move_to_editor_location_with(&mut self, position: NiPoint3, rotation: NiPoint3) -> bool {
        let editor_location = self.get_editor_location();
        if editor_location.is_null() {
            return false;
        }

        let handle = unsafe { (*editor_location).world_loc_marker };
        let world_loc_ref = handle.get().get();
        if world_loc_ref.is_null() {
            return false;
        }

        let world_loc_ref = unsafe { &*world_loc_ref };
        self.move_to_impl(
            &handle,
            world_loc_ref.get_parent_cell(),
            world_loc_ref.get_worldspace(),
            &position,
            &rotation,
        );
        true
    }

    #[inline(always)]
    pub fn move_to_editor_location_current(&mut self) -> bool {
        self.move_to_editor_location_with(self.get_starting_location(), self.get_starting_angle())
    }

    pub fn move_to_nearest_navmesh(&mut self, minimum_offset: f32) -> bool {
        let Some(nearest_vertex) = self.find_nearest_vertex(minimum_offset) else {
            return false;
        };

        let handle = self.create_ref_handle();
        let angle = self.get_angle();
        self.move_to_impl(
            &handle,
            self.get_parent_cell(),
            self.get_worldspace(),
            &nearest_vertex,
            &angle,
        );
        true
    }

    pub fn move_to_node_by_name(
        &mut self,
        target: *mut TESObjectREFR,
        node_name: &BSFixedString,
    ) -> bool {
        assert!(
            !target.is_null(),
            "TESObjectREFR::MoveToNode target must not be null"
        );

        let node = unsafe { (&*target).get_3d() };
        if node.is_null() {
            return false;
        }

        let object = unsafe { (&mut *node).get_object_by_name(node_name as *const _) };
        if object.is_null() {
            return false;
        }

        self.move_to_node(target, object)
    }

    pub fn move_to_node(&mut self, target: *mut TESObjectREFR, node: *mut NiAVObject) -> bool {
        assert!(
            !target.is_null() && !node.is_null(),
            "TESObjectREFR::MoveToNode target and node must not be null"
        );

        let mut rotation = NiPoint3::default();
        let world = unsafe { (*node).world };
        world.rotate.to_euler_angles_xyz(&mut rotation);

        let target_ref = unsafe { &*target };
        let handle = target_ref.get_handle();
        self.move_to_impl(
            &handle,
            target_ref.get_parent_cell(),
            self.get_worldspace(),
            &world.translate,
            &rotation,
        );
        true
    }

    #[inline(always)]
    pub fn name_includes(&self, word: &str) -> bool {
        self.get_name_as_str().contains(word)
    }

    pub fn place_object_at_me(
        &self,
        base_to_place: *mut TESBoundObject,
        force_persist: bool,
    ) -> NiPointer<TESObjectREFR> {
        let data_handler = TESDataHandler::get_singleton(true);
        if data_handler.is_null() {
            return NiPointer::null();
        }

        let linked_room_ref_handle = ObjectRefHandle::new();
        let handle = unsafe {
            (*data_handler).create_reference_at_location(
                base_to_place,
                &self.get_position(),
                &self.get_angle(),
                self.get_parent_cell(),
                self.get_worldspace(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &linked_room_ref_handle,
                force_persist,
                true,
            )
        };
        handle.get()
    }

    pub fn play_animation_by_name(&mut self, from: &str, to: &str) {
        let node = self.get_3d();
        if node.is_null() {
            return;
        }

        let controller = unsafe { (&*node).get_controllers() };
        if controller.is_null() {
            return;
        }

        let manager = unsafe { (&mut *controller).as_ni_controller_manager() };
        if manager.is_null() {
            return;
        }

        let from_seq = unsafe { (*manager).get_sequence_by_name(from) };
        let to_seq = unsafe { (*manager).get_sequence_by_name(to) };
        if from_seq.is_null() || to_seq.is_null() {
            return;
        }

        self.play_animation_sequences(manager, to_seq, from_seq);
    }

    #[inline(always)]
    pub fn play_animation_sequences(
        &mut self,
        manager: *mut NiControllerManager,
        to_seq: *mut NiControllerSequence,
        from_seq: *mut NiControllerSequence,
    ) {
        self.play_animation_impl(manager, to_seq, from_seq, false);
    }

    #[inline(always)]
    pub fn set_activation_blocked(&mut self, blocked: bool) {
        self.extra_list
            .set_extra_flags(ExtraFlagsFlag::BlockActivate, blocked);
    }

    pub fn set_collision(&mut self, enable: bool) {
        let mut bits = self.base.form_flags.bits();
        if enable {
            bits &= !TESObjectREFRRecordFlags::COLLISIONS_DISABLED.bits();
        } else {
            bits |= TESObjectREFRRecordFlags::COLLISIONS_DISABLED.bits();
        }
        self.base.form_flags = TESFormRecordFlag::from_bits_retain(bits);
    }

    pub fn set_display_name(&mut self, name: &BSFixedString, force: bool) -> bool {
        let renamed;
        let mut x_text_data = self.extra_list.get_by_type_typed::<ExtraTextDisplayData>();

        if !x_text_data.is_null() {
            let in_use = unsafe {
                !(*x_text_data).display_name_text.is_null() || !(*x_text_data).owner_quest.is_null()
            };
            if in_use && force {
                unsafe {
                    (*x_text_data).display_name_text = core::ptr::null_mut();
                    (*x_text_data).owner_quest = core::ptr::null_mut();
                }
            }
            renamed = !in_use || force;
            unsafe { (*x_text_data).set_name(name.as_ptr()) };
        } else {
            let extra = Box::new(ExtraTextDisplayData::new_with_name(name.as_ptr()));
            x_text_data = Box::into_raw(extra);
            self.extra_list.add(x_text_data.cast());
            renamed = true;
        }

        renamed
    }

    #[inline(always)]
    pub fn set_encounter_zone(&mut self, zone: *mut BGSEncounterZone) {
        self.extra_list.set_encounter_zone(zone);
        self.base
            .add_change(TESObjectREFRChangeFlags::ENC_ZONE_EXTRA.bits());
    }

    pub fn set_motion_type(
        &mut self,
        motion_type: hkpMotionMotionType,
        allow_activate: bool,
    ) -> bool {
        let node = self.get_3d();
        if node.is_null() {
            return false;
        }

        let result = unsafe {
            (&mut *node).set_motion_type(motion_type as u32, true, false, allow_activate)
        };
        self.base
            .add_change(TESObjectREFRChangeFlags::HAVOK_MOVED.bits());
        result
    }

    #[inline(always)]
    pub fn set_position_xyz(&mut self, x: f32, y: f32, z: f32) {
        self.set_position(&NiPoint3 { x, y, z });
    }

    fn force_init_inventory_changes(&mut self) -> *mut InventoryChanges {
        let changes = self.make_inventory_changes();
        if !changes.is_null() {
            unsafe {
                (*changes).init_leveled_items();
                (*changes).init_from_container_extra();
                (*changes).init_scripts();
            }
        }
        changes
    }
}

impl TESObjectREFR {
    // TODO: `AttachWeapon` is still omitted here. CommonLib exposes it as a VR-only virtual slot
    // before `RemoveWeapon`, but the flat-runtime path is non-virtual and no source-backed
    // relocation/helper body is vendored for a unified cross-runtime wrapper yet.
    crate::relocated_virtual_method! {
        pub const VFUNC_REMOVE_WEAPON: VariantOffset = VariantOffset::new_se_ae(0x82, 0x83);
        pub fn remove_weapon(&mut self, equip_index: BIPED_OBJECT)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_83: VariantOffset = VariantOffset::new_se_ae(0x83, 0x84);
        pub fn unk_83(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_OBJECT_REFERENCE: VariantOffset = VariantOffset::new_se_ae(0x84, 0x85);
        pub fn set_object_reference(&mut self, object: *mut TESBoundObject)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_MOVE_HAVOK: VariantOffset = VariantOffset::new_se_ae(0x85, 0x86);
        pub fn move_havok(&mut self, force_rec: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_LINEAR_VELOCITY: VariantOffset = VariantOffset::new_se_ae(0x86, 0x87);
        pub fn get_linear_velocity(&self, velocity: &mut NiPoint3)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_ACTION_COMPLETE: VariantOffset = VariantOffset::new_se_ae(0x87, 0x88);
        pub fn set_action_complete(&mut self, set: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_MOVEMENT_COMPLETE: VariantOffset = VariantOffset::new_se_ae(0x88, 0x89);
        pub fn set_movement_complete(&mut self, set: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_DISABLE: VariantOffset = VariantOffset::new_se_ae(0x89, 0x8A);
        pub fn disable(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_RESET_INVENTORY: VariantOffset = VariantOffset::new_se_ae(0x8A, 0x8B);
        pub fn reset_inventory(&mut self, leveled_only: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_FIRE_NODE: VariantOffset = VariantOffset::new_se_ae(0x8B, 0x8C);
        pub fn get_fire_node(&mut self) -> *mut NiNode
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_FIRE_NODE: VariantOffset = VariantOffset::new_se_ae(0x8C, 0x8D);
        pub fn set_fire_node(&mut self, fire_node: *mut NiNode)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_CURRENT_3D: VariantOffset = VariantOffset::new_se_ae(0x8D, 0x8E);
        pub fn get_current_3d(&self) -> *mut NiAVObject
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_AS_EXPLOSION: VariantOffset = VariantOffset::new_se_ae(0x8E, 0x8F);
        pub fn as_explosion(&mut self) -> *mut Explosion
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_AS_PROJECTILE: VariantOffset = VariantOffset::new_se_ae(0x8F, 0x90);
        pub fn as_projectile(&mut self) -> *mut Projectile
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ON_ADD_CELL_PERFORM_QUEUE_REFERENCE: VariantOffset =
            VariantOffset::new_se_ae(0x90, 0x91);
        pub fn on_add_cell_perform_queue_reference(&self, cell: &mut TESObjectCELL) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_DO_MOVE_TO_HIGH: VariantOffset = VariantOffset::new_se_ae(0x91, 0x92);
        pub fn do_move_to_high(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_TRY_MOVE_TO_MIDDLE_LOW: VariantOffset = VariantOffset::new_se_ae(0x92, 0x93);
        pub fn try_move_to_middle_low(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_TRY_CHANGE_SKY_CELL_ACTORS_PROCESS_LEVEL: VariantOffset =
            VariantOffset::new_se_ae(0x93, 0x94);
        pub fn try_change_sky_cell_actors_process_level(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_94: VariantOffset = VariantOffset::new_se_ae(0x94, 0x95);
        pub fn unk_94(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_TRY_UPDATE_ACTOR_LAST_SEEN_TIME: VariantOffset =
            VariantOffset::new_se_ae(0x95, 0x96);
        pub fn try_update_actor_last_seen_time(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_96: VariantOffset = VariantOffset::new_se_ae(0x96, 0x97);
        pub fn unk_96(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_SAVE_PARENT_CELL: VariantOffset = VariantOffset::new_se_ae(0x97, 0x98);
        pub fn get_save_parent_cell(&self) -> *mut TESObjectCELL
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_PARENT_CELL: VariantOffset = VariantOffset::new_se_ae(0x98, 0x99);
        pub fn set_parent_cell(&mut self, cell: *mut TESObjectCELL)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_IS_DEAD: VariantOffset = VariantOffset::new_se_ae(0x99, 0x9A);
        pub fn is_dead(&self, not_essential: bool) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_CREATE_ANIM_NOTE_RECEIVER: VariantOffset =
            VariantOffset::new_se_ae(0x9A, 0x9B);
        pub fn create_anim_note_receiver(&mut self) -> *mut BSAnimNoteReceiver
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_ANIM_NOTE_RECEIVER: VariantOffset =
            VariantOffset::new_se_ae(0x9B, 0x9C);
        pub fn get_anim_note_receiver(&mut self) -> *mut BSAnimNoteReceiver
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_PROCESS_IN_WATER: VariantOffset = VariantOffset::new_se_ae(0x9C, 0x9D);
        pub fn process_in_water(
            &mut self,
            collidable: *mut hkpCollidable,
            water_height: f32,
            delta_time: f32
        ) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_APPLY_CURRENT: VariantOffset = VariantOffset::new_se_ae(0x9D, 0x9E);
        pub fn apply_current(&mut self, velocity_time: f32, velocity: &hkVector4) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_CURRENT_AMMO: VariantOffset = VariantOffset::new_se_ae(0x9E, 0x9F);
        pub fn get_current_ammo(&self) -> *mut TESAmmo
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_DECAL_GROUP: VariantOffset = VariantOffset::new_se_ae(0x9F, 0xA0);
        pub fn get_decal_group(&self) -> *mut BGSDecalGroup
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_A0: VariantOffset = VariantOffset::new_se_ae(0xA0, 0xA1);
        pub fn unk_a0(
            &mut self,
            node: *mut NiAVObject,
            angle_x: &mut f32,
            angle_z: &mut f32,
            pos: &mut NiPoint3
        ) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNEQUIP_ITEM: VariantOffset = VariantOffset::new_se_ae(0xA1, 0xA2);
        pub fn unequip_item(&mut self, arg1: u64, object: *mut TESBoundObject)
    }
}

pub trait TESObjectREFRExt {
    fn predestroy(&mut self);
    fn enable(&mut self, reset_inventory: bool);
    fn get_3d(&self) -> *mut NiAVObject;
    fn get_3d_with_view(&self, first_person: bool) -> *mut NiAVObject;
    fn get_angle(&self) -> NiPoint3;
    fn get_angle_x(&self) -> f32;
    fn get_angle_y(&self) -> f32;
    fn get_angle_z(&self) -> f32;
    fn get_base_height(&self) -> f32;
    fn get_base_object(&self) -> *mut TESBoundObject;
    fn get_biped(&self) -> &BSTSmartPointer<BipedAnim>;
    fn get_biped_with_view(&self, first_person: bool) -> &BSTSmartPointer<BipedAnim>;
    fn get_current_biped(&self) -> &BSTSmartPointer<BipedAnim>;
    fn get_container(&self) -> *mut TESContainer;
    fn get_current_location(&self) -> *mut BGSLocation;
    fn get_reference_runtime_data(&self) -> &REFERENCE_RUNTIME_DATA;
    fn get_reference_runtime_data_mut(&mut self) -> &mut REFERENCE_RUNTIME_DATA;
    fn get_magic_caster(&mut self, source: CastingSource) -> *mut MagicCaster;
    fn get_display_full_name(&self) -> *const c_char;
    fn get_display_full_name_as_str(&self) -> &str;
    fn get_distance(
        &self,
        other: *mut TESObjectREFR,
        disabled_refs: bool,
        ignore_worldspace: bool,
    ) -> f32;
    fn get_dropped_inventory(&self) -> TESObjectREFRInventoryDropMap;
    fn get_editor_location(&self) -> *mut BGSLocation;
    fn get_editor_location_data(
        &mut self,
        out_pos: &mut NiPoint3,
        out_rot: &mut NiPoint3,
        out_world_or_cell: &mut *mut TESForm,
        fallback: *mut TESObjectCELL,
    ) -> bool;
    fn get_enchantment(&self) -> *mut EnchantmentItem;
    fn get_enchantment_charge(&self) -> Option<f64>;
    fn get_actor_owner(&self) -> *mut TESNPC;
    fn get_face_node_skinned(&mut self) -> *mut BSFaceGenNiNode;
    fn get_face_node(&mut self) -> *mut BSFaceGenNiNode;
    fn get_face_gen_animation_data(&mut self) -> *mut BSFaceGenAnimationData;
    fn get_faction_owner(&self) -> *mut crate::re::TESFaction;
    fn get_handle(&self) -> ObjectRefHandle;
    fn get_heading_angle(&self, pos: &NiPoint3, abs: bool) -> f32;
    fn get_height(&self) -> f32;
    fn get_inventory(&mut self) -> TESObjectREFRInventoryItemMap;
    fn get_inventory_count(&mut self, no_init: bool) -> i32;
    fn get_inventory_counts(&mut self) -> TESObjectREFRInventoryCountMap;
    fn get_inventory_changes(&mut self, no_init: bool) -> *mut InventoryChanges;
    fn get_linked_ref(&self, keyword: *mut BGSKeyword) -> *mut TESObjectREFR;
    fn get_lock(&self) -> *mut REFR_LOCK;
    fn get_lock_level(&self) -> LOCK_LEVEL;
    fn get_magic_target(&mut self) -> *mut MagicTarget;
    fn get_name(&self) -> *const c_char;
    fn get_name_as_str(&self) -> &str;
    fn get_node_by_name(&self, node_name: &BSFixedString) -> *mut NiAVObject;
    fn get_object_reference(&self) -> *mut TESBoundObject;
    fn get_owner(&self) -> *mut TESForm;
    fn get_parent_cell(&self) -> *mut TESObjectCELL;
    fn get_position(&self) -> NiPoint3;
    fn get_current_3d(&self) -> *mut NiAVObject;
    fn get_current_ammo(&self) -> *mut TESAmmo;
    fn get_decal_group(&self) -> *mut BGSDecalGroup;
    fn get_scale(&self) -> f32;
    fn get_save_parent_cell(&self) -> *mut TESObjectCELL;
    fn get_sequence(&self, name: &str) -> *mut NiControllerSequence;
    fn get_submerge_level(&self, z_pos: f32, cell: *mut TESObjectCELL) -> f32;
    fn get_water_height(&self) -> f32;
    fn get_weight(&self) -> f32;
    fn get_weight_in_container(&self) -> f32;
    fn get_world_location(&self) -> BGSWorldLocation;
    fn get_worldspace(&self) -> *mut TESWorldSpace;
    fn has_collision(&self) -> bool;
    fn has_container(&self) -> bool;
    fn has_keyword(&self, keyword: *const BGSKeyword) -> bool;
    fn has_keyword_in_array(&self, keywords: &[*mut BGSKeyword], match_all: bool) -> bool;
    fn has_keyword_in_list(&self, keyword_list: *mut BGSListForm, match_all: bool) -> bool;
    fn has_keyword_with_type(&self, keyword_type: u32) -> bool;
    fn has_quest_object(&self) -> bool;
    fn is_3d_loaded(&self) -> bool;
    fn is_activation_blocked(&self) -> bool;
    fn is_animal(&self) -> bool;
    fn is_an_owner(
        &self,
        test_owner: *const Actor,
        use_faction: bool,
        requires_owner: bool,
    ) -> bool;
    fn is_crime_to_activate(&self) -> bool;
    fn is_disabled(&self) -> bool;
    fn is_dragon(&self) -> bool;
    fn is_enchanted(&self) -> bool;
    fn is_horse(&self) -> bool;
    fn is_humanoid(&self) -> bool;
    fn is_in_water(&self) -> bool;
    fn is_dead(&self, not_essential: bool) -> bool;
    fn is_initially_disabled(&self) -> bool;
    fn is_jewelry(&self) -> bool;
    fn is_locked(&self) -> bool;
    fn is_marked_for_deletion(&self) -> bool;
    fn is_off_limits(&self) -> bool;
    fn is_persistent(&self) -> bool;
    fn is_point_submerged_more_than(
        &self,
        pos: NiPoint3,
        cell: *mut TESObjectCELL,
        water_level: f32,
    ) -> bool;
    fn find_nearest_vertex(&self, minimum_offset: f32) -> Option<NiPoint3>;
    fn move_to(&mut self, target: *mut TESObjectREFR);
    fn move_to_editor_location_with(&mut self, position: NiPoint3, rotation: NiPoint3) -> bool;
    fn move_to_editor_location_current(&mut self) -> bool;
    fn move_to_nearest_navmesh(&mut self, minimum_offset: f32) -> bool;
    fn move_to_node_by_name(
        &mut self,
        target: *mut TESObjectREFR,
        node_name: &BSFixedString,
    ) -> bool;
    fn move_to_node(&mut self, target: *mut TESObjectREFR, node: *mut NiAVObject) -> bool;
    fn name_includes(&self, word: &str) -> bool;
    fn place_object_at_me(
        &self,
        base_to_place: *mut TESBoundObject,
        force_persist: bool,
    ) -> NiPointer<TESObjectREFR>;
    fn play_animation_by_name(&mut self, from: &str, to: &str);
    fn remove_item(
        &mut self,
        item: *mut TESBoundObject,
        count: i32,
        reason: ITEM_REMOVE_REASON,
        extra_list: *mut ExtraDataList,
        move_to_ref: *mut TESObjectREFR,
        drop_loc: *const NiPoint3,
        rotate: *const NiPoint3,
    ) -> ObjectRefHandle;
    fn process_animation_graph_event(
        &mut self,
        event: *const BSAnimationGraphEvent,
        event_source: *mut BSTEventSource<BSAnimationGraphEvent>,
    ) -> BSEventNotifyControl;
    fn remove_weapon(&mut self, equip_index: BIPED_OBJECT);
    fn move_havok(&mut self, force_rec: bool);
    fn get_linear_velocity(&self, velocity: &mut NiPoint3);
    fn disable(&mut self);
    fn reset_inventory(&mut self, leveled_only: bool);
    fn get_fire_node(&mut self) -> *mut NiNode;
    fn set_fire_node(&mut self, fire_node: *mut NiNode);
    fn as_explosion(&mut self) -> *mut Explosion;
    fn as_projectile(&mut self) -> *mut Projectile;
    fn on_add_cell_perform_queue_reference(&self, cell: &mut TESObjectCELL) -> bool;
    fn do_move_to_high(&mut self);
    fn try_move_to_middle_low(&mut self);
    fn try_change_sky_cell_actors_process_level(&mut self) -> bool;
    fn try_update_actor_last_seen_time(&mut self);
    fn create_anim_note_receiver(&mut self) -> *mut BSAnimNoteReceiver;
    fn get_anim_note_receiver(&mut self) -> *mut BSAnimNoteReceiver;
    fn process_in_water(
        &mut self,
        collidable: *mut hkpCollidable,
        water_height: f32,
        delta_time: f32,
    ) -> bool;
    fn apply_current(&mut self, velocity_time: f32, velocity: &hkVector4) -> bool;
    fn unequip_item(&mut self, arg1: u64, object: *mut TESBoundObject);
    fn set_activation_blocked(&mut self, blocked: bool);
    fn set_angle(&mut self, angle: &NiPoint3);
    fn set_biped(&mut self, biped: &BSTSmartPointer<BipedAnim>);
    fn set_collision(&mut self, enable: bool);
    fn set_display_name(&mut self, name: &BSFixedString, force: bool) -> bool;
    fn set_encounter_zone(&mut self, zone: *mut BGSEncounterZone);
    fn set_motion_type(&mut self, motion_type: hkpMotionMotionType, allow_activate: bool) -> bool;
    fn set_object_reference(&mut self, object: *mut TESBoundObject);
    fn set_owner(&mut self, owner: *mut TESForm);
    fn set_position(&mut self, pos: &NiPoint3);
    fn set_position_xyz(&mut self, x: f32, y: f32, z: f32);
    fn set_scale(&mut self, scale: f32);
    fn set_action_complete(&mut self, set: bool);
    fn set_movement_complete(&mut self, set: bool);
    fn get_template_actor_base(&mut self) -> *mut TESActorBase;
    fn set_template_actor_base(&mut self, template: *mut TESActorBase);
    fn set_temporary(&mut self);
    fn instantiate_hit_art(
        &self,
        art: *mut BGSArtObject,
        duration: f32,
        facing_ref: *mut TESObjectREFR,
        face_target: bool,
        attach_to_camera: bool,
        attach_node: *mut NiAVObject,
        interface_effect: bool,
    ) -> *mut ModelReferenceEffect;
    fn instantiate_hit_shader(
        &self,
        shader: *mut TESEffectShader,
        duration: f32,
        facing_ref: *mut TESObjectREFR,
        face_target: bool,
        attach_to_camera: bool,
        attach_node: *mut NiAVObject,
        interface_effect: bool,
    ) -> *mut ShaderReferenceEffect;
    fn is_child(&self) -> bool;
}

impl<T: AsRef<TESObjectREFR> + AsMut<TESObjectREFR>> TESObjectREFRExt for T {
    fn predestroy(&mut self) {
        TESObjectREFR::predestroy(self.as_mut())
    }

    fn enable(&mut self, reset_inventory: bool) {
        TESObjectREFR::enable(self.as_mut(), reset_inventory)
    }

    fn get_3d(&self) -> *mut NiAVObject {
        TESObjectREFR::get_3d(self.as_ref())
    }

    fn get_3d_with_view(&self, first_person: bool) -> *mut NiAVObject {
        TESObjectREFR::get_3d_with_view(self.as_ref(), first_person)
    }

    fn get_angle(&self) -> NiPoint3 {
        TESObjectREFR::get_angle(self.as_ref())
    }

    fn get_angle_x(&self) -> f32 {
        TESObjectREFR::get_angle_x(self.as_ref())
    }

    fn get_angle_y(&self) -> f32 {
        TESObjectREFR::get_angle_y(self.as_ref())
    }

    fn get_angle_z(&self) -> f32 {
        TESObjectREFR::get_angle_z(self.as_ref())
    }

    fn get_base_height(&self) -> f32 {
        TESObjectREFR::get_base_height(self.as_ref())
    }

    fn get_base_object(&self) -> *mut TESBoundObject {
        TESObjectREFR::get_base_object(self.as_ref())
    }

    fn get_biped(&self) -> &BSTSmartPointer<BipedAnim> {
        TESObjectREFR::get_biped(self.as_ref())
    }

    fn get_biped_with_view(&self, first_person: bool) -> &BSTSmartPointer<BipedAnim> {
        TESObjectREFR::get_biped_with_view(self.as_ref(), first_person)
    }

    fn get_current_biped(&self) -> &BSTSmartPointer<BipedAnim> {
        TESObjectREFR::get_current_biped(self.as_ref())
    }

    fn get_container(&self) -> *mut TESContainer {
        TESObjectREFR::get_container(self.as_ref())
    }

    fn get_current_location(&self) -> *mut BGSLocation {
        TESObjectREFR::get_current_location(self.as_ref())
    }

    fn get_reference_runtime_data(&self) -> &REFERENCE_RUNTIME_DATA {
        TESObjectREFR::get_reference_runtime_data(self.as_ref())
    }

    fn get_reference_runtime_data_mut(&mut self) -> &mut REFERENCE_RUNTIME_DATA {
        TESObjectREFR::get_reference_runtime_data_mut(self.as_mut())
    }

    fn get_magic_caster(&mut self, source: CastingSource) -> *mut MagicCaster {
        TESObjectREFR::get_magic_caster(self.as_mut(), source)
    }

    fn get_display_full_name(&self) -> *const c_char {
        TESObjectREFR::get_display_full_name(self.as_ref())
    }

    fn get_display_full_name_as_str(&self) -> &str {
        TESObjectREFR::get_display_full_name_as_str(self.as_ref())
    }

    fn get_distance(
        &self,
        other: *mut TESObjectREFR,
        disabled_refs: bool,
        ignore_worldspace: bool,
    ) -> f32 {
        TESObjectREFR::get_distance(self.as_ref(), other, disabled_refs, ignore_worldspace)
    }

    fn get_dropped_inventory(&self) -> TESObjectREFRInventoryDropMap {
        TESObjectREFR::get_dropped_inventory(self.as_ref())
    }

    fn get_editor_location(&self) -> *mut BGSLocation {
        TESObjectREFR::get_editor_location(self.as_ref())
    }

    fn get_editor_location_data(
        &mut self,
        out_pos: &mut NiPoint3,
        out_rot: &mut NiPoint3,
        out_world_or_cell: &mut *mut TESForm,
        fallback: *mut TESObjectCELL,
    ) -> bool {
        TESObjectREFR::get_editor_location_data(
            self.as_mut(),
            out_pos,
            out_rot,
            out_world_or_cell,
            fallback,
        )
    }

    fn get_enchantment(&self) -> *mut EnchantmentItem {
        TESObjectREFR::get_enchantment(self.as_ref())
    }

    fn get_enchantment_charge(&self) -> Option<f64> {
        TESObjectREFR::get_enchantment_charge(self.as_ref())
    }

    fn get_actor_owner(&self) -> *mut TESNPC {
        TESObjectREFR::get_actor_owner(self.as_ref())
    }

    fn get_face_node_skinned(&mut self) -> *mut BSFaceGenNiNode {
        TESObjectREFR::get_face_node_skinned(self.as_mut())
    }

    fn get_face_node(&mut self) -> *mut BSFaceGenNiNode {
        TESObjectREFR::get_face_node(self.as_mut())
    }

    fn get_face_gen_animation_data(&mut self) -> *mut BSFaceGenAnimationData {
        TESObjectREFR::get_face_gen_animation_data(self.as_mut())
    }

    fn get_faction_owner(&self) -> *mut crate::re::TESFaction {
        TESObjectREFR::get_faction_owner(self.as_ref())
    }

    fn get_handle(&self) -> ObjectRefHandle {
        TESObjectREFR::get_handle(self.as_ref())
    }

    fn get_heading_angle(&self, pos: &NiPoint3, abs: bool) -> f32 {
        TESObjectREFR::get_heading_angle(self.as_ref(), pos, abs)
    }

    fn get_height(&self) -> f32 {
        TESObjectREFR::get_height(self.as_ref())
    }

    fn get_inventory(&mut self) -> TESObjectREFRInventoryItemMap {
        TESObjectREFR::get_inventory(self.as_mut())
    }

    fn get_inventory_count(&mut self, no_init: bool) -> i32 {
        TESObjectREFR::get_inventory_count(self.as_mut(), no_init)
    }

    fn get_inventory_counts(&mut self) -> TESObjectREFRInventoryCountMap {
        TESObjectREFR::get_inventory_counts(self.as_mut())
    }

    fn get_inventory_changes(&mut self, no_init: bool) -> *mut InventoryChanges {
        TESObjectREFR::get_inventory_changes(self.as_mut(), no_init)
    }

    fn get_linked_ref(&self, keyword: *mut BGSKeyword) -> *mut TESObjectREFR {
        TESObjectREFR::get_linked_ref(self.as_ref(), keyword)
    }

    fn get_lock(&self) -> *mut REFR_LOCK {
        TESObjectREFR::get_lock(self.as_ref())
    }

    fn get_lock_level(&self) -> LOCK_LEVEL {
        TESObjectREFR::get_lock_level(self.as_ref())
    }

    fn get_magic_target(&mut self) -> *mut MagicTarget {
        TESObjectREFR::get_magic_target(self.as_mut())
    }

    fn get_name(&self) -> *const c_char {
        TESObjectREFR::get_name(self.as_ref())
    }

    fn get_name_as_str(&self) -> &str {
        TESObjectREFR::get_name_as_str(self.as_ref())
    }

    fn get_node_by_name(&self, node_name: &BSFixedString) -> *mut NiAVObject {
        TESObjectREFR::get_node_by_name(self.as_ref(), node_name)
    }

    fn get_object_reference(&self) -> *mut TESBoundObject {
        TESObjectREFR::get_object_reference(self.as_ref())
    }

    fn get_owner(&self) -> *mut TESForm {
        TESObjectREFR::get_owner(self.as_ref())
    }

    fn get_parent_cell(&self) -> *mut TESObjectCELL {
        TESObjectREFR::get_parent_cell(self.as_ref())
    }

    fn get_position(&self) -> NiPoint3 {
        TESObjectREFR::get_position(self.as_ref())
    }

    fn get_current_3d(&self) -> *mut NiAVObject {
        TESObjectREFR::get_current_3d(self.as_ref())
    }

    fn get_current_ammo(&self) -> *mut TESAmmo {
        TESObjectREFR::get_current_ammo(self.as_ref())
    }

    fn get_decal_group(&self) -> *mut BGSDecalGroup {
        TESObjectREFR::get_decal_group(self.as_ref())
    }

    fn get_scale(&self) -> f32 {
        TESObjectREFR::get_scale(self.as_ref())
    }

    fn get_save_parent_cell(&self) -> *mut TESObjectCELL {
        TESObjectREFR::get_save_parent_cell(self.as_ref())
    }

    fn get_sequence(&self, name: &str) -> *mut NiControllerSequence {
        TESObjectREFR::get_sequence(self.as_ref(), name)
    }

    fn get_submerge_level(&self, z_pos: f32, cell: *mut TESObjectCELL) -> f32 {
        TESObjectREFR::get_submerge_level(self.as_ref(), z_pos, cell)
    }

    fn get_water_height(&self) -> f32 {
        TESObjectREFR::get_water_height(self.as_ref())
    }

    fn get_weight(&self) -> f32 {
        TESObjectREFR::get_weight(self.as_ref())
    }

    fn get_weight_in_container(&self) -> f32 {
        TESObjectREFR::get_weight_in_container(self.as_ref())
    }

    fn get_world_location(&self) -> BGSWorldLocation {
        TESObjectREFR::get_world_location(self.as_ref())
    }

    fn get_worldspace(&self) -> *mut TESWorldSpace {
        TESObjectREFR::get_worldspace(self.as_ref())
    }

    fn has_collision(&self) -> bool {
        TESObjectREFR::has_collision(self.as_ref())
    }

    fn has_container(&self) -> bool {
        TESObjectREFR::has_container(self.as_ref())
    }

    fn has_keyword(&self, keyword: *const BGSKeyword) -> bool {
        TESObjectREFR::has_keyword(self.as_ref(), keyword)
    }

    fn has_keyword_in_array(&self, keywords: &[*mut BGSKeyword], match_all: bool) -> bool {
        TESObjectREFR::has_keyword_in_array(self.as_ref(), keywords, match_all)
    }

    fn has_keyword_in_list(&self, keyword_list: *mut BGSListForm, match_all: bool) -> bool {
        TESObjectREFR::has_keyword_in_list(self.as_ref(), keyword_list, match_all)
    }

    fn has_keyword_with_type(&self, keyword_type: u32) -> bool {
        TESObjectREFR::has_keyword_with_type(self.as_ref(), keyword_type)
    }

    fn has_quest_object(&self) -> bool {
        TESObjectREFR::has_quest_object(self.as_ref())
    }

    fn is_3d_loaded(&self) -> bool {
        TESObjectREFR::is_3d_loaded(self.as_ref())
    }

    fn is_activation_blocked(&self) -> bool {
        TESObjectREFR::is_activation_blocked(self.as_ref())
    }

    fn is_animal(&self) -> bool {
        TESObjectREFR::is_animal(self.as_ref())
    }

    fn is_an_owner(
        &self,
        test_owner: *const Actor,
        use_faction: bool,
        requires_owner: bool,
    ) -> bool {
        TESObjectREFR::is_an_owner(self.as_ref(), test_owner, use_faction, requires_owner)
    }

    fn is_crime_to_activate(&self) -> bool {
        TESObjectREFR::is_crime_to_activate(self.as_ref())
    }

    fn is_disabled(&self) -> bool {
        TESObjectREFR::is_disabled(self.as_ref())
    }

    fn is_dragon(&self) -> bool {
        TESObjectREFR::is_dragon(self.as_ref())
    }

    fn is_enchanted(&self) -> bool {
        TESObjectREFR::is_enchanted(self.as_ref())
    }

    fn is_horse(&self) -> bool {
        TESObjectREFR::is_horse(self.as_ref())
    }

    fn is_humanoid(&self) -> bool {
        TESObjectREFR::is_humanoid(self.as_ref())
    }

    fn is_in_water(&self) -> bool {
        TESObjectREFR::is_in_water(self.as_ref())
    }

    fn is_dead(&self, not_essential: bool) -> bool {
        TESObjectREFR::is_dead(self.as_ref(), not_essential)
    }

    fn is_initially_disabled(&self) -> bool {
        TESObjectREFR::is_initially_disabled(self.as_ref())
    }

    fn is_jewelry(&self) -> bool {
        TESObjectREFR::is_jewelry(self.as_ref())
    }

    fn is_locked(&self) -> bool {
        TESObjectREFR::is_locked(self.as_ref())
    }

    fn is_marked_for_deletion(&self) -> bool {
        TESObjectREFR::is_marked_for_deletion(self.as_ref())
    }

    fn is_off_limits(&self) -> bool {
        TESObjectREFR::is_off_limits(self.as_ref())
    }

    fn is_persistent(&self) -> bool {
        TESObjectREFR::is_persistent(self.as_ref())
    }

    fn is_point_submerged_more_than(
        &self,
        pos: NiPoint3,
        cell: *mut TESObjectCELL,
        water_level: f32,
    ) -> bool {
        TESObjectREFR::is_point_submerged_more_than(self.as_ref(), pos, cell, water_level)
    }

    fn find_nearest_vertex(&self, minimum_offset: f32) -> Option<NiPoint3> {
        TESObjectREFR::find_nearest_vertex(self.as_ref(), minimum_offset)
    }

    fn move_to(&mut self, target: *mut TESObjectREFR) {
        TESObjectREFR::move_to(self.as_mut(), target)
    }

    fn move_to_editor_location_with(&mut self, position: NiPoint3, rotation: NiPoint3) -> bool {
        TESObjectREFR::move_to_editor_location_with(self.as_mut(), position, rotation)
    }

    fn move_to_editor_location_current(&mut self) -> bool {
        TESObjectREFR::move_to_editor_location_current(self.as_mut())
    }

    fn move_to_nearest_navmesh(&mut self, minimum_offset: f32) -> bool {
        TESObjectREFR::move_to_nearest_navmesh(self.as_mut(), minimum_offset)
    }

    fn move_to_node_by_name(
        &mut self,
        target: *mut TESObjectREFR,
        node_name: &BSFixedString,
    ) -> bool {
        TESObjectREFR::move_to_node_by_name(self.as_mut(), target, node_name)
    }

    fn move_to_node(&mut self, target: *mut TESObjectREFR, node: *mut NiAVObject) -> bool {
        TESObjectREFR::move_to_node(self.as_mut(), target, node)
    }

    fn name_includes(&self, word: &str) -> bool {
        TESObjectREFR::name_includes(self.as_ref(), word)
    }

    fn place_object_at_me(
        &self,
        base_to_place: *mut TESBoundObject,
        force_persist: bool,
    ) -> NiPointer<TESObjectREFR> {
        TESObjectREFR::place_object_at_me(self.as_ref(), base_to_place, force_persist)
    }

    fn play_animation_by_name(&mut self, from: &str, to: &str) {
        TESObjectREFR::play_animation_by_name(self.as_mut(), from, to)
    }

    fn remove_item(
        &mut self,
        item: *mut TESBoundObject,
        count: i32,
        reason: ITEM_REMOVE_REASON,
        extra_list: *mut ExtraDataList,
        move_to_ref: *mut TESObjectREFR,
        drop_loc: *const NiPoint3,
        rotate: *const NiPoint3,
    ) -> ObjectRefHandle {
        TESObjectREFR::remove_item(
            self.as_mut(),
            item,
            count,
            reason,
            extra_list,
            move_to_ref,
            drop_loc,
            rotate,
        )
    }

    fn process_animation_graph_event(
        &mut self,
        event: *const BSAnimationGraphEvent,
        event_source: *mut BSTEventSource<BSAnimationGraphEvent>,
    ) -> BSEventNotifyControl {
        TESObjectREFR::process_animation_graph_event(self.as_mut(), event, event_source)
    }

    fn remove_weapon(&mut self, equip_index: BIPED_OBJECT) {
        TESObjectREFR::remove_weapon(self.as_mut(), equip_index)
    }

    fn move_havok(&mut self, force_rec: bool) {
        TESObjectREFR::move_havok(self.as_mut(), force_rec)
    }

    fn get_linear_velocity(&self, velocity: &mut NiPoint3) {
        TESObjectREFR::get_linear_velocity(self.as_ref(), velocity)
    }

    fn disable(&mut self) {
        TESObjectREFR::disable(self.as_mut())
    }

    fn reset_inventory(&mut self, leveled_only: bool) {
        TESObjectREFR::reset_inventory(self.as_mut(), leveled_only)
    }

    fn get_fire_node(&mut self) -> *mut NiNode {
        TESObjectREFR::get_fire_node(self.as_mut())
    }

    fn set_fire_node(&mut self, fire_node: *mut NiNode) {
        TESObjectREFR::set_fire_node(self.as_mut(), fire_node)
    }

    fn as_explosion(&mut self) -> *mut Explosion {
        TESObjectREFR::as_explosion(self.as_mut())
    }

    fn as_projectile(&mut self) -> *mut Projectile {
        TESObjectREFR::as_projectile(self.as_mut())
    }

    fn on_add_cell_perform_queue_reference(&self, cell: &mut TESObjectCELL) -> bool {
        TESObjectREFR::on_add_cell_perform_queue_reference(self.as_ref(), cell)
    }

    fn do_move_to_high(&mut self) {
        TESObjectREFR::do_move_to_high(self.as_mut())
    }

    fn try_move_to_middle_low(&mut self) {
        TESObjectREFR::try_move_to_middle_low(self.as_mut())
    }

    fn try_change_sky_cell_actors_process_level(&mut self) -> bool {
        TESObjectREFR::try_change_sky_cell_actors_process_level(self.as_mut())
    }

    fn try_update_actor_last_seen_time(&mut self) {
        TESObjectREFR::try_update_actor_last_seen_time(self.as_mut())
    }

    fn create_anim_note_receiver(&mut self) -> *mut BSAnimNoteReceiver {
        TESObjectREFR::create_anim_note_receiver(self.as_mut())
    }

    fn get_anim_note_receiver(&mut self) -> *mut BSAnimNoteReceiver {
        TESObjectREFR::get_anim_note_receiver(self.as_mut())
    }

    fn process_in_water(
        &mut self,
        collidable: *mut hkpCollidable,
        water_height: f32,
        delta_time: f32,
    ) -> bool {
        TESObjectREFR::process_in_water(self.as_mut(), collidable, water_height, delta_time)
    }

    fn apply_current(&mut self, velocity_time: f32, velocity: &hkVector4) -> bool {
        TESObjectREFR::apply_current(self.as_mut(), velocity_time, velocity)
    }

    fn unequip_item(&mut self, arg1: u64, object: *mut TESBoundObject) {
        TESObjectREFR::unequip_item(self.as_mut(), arg1, object)
    }

    fn set_activation_blocked(&mut self, blocked: bool) {
        TESObjectREFR::set_activation_blocked(self.as_mut(), blocked)
    }

    fn set_angle(&mut self, angle: &NiPoint3) {
        TESObjectREFR::set_angle(self.as_mut(), angle)
    }

    fn set_biped(&mut self, biped: &BSTSmartPointer<BipedAnim>) {
        TESObjectREFR::set_biped(self.as_mut(), biped)
    }

    fn set_collision(&mut self, enable: bool) {
        TESObjectREFR::set_collision(self.as_mut(), enable)
    }

    fn set_display_name(&mut self, name: &BSFixedString, force: bool) -> bool {
        TESObjectREFR::set_display_name(self.as_mut(), name, force)
    }

    fn set_encounter_zone(&mut self, zone: *mut BGSEncounterZone) {
        TESObjectREFR::set_encounter_zone(self.as_mut(), zone)
    }

    fn set_motion_type(&mut self, motion_type: hkpMotionMotionType, allow_activate: bool) -> bool {
        TESObjectREFR::set_motion_type(self.as_mut(), motion_type, allow_activate)
    }

    fn set_object_reference(&mut self, object: *mut TESBoundObject) {
        TESObjectREFR::set_object_reference(self.as_mut(), object)
    }

    fn set_owner(&mut self, owner: *mut TESForm) {
        TESObjectREFR::set_owner(self.as_mut(), owner)
    }

    fn set_position(&mut self, pos: &NiPoint3) {
        TESObjectREFR::set_position(self.as_mut(), pos)
    }

    fn set_position_xyz(&mut self, x: f32, y: f32, z: f32) {
        TESObjectREFR::set_position_xyz(self.as_mut(), x, y, z)
    }

    fn set_scale(&mut self, scale: f32) {
        TESObjectREFR::set_scale(self.as_mut(), scale)
    }

    fn set_action_complete(&mut self, set: bool) {
        TESObjectREFR::set_action_complete(self.as_mut(), set)
    }

    fn set_movement_complete(&mut self, set: bool) {
        TESObjectREFR::set_movement_complete(self.as_mut(), set)
    }

    fn get_template_actor_base(&mut self) -> *mut TESActorBase {
        TESObjectREFR::get_template_actor_base(self.as_mut())
    }

    fn set_template_actor_base(&mut self, template: *mut TESActorBase) {
        TESObjectREFR::set_template_actor_base(self.as_mut(), template)
    }

    fn set_temporary(&mut self) {
        TESObjectREFR::set_temporary(self.as_mut())
    }

    fn instantiate_hit_art(
        &self,
        art: *mut BGSArtObject,
        duration: f32,
        facing_ref: *mut TESObjectREFR,
        face_target: bool,
        attach_to_camera: bool,
        attach_node: *mut NiAVObject,
        interface_effect: bool,
    ) -> *mut ModelReferenceEffect {
        TESObjectREFR::instantiate_hit_art(
            self.as_ref(),
            art,
            duration,
            facing_ref,
            face_target,
            attach_to_camera,
            attach_node,
            interface_effect,
        )
    }

    fn instantiate_hit_shader(
        &self,
        shader: *mut TESEffectShader,
        duration: f32,
        facing_ref: *mut TESObjectREFR,
        face_target: bool,
        attach_to_camera: bool,
        attach_node: *mut NiAVObject,
        interface_effect: bool,
    ) -> *mut ShaderReferenceEffect {
        TESObjectREFR::instantiate_hit_shader(
            self.as_ref(),
            shader,
            duration,
            facing_ref,
            face_target,
            attach_to_camera,
            attach_node,
            interface_effect,
        )
    }

    fn is_child(&self) -> bool {
        TESObjectREFR::is_child(self.as_ref())
    }
}

unsafe extern "C" {
    fn atan2f(y: f32, x: f32) -> f32;
}
