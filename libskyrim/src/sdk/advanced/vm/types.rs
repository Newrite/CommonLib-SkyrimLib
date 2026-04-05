use alloc::vec::Vec;

use crate::re::{
    BSTSmartPointer, FunctionArguments, IStackCallbackFunctor, PapyrusReturn,
    PapyrusReturnConvertible, Variable, VirtualMachine,
};
use crate::sdk::core::GamePtr;

/// Public VM-argument surface accepted by the SDK dispatch helpers.
///
/// This trait is the bridge between ergonomic Rust-side call sites and the
/// lower-level [`FunctionArguments`] container expected by the Papyrus VM.
/// Most callers never implement this trait manually; they pass one of the
/// already-supported shapes instead:
///
/// - `()` for no arguments
/// - tuple packs of Papyrus-convertible values
/// - raw `Vec<Variable>` when arguments are already packed
/// - prebuilt [`FunctionArguments`] when the caller wants full control
pub trait VmArguments {
    /// Packs the caller-provided argument shape into a VM-owned argument list.
    ///
    /// Returning [`None`] means at least one argument could not be packed into
    /// a [`Variable`] for the current VM, so the dispatch helper should abort
    /// before attempting the call.
    fn into_function_arguments(self, vm: &mut VirtualMachine) -> Option<FunctionArguments>;
}

impl VmArguments for FunctionArguments {
    #[inline(always)]
    fn into_function_arguments(self, _vm: &mut VirtualMachine) -> Option<FunctionArguments> {
        Some(self)
    }
}

impl VmArguments for Vec<Variable> {
    #[inline(always)]
    fn into_function_arguments(self, _vm: &mut VirtualMachine) -> Option<FunctionArguments> {
        FunctionArguments::from_variables(self)
    }
}

impl VmArguments for () {
    #[inline(always)]
    fn into_function_arguments(self, _vm: &mut VirtualMachine) -> Option<FunctionArguments> {
        FunctionArguments::empty()
    }
}

macro_rules! impl_vm_arguments_tuple {
    ($(($ty:ident, $var:ident)),+ $(,)?) => {
        impl<$($ty),+> VmArguments for ($($ty,)+)
        where
            $($ty: PapyrusReturn + PapyrusReturnConvertible,)+
        {
            #[inline(always)]
            fn into_function_arguments(self, vm: &mut VirtualMachine) -> Option<FunctionArguments> {
                let ($($var,)+) = self;
                let mut values = Vec::new();
                $(
                    let mut value = Variable::default();
                    if !$var.pack_return(&mut value, vm) {
                        return None;
                    }
                    values.push(value);
                )+
                FunctionArguments::from_variables(values)
            }
        }
    };
}

impl_vm_arguments_tuple!((A0, a0));
impl_vm_arguments_tuple!((A0, a0), (A1, a1));
impl_vm_arguments_tuple!((A0, a0), (A1, a1), (A2, a2));
impl_vm_arguments_tuple!((A0, a0), (A1, a1), (A2, a2), (A3, a3));
impl_vm_arguments_tuple!((A0, a0), (A1, a1), (A2, a2), (A3, a3), (A4, a4));
impl_vm_arguments_tuple!((A0, a0), (A1, a1), (A2, a2), (A3, a3), (A4, a4), (A5, a5));
impl_vm_arguments_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6)
);
impl_vm_arguments_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7)
);
impl_vm_arguments_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8)
);
impl_vm_arguments_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9)
);
impl_vm_arguments_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9),
    (A10, a10)
);
impl_vm_arguments_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9),
    (A10, a10),
    (A11, a11)
);
impl_vm_arguments_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9),
    (A10, a10),
    (A11, a11),
    (A12, a12)
);
impl_vm_arguments_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9),
    (A10, a10),
    (A11, a11),
    (A12, a12),
    (A13, a13)
);
impl_vm_arguments_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9),
    (A10, a10),
    (A11, a11),
    (A12, a12),
    (A13, a13),
    (A14, a14)
);
impl_vm_arguments_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9),
    (A10, a10),
    (A11, a11),
    (A12, a12),
    (A13, a13),
    (A14, a14),
    (A15, a15)
);

/// Outcome of a VM method/static dispatch request.
///
/// This reports whether the request was accepted by the VM and, when
/// available, carries the callback functor that can later be observed or
/// awaited by higher-level helpers.
#[derive(Default)]
pub struct VmDispatchOutcome {
    /// Whether the VM accepted the dispatch request.
    ///
    /// This does not mean the target method finished running successfully; it
    /// only means the request was submitted.
    pub dispatched: bool,
    /// Optional callback functor produced by the VM for this dispatch.
    ///
    /// Fire-and-forget paths may leave this null even when [`Self::dispatched`]
    /// is `true`.
    pub callback: BSTSmartPointer<IStackCallbackFunctor>,
}

impl VmDispatchOutcome {
    /// Creates a new dispatch outcome from the raw VM submission result.
    #[inline(always)]
    pub const fn new(dispatched: bool, callback: BSTSmartPointer<IStackCallbackFunctor>) -> Self {
        Self {
            dispatched,
            callback,
        }
    }

    /// Returns whether the VM accepted the request for dispatch.
    #[inline(always)]
    pub const fn was_dispatched(&self) -> bool {
        self.dispatched
    }

    /// Returns the callback functor as a nullable gameplay pointer.
    ///
    /// This is useful when a caller wants to inspect or forward the callback
    /// without taking ownership of the underlying smart pointer.
    #[inline(always)]
    pub fn callback_ptr(&self) -> GamePtr<IStackCallbackFunctor> {
        unsafe { GamePtr::from_raw(self.callback.get()) }
    }

    /// Returns whether the dispatch produced a callback functor.
    #[inline(always)]
    pub fn has_callback(&self) -> bool {
        !self.callback.is_null()
    }

    /// Consumes the outcome and returns the owned callback functor.
    #[inline(always)]
    pub fn into_callback(self) -> BSTSmartPointer<IStackCallbackFunctor> {
        self.callback
    }
}
