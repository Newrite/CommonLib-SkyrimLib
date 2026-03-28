use alloc::vec::Vec;

use crate::re::{FunctionArguments, PapyrusReturnConvertible, Variable, VirtualMachine};

pub trait RegistrationEventArgs: Clone {
    fn to_function_arguments(&self, vm: &mut VirtualMachine) -> Option<FunctionArguments>;
}

impl RegistrationEventArgs for () {
    #[inline(always)]
    fn to_function_arguments(&self, _vm: &mut VirtualMachine) -> Option<FunctionArguments> {
        FunctionArguments::empty()
    }
}

macro_rules! impl_registration_event_args_tuple {
    ($(($ty:ident, $var:ident)),+ $(,)?) => {
        impl<$($ty),+> RegistrationEventArgs for ($($ty,)+)
        where
            $($ty: PapyrusReturnConvertible + Clone,)+
        {
            #[inline(always)]
            fn to_function_arguments(&self, vm: &mut VirtualMachine) -> Option<FunctionArguments> {
                let ($($var,)+) = self;
                let mut values = Vec::new();
                $(
                    let mut value = Variable::default();
                    if !$var.clone().pack_return(&mut value, vm) {
                        return None;
                    }
                    values.push(value);
                )+
                FunctionArguments::from_variables(values)
            }
        }
    };
}

impl_registration_event_args_tuple!((A0, a0));
impl_registration_event_args_tuple!((A0, a0), (A1, a1));
impl_registration_event_args_tuple!((A0, a0), (A1, a1), (A2, a2));
impl_registration_event_args_tuple!((A0, a0), (A1, a1), (A2, a2), (A3, a3));
impl_registration_event_args_tuple!((A0, a0), (A1, a1), (A2, a2), (A3, a3), (A4, a4));
impl_registration_event_args_tuple!((A0, a0), (A1, a1), (A2, a2), (A3, a3), (A4, a4), (A5, a5));
impl_registration_event_args_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6)
);
impl_registration_event_args_tuple!(
    (A0, a0),
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7)
);
impl_registration_event_args_tuple!(
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
impl_registration_event_args_tuple!(
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
impl_registration_event_args_tuple!(
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
impl_registration_event_args_tuple!(
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
impl_registration_event_args_tuple!(
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
impl_registration_event_args_tuple!(
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
impl_registration_event_args_tuple!(
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
impl_registration_event_args_tuple!(
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

#[inline(always)]
pub(crate) fn with_vm<T>(f: impl FnOnce(&mut VirtualMachine) -> T) -> Option<T> {
    let vm = VirtualMachine::get_singleton();
    if vm.is_null() {
        None
    } else {
        Some(f(unsafe { &mut *vm }))
    }
}
