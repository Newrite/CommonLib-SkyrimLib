use crate::re::{BSFixedString, BSTSmartPointer, IFunctionArguments, Object, ObjectTypeInfo};

use super::access::virtual_machine;
use super::types::VmArguments;

#[inline(always)]
pub(crate) fn fixed(value: &str) -> BSFixedString {
    BSFixedString::from_str(value)
}

#[inline(always)]
pub(crate) fn null_object() -> BSTSmartPointer<Object> {
    BSTSmartPointer::default()
}

#[inline(always)]
pub(crate) fn null_type_info() -> BSTSmartPointer<ObjectTypeInfo> {
    BSTSmartPointer::default()
}

pub(crate) fn with_vm_arguments<Args, R>(
    args: Args,
    f: impl FnOnce(&mut crate::re::VirtualMachine, *mut IFunctionArguments) -> R,
) -> Option<R>
where
    Args: VmArguments,
{
    unsafe {
        virtual_machine().with_mut_unchecked(|vm| {
            let arguments = args.into_function_arguments(vm)?;
            Some(f(vm, arguments.as_ptr()))
        })
    }
}
