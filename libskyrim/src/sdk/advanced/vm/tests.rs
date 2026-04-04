use alloc::vec::Vec;

use super::{VmArguments, VmDispatchOutcome};
use crate::re::{FunctionArguments, Variable};

#[test]
fn vm_dispatch_outcome_defaults_to_empty() {
    let outcome = VmDispatchOutcome::default();
    assert!(!outcome.was_dispatched());
    assert!(!outcome.has_callback());
    assert!(outcome.callback_ptr().is_null());
}

#[test]
fn vm_arguments_trait_is_implemented_for_sdk_supported_inputs() {
    fn assert_vm_arguments<T: VmArguments>() {}

    assert_vm_arguments::<()>();
    assert_vm_arguments::<(i32,)>();
    assert_vm_arguments::<(i32, bool)>();
    assert_vm_arguments::<FunctionArguments>();
    assert_vm_arguments::<Vec<Variable>>();
}
