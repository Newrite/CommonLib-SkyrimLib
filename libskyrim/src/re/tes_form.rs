use core_util::inherit;
use bitflags::bitflags;

use crate::re::base_form_component::BaseFormComponent;
use crate::re::form_type::FormType;
use crate::relocation::{VariantID, RttiType};

use crate::offsets::offsets_rtti::RTTI_TESForm;
use crate::offsets::offsets_vtable::VTABLE_TESForm;

// Dependencies
use crate::re::bgs_load_form_buffer::BGSLoadFormBuffer;
use crate::re::bgs_save_form_buffer::BGSSaveFormBuffer;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::form::Form;
use crate::re::form::FormGroup;
use crate::re::ni_t_pointer_map::NiTPointerMap;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_file::TESFile;
use crate::re::tes_object_refr::TESObjectREFR;

pub type FormID = u32;

bitflags! {
    /// C++ `RE::TESForm::ChangeFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChangeFlags: u32 {
        const CREATED = 0;
        const FLAGS = 1 << 0;
    }
}

bitflags! {
    /// C++ `RE::TESForm::RecordFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RecordFlags: u32 {
        const DESTRUCTIBLE = 1 << 0;
        const MASTER = 1 << 0;
        const UNLOCKED = 1 << 0;
        const ALTERED = 1 << 1;
        const NON_PLAYABLE = 1 << 2;
        const INITIALIZED = 1 << 3;
        const NON_OCCLUDER = 1 << 4;
        const DELETED = 1 << 5;
        const BORDER_REGION = 1 << 6;
        const GLOBAL_CONSTANT = 1 << 6;
        const HAS_SPOKEN_FLAG = 1 << 6;
        const KNOWN = 1 << 6;
        const IN_PLACEABLE_WATER = 1 << 6;
        const FIRE_OFF = 1 << 7;
        const MUST_UPDATE = 1 << 8;
        const ON_LOCAL_MAP = 1 << 9;
        const PERSISTENT = 1 << 10;
        const DISABLED = 1 << 11;
        const USED_AS_MOVING_PLATFORM = 1 << 11;
        const IGNORED = 1 << 12;
        const EMPTY = 1 << 13;
        const RESET_DESTRUCTION = 1 << 13;
        const TEMPORARY = 1 << 14;
        const VISIBLE_WHEN_DISTANT = 1 << 15;
        const RANDOM_ANIM = 1 << 16;
        const DANGEROUS = 1 << 17;
        const HAS_CURRENTS = 1 << 19;
        const IGNORE_FRIENDLY_HITS = 1 << 20;
        const STILL_LOADING = 1 << 21;
        const FORM_RETAINS_ID = 1 << 22;
        const DESTROYED = 1 << 23;
        const UNK24 = 1 << 24;
        const NO_AI_ACQUIRE = 1 << 25;
        const OBSTACLE = 1 << 25;
        const VATS_TARGET_OVERRIDE = 1 << 26;
        const DISABLE_FADE = 1 << 27;
        const REFLECTED_BY_AUTO_WATER = 1 << 28;
        const SHOW_ON_WORLD_MAP = 1 << 28;
        const CHILD_CAN_USE = 1 << 29;
    }
}

bitflags! {
    /// C++ `RE::TESForm::InGameFormFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct InGameFormFlag: u16 {
        const NONE = 0;
        const WANTS_DELETE = 1 << 0;
        const FORCED_PERSISTENT = 1 << 1;
        const NO_FAVOR_ALLOWED = 1 << 4;
        const IS_SKY_OBJECT = 1 << 5;
        const REF_ORIGINAL_PERSISTENT = 1 << 6;
        const REF_PERMANENTLY_DELETED = 1 << 7;
    }
}

/// C++ `RE::TESFileArray : public BSStaticArray<TESFile*>`
#[repr(C)]
pub struct TESFileArray {
    pub ptr: *mut *mut TESFile,
    pub size: u32,
    pub pad0c: u32,
}

const _: () = assert!(core::mem::size_of::<TESFileArray>() == 0x10);

/// C++ `RE::TESFileContainer`
#[repr(C)]
pub struct TESFileContainer {
    pub array: *mut TESFileArray,
}

const _: () = assert!(core::mem::size_of::<TESFileContainer>() == 0x8);

