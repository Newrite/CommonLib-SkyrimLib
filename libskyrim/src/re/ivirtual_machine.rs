use alloc::boxed::Box;
use alloc::ffi::CString;
use alloc::format;
use alloc::vec::Vec;
use core::ffi::{CStr, c_char, c_void};

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSScript__IVirtualMachine;
use crate::offsets::offsets_vtable::VTABLE_BSScript__IVirtualMachine;
use crate::re::bs_core_types::{VMHandle, VMStackID, VMTypeID};
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bs_intrusive_ref_counted::BSIntrusiveRefCounted;
use crate::re::bs_jobs::BSJobsJobList;
use crate::re::bst_array::BSScrapArray;
use crate::re::bst_event::BSTEventSink;
use crate::re::bst_smart_pointer::{BSTSmartPointer, BSTSmartPointerIntrusiveRefCountable};
use crate::re::error_logger::{ErrorLogger, Severity};
use crate::re::i_object_handle_policy::IObjectHandlePolicy;
use crate::re::i_save_patcher_interface::ISavePatcherInterface;
use crate::re::i_stack_callback_functor::IStackCallbackFunctor;
use crate::re::ifunction::IFunction;
use crate::re::ifunction_arguments::IFunctionArguments;
use crate::re::log_event::LogEvent;
use crate::re::native_function::{NativeFunction, NativeFunctionDesc, NativeFunctionHandler};
use crate::re::native_latent_function::{
    LatentStatus, NativeLatentFunction, NativeLatentFunctionDesc,
};
use crate::re::stack_frame::StackFrame;
use crate::re::stats_event::StatsEvent;
use crate::re::tes_form::TESForm;
use crate::re::type_info::{RawType, TypeInfo};
use crate::re::type_traits::{
    PapyrusBase, PapyrusParameter, PapyrusParameterConvertible, PapyrusReturn,
    PapyrusReturnConvertible, PapyrusValidBase,
};
use crate::re::variable::Variable;
use crate::re::virtual_machine::VirtualMachine;
use crate::relocation::{RttiType, VariantID, VariantOffset};
use crate::virtual_method;

core_util::abstract_type! {
    pub type IForEachScriptObjectFunctor;
    pub type ITypeLinkedCallback;
}

/// C++ `RE::BSScript::IVirtualMachine::Awaitable::CallbackFunctor`
#[repr(C)]
pub struct CallbackFunctor {
    pub base: IStackCallbackFunctor, // 00
    pub pending: bool,               // 10
    pub pad11: [u8; 7],              // 11
    pub result: Variable,            // 18
    pub continuation: *mut c_void,   // 28
}

const _: () = assert!(core::mem::size_of::<CallbackFunctor>() == 0x30);
const _: () = assert!(core::mem::offset_of!(CallbackFunctor, pending) == 0x10);
const _: () = assert!(core::mem::offset_of!(CallbackFunctor, result) == 0x18);
const _: () = assert!(core::mem::offset_of!(CallbackFunctor, continuation) == 0x28);

inherit!(CallbackFunctor : IStackCallbackFunctor);

unsafe extern "C" fn callback_functor_dtor(this: *mut CallbackFunctor) {
    drop(unsafe { Box::from_raw(this) });
}

unsafe extern "C" fn callback_functor_call(this: *mut CallbackFunctor, result: Variable) {
    unsafe { (*this).call_override(result) };
}

unsafe extern "C" fn callback_functor_can_save(_this: *const CallbackFunctor) -> bool {
    false
}

unsafe extern "C" fn callback_functor_set_object(
    this: *mut CallbackFunctor,
    object: &BSTSmartPointer<Object>,
) {
    unsafe { (*this).set_object_override(object) };
}

struct CallbackFunctorVTable([*const (); 4]);

unsafe impl Sync for CallbackFunctorVTable {}

static CALLBACK_FUNCTOR_VTABLE: CallbackFunctorVTable = CallbackFunctorVTable([
    callback_functor_dtor as *const (),
    callback_functor_call as *const (),
    callback_functor_can_save as *const (),
    callback_functor_set_object as *const (),
]);

impl CallbackFunctor {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            base: IStackCallbackFunctor {
                vtable: CALLBACK_FUNCTOR_VTABLE.0.as_ptr().cast(),
                base: BSIntrusiveRefCounted::default(),
                pad0c: 0,
            },
            pending: false,
            pad11: [0; 7],
            result: Variable::default(),
            continuation: core::ptr::null_mut(),
        }
    }

    // override (IStackCallbackFunctor)
    // void operator()(Variable a_result) override; // 01
    // void SetObject(const BSTSmartPointer<Object>&) override; // 03

    #[inline(always)]
    pub fn call_override(&mut self, result: Variable) {
        self.pending = false;
        self.result = result;
    }

    #[inline(always)]
    pub fn set_object_override(&mut self, _object: &BSTSmartPointer<Object>) {}
}

/// C++ `RE::BSScript::IVirtualMachine::Awaitable`
#[repr(C)]
pub struct Awaitable {
    pub callback: BSTSmartPointer<IStackCallbackFunctor>, // 00
}

const _: () = assert!(core::mem::size_of::<Awaitable>() == 0x8);
const _: () = assert!(core::mem::offset_of!(Awaitable, callback) == 0x0);

impl Default for Awaitable {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Awaitable {
    #[inline]
    pub fn new() -> Self {
        let callback =
            Box::into_raw(Box::new(CallbackFunctor::new())).cast::<IStackCallbackFunctor>();
        Self {
            callback: unsafe { BSTSmartPointer::new(callback) },
        }
    }

