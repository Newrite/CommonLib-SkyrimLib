use alloc::string::String;

use crate::re::{BSSimpleList, BSString, TESForm, TESObjectREFR};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SCRIPT_PARAM_TYPE {
    kChar = 0x0,
    kInt = 0x1,
    kFloat = 0x2,
    kInventoryObject = 0x3,
    kObjectRef = 0x4,
    kActorValue = 0x5,
    kActor = 0x6,
    kSpellItem = 0x7,
    kAxis = 0x8,
    kCell = 0x9,
    kAnimGroup = 0x0A,
    kMagicItem = 0x0B,
    kSound = 0x0C,
    kTopic = 0x0D,
    kQuest = 0x0E,
    kRace = 0x0F,
    kClass = 0x10,
    kFaction = 0x11,
    kSex = 0x12,
    kGlobal = 0x13,
    kFurnitureOrFormList = 0x14,
    kObject = 0x15,
    kScriptVar = 0x16,
    kStage = 0x17,
    kMapMarker = 0x18,
    kActorBase = 0x19,
    kContainerRef = 0x1A,
    kWorldOrList = 0x1B,
    kCrimeType = 0x1C,
    kPackage = 0x1D,
    kCombatStyle = 0x1E,
    kMagicEffect = 0x1F,
    kFormType = 0x20,
    kWeather = 0x21,
    kNPC = 0x22,
    kOwner = 0x23,
    kShaderEffect = 0x24,
    kFormList = 0x25,
    kMenuIcon = 0x26,
    kPerk = 0x27,
    kNote = 0x28,
    kMiscStat = 0x29,
    kImagespaceMod = 0x2A,
    kImagespace = 0x2B,
    kVATSValue = 0x2C,
    kVATSValueData = 0x2D,
    kEventFunction = 0x2E,
    kEventFunctionMember = 0x2F,
    kEventFunctionData = 0x30,
    kVoiceType = 0x31,
    kEncounterZone = 0x32,
    kIdleForm = 0x33,
    kMessage = 0x34,
    kInvObjectOrFormList = 0x35,
    kAlignment = 0x36,
    kEquipType = 0x37,
    kObjectOrFormList = 0x38,
    kMusic = 0x39,
    kCritStage = 0x3A,
    kKeyword = 0x3B,
    kRefType = 0x3C,
    kLocation = 0x3D,
    kForm = 0x3E,
    kAlias = 0x3F,
    kShout = 0x40,
    kWordOfPower = 0x41,
    kRelationshipRank = 0x42,
    kBGSScene = 0x43,
    kCastingSource = 0x44,
    kAssociationType = 0x45,
    kWardState = 0x46,
    kPackageDataCanBeNull = 0x47,
    kPackageDataNumeric = 0x48,
    kFurnitureAnimType = 0x49,
    kFurnitureEntryType = 0x4A,
    kVMScriptVar = 0x4C,
    kReferenceEffect = 0x4D,
    kPackageData = 0x4E,
    kSkillAction = 0x50,
    kKnowableForm = 0x51,
    kRegion = 0x52,
}