/// C++ `RE::TESForm`
#[repr(C)]
pub struct TESForm {
    pub base: BaseFormComponent,            // 00
    pub source_files: TESFileContainer,     // 08
    pub form_flags: RecordFlags,            // 10
    pub form_id: FormID,                    // 14
    pub in_game_form_flags: InGameFormFlag, // 18
    pub form_type: u8,                      // 1A (C++ uses EnumSet<FormType, uint8_t>)
    pub pad1b: u8,                          // 1B
    pub pad1c: u32,                         // 1C
}

const _: () = assert!(core::mem::size_of::<TESForm>() == 0x20);

impl RttiType for TESForm {
    const RTTI: VariantID = RTTI_TESForm;
}

inherit!(TESForm : BaseFormComponent);

use core::ffi::c_char;
use crate::re::form_traits::FormCastable;
use crate::virtual_method;

impl TESForm {
    pub const RTTI: VariantID = RTTI_TESForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESForm;

    #[inline(always)]
    pub fn get_form_type(&self) -> FormType {
        // Safe to transmute or cast because Enum is repr(u32) but values fit in u8
        unsafe { core::mem::transmute::<u32, FormType>(self.form_type as u32) }
    }

    // ─── Virtual Methods ───────────────────────────────────────────────────

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }
    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA_COMPONENT: usize = 0x01;
        pub fn initialize_data_component()
    }
    virtual_method! {
        pub const VFUNC_CLEAR_DATA_COMPONENT: usize = 0x02;
        pub fn clear_data_component()
    }
    virtual_method! {
        pub const VFUNC_COPY_COMPONENT: usize = 0x03;
        pub fn copy_component(rhs: *mut BaseFormComponent)
    }
    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA: usize = 0x04;
        pub fn initialize_data()
    }
    virtual_method! {
        pub const VFUNC_CLEAR_DATA: usize = 0x05;
        pub fn clear_data()
    }
    virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(mod_file: *mut TESFile) -> bool
    }
    virtual_method! {
        pub const VFUNC_LOAD_PARTIAL: usize = 0x07;
        pub fn load_partial(mod_file: *mut TESFile) -> bool
    }
    virtual_method! {
        pub const VFUNC_LOAD_EDIT: usize = 0x08;
        pub fn load_edit(mod_file: *mut TESFile) -> bool
    }
    virtual_method! {
        pub const VFUNC_CREATE_DUPLICATE_FORM: usize = 0x09;
        pub fn create_duplicate_form(create_editor_id: bool, copy_map: *mut NiTPointerMap) -> *mut TESForm
    }
    virtual_method! {
        pub const VFUNC_ADD_CHANGE: usize = 0x0A;
        pub fn add_change(change_flags: u32) -> bool
    }
    virtual_method! {
        pub const VFUNC_REMOVE_CHANGE: usize = 0x0B;
        pub fn remove_change(change_flags: u32)
    }
    virtual_method! {
        pub const VFUNC_FIND_IN_FILE_FAST: usize = 0x0C;
        pub fn find_in_file_fast(mod_file: *mut TESFile) -> bool
    }
    virtual_method! {
        pub const VFUNC_CHECK_SAVE_GAME: usize = 0x0D;
        pub fn check_save_game(buf: *mut BGSSaveFormBuffer) -> bool
    }
    virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x0E;
        pub fn save_game(buf: *mut BGSSaveFormBuffer)
    }
    virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x0F;
        pub fn load_game(buf: *mut BGSLoadFormBuffer)
    }
    virtual_method! {
        pub const VFUNC_INIT_LOAD_GAME: usize = 0x10;
        pub fn init_load_game(buf: *mut BGSLoadFormBuffer)
    }
    virtual_method! {
        pub const VFUNC_FINISH_LOAD_GAME: usize = 0x11;
        pub fn finish_load_game(buf: *mut BGSLoadFormBuffer)
    }
    virtual_method! {
        pub const VFUNC_REVERT: usize = 0x12;
        pub fn revert(buf: *mut BGSLoadFormBuffer)
    }
    virtual_method! {
        pub const VFUNC_INIT_ITEM_IMPL: usize = 0x13;
        pub fn init_item_impl()
    }
    virtual_method! {
        pub const VFUNC_GET_DESCRIPTION_OWNER_FILE: usize = 0x14;
        pub fn get_description_owner_file() -> *mut TESFile
    }
    virtual_method! {
        pub const VFUNC_GET_SAVED_FORM_TYPE: usize = 0x15;
        pub fn get_saved_form_type() -> FormType
    }
    virtual_method! {
        pub const VFUNC_GET_FORM_DETAILED_STRING: usize = 0x16;
        pub fn get_form_detailed_string(buf: *mut c_char, buf_len: u32)
    }
    virtual_method! {
        pub const VFUNC_GET_KNOWN: usize = 0x17;
        pub fn get_known() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_RANDOM_ANIM: usize = 0x18;
        pub fn get_random_anim() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_PLAYABLE: usize = 0x19;
        pub fn get_playable() -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_HEADING_MARKER: usize = 0x1A;
        pub fn is_heading_marker() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_DANGEROUS: usize = 0x1B;
        pub fn get_dangerous() -> bool
    }
    virtual_method! {
        pub const VFUNC_Q_HAS_CURRENTS: usize = 0x1C;
        pub fn q_has_currents() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_OBSTACLE: usize = 0x1D;
        pub fn get_obstacle() -> bool
    }
    virtual_method! {
        pub const VFUNC_Q_IS_LOD_LAND_OBJECT: usize = 0x1E;
        pub fn q_is_lod_land_object() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_ON_LOCAL_MAP: usize = 0x1F;
        pub fn get_on_local_map() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_MUST_UPDATE: usize = 0x20;
        pub fn get_must_update() -> bool
    }
    virtual_method! {
        pub const VFUNC_SET_ON_LOCAL_MAP: usize = 0x21;
        pub fn set_on_local_map(set: bool)
    }
    virtual_method! {
        pub const VFUNC_GET_IGNORED_BY_SANDBOX: usize = 0x22;
        pub fn get_ignored_by_sandbox() -> bool
    }
    virtual_method! {
        pub const VFUNC_SET_DELETE: usize = 0x23;
        pub fn set_delete(set: bool)
    }
    virtual_method! {
        pub const VFUNC_SET_ALTERED: usize = 0x24;
        pub fn set_altered(set: bool)
    }
    virtual_method! {
        pub const VFUNC_SAVE_OBJECT_BOUND: usize = 0x25;
        pub fn save_object_bound()
    }
    virtual_method! {
        pub const VFUNC_LOAD_OBJECT_BOUND: usize = 0x26;
        pub fn load_object_bound(mod_file: *mut TESFile)
    }
    virtual_method! {
        pub const VFUNC_IS_BOUND_OBJECT: usize = 0x27;
        pub fn is_bound_object() -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_OBJECT: usize = 0x28;
        pub fn is_object() -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_MAGIC_ITEM: usize = 0x29;
        pub fn is_magic_item() -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_WATER: usize = 0x2A;
        pub fn is_water() -> bool
    }
    virtual_method! {
        pub const VFUNC_AS_REFERENCE1: usize = 0x2B;
        pub fn as_reference1() -> *mut TESObjectREFR
    }
    virtual_method! {
        pub const VFUNC_AS_REFERENCE2: usize = 0x2C;
        pub fn as_reference2() -> *const TESObjectREFR
    }
    virtual_method! {
        pub const VFUNC_GET_REF_COUNT: usize = 0x2D;
        pub fn get_ref_count() -> u32
    }
    virtual_method! {
        pub const VFUNC_GET_TEXT_FOR_PARSED_SUB_TAG: usize = 0x2E;
        pub fn get_text_for_parsed_sub_tag(tag: *const BSFixedString) -> *const c_char
    }
    virtual_method! {
        pub const VFUNC_COPY: usize = 0x2F;
        pub fn copy(src_form: *mut TESForm)
    }
    virtual_method! {
        pub const VFUNC_BELONGS_IN_GROUP: usize = 0x30;
        pub fn belongs_in_group(form: *mut Form, allow_parent_groups: bool, current_only: bool) -> bool
    }
    virtual_method! {
        pub const VFUNC_CREATE_GROUP_DATA: usize = 0x31;
        pub fn create_group_data(form: *mut Form, group: *mut FormGroup)
    }
    virtual_method! {
        pub const VFUNC_GET_FORM_EDITOR_ID: usize = 0x32;
        pub fn get_form_editor_id() -> *const c_char
    }
    virtual_method! {
        pub const VFUNC_SET_FORM_EDITOR_ID: usize = 0x33;
        pub fn set_form_editor_id(str: *const c_char) -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_PARENT_FORM: usize = 0x34;
        pub fn is_parent_form() -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_PARENT_FORM_TREE: usize = 0x35;
        pub fn is_parent_form_tree() -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_FORM_TYPE_CHILD: usize = 0x36;
        pub fn is_form_type_child(form_type: FormType) -> bool
    }
    virtual_method! {
        pub const VFUNC_ACTIVATE: usize = 0x37;
        pub fn activate(target_ref: *mut TESObjectREFR, activator_ref: *mut TESObjectREFR, arg3: u8, object: *mut TESBoundObject, target_count: i32) -> bool
    }
    virtual_method! {
        pub const VFUNC_SET_FORM_ID: usize = 0x38;
        pub fn set_form_id(id: FormID, update_file: bool)
    }
    virtual_method! {
        pub const VFUNC_GET_OBJECT_TYPE_NAME: usize = 0x39;
        pub fn get_object_type_name() -> *const c_char
    }
    virtual_method! {
        pub const VFUNC_Q_AVAILABLE_IN_GAME: usize = 0x3A;
        pub fn q_available_in_game() -> bool
    }

    // ─── Non-Virtual Accessors and Helpers ─────────────────────────────────

    #[inline(always)]
    pub fn get_form_flags(&self) -> RecordFlags {
        self.form_flags
    }

    #[inline(always)]
    pub fn get_form_id(&self) -> FormID {
        self.form_id
    }

    #[inline(always)]
    pub fn is(&self, form_type: FormType) -> bool {
        self.get_form_type() == form_type
    }

    #[inline(always)]
    pub fn is_not(&self, form_type: FormType) -> bool {
        self.get_form_type() != form_type
    }

    #[inline(always)]
    pub fn is_actor(&self) -> bool {
        self.is(FormType::ActorCharacter)
    }

    #[inline(always)]
    pub fn is_ammo(&self) -> bool {
        self.is(FormType::Ammo)
    }

    #[inline(always)]
    pub fn is_armor(&self) -> bool {
        self.is(FormType::Armor)
    }

    #[inline(always)]
    pub fn is_book(&self) -> bool {
        self.is(FormType::Book)
    }

    #[inline(always)]
    pub fn is_deleted(&self) -> bool {
        self.form_flags.contains(RecordFlags::DELETED)
    }

    #[inline(always)]
    pub fn is_destroyed(&self) -> bool {
        self.form_flags.contains(RecordFlags::DESTROYED)
    }

    #[inline(always)]
    pub fn is_dynamic_form(&self) -> bool {
        self.form_id >= 0xFF000000
    }

    #[inline(always)]
    pub fn is_gold(&self) -> bool {
        self.form_id == 0x0000000F
    }

    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        self.form_flags.contains(RecordFlags::IGNORED)
    }

    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.form_flags.contains(RecordFlags::INITIALIZED)
    }

    #[inline(always)]
    pub fn is_key(&self) -> bool {
        self.is(FormType::KeyMaster)
    }

    #[inline(always)]
    pub fn is_lockpick(&self) -> bool {
        self.form_id == 0x0000000A
    }

    #[inline(always)]
    pub fn is_note(&self) -> bool {
        self.is(FormType::Note)
    }

    #[inline(always)]
    pub fn is_player(&self) -> bool {
        self.form_id == 0x00000007
    }

    #[inline(always)]
    pub fn is_player_ref(&self) -> bool {
        self.form_id == 0x00000014
    }

    #[inline(always)]
    pub fn is_skooma(&self) -> bool {
        self.form_id == 0x00057A7A || self.form_id == 0x0201391D
    }

    #[inline(always)]
    pub fn is_soul_gem(&self) -> bool {
        self.is(FormType::SoulGem)
    }

    #[inline(always)]
    pub fn is_weapon(&self) -> bool {
        self.is(FormType::Weapon)
    }

    #[inline(always)]
    pub fn as_reference(&mut self) -> *mut TESObjectREFR {
        self.as_reference1()
    }

    #[inline(always)]
    pub fn as_reference_const(&self) -> *const TESObjectREFR {
        self.as_reference2()
    }

    // ─── Relocated Engine Functions ────────────────────────────────────────

    crate::relocation_func! {
        pub fn add_compile_index(id: *mut FormID, file: *mut TESFile) => VariantID::new(14509, 14667, 0)
    }

    crate::relocation_func! {
        pub fn get_weight(this: &TESForm) -> f32 => VariantID::new(14809, 14988, 0)
    }

    crate::relocation_func! {
        pub fn set_file(this: &TESForm, file: *mut TESFile) => VariantID::new(14467, 14623, 0)
    }

    crate::relocation_func! {
        pub fn set_player_knows(this: &TESForm, known: bool) => VariantID::new(14482, 14639, 0)
    }

    // ─── Global Variables (Maps) ───────────────────────────────────────────

    crate::relocation_variable! {
        pub fn get_all_forms_map() -> &'static *mut crate::re::bst_hash_map::BSTHashMap<FormID, *mut TESForm> => VariantID::new(514351, 400507, 0)
    }

    crate::relocation_variable! {
        pub fn get_all_forms_map_lock() -> &'static *mut crate::re::bs_read_write_lock::BSReadWriteLock => VariantID::new(514360, 400517, 0)
    }

    crate::relocation_variable! {
        pub fn get_all_forms_by_editor_id_map() -> &'static *mut crate::re::bst_hash_map::BSTHashMap<BSFixedString, *mut TESForm> => VariantID::new(514352, 400509, 0)
    }

    crate::relocation_variable! {
        pub fn get_all_forms_editor_id_map_lock() -> &'static *mut crate::re::bs_read_write_lock::BSReadWriteLock => VariantID::new(514361, 400518, 0)
    }

    #[inline]
    pub fn get_form_editor_id_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_form_editor_id())
    }

    #[inline]
    pub fn get_object_type_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_object_type_name())
    }
}

