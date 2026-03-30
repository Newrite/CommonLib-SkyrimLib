use alloc::string::String;

use crate::re::{BSSimpleList, BSString, TESForm, TESObjectREFR};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptParamType {
    Char = 0x0,
    Int = 0x1,
    Float = 0x2,
    InventoryObject = 0x3,
    ObjectRef = 0x4,
    ActorValue = 0x5,
    Actor = 0x6,
    SpellItem = 0x7,
    Axis = 0x8,
    Cell = 0x9,
    AnimGroup = 0x0A,
    MagicItem = 0x0B,
    Sound = 0x0C,
    Topic = 0x0D,
    Quest = 0x0E,
    Race = 0x0F,
    Class = 0x10,
    Faction = 0x11,
    Sex = 0x12,
    Global = 0x13,
    FurnitureOrFormList = 0x14,
    Object = 0x15,
    ScriptVar = 0x16,
    Stage = 0x17,
    MapMarker = 0x18,
    ActorBase = 0x19,
    ContainerRef = 0x1A,
    WorldOrList = 0x1B,
    CrimeType = 0x1C,
    Package = 0x1D,
    CombatStyle = 0x1E,
    MagicEffect = 0x1F,
    FormType = 0x20,
    Weather = 0x21,
    Npc = 0x22,
    Owner = 0x23,
    ShaderEffect = 0x24,
    FormList = 0x25,
    MenuIcon = 0x26,
    Perk = 0x27,
    Note = 0x28,
    MiscStat = 0x29,
    ImagespaceMod = 0x2A,
    Imagespace = 0x2B,
    VatsValue = 0x2C,
    VatsValueData = 0x2D,
    EventFunction = 0x2E,
    EventFunctionMember = 0x2F,
    EventFunctionData = 0x30,
    VoiceType = 0x31,
    EncounterZone = 0x32,
    IdleForm = 0x33,
    Message = 0x34,
    InvObjectOrFormList = 0x35,
    Alignment = 0x36,
    EquipType = 0x37,
    ObjectOrFormList = 0x38,
    Music = 0x39,
    CritStage = 0x3A,
    Keyword = 0x3B,
    RefType = 0x3C,
    Location = 0x3D,
    Form = 0x3E,
    Alias = 0x3F,
    Shout = 0x40,
    WordOfPower = 0x41,
    RelationshipRank = 0x42,
    BgsScene = 0x43,
    CastingSource = 0x44,
    AssociationType = 0x45,
    WardState = 0x46,
    PackageDataCanBeNull = 0x47,
    PackageDataNumeric = 0x48,
    FurnitureAnimType = 0x49,
    FurnitureEntryType = 0x4A,
    VmScriptVar = 0x4C,
    ReferenceEffect = 0x4D,
    PackageData = 0x4E,
    SkillAction = 0x50,
    KnowableForm = 0x51,
    Region = 0x52,
}