core_util::impl_enumset_type!(SCRIPT_PARAM_TYPE => u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SCRIPT_OUTPUT(pub u32);

impl core_util::EnumSetType<u32> for SCRIPT_OUTPUT {
    fn to_underlying(self) -> u32 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SCRIPT_ERROR(pub u32);

impl core_util::EnumSetType<u32> for SCRIPT_ERROR {
    fn to_underlying(self) -> u32 {
        self.0
    }
}

#[repr(C)]
pub struct SCRIPT_HEADER {
    pub variable_count: u32,
    pub ref_object_count: u32,
    pub data_size: u32,
    pub last_id: u32,
    pub is_quest_script: bool,
    pub is_magic_effect_script: bool,
    pub is_compiled: bool,
    pub pad13: u8,
}

const _: () = assert!(core::mem::size_of::<SCRIPT_HEADER>() == 0x14);

#[repr(C)]
pub struct SCRIPT_PARAMETER {
    pub param_name: *const i8,
    pub param_type: core_util::EnumSet<SCRIPT_PARAM_TYPE, u32>,
    pub optional: bool,
    pub pad0d: u8,
    pub pad0e: u16,
}

const _: () = assert!(core::mem::size_of::<SCRIPT_PARAMETER>() == 0x10);

#[repr(C)]
pub struct SCRIPT_REFERENCED_OBJECT {
    pub editor_id: BSString,
    pub form: *mut TESForm,
    pub variable_id: u32,
    pub pad1c: u32,
}

const _: () = assert!(core::mem::size_of::<SCRIPT_REFERENCED_OBJECT>() == 0x20);

#[repr(C)]
pub struct ACTION_OBJECT {
    pub form: *mut TESForm,
    pub flags: u32,
    pub pad0c: u32,
}

const _: () = assert!(core::mem::size_of::<ACTION_OBJECT>() == 0x10);

#[repr(C)]
pub struct SCRIPT_LOCAL {
    pub id: u32,
    pub value: f32,
    pub is_integer: bool,
    pub pad9: u8,
    pub pad_a: u16,
}

const _: () = assert!(core::mem::size_of::<SCRIPT_LOCAL>() == 0x0C);

#[repr(C)]
pub struct SCRIPT_EFFECT_DATA {
    pub script_effect_start: bool,
    pub script_effect_finish: bool,
    pub pad02: u16,
    pub seconds_elapsed: f32,
}

const _: () = assert!(core::mem::size_of::<SCRIPT_EFFECT_DATA>() == 0x8);

#[repr(C)]
pub struct ScriptLocals {
    pub master_script: *mut crate::re::Script,
    pub flags: i8,
    pub pad09: u8,
    pub pad0a: u16,
    pub pad0c: u32,
    pub action_list: *mut BSSimpleList<*mut ACTION_OBJECT>,
    pub local_list: *mut BSSimpleList<*mut SCRIPT_LOCAL>,
    pub script_effect_data: *mut SCRIPT_EFFECT_DATA,
}

const _: () = assert!(core::mem::size_of::<ScriptLocals>() == 0x28);

#[repr(C)]
pub struct ScriptVariable {
    pub data: SCRIPT_LOCAL,
    pub pad0c: u32,
    pub name: BSString,
}

const _: () = assert!(core::mem::size_of::<ScriptVariable>() == 0x20);

#[repr(C)]
pub struct SCRIPT_LINE {
    pub line_number: u32,
    pub line: [u8; 512],
    pub size: u32,
    pub offset: u32,
    pub output: [u8; 512],
    pub output_size: u32,
    pub expression: core_util::EnumSet<SCRIPT_OUTPUT, u32>,
    pub ref_object_index: u32,
    pub script_error: core_util::EnumSet<SCRIPT_ERROR, u32>,
}

const _: () = assert!(core::mem::size_of::<SCRIPT_LINE>() == 0x41C);

#[repr(C)]
pub struct Chunk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ScriptCompileData {
    _private: [u8; 0],
}

#[repr(C)]
pub struct StringChunk {
    pub length: u16,
    pub str_: [u8; 0],
}

const _: () = assert!(core::mem::size_of::<StringChunk>() == 0x2);

#[repr(C, packed)]
pub struct IntegerChunk {
    pub magic: i8,
    pub value: i32,
}

const _: () = assert!(core::mem::size_of::<IntegerChunk>() == 0x5);
const _: () = assert!(core::mem::offset_of!(IntegerChunk, value) == 0x1);

#[repr(C)]
pub struct ScriptData {
    pub opcode: u16,
    pub chunk_size: u16,
    pub num_params: u16,
}

const _: () = assert!(core::mem::size_of::<ScriptData>() == 0x6);

pub type Execute_t = extern "C" fn(
    *const SCRIPT_PARAMETER,
    *mut ScriptData,
    *mut TESObjectREFR,
    *mut TESObjectREFR,
    *mut crate::re::Script,
    *mut ScriptLocals,
    *mut f64,
    *mut u32,
) -> bool;

pub type Compile_t =
    extern "C" fn(u16, *const SCRIPT_PARAMETER, *mut SCRIPT_LINE, *mut ScriptCompileData) -> bool;

pub type Condition_t = extern "C" fn(
    *mut TESObjectREFR,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut f64,
) -> bool;

impl Chunk {
    #[inline(always)]
    pub fn as_string(&self) -> *mut StringChunk {
        self as *const Self as *mut StringChunk
    }

    #[inline(always)]
    pub fn as_integer(&self) -> *mut IntegerChunk {
        self as *const Self as *mut IntegerChunk
    }
}

impl StringChunk {
    #[inline(always)]
    pub fn get_string(&self) -> String {
        if self.length == 0 {
            String::new()
        } else {
            unsafe {
                let slice = core::slice::from_raw_parts(self.str_.as_ptr(), self.length as usize);
                String::from_utf8_lossy(slice).into_owned()
            }
        }
    }

    #[inline(always)]
    pub fn get_next(&self) -> *mut Chunk {
        unsafe {
            self.str_
                .as_ptr()
                .add(self.length as usize)
                .cast_mut()
                .cast()
        }
    }
}

impl IntegerChunk {
    #[inline(always)]
    pub fn get_integer(&self) -> i32 {
        unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(self.value)) }
    }

    #[inline(always)]
    pub fn get_next(&self) -> *mut Chunk {
        unsafe { (self as *const Self).add(1).cast_mut().cast() }
    }
}