pub trait TESFormExt {
    fn get_form_type(&self) -> FormType;
    fn get_form_flags(&self) -> RecordFlags;
    fn get_form_id(&self) -> FormID;
    fn is(&self, form_type: FormType) -> bool;
    fn is_deleted(&self) -> bool;
    fn get_weight(&self) -> f32;
    fn set_player_knows(&self, known: bool);

    fn get_form_editor_id_as_str(&self) -> &str;
    fn get_object_type_name_as_str(&self) -> &str;

    // Selected virtuals
    fn initialize_data(&mut self);
    fn clear_data(&mut self);
    fn load(&mut self, mod_file: *mut TESFile) -> bool;
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer);
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn activate(&mut self, target_ref: *mut TESObjectREFR, activator_ref: *mut TESObjectREFR, arg3: u8, object: *mut TESBoundObject, target_count: i32) -> bool;
}

impl<T: AsRef<TESForm> + AsMut<TESForm>> TESFormExt for T {
    fn get_form_type(&self) -> FormType {
        self.as_ref().get_form_type()
    }

    fn get_form_flags(&self) -> RecordFlags {
        self.as_ref().get_form_flags()
    }

    fn get_form_id(&self) -> FormID {
        self.as_ref().get_form_id()
    }

    fn is(&self, form_type: FormType) -> bool {
        self.as_ref().is(form_type)
    }

