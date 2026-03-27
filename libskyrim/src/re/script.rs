use alloc::string::String;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_Script;
use crate::offsets::offsets_vtable::VTABLE_Script;
use crate::re::form_traits::FormCastable;
use crate::re::{
    BSSimpleList, FormType, SCRIPT_HEADER, SCRIPT_PARAMETER, SCRIPT_REFERENCED_OBJECT, ScriptData,
    ScriptLocals, ScriptVariable, TESForm, TESObjectREFR, TESQuest,
};
use crate::relocation::{Relocation, RelocationID, RttiType, VariantID, VersionedRelocationID};
use crate::version::RUNTIME_SSE_1_6_1130;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum COMPILER_NAME {
    kDefaultCompiler = 0,
    kSystemWindowCompiler = 1,
    kDialogueCompiler = 2,
}

#[repr(C)]
pub struct ScriptCompiler {
    pub pad00: u8,
}

const _: () = assert!(core::mem::size_of::<ScriptCompiler>() == 0x1);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct RecordFlags(pub u32);

impl core_util::EnumSetType<u32> for RecordFlags {
    fn to_underlying(self) -> u32 {
        self.0
    }
}

mod script_parse_parameters_private {
    pub trait Sealed {}

    impl Sealed for () {}
    impl<T> Sealed for *const T {}
    impl<T> Sealed for *mut T {}
}

pub trait ScriptParseParameterArg: script_parse_parameters_private::Sealed + Copy {}

impl<T> ScriptParseParameterArg for *const T {}
impl<T> ScriptParseParameterArg for *mut T {}

type ParseParametersImpl = unsafe extern "C" fn(
    *const SCRIPT_PARAMETER,
    *mut ScriptData,
    *mut u32,
    *mut TESObjectREFR,
    *mut TESObjectREFR,
    *mut Script,
    *mut ScriptLocals,
    ...
) -> bool;

pub trait ScriptParseParameterArgs {
    /// Calls `Script::ParseParameters(...)` with a compile-time pointer pack.
    ///
    /// This mirrors the CommonLib template convenience wrapper, which only
    /// accepts pointer arguments in its variadic tail.
    unsafe fn call(
        self,
        func: ParseParametersImpl,
        param_info: *const SCRIPT_PARAMETER,
        script_data: *mut ScriptData,
        opcode_offset_ptr: *mut u32,
        this_obj: *mut TESObjectREFR,
        containing_obj: *mut TESObjectREFR,
        script_obj: *mut Script,
        locals: *mut ScriptLocals,
    ) -> bool;
}

impl ScriptParseParameterArgs for () {
    #[inline(always)]
    unsafe fn call(
        self,
        func: ParseParametersImpl,
        param_info: *const SCRIPT_PARAMETER,
        script_data: *mut ScriptData,
        opcode_offset_ptr: *mut u32,
        this_obj: *mut TESObjectREFR,
        containing_obj: *mut TESObjectREFR,
        script_obj: *mut Script,
        locals: *mut ScriptLocals,
    ) -> bool {
        unsafe {
            func(
                param_info,
                script_data,
                opcode_offset_ptr,
                this_obj,
                containing_obj,
                script_obj,
                locals,
            )
        }
    }
}

macro_rules! impl_script_parse_parameter_args {
    ($(($($ty:ident $arg:ident),+)),+ $(,)?) => {
        $(
            impl<$($ty: ScriptParseParameterArg),+> ScriptParseParameterArgs for ($($ty,)+) {
                #[inline(always)]
                unsafe fn call(
                    self,
                    func: ParseParametersImpl,
                    param_info: *const SCRIPT_PARAMETER,
                    script_data: *mut ScriptData,
                    opcode_offset_ptr: *mut u32,
                    this_obj: *mut TESObjectREFR,
                    containing_obj: *mut TESObjectREFR,
                    script_obj: *mut Script,
                    locals: *mut ScriptLocals,
                ) -> bool {
                    let ($($arg,)+) = self;
                    unsafe {
                        func(
                            param_info,
                            script_data,
                            opcode_offset_ptr,
                            this_obj,
                            containing_obj,
                            script_obj,
                            locals,
                            $($arg,)+
                        )
                    }
                }
            }
        )+
    };
}

impl_script_parse_parameter_args!(
    (A a),
    (A a, B b),
    (A a, B b, C c),
    (A a, B b, C c, D d),
    (A a, B b, C c, D d, E e),
    (A a, B b, C c, D d, E e, F f),
    (A a, B b, C c, D d, E e, F f, G g),
    (A a, B b, C c, D d, E e, F f, G g, H h),
    (A a, B b, C c, D d, E e, F f, G g, H h, I i),
    (A a, B b, C c, D d, E e, F f, G g, H h, I i, J j),
    (A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k),
    (A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l),
    (A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m),
    (A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m, N n),
    (A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m, N n, O o),
    (A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m, N n, O o, P p)
);