impl ScriptData {
    #[inline(always)]
    pub fn get_chunk(&self) -> *mut Chunk {
        unsafe { (self as *const Self).add(1).cast_mut().cast() }
    }

    #[inline(always)]
    pub fn get_string_chunk(&self) -> *mut StringChunk {
        self.get_chunk().cast()
    }

    #[inline(always)]
    pub fn get_integer_chunk(&self) -> *mut IntegerChunk {
        self.get_chunk().cast()
    }
}

#[repr(C)]
pub struct SCRIPT_FUNCTION {
    pub function_name: *const i8,                // 00
    pub short_name: *const i8,                   // 08
    pub output: SCRIPT_OUTPUT,                   // 10
    pub pad14: u32,                              // 14
    pub help_string: *const i8,                  // 18
    pub reference_function: bool,                // 20
    pub pad21: u8,                               // 21
    pub num_params: u16,                         // 22
    pub pad24: u32,                              // 24
    pub params: *mut SCRIPT_PARAMETER,           // 28
    pub execute_function: Option<Execute_t>,     // 30
    pub compile_function: Option<Compile_t>,     // 38
    pub condition_function: Option<Condition_t>, // 40
    pub editor_filter: bool,                     // 48
    pub invalidates_cell_list: bool,             // 49
    pub pad4a: u16,                              // 4A
    pub pad4c: u32,                              // 4C
}

const _: () = assert!(core::mem::size_of::<SCRIPT_FUNCTION>() == 0x50);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, function_name) == 0x00);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, short_name) == 0x08);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, output) == 0x10);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, help_string) == 0x18);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, reference_function) == 0x20);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, num_params) == 0x22);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, params) == 0x28);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, execute_function) == 0x30);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, compile_function) == 0x38);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, condition_function) == 0x40);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, editor_filter) == 0x48);
const _: () = assert!(core::mem::offset_of!(SCRIPT_FUNCTION, invalidates_cell_list) == 0x49);

impl SCRIPT_FUNCTION {
    pub const CONSOLE_COMMANDS_END: u32 = 0x01B4;
    pub const CONSOLE_OP_BASE: u32 = 0x0100;
    pub const SCRIPT_COMMANDS_END: u32 = 0x02E0;
    pub const SCRIPT_OP_BASE: u32 = 0x1000;

    crate::relocation_variable! {
        fn first_script_command_ptr() -> *mut SCRIPT_FUNCTION => crate::relocation::RelocationID::new(501789, 361120), is_ptr
    }

    crate::relocation_variable! {
        fn first_console_command_ptr() -> *mut SCRIPT_FUNCTION => crate::relocation::RelocationID::new(501797, 365650), is_ptr
    }

    #[inline(always)]
    pub fn get_first_script_command() -> *mut SCRIPT_FUNCTION {
        Self::first_script_command_ptr()
    }

    #[inline(always)]
    pub fn locate_script_command(long_name: &str) -> *mut SCRIPT_FUNCTION {
        let base = Self::get_first_script_command();
        if base.is_null() {
            return core::ptr::null_mut();
        }
        for i in 0..Self::SCRIPT_COMMANDS_END as usize {
            let entry = unsafe { base.add(i) };
            let name = unsafe { (*entry).function_name };
            if !name.is_null() {
                let c = unsafe { core::ffi::CStr::from_ptr(name) };
                if c.to_bytes().eq_ignore_ascii_case(long_name.as_bytes()) {
                    return entry;
                }
            }
        }
        core::ptr::null_mut()
    }

    #[inline(always)]
    pub fn get_first_console_command() -> *mut SCRIPT_FUNCTION {
        Self::first_console_command_ptr()
    }

    #[inline(always)]
    pub fn locate_console_command(long_name: &str) -> *mut SCRIPT_FUNCTION {
        let base = Self::get_first_console_command();
        if base.is_null() {
            return core::ptr::null_mut();
        }
        for i in 0..Self::CONSOLE_COMMANDS_END as usize {
            let entry = unsafe { base.add(i) };
            let name = unsafe { (*entry).function_name };
            if !name.is_null() {
                let c = unsafe { core::ffi::CStr::from_ptr(name) };
                if c.to_bytes().eq_ignore_ascii_case(long_name.as_bytes()) {
                    return entry;
                }
            }
        }
        core::ptr::null_mut()
    }

    #[inline(always)]
    pub fn set_parameters_slice(&mut self, params: &mut [SCRIPT_PARAMETER]) {
        self.num_params = params.len() as u16;
        self.params = params.as_mut_ptr();
    }

    #[inline(always)]
    pub fn set_parameters(&mut self) {
        self.num_params = 0;
        self.params = core::ptr::null_mut();
    }
}