    fn is_deleted(&self) -> bool {
        self.as_ref().is_deleted()
    }

    fn get_weight(&self) -> f32 {
        TESForm::get_weight(self.as_ref())
    }

    fn set_player_knows(&self, known: bool) {
        TESForm::set_player_knows(self.as_ref(), known)
    }

    fn get_form_editor_id_as_str(&self) -> &str {
        self.as_ref().get_form_editor_id_as_str()
    }

    fn get_object_type_name_as_str(&self) -> &str {
        self.as_ref().get_object_type_name_as_str()
    }

    fn initialize_data(&mut self) {
        self.as_mut().initialize_data()
    }

    fn clear_data(&mut self) {
        self.as_mut().clear_data()
    }

    fn load(&mut self, mod_file: *mut TESFile) -> bool {
        self.as_mut().load(mod_file)
    }

    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer) {
        self.as_mut().save_game(buf)
    }

    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        self.as_mut().load_game(buf)
    }

    fn activate(&mut self, target_ref: *mut TESObjectREFR, activator_ref: *mut TESObjectREFR, arg3: u8, object: *mut TESBoundObject, target_count: i32) -> bool {
        self.as_mut().activate(target_ref, activator_ref, arg3, object, target_count)
    }
}

// ─── Fast Form Downcasting ─────────────────────────────────────────────

impl FormCastable for TESForm {
    const TARGET_FORM_TYPE: FormType = FormType::None;
}