#[repr(C)]
pub struct Script {
    pub base: TESForm,                                            // 00
    pub header: SCRIPT_HEADER,                                    // 20
    pub pad34: u32,                                               // 34
    pub text: *mut i8,                                            // 38
    pub data: *mut crate::re::ScriptData,                         // 40
    pub profiler_timer: f32,                                      // 48
    pub quest_script_delay: f32,                                  // 4C
    pub quest_script_get_seconds_buffer: f32,                     // 50
    pub pad54: u32,                                               // 54
    pub parent_quest: *mut TESQuest,                              // 58
    pub ref_objects: BSSimpleList<*mut SCRIPT_REFERENCED_OBJECT>, // 60
    pub variables: BSSimpleList<*mut ScriptVariable>,             // 70
}

const _: () = assert!(core::mem::size_of::<Script>() == 0x80);

inherit!(Script : TESForm);

impl RttiType for Script {
    const RTTI: VariantID = RTTI_Script;
}

impl FormCastable for Script {
    const TARGET_FORM_TYPE: FormType = FormType::Script;
}

impl Script {
    pub const RTTI: VariantID = RTTI_Script;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Script;
    pub const FORMTYPE: FormType = FormType::Script;

    crate::relocation_func! {
        pub fn get_process_scripts() -> bool => RelocationID::new(21436, 21921)
    }

    crate::relocation_func! {
        pub fn set_process_scripts(process_scripts: bool) => RelocationID::new(21435, 21920)
    }

    #[inline(always)]
    pub fn clear_command(&mut self) {
        if !self.text.is_null() {
            unsafe { crate::ffi::commonlib_free(self.text.cast()) };
            self.text = core::ptr::null_mut();
        }
    }

    #[inline(always)]
    pub fn compile_and_run(&mut self, target_ref: *mut TESObjectREFR, name: COMPILER_NAME) {
        let mut compiler = ScriptCompiler { pad00: 0 };
        self.compile_and_run_with_compiler(&mut compiler, target_ref, name);
    }

    #[inline(always)]
    pub fn compile_and_run_with_compiler(
        &mut self,
        compiler: *mut ScriptCompiler,
        target_ref: *mut TESObjectREFR,
        name: COMPILER_NAME,
    ) {
        type CompileAndRunImpl = unsafe extern "C-unwind" fn(
            *mut Script,
            *mut ScriptCompiler,
            COMPILER_NAME,
            *mut TESObjectREFR,
        );

        let func = unsafe {
            Relocation::<CompileAndRunImpl>::new(VersionedRelocationID::new(
                RUNTIME_SSE_1_6_1130,
                21416,
                21890,
                441582,
            ))
            .get()
        };
        unsafe { func(self, compiler, name, target_ref) }
    }

    #[inline(always)]
    pub fn parse_parameters<A: ScriptParseParameterArgs>(
        param_info: *const SCRIPT_PARAMETER,
        script_data: *mut ScriptData,
        opcode_offset_ptr: &mut u32,
        this_obj: *mut TESObjectREFR,
        containing_obj: *mut TESObjectREFR,
        script_obj: *mut Script,
        locals: *mut ScriptLocals,
        args: A,
    ) -> bool {
        let func = unsafe {
            Relocation::<ParseParametersImpl>::new(RelocationID::new(21425, 21910)).get()
        };
        unsafe {
            args.call(
                func,
                param_info,
                script_data,
                opcode_offset_ptr,
                this_obj,
                containing_obj,
                script_obj,
                locals,
            )
        }
    }

    #[inline(always)]
    pub fn get_command(&self) -> String {
        if self.text.is_null() {
            String::new()
        } else {
            unsafe {
                core::ffi::CStr::from_ptr(self.text)
                    .to_string_lossy()
                    .into_owned()
            }
        }
    }

    #[inline(always)]
    pub fn set_command(&mut self, command: &str) {
        self.clear_command();
        let len = command.len() + 1;
        let ptr = unsafe { crate::ffi::commonlib_malloc(len) }.cast::<u8>();
        if ptr.is_null() {
            return;
        }
        unsafe {
            core::ptr::copy_nonoverlapping(command.as_ptr(), ptr, command.len());
            *ptr.add(command.len()) = 0;
        }
        self.text = ptr.cast();
    }

    // TODO: SOURCE - `Script::ParseParameters(...)` is open-ended in CommonLib's
    // template form. Extend `ScriptParseParameterArgs` if a same-source command
    // needs more than 16 pointer arguments in the variadic tail.
}