core_util::impl_enumset_type!(ScriptParamType => u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ScriptOutput(pub u32);

impl core_util::EnumSetType<u32> for ScriptOutput {
    fn to_underlying(self) -> u32 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ScriptError(pub u32);

impl core_util::EnumSetType<u32> for ScriptError {
    fn to_underlying(self) -> u32 {
        self.0
    }
}

#[repr(C)]
pub struct ScriptHeader {
    pub variable_count: u32,
    pub ref_object_count: u32,
    pub data_size: u32,
    pub last_id: u32,
    pub is_quest_script: bool,
    pub is_magic_effect_script: bool,
    pub is_compiled: bool,
    pub pad13: u8,
}

const _: () = assert!(core::mem::size_of::<ScriptHeader>() == 0x14);

#[repr(C)]
pub struct ScriptParameter {
    pub param_name: *const i8,
    pub param_type: core_util::EnumSet<ScriptParamType, u32>,
    pub optional: bool,
    pub pad0d: u8,
    pub pad0e: u16,
}

const _: () = assert!(core::mem::size_of::<ScriptParameter>() == 0x10);

#[repr(C)]
pub struct ScriptReferencedObject {
    pub editor_id: BSString,
    pub form: *mut TESForm,
    pub variable_id: u32,
    pub pad1c: u32,
}

const _: () = assert!(core::mem::size_of::<ScriptReferencedObject>() == 0x20);

#[repr(C)]
pub struct ActionObject {
    pub form: *mut TESForm,
    pub flags: u32,
    pub pad0c: u32,
}

const _: () = assert!(core::mem::size_of::<ActionObject>() == 0x10);

#[repr(C)]
pub struct ScriptLocal {
    pub id: u32,
    pub value: f32,
    pub is_integer: bool,
    pub pad9: u8,
    pub pad_a: u16,
}

const _: () = assert!(core::mem::size_of::<ScriptLocal>() == 0x0C);

#[repr(C)]
pub struct ScriptEffectData {
    pub script_effect_start: bool,
    pub script_effect_finish: bool,
    pub pad02: u16,
    pub seconds_elapsed: f32,
}

const _: () = assert!(core::mem::size_of::<ScriptEffectData>() == 0x8);

#[repr(C)]
pub struct ScriptLocals {
    pub master_script: *mut crate::re::Script,
    pub flags: i8,
    pub pad09: u8,
    pub pad0a: u16,
    pub pad0c: u32,
    pub action_list: *mut BSSimpleList<*mut ActionObject>,
    pub local_list: *mut BSSimpleList<*mut ScriptLocal>,
    pub script_effect_data: *mut ScriptEffectData,
}

const _: () = assert!(core::mem::size_of::<ScriptLocals>() == 0x28);

#[repr(C)]
pub struct ScriptVariable {
    pub data: ScriptLocal,
    pub pad0c: u32,
    pub name: BSString,
}

const _: () = assert!(core::mem::size_of::<ScriptVariable>() == 0x20);

#[repr(C)]
pub struct ScriptLine {
    pub line_number: u32,
    pub line: [u8; 512],
    pub size: u32,
    pub offset: u32,
    pub output: [u8; 512],
    pub output_size: u32,
    pub expression: core_util::EnumSet<ScriptOutput, u32>,
    pub ref_object_index: u32,
    pub script_error: core_util::EnumSet<ScriptError, u32>,
}

const _: () = assert!(core::mem::size_of::<ScriptLine>() == 0x41C);

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

pub type ExecuteFn = extern "C" fn(
    *const ScriptParameter,
    *mut ScriptData,
    *mut TESObjectREFR,
    *mut TESObjectREFR,
    *mut crate::re::Script,
    *mut ScriptLocals,
    *mut f64,
    *mut u32,
) -> bool;

pub type CompileFn =
    extern "C" fn(u16, *const ScriptParameter, *mut ScriptLine, *mut ScriptCompileData) -> bool;

pub type ConditionFn = extern "C" fn(
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
pub struct ScriptFunction {
    pub function_name: *const i8,                // 00
    pub short_name: *const i8,                   // 08
    pub output: ScriptOutput,                    // 10
    pub pad14: u32,                              // 14
    pub help_string: *const i8,                  // 18
    pub reference_function: bool,                // 20
    pub pad21: u8,                               // 21
    pub num_params: u16,                         // 22
    pub pad24: u32,                              // 24
    pub params: *mut ScriptParameter,            // 28
    pub execute_function: Option<ExecuteFn>,     // 30
    pub compile_function: Option<CompileFn>,     // 38
    pub condition_function: Option<ConditionFn>, // 40
    pub editor_filter: bool,                     // 48
    pub invalidates_cell_list: bool,             // 49
    pub pad4a: u16,                              // 4A
    pub pad4c: u32,                              // 4C
}

const _: () = assert!(core::mem::size_of::<ScriptFunction>() == 0x50);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, function_name) == 0x00);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, short_name) == 0x08);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, output) == 0x10);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, help_string) == 0x18);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, reference_function) == 0x20);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, num_params) == 0x22);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, params) == 0x28);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, execute_function) == 0x30);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, compile_function) == 0x38);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, condition_function) == 0x40);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, editor_filter) == 0x48);
const _: () = assert!(core::mem::offset_of!(ScriptFunction, invalidates_cell_list) == 0x49);

impl ScriptFunction {
    pub const CONSOLE_COMMANDS_END: u32 = 0x01B4;
    pub const CONSOLE_OP_BASE: u32 = 0x0100;
    pub const SCRIPT_COMMANDS_END: u32 = 0x02E0;
    pub const SCRIPT_OP_BASE: u32 = 0x1000;

    crate::relocation_variable! {
        fn first_script_command_ptr() -> *mut ScriptFunction => crate::relocation::RelocationID::new(501789, 361120), is_ptr
    }

    crate::relocation_variable! {
        fn first_console_command_ptr() -> *mut ScriptFunction => crate::relocation::RelocationID::new(501797, 365650), is_ptr
    }

    #[inline(always)]
    pub fn get_first_script_command() -> *mut ScriptFunction {
        Self::first_script_command_ptr()
    }

    #[inline(always)]
    pub fn locate_script_command(long_name: &str) -> *mut ScriptFunction {
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
    pub fn get_first_console_command() -> *mut ScriptFunction {
        Self::first_console_command_ptr()
    }

    #[inline(always)]
    pub fn locate_console_command(long_name: &str) -> *mut ScriptFunction {
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
    pub fn set_parameters_slice(&mut self, params: &mut [ScriptParameter]) {
        self.num_params = params.len() as u16;
        self.params = params.as_mut_ptr();
    }

    #[inline(always)]
    pub fn set_parameters(&mut self) {
        self.num_params = 0;
        self.params = core::ptr::null_mut();
    }
}