    #[inline(always)]
    pub fn set_pending(&mut self, pending: bool) {
        let callback = self.callback.get();
        if callback.is_null() {
            return;
        }
        let callback = unsafe { &mut *(callback as *mut CallbackFunctor) };
        callback.pending = pending;
    }

    #[inline(always)]
    pub fn await_ready(&self) -> bool {
        let callback = self.callback.get();
        if callback.is_null() {
            return true;
        }
        let callback = unsafe { &*(callback as *const CallbackFunctor) };
        !callback.pending
    }

    #[inline(always)]
    pub fn await_suspend(&mut self, continuation: *mut c_void) {
        let callback = self.callback.get();
        if callback.is_null() {
            return;
        }
        let callback = unsafe { &mut *(callback as *mut CallbackFunctor) };
        callback.continuation = continuation;
    }

    #[inline(always)]
    pub fn await_resume(&self) -> Variable {
        let callback = self.callback.get();
        if callback.is_null() {
            Variable::default()
        } else {
            let callback = unsafe { &*(callback as *const CallbackFunctor) };
            callback.result.clone()
        }
    }
}

/// C++ `RE::BSScript::IVirtualMachine`
#[repr(C)]
pub struct IVirtualMachine {
    pub vtable: *const usize,        // 00
    pub base: BSIntrusiveRefCounted, // 08
    pub pad0c: u32,                  // 0C
}

const _: () = assert!(core::mem::size_of::<IVirtualMachine>() == 0x10);
const _: () = assert!(core::mem::offset_of!(IVirtualMachine, base) == 0x8);
const _: () = assert!(core::mem::offset_of!(IVirtualMachine, pad0c) == 0xC);

inherit!(IVirtualMachine : BSIntrusiveRefCounted);

impl RttiType for IVirtualMachine {
    const RTTI: VariantID = RTTI_BSScript__IVirtualMachine;
}

impl BSTSmartPointerIntrusiveRefCountable for IVirtualMachine {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        self.dtor();
    }
}

pub trait PapyrusFunctionSignature: Copy + 'static {
    const IS_STATIC: bool;

    fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo>;

    fn parameter_type_infos(vm: &IVirtualMachine) -> Option<Vec<TypeInfo>>;

    unsafe fn dispatch(
        self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool;
}

pub trait PapyrusLongFunctionSignature: Copy + 'static {
    const IS_STATIC: bool;

    fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo>;

    fn parameter_type_infos(vm: &IVirtualMachine) -> Option<Vec<TypeInfo>>;

    unsafe fn dispatch(
        self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool;
}

pub trait PapyrusLatentFunctionSignature: Copy + 'static {
    const IS_STATIC: bool;

    fn parameter_type_infos(vm: &IVirtualMachine) -> Option<Vec<TypeInfo>>;

    unsafe fn dispatch(
        self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool;
}

struct RegisteredNativeFunctionHandler<F> {
    callback: F,
}

impl<F> NativeFunctionHandler for RegisteredNativeFunctionHandler<F>
where
    F: PapyrusFunctionSignature,
{
    #[inline(always)]
    unsafe fn marshall_and_dispatch(
        &self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool {
        unsafe {
            self.callback
                .dispatch(base_value, vm, stack_id, result_value, frame)
        }
    }
}

struct RegisteredLongNativeFunctionHandler<F> {
    callback: F,
}

impl<F> NativeFunctionHandler for RegisteredLongNativeFunctionHandler<F>
where
    F: PapyrusLongFunctionSignature,
{
    #[inline(always)]
    unsafe fn marshall_and_dispatch(
        &self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool {
        unsafe {
            self.callback
                .dispatch(base_value, vm, stack_id, result_value, frame)
        }
    }
}

struct RegisteredLatentNativeFunctionHandler<F> {
    callback: F,
}

impl<F> NativeFunctionHandler for RegisteredLatentNativeFunctionHandler<F>
where
    F: PapyrusLatentFunctionSignature,
{
    #[inline(always)]
    unsafe fn marshall_and_dispatch(
        &self,
        base_value: &mut Variable,
        vm: &mut VirtualMachine,
        stack_id: VMStackID,
        result_value: &mut Variable,
        frame: &StackFrame,
    ) -> bool {
        unsafe {
            self.callback
                .dispatch(base_value, vm, stack_id, result_value, frame)
        }
    }
}

macro_rules! impl_papyrus_function_signature_short {
    ($(($param:ident, $arg:ident, $idx:expr)),* $(,)?) => {
        impl<R, Base, $($param),*> PapyrusFunctionSignature for fn(Base, $($param),*) -> R
        where
            R: PapyrusReturn + PapyrusReturnConvertible + 'static,
            Base: PapyrusBase + PapyrusValidBase + 'static,
            $($param: PapyrusParameter + PapyrusParameterConvertible + 'static),*
        {
            const IS_STATIC: bool = Base::IS_STATIC;

            #[inline(always)]
            fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
                R::return_type_info(vm)
            }

            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                base_value: &mut Variable,
                vm: &mut VirtualMachine,
                _stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                let base = unsafe { Base::unpack_base(base_value) };
                $(let $arg = unsafe {
                    $param::unpack_parameter(
                        frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                    )
                };)*
                R::pack_return(self(base, $($arg),*), result_value, vm)
            }
        }
    };
}

macro_rules! impl_papyrus_function_signature_long {
    ($(($param:ident, $arg:ident, $idx:expr)),* $(,)?) => {
        impl<R, Base, $($param),*> PapyrusLongFunctionSignature
            for fn(*mut IVirtualMachine, VMStackID, Base, $($param),*) -> R
        where
            R: PapyrusReturn + PapyrusReturnConvertible + 'static,
            Base: PapyrusBase + PapyrusValidBase + 'static,
            $($param: PapyrusParameter + PapyrusParameterConvertible + 'static),*
        {
            const IS_STATIC: bool = Base::IS_STATIC;

            #[inline(always)]
            fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
                R::return_type_info(vm)
            }

            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                base_value: &mut Variable,
                vm: &mut VirtualMachine,
                stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                let base = unsafe { Base::unpack_base(base_value) };
                $(let $arg = unsafe {
                    $param::unpack_parameter(
                        frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                    )
                };)*
                R::pack_return(
                    self(vm as *mut VirtualMachine as *mut IVirtualMachine, stack_id, base, $($arg),*),
                    result_value,
                    vm,
                )
            }
        }

        impl<R, Base, $($param),*> PapyrusLongFunctionSignature
            for fn(*mut VirtualMachine, VMStackID, Base, $($param),*) -> R
        where
            R: PapyrusReturn + PapyrusReturnConvertible + 'static,
            Base: PapyrusBase + PapyrusValidBase + 'static,
            $($param: PapyrusParameter + PapyrusParameterConvertible + 'static),*
        {
            const IS_STATIC: bool = Base::IS_STATIC;

            #[inline(always)]
            fn return_type_info(vm: &IVirtualMachine) -> Option<TypeInfo> {
                R::return_type_info(vm)
            }

            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                base_value: &mut Variable,
                vm: &mut VirtualMachine,
                stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                let base = unsafe { Base::unpack_base(base_value) };
                $(let $arg = unsafe {
                    $param::unpack_parameter(
                        frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                    )
                };)*
                R::pack_return(self(vm, stack_id, base, $($arg),*), result_value, vm)
            }
        }
    };
}

