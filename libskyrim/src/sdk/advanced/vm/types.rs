use alloc::vec::Vec;

use crate::re::{
    BSTSmartPointer, FunctionArguments, IStackCallbackFunctor, PapyrusReturn,
    PapyrusReturnConvertible, Variable, VirtualMachine,
};
use crate::sdk::core::GamePtr;

/// Public VM-argument surface accepted by the SDK dispatch helpers.
///
/// This supports:
/// - `()`
/// - tuple packs of Papyrus-convertible values
/// - raw `Vec<Variable>`
/// - prebuilt [`FunctionArguments`]
pub trait VmArguments {
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
#[derive(Default)]
pub struct VmDispatchOutcome {
    pub dispatched: bool,
    pub callback: BSTSmartPointer<IStackCallbackFunctor>,
}

impl VmDispatchOutcome {
    #[inline(always)]
    pub const fn new(dispatched: bool, callback: BSTSmartPointer<IStackCallbackFunctor>) -> Self {
        Self {
            dispatched,
            callback,
        }
    }

    #[inline(always)]
    pub const fn was_dispatched(&self) -> bool {
        self.dispatched
    }

    #[inline(always)]
    pub fn callback_ptr(&self) -> GamePtr<IStackCallbackFunctor> {
        unsafe { GamePtr::from_raw(self.callback.get()) }
    }

    #[inline(always)]
    pub fn has_callback(&self) -> bool {
        !self.callback.is_null()
    }

    #[inline(always)]
    pub fn into_callback(self) -> BSTSmartPointer<IStackCallbackFunctor> {
        self.callback
    }
}