macro_rules! impl_papyrus_function_signature_latent {
    ($(($param:ident, $arg:ident, $idx:expr)),* $(,)?) => {
        impl<Base, $($param),*> PapyrusLatentFunctionSignature
            for fn(*mut IVirtualMachine, VMStackID, Base, $($param),*) -> LatentStatus
        where
            Base: PapyrusBase + PapyrusValidBase + 'static,
            $($param: PapyrusParameter + PapyrusParameterConvertible + 'static),*
        {
            const IS_STATIC: bool = Base::IS_STATIC;

            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                base_value: &mut Variable,
                vm: &mut VirtualMachine,
                stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                let base = unsafe { Base::unpack_base(base_value) };
                $(let $arg = unsafe {
                    $param::unpack_parameter(
                        frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                    )
                };)*
                result_value.set_bool(
                    self(
                        vm as *mut VirtualMachine as *mut IVirtualMachine,
                        stack_id,
                        base,
                        $($arg),*
                    )
                    .0,
                );
                true
            }
        }

        impl<Base, $($param),*> PapyrusLatentFunctionSignature
            for fn(*mut VirtualMachine, VMStackID, Base, $($param),*) -> LatentStatus
        where
            Base: PapyrusBase + PapyrusValidBase + 'static,
            $($param: PapyrusParameter + PapyrusParameterConvertible + 'static),*
        {
            const IS_STATIC: bool = Base::IS_STATIC;

            #[inline(always)]
            fn parameter_type_infos(vm: &IVirtualMachine) -> Option<Vec<TypeInfo>> {
                let _ = vm;
                Some(alloc::vec![$($param::parameter_type_info(vm)?),*])
            }

            #[inline(always)]
            unsafe fn dispatch(
                self,
                base_value: &mut Variable,
                vm: &mut VirtualMachine,
                stack_id: VMStackID,
                result_value: &mut Variable,
                frame: &StackFrame,
            ) -> bool {
                let _ = frame;
                let base = unsafe { Base::unpack_base(base_value) };
                $(let $arg = unsafe {
                    $param::unpack_parameter(
                        frame.get_stack_frame_variable($idx, frame.get_page_for_frame()),
                    )
                };)*
                result_value.set_bool(self(vm, stack_id, base, $($arg),*).0);
                true
            }
        }
    };
}

impl_papyrus_function_signature_short!();
impl_papyrus_function_signature_short!((A0, a0, 0));
impl_papyrus_function_signature_short!((A0, a0, 0), (A1, a1, 1));
impl_papyrus_function_signature_short!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2));
impl_papyrus_function_signature_short!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2), (A3, a3, 3));
impl_papyrus_function_signature_short!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4)
);
impl_papyrus_function_signature_short!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4),
    (A5, a5, 5)
);

impl_papyrus_function_signature_long!();
impl_papyrus_function_signature_long!((A0, a0, 0));
impl_papyrus_function_signature_long!((A0, a0, 0), (A1, a1, 1));
impl_papyrus_function_signature_long!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2));
impl_papyrus_function_signature_long!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2), (A3, a3, 3));
impl_papyrus_function_signature_long!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4)
);
impl_papyrus_function_signature_long!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4),
    (A5, a5, 5)
);

impl_papyrus_function_signature_latent!();
impl_papyrus_function_signature_latent!((A0, a0, 0));
impl_papyrus_function_signature_latent!((A0, a0, 0), (A1, a1, 1));
impl_papyrus_function_signature_latent!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2));
impl_papyrus_function_signature_latent!((A0, a0, 0), (A1, a1, 1), (A2, a2, 2), (A3, a3, 3));
impl_papyrus_function_signature_latent!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4)
);
impl_papyrus_function_signature_latent!(
    (A0, a0, 0),
    (A1, a1, 1),
    (A2, a2, 2),
    (A3, a3, 3),
    (A4, a4, 4),
    (A5, a5, 5)
);

impl IVirtualMachine {
    pub const RTTI: VariantID = RTTI_BSScript__IVirtualMachine;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSScript__IVirtualMachine;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_SET_LINKED_CALLBACK: usize = 0x01;
        pub fn set_linked_callback(callback: *mut ITypeLinkedCallback)
    }

    virtual_method! {
        pub const VFUNC_TRACE_STACK: usize = 0x02;
        pub fn trace_stack(message: *const c_char, stack_id: VMStackID, severity: Severity)
    }

    virtual_method! {
        pub const VFUNC_FORMAT_AND_POST_MESSAGE: usize = 0x03;
        pub fn format_and_post_message(message: *const c_char, severity: Severity)
    }

    virtual_method! {
        pub const VFUNC_UPDATE: usize = 0x04;
        pub fn update(budget: f32)
    }

    virtual_method! {
        pub const VFUNC_UPDATE_TASKLETS: usize = 0x05;
        pub fn update_tasklets(budget: f32)
    }

    virtual_method! {
        pub const VFUNC_SET_OVERSTRESSED: usize = 0x06;
        pub fn set_overstressed(set: bool)
    }

    virtual_method! {
        pub const VFUNC_IS_COMPLETELY_FROZEN: usize = 0x07;
        pub fn is_completely_frozen() -> bool
    }

    virtual_method! {
        pub const VFUNC_REGISTER_OBJECT_TYPE: usize = 0x08;
        pub fn register_object_type(type_id: VMTypeID, class_name: *const c_char) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_SCRIPT_OBJECT_TYPE1: usize = 0x09;
        pub fn get_script_object_type1(class_name: &BSFixedString, out_type_info: &mut BSTSmartPointer<ObjectTypeInfo>) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_SCRIPT_OBJECT_TYPE2: usize = 0x0A;
        pub fn get_script_object_type2(type_id: VMTypeID, out_type_info: &mut BSTSmartPointer<ObjectTypeInfo>) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_SCRIPT_OBJECT_TYPE_NO_LOAD1: usize = 0x0B;
        pub fn get_script_object_type_no_load1(class_name: &BSFixedString, out_type_info: &mut BSTSmartPointer<ObjectTypeInfo>) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_SCRIPT_OBJECT_TYPE_NO_LOAD2: usize = 0x0C;
        pub fn get_script_object_type_no_load2(type_id: VMTypeID, out_type_info: &mut BSTSmartPointer<ObjectTypeInfo>) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_TYPE_ID_FOR_SCRIPT_OBJECT: usize = 0x0D;
        pub fn get_type_id_for_script_object(class_name: &BSFixedString, type_id: &mut VMTypeID) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_SCRIPT_OBJECTS_WITH_TYPE_ID: usize = 0x0E;
        pub fn get_script_objects_with_a_type_id(classes: &mut BSScrapArray<BSFixedString>)
    }

    virtual_method! {
        pub const VFUNC_GET_PARENT_NATIVE_TYPE: usize = 0x0F;
        pub fn get_parent_native_type(class_name: &BSFixedString, out_type_info: &mut BSTSmartPointer<ObjectTypeInfo>) -> bool
    }

    virtual_method! {
        pub const VFUNC_TYPE_IS_VALID: usize = 0x10;
        pub fn type_is_valid(class_name: &BSFixedString) -> bool
    }

    virtual_method! {
        pub const VFUNC_RELOAD_TYPE: usize = 0x11;
        pub fn reload_type(class_name: *const c_char) -> bool
    }

    virtual_method! {
        pub const VFUNC_TASKS_TO_JOBS: usize = 0x12;
        pub fn tasks_to_jobs(job_list: &mut BSJobsJobList)
    }

    virtual_method! {
        pub const VFUNC_CALCULATE_FULL_RELOAD_LIST: usize = 0x13;
        pub fn calculate_full_reload_list()
    }

    virtual_method! {
        pub const VFUNC_CREATE_OBJECT1: usize = 0x14;
        pub fn create_object1(class_name: &BSFixedString, property: *mut c_void, out_obj: &mut BSTSmartPointer<Object>) -> bool
    }

    virtual_method! {
        pub const VFUNC_CREATE_OBJECT2: usize = 0x15;
        pub fn create_object2(class_name: &BSFixedString, out_obj: &mut BSTSmartPointer<Object>) -> bool
    }

    virtual_method! {
        pub const VFUNC_CREATE_ARRAY1: usize = 0x16;
        pub fn create_array1(type_info: &TypeInfo, size: u32, out_array: &mut BSTSmartPointer<Array>) -> bool
    }

    virtual_method! {
        pub const VFUNC_CREATE_ARRAY2: usize = 0x17;
        pub fn create_array2(type_id: RawType, class_name: &BSFixedString, size: u32, out_array: &mut BSTSmartPointer<Array>) -> bool
    }

    virtual_method! {
        pub const VFUNC_BIND_NATIVE_METHOD: usize = 0x18;
        pub fn bind_native_method(function: *mut IFunction) -> bool
    }

    virtual_method! {
        pub const VFUNC_SET_CALLABLE_FROM_TASKLETS1: usize = 0x19;
        pub fn set_callable_from_tasklets1(class_name: *const c_char, state_name: *const c_char, fn_name: *const c_char, callable: bool)
    }

    virtual_method! {
        pub const VFUNC_SET_CALLABLE_FROM_TASKLETS2: usize = 0x1A;
        pub fn set_callable_from_tasklets2(class_name: *const c_char, fn_name: *const c_char, callable: bool)
    }

    // vtbl SE/AE: 0x1B, VR: 0x1C
    virtual_method! {
        pub const VFUNC_FOR_EACH_BOUND_OBJECT: VariantOffset = VariantOffset::new_se_ae(0x1B, 0x1C);
        pub fn for_each_bound_object(handle: VMHandle, functor: *mut IForEachScriptObjectFunctor)
    }

    // vtbl SE/AE: 0x1C, VR: 0x1E
    virtual_method! {
        pub const VFUNC_FIND_BOUND_OBJECT: VariantOffset = VariantOffset::new_se_ae(0x1C, 0x1E);
        pub fn find_bound_object(handle: VMHandle, class_name: *const c_char, result: &mut BSTSmartPointer<Object>) -> bool
    }

    // vtbl SE/AE: 0x1D, VR: 0x1F
    virtual_method! {
        pub const VFUNC_MOVE_BOUND_OBJECTS: VariantOffset = VariantOffset::new_se_ae(0x1D, 0x1F);
        pub fn move_bound_objects(from: VMHandle, to: VMHandle)
    }

    // vtbl SE/AE: 0x1E, VR: 0x20
    virtual_method! {
        pub const VFUNC_RESET_ALL_BOUND_OBJECTS: VariantOffset = VariantOffset::new_se_ae(0x1E, 0x20);
        pub fn reset_all_bound_objects(handle: VMHandle)
    }

    // vtbl SE/AE: 0x1F, VR: 0x21
    virtual_method! {
        pub const VFUNC_CAST_OBJECT: VariantOffset = VariantOffset::new_se_ae(0x1F, 0x21);
        pub fn cast_object(from_obj: &BSTSmartPointer<Object>, to_type_info: &BSTSmartPointer<ObjectTypeInfo>, out_obj: &mut BSTSmartPointer<Object>) -> bool
    }

    // vtbl SE/AE: 0x20, VR: 0x22
    virtual_method! {
        pub const VFUNC_SET_PROPERTY_VALUE: VariantOffset = VariantOffset::new_se_ae(0x20, 0x22);
        pub fn set_property_value(object: &mut BSTSmartPointer<Object>, property_name: *const c_char, set_value: &mut Variable) -> bool
    }

    // vtbl SE/AE: 0x21, VR: 0x23
    virtual_method! {
        pub const VFUNC_GET_PROPERTY_VALUE: VariantOffset = VariantOffset::new_se_ae(0x21, 0x23);
        pub fn get_property_value(object: &mut BSTSmartPointer<Object>, property_name: *const c_char, out_value: &mut Variable) -> bool
    }

    // vtbl SE/AE: 0x22, VR: 0x24
    virtual_method! {
        pub const VFUNC_GET_VARIABLE_VALUE1: VariantOffset = VariantOffset::new_se_ae(0x22, 0x24);
        pub fn get_variable_value1(object: &BSTSmartPointer<Object>, index: u32, out_value: &mut Variable) -> bool
    }

    // vtbl SE/AE: 0x23, VR: 0x25
    virtual_method! {
        pub const VFUNC_GET_VARIABLE_VALUE2: VariantOffset = VariantOffset::new_se_ae(0x23, 0x25);
        pub fn get_variable_value2(handle: VMHandle, class_name: &BSFixedString, variable_index: i32, out_value: &mut Variable) -> bool
    }

    // vtbl SE/AE: 0x24, VR: 0x26
    virtual_method! {
        pub const VFUNC_SEND_EVENT: VariantOffset = VariantOffset::new_se_ae(0x24, 0x26);
        pub fn send_event(handle: VMHandle, event_name: &BSFixedString, args: *mut IFunctionArguments)
    }

    // vtbl SE/AE: 0x25, VR: 0x27
    virtual_method! {
        pub const VFUNC_SEND_EVENT_ALL: VariantOffset = VariantOffset::new_se_ae(0x25, 0x27);
        pub fn send_event_all(event_name: &BSFixedString, args: *mut IFunctionArguments)
    }

    // vtbl SE/AE: 0x26, VR: 0x28
    virtual_method! {
        pub const VFUNC_DISPATCH_STATIC_CALL: VariantOffset = VariantOffset::new_se_ae(0x26, 0x28);
        pub fn dispatch_static_call(class_name: &BSFixedString, fn_name: &BSFixedString, args: *mut IFunctionArguments, result: &mut BSTSmartPointer<IStackCallbackFunctor>) -> bool
    }

    // vtbl SE/AE: 0x27, VR: 0x29
    virtual_method! {
        pub const VFUNC_DISPATCH_METHOD_CALL1: VariantOffset = VariantOffset::new_se_ae(0x27, 0x29);
        pub fn dispatch_method_call1(object: &mut BSTSmartPointer<Object>, fn_name: &BSFixedString, args: *mut IFunctionArguments, result: &mut BSTSmartPointer<IStackCallbackFunctor>) -> bool
    }

    // vtbl SE/AE: 0x28, VR: 0x2A
    virtual_method! {
        pub const VFUNC_DISPATCH_METHOD_CALL2: VariantOffset = VariantOffset::new_se_ae(0x28, 0x2A);
        pub fn dispatch_method_call2(handle: VMHandle, class_name: &BSFixedString, fn_name: &BSFixedString, args: *mut IFunctionArguments, result: &mut BSTSmartPointer<IStackCallbackFunctor>) -> bool
    }

    // vtbl SE/AE: 0x29, VR: 0x2B
    virtual_method! {
        pub const VFUNC_DISPATCH_UNBOUND_METHOD_CALL: VariantOffset = VariantOffset::new_se_ae(0x29, 0x2B);
        pub fn dispatch_unbound_method_call() -> bool
    }

    // vtbl SE/AE: 0x2A, VR: 0x2C
    virtual_method! {
        pub const VFUNC_IS_WAITING_ON_LATENT: VariantOffset = VariantOffset::new_se_ae(0x2A, 0x2C);
        pub fn is_waiting_on_latent(stack_id: VMStackID) -> bool
    }

    // vtbl SE/AE: 0x2B, VR: 0x2D
    virtual_method! {
        pub const VFUNC_RETURN_FROM_LATENT: VariantOffset = VariantOffset::new_se_ae(0x2B, 0x2D);
        pub fn return_from_latent(stack_id: VMStackID, value: &Variable)
    }

    // vtbl SE/AE: 0x2C, VR: 0x2E
    virtual_method! {
        pub const VFUNC_GET_ERROR_LOGGER: VariantOffset = VariantOffset::new_se_ae(0x2C, 0x2E);
        pub fn get_error_logger() -> *mut ErrorLogger
    }

    // vtbl SE/AE: 0x2D, VR: 0x2F
    virtual_method! {
        pub const VFUNC_GET_OBJECT_HANDLE_POLICY1: VariantOffset = VariantOffset::new_se_ae(0x2D, 0x2F);
        pub fn get_object_handle_policy1() -> *mut IObjectHandlePolicy
    }

    // vtbl SE/AE: 0x2E, VR: 0x30
    virtual_method! {
        pub const VFUNC_GET_OBJECT_HANDLE_POLICY2: VariantOffset = VariantOffset::new_se_ae(0x2E, 0x30);
        pub fn get_object_handle_policy2() -> *const IObjectHandlePolicy
    }

    // vtbl SE/AE: 0x2F, VR: 0x31
    virtual_method! {
        pub const VFUNC_GET_OBJECT_BIND_POLICY1: VariantOffset = VariantOffset::new_se_ae(0x2F, 0x31);
        pub fn get_object_bind_policy1() -> *mut ObjectBindPolicy
    }

    // vtbl SE/AE: 0x30, VR: 0x32
    virtual_method! {
        pub const VFUNC_GET_OBJECT_BIND_POLICY2: VariantOffset = VariantOffset::new_se_ae(0x30, 0x32);
        pub fn get_object_bind_policy2() -> *const ObjectBindPolicy
    }

    // vtbl SE/AE: 0x31, VR: 0x33
    virtual_method! {
        pub const VFUNC_GET_SAVE_PATCHER_INTERFACE: VariantOffset = VariantOffset::new_se_ae(0x31, 0x33);
        pub fn get_save_patcher_interface() -> *mut ISavePatcherInterface
    }

    // vtbl SE/AE: 0x32, VR: 0x34
    virtual_method! {
        pub const VFUNC_REGISTER_FOR_LOG_EVENT: VariantOffset = VariantOffset::new_se_ae(0x32, 0x34);
        pub fn register_for_log_event(sink: *mut BSTEventSink<LogEvent>)
    }

    // vtbl SE/AE: 0x33, VR: 0x35
    virtual_method! {
        pub const VFUNC_UNREGISTER_FOR_LOG_EVENT: VariantOffset = VariantOffset::new_se_ae(0x33, 0x35);
        pub fn unregister_for_log_event(sink: *mut BSTEventSink<LogEvent>)
    }

    // vtbl SE/AE: 0x34, VR: 0x36
    virtual_method! {
        pub const VFUNC_REGISTER_FOR_STATS_EVENT: VariantOffset = VariantOffset::new_se_ae(0x34, 0x36);
        pub fn register_for_stats_event(sink: *mut BSTEventSink<StatsEvent>)
    }

    // vtbl SE/AE: 0x35, VR: 0x37
    virtual_method! {
        pub const VFUNC_UNREGISTER_FOR_STATS_EVENT: VariantOffset = VariantOffset::new_se_ae(0x35, 0x37);
        pub fn unregister_for_stats_event(sink: *mut BSTEventSink<StatsEvent>)
    }

    #[inline(always)]
    pub fn create_array(
        &self,
        type_info: &TypeInfo,
        size: u32,
        out_array: &mut BSTSmartPointer<Array>,
    ) -> bool {
        self.create_array1(type_info, size, out_array)
    }

    #[inline(always)]
    pub fn create_array_raw(
        &self,
        type_id: RawType,
        class_name: &BSFixedString,
        size: u32,
        out_array: &mut BSTSmartPointer<Array>,
    ) -> bool {
        self.create_array2(type_id, class_name, size, out_array)
    }

    #[inline(always)]
    pub fn create_object_with_property(
        &self,
        class_name: &BSFixedString,
        property: *mut c_void,
        out_obj: &mut BSTSmartPointer<Object>,
    ) -> bool {
        self.create_object1(class_name, property, out_obj)
    }

    #[inline(always)]
    pub fn create_object(
        &self,
        class_name: &BSFixedString,
        out_obj: &mut BSTSmartPointer<Object>,
    ) -> bool {
        self.create_object2(class_name, out_obj)
    }

    #[inline(always)]
    pub fn dispatch_method_call_object(
        &self,
        object: &mut BSTSmartPointer<Object>,
        fn_name: &BSFixedString,
        args: *mut IFunctionArguments,
        result: &mut BSTSmartPointer<IStackCallbackFunctor>,
    ) -> bool {
        self.dispatch_method_call1(object, fn_name, args, result)
    }

    pub fn adispatch_method_call_object(
        &self,
        object: &mut BSTSmartPointer<Object>,
        fn_name: &BSFixedString,
        args: *mut IFunctionArguments,
    ) -> Awaitable {
        let mut awaitable = Awaitable::new();
        awaitable.set_pending(true);
        if !self.dispatch_method_call1(object, fn_name, args, &mut awaitable.callback) {
            awaitable.set_pending(false);
        }
        awaitable
    }

    #[inline(always)]
    pub fn dispatch_method_call_handle(
        &self,
        handle: VMHandle,
        class_name: &BSFixedString,
        fn_name: &BSFixedString,
        args: *mut IFunctionArguments,
        result: &mut BSTSmartPointer<IStackCallbackFunctor>,
    ) -> bool {
        self.dispatch_method_call2(handle, class_name, fn_name, args, result)
    }

    pub fn adispatch_method_call_handle(
        &self,
        handle: VMHandle,
        class_name: &BSFixedString,
        fn_name: &BSFixedString,
        args: *mut IFunctionArguments,
    ) -> Awaitable {
        let mut awaitable = Awaitable::new();
        awaitable.set_pending(true);
        if !self.dispatch_method_call2(handle, class_name, fn_name, args, &mut awaitable.callback) {
            awaitable.set_pending(false);
        }
        awaitable
    }

    pub fn adispatch_static_call(
        &self,
        class_name: &BSFixedString,
        fn_name: &BSFixedString,
        args: *mut IFunctionArguments,
    ) -> Awaitable {
        let mut awaitable = Awaitable::new();
        awaitable.set_pending(true);
        if !self.dispatch_static_call(class_name, fn_name, args, &mut awaitable.callback) {
            awaitable.set_pending(false);
        }
        awaitable
    }

    #[inline(always)]
    pub fn get_object_bind_policy(&self) -> *mut ObjectBindPolicy {
        self.get_object_bind_policy1()
    }

    #[inline(always)]
    pub fn get_object_bind_policy_const(&self) -> *const ObjectBindPolicy {
        self.get_object_bind_policy2()
    }

    #[inline(always)]
    pub fn get_object_handle_policy(&self) -> *mut IObjectHandlePolicy {
        self.get_object_handle_policy1()
    }

    #[inline(always)]
    pub fn get_object_handle_policy_const(&self) -> *const IObjectHandlePolicy {
        self.get_object_handle_policy2()
    }

    #[inline(always)]
    pub fn get_script_object_type(
        &self,
        class_name: &BSFixedString,
        out_type_info: &mut BSTSmartPointer<ObjectTypeInfo>,
    ) -> bool {
        self.get_script_object_type1(class_name, out_type_info)
    }

    #[inline(always)]
    pub fn get_script_object_type_by_id(
        &self,
        type_id: VMTypeID,
        out_type_info: &mut BSTSmartPointer<ObjectTypeInfo>,
    ) -> bool {
        self.get_script_object_type2(type_id, out_type_info)
    }

    #[inline(always)]
    pub fn get_script_object_type_no_load(
        &self,
        class_name: &BSFixedString,
        out_type_info: &mut BSTSmartPointer<ObjectTypeInfo>,
    ) -> bool {
        self.get_script_object_type_no_load1(class_name, out_type_info)
    }

    #[inline(always)]
    pub fn get_script_object_type_no_load_by_id(
        &self,
        type_id: VMTypeID,
        out_type_info: &mut BSTSmartPointer<ObjectTypeInfo>,
    ) -> bool {
        self.get_script_object_type_no_load2(type_id, out_type_info)
    }

    #[inline(always)]
    pub fn get_variable_value(
        &self,
        object: &BSTSmartPointer<Object>,
        index: u32,
        out_value: &mut Variable,
    ) -> bool {
        self.get_variable_value1(object, index, out_value)
    }

    #[inline(always)]
    pub fn get_variable_value_by_handle(
        &self,
        handle: VMHandle,
        class_name: &BSFixedString,
        variable_index: i32,
        out_value: &mut Variable,
    ) -> bool {
        self.get_variable_value2(handle, class_name, variable_index, out_value)
    }

    #[inline(always)]
    pub fn set_callable_from_tasklets_state(
        &self,
        class_name: *const c_char,
        state_name: *const c_char,
        fn_name: *const c_char,
        callable: bool,
    ) {
        self.set_callable_from_tasklets1(class_name, state_name, fn_name, callable);
    }

    #[inline(always)]
    pub fn set_callable_from_tasklets_simple(
        &self,
        class_name: *const c_char,
        fn_name: *const c_char,
        callable: bool,
    ) {
        self.set_callable_from_tasklets2(class_name, fn_name, callable);
    }

    pub fn trace_form(
        &self,
        form: *mut TESForm,
        message: *const c_char,
        stack_id: VMStackID,
        severity: Severity,
    ) {
        assert!(!message.is_null());

        let mut name = BSFixedString::default();
        if !form.is_null() {
            let policy = self.get_object_handle_policy();
            if !policy.is_null() {
                let form_ref = unsafe { &*form };
                let handle =
                    unsafe { (*policy).get_handle_for_form(form_ref.get_form_type(), form) };
                unsafe {
                    (*policy).convert_handle_to_string(handle, &mut name);
                }
            }
        }

        if name.is_empty() {
            name = BSFixedString::from_str("None");
        }

        let message = unsafe { CStr::from_ptr(message) }.to_str().unwrap_or("");

        let formatted = format!("{}: {}", name, message);
        if let Ok(c_message) = CString::new(formatted) {
            self.trace_stack(c_message.as_ptr(), stack_id, severity);
        }
    }

    #[inline]
    pub fn vtrace_stack_message(
        &self,
        stack_id: VMStackID,
        severity: Severity,
        message: *const c_char,
    ) {
        if !message.is_null() {
            self.trace_stack(message, stack_id, severity);
        }
    }

    #[inline]
    pub fn vtrace_stack(
        &self,
        stack_id: VMStackID,
        severity: Severity,
        args: core::fmt::Arguments<'_>,
    ) {
        let formatted = format!("{args}");
        if let Ok(c_message) = CString::new(formatted) {
            self.trace_stack(c_message.as_ptr(), stack_id, severity);
        }
    }

    #[inline]
    pub fn bind_native_function(
        &self,
        fn_name: &str,
        class_name: &str,
        function: NativeFunction,
        callable_from_tasklets: bool,
    ) -> bool {
        let raw = function.into_raw();
        if !self.bind_native_method(raw.cast()) {
            unsafe {
                crate::ffi::commonlib_native_function_destroy(raw.cast());
            }
            return false;
        }

        if callable_from_tasklets {
            let Ok(c_class_name) = CString::new(class_name) else {
                return false;
            };
            let Ok(c_fn_name) = CString::new(fn_name) else {
                return false;
            };
            self.set_callable_from_tasklets_simple(c_class_name.as_ptr(), c_fn_name.as_ptr(), true);
        }

        true
    }

    #[inline]
    pub fn bind_native_latent_function(
        &self,
        fn_name: &str,
        class_name: &str,
        function: NativeLatentFunction,
        callable_from_tasklets: bool,
    ) -> bool {
        let raw = function.into_raw();
        if !self.bind_native_method(raw.cast()) {
            unsafe {
                crate::ffi::commonlib_native_function_destroy(raw.cast());
            }
            return false;
        }

        if callable_from_tasklets {
            let Ok(c_class_name) = CString::new(class_name) else {
                return false;
            };
            let Ok(c_fn_name) = CString::new(fn_name) else {
                return false;
            };
            self.set_callable_from_tasklets_simple(c_class_name.as_ptr(), c_fn_name.as_ptr(), true);
        }

        true
    }

    #[inline]
    pub fn register_function<F>(
        &self,
        fn_name: &str,
        class_name: &str,
        callback: F,
        callable_from_tasklets: bool,
    ) -> bool
    where
        F: PapyrusFunctionSignature,
    {
        let Some(param_types) = F::parameter_type_infos(self) else {
            return false;
        };
        let Some(return_type) = F::return_type_info(self) else {
            return false;
        };
        let Some(function) = NativeFunction::new(
            NativeFunctionDesc {
                fn_name,
                class_name,
                is_static: F::IS_STATIC,
                return_type,
                param_types: &param_types,
            },
            RegisteredNativeFunctionHandler { callback },
        ) else {
            return false;
        };

        self.bind_native_function(fn_name, class_name, function, callable_from_tasklets)
    }

    #[inline]
    pub fn register_long_function<F>(
        &self,
        fn_name: &str,
        class_name: &str,
        callback: F,
        callable_from_tasklets: bool,
    ) -> bool
    where
        F: PapyrusLongFunctionSignature,
    {
        let Some(param_types) = F::parameter_type_infos(self) else {
            return false;
        };
        let Some(return_type) = F::return_type_info(self) else {
            return false;
        };
        let Some(function) = NativeFunction::new(
            NativeFunctionDesc {
                fn_name,
                class_name,
                is_static: F::IS_STATIC,
                return_type,
                param_types: &param_types,
            },
            RegisteredLongNativeFunctionHandler { callback },
        ) else {
            return false;
        };

        self.bind_native_function(fn_name, class_name, function, callable_from_tasklets)
    }

    #[inline]
    pub fn register_latent_function<R, F>(
        &self,
        fn_name: &str,
        class_name: &str,
        callback: F,
        callable_from_tasklets: bool,
    ) -> bool
    where
        R: PapyrusReturn + PapyrusReturnConvertible + 'static,
        F: PapyrusLatentFunctionSignature,
    {
        let Some(param_types) = F::parameter_type_infos(self) else {
            return false;
        };
        let Some(latent_return_type) = R::return_type_info(self) else {
            return false;
        };
        let Some(function) = NativeLatentFunction::new(
            NativeLatentFunctionDesc {
                fn_name,
                class_name,
                is_static: F::IS_STATIC,
                latent_return_type,
                param_types: &param_types,
            },
            RegisteredLatentNativeFunctionHandler { callback },
        ) else {
            return false;
        };

        self.bind_native_latent_function(fn_name, class_name, function, callable_from_tasklets)
    }

    #[inline]
    pub fn return_latent_result<R>(&self, stack_id: VMStackID, result: R) -> bool
    where
        R: PapyrusReturn + PapyrusReturnConvertible,
    {
        let vm = VirtualMachine::get_singleton();
        if vm.is_null() {
            return false;
        }

        let vm = unsafe { &mut *vm };
        let mut value = Variable::default();
        if !R::pack_return(result, &mut value, vm) {
            return false;
        }

        self.return_from_latent(stack_id, &value);
        true
    }

    #[inline(always)]
    pub fn return_latent_variable(&self, stack_id: VMStackID, result: &Variable) {
        self.return_from_latent(stack_id, result);
    }
}
use crate::re::bsscript_array::Array;
use crate::re::bsscript_object::Object;
use crate::re::bsscript_object_bind_policy::ObjectBindPolicy;
use crate::re::bsscript_object_type_info::ObjectTypeInfo;
