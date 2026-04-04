use crate::re::{
    BSFixedString, BSTSmartPointer, Object, ObjectTypeInfo, PapyrusReturn,
    PapyrusReturnConvertible, TESForm, VMHandle, VMTypeID, Variable, pack_unpack,
};
use crate::sdk::core::GamePtr;

use super::access::{form_handle, is_empty_handle, virtual_machine, vm_interface};
use super::shared::{fixed, null_object, null_type_info};

#[inline]
pub fn script_type_id(class_name: &str) -> Option<VMTypeID> {
    let class_name = fixed(class_name);
    let mut type_id = 0;
    if vm_interface().get_type_id_for_script_object(&class_name, &mut type_id) {
        Some(type_id)
    } else {
        None
    }
}

#[inline]
pub fn script_type(class_name: &str) -> BSTSmartPointer<ObjectTypeInfo> {
    let class_name = fixed(class_name);
    let mut type_info = null_type_info();
    if vm_interface().get_script_object_type(&class_name, &mut type_info) {
        type_info
    } else {
        null_type_info()
    }
}

#[inline]
pub fn script_type_no_load(class_name: &str) -> BSTSmartPointer<ObjectTypeInfo> {
    let class_name = fixed(class_name);
    let mut type_info = null_type_info();
    if vm_interface().get_script_object_type_no_load(&class_name, &mut type_info) {
        type_info
    } else {
        null_type_info()
    }
}

#[inline]
pub fn script_type_by_id(type_id: VMTypeID) -> BSTSmartPointer<ObjectTypeInfo> {
    let mut type_info = null_type_info();
    if vm_interface().get_script_object_type_by_id(type_id, &mut type_info) {
        type_info
    } else {
        null_type_info()
    }
}

#[inline]
pub fn create_object(class_name: &str) -> BSTSmartPointer<Object> {
    let class_name = fixed(class_name);
    let mut object = null_object();
    if vm_interface().create_object(&class_name, &mut object) {
        object
    } else {
        null_object()
    }
}

#[inline]
pub fn find_bound_object(handle: VMHandle, class_name: &str) -> BSTSmartPointer<Object> {
    if is_empty_handle(handle) {
        return null_object();
    }

    let class_name = fixed(class_name);
    let mut object = null_object();
    if vm_interface().find_bound_object(handle, class_name.as_ptr(), &mut object) {
        object
    } else {
        null_object()
    }
}

#[inline]
pub fn find_form_bound_object(
    form: impl Into<GamePtr<TESForm>>,
    class_name: &str,
) -> BSTSmartPointer<Object> {
    let Some(handle) = form_handle(form) else {
        return null_object();
    };
    find_bound_object(handle, class_name)
}

#[inline]
pub fn find_or_create_form_bound_object(
    form: impl Into<GamePtr<TESForm>>,
    class_name: &str,
) -> BSTSmartPointer<Object> {
    let form = form.into();
    let Some(form) = form.into_option() else {
        return null_object();
    };
    let Some(handle) = form_handle(form.into_ptr()) else {
        return null_object();
    };

    let mut object = find_bound_object(handle, class_name);
    if !object.is_null() {
        return object;
    }

    object = create_object(class_name);
    if object.is_null() {
        return object;
    }

    pack_unpack::bind_id(
        &mut object,
        form.as_ptr().cast(),
        form.get_form_type() as VMTypeID,
    );
    object
}

#[inline]
pub fn object_class_name(object: &Object) -> BSFixedString {
    let type_info = object.get_type_info();
    if type_info.is_null() {
        BSFixedString::default()
    } else {
        unsafe { (*type_info).name.clone() }
    }
}

#[inline]
pub fn object_property_value(object: &Object, property_name: &str) -> Option<Variable> {
    let property_name = fixed(property_name);
    let variable = object.get_property(&property_name);
    unsafe { variable.as_ref().cloned() }
}

#[inline]
pub fn object_variable_value(object: &Object, variable_name: &str) -> Option<Variable> {
    let variable_name = fixed(variable_name);
    let variable = object.get_variable(&variable_name);
    unsafe { variable.as_ref().cloned() }
}

#[inline]
pub fn set_object_property<T>(object: &Object, property_name: &str, value: T) -> bool
where
    T: PapyrusReturn + PapyrusReturnConvertible,
{
    let property_name = fixed(property_name);
    let Some(variable) = (unsafe { object.get_property(&property_name).as_mut() }) else {
        return false;
    };

    unsafe { virtual_machine().with_mut_unchecked(|vm| T::pack_return(value, variable, vm)) }
}

#[inline]
pub fn set_object_variable<T>(object: &Object, variable_name: &str, value: T) -> bool
where
    T: PapyrusReturn + PapyrusReturnConvertible,
{
    let variable_name = fixed(variable_name);
    let Some(variable) = (unsafe { object.get_variable(&variable_name).as_mut() }) else {
        return false;
    };

    unsafe { virtual_machine().with_mut_unchecked(|vm| T::pack_return(value, variable, vm)) }
}

#[inline]
pub fn bound_property_value(
    handle: VMHandle,
    class_name: &str,
    property_name: &str,
) -> Option<Variable> {
    let object = find_bound_object(handle, class_name);
    let Some(object) = (unsafe { object.get().as_ref() }) else {
        return None;
    };
    object_property_value(object, property_name)
}

#[inline]
pub fn bound_variable_value(
    handle: VMHandle,
    class_name: &str,
    variable_name: &str,
) -> Option<Variable> {
    let object = find_bound_object(handle, class_name);
    let Some(object) = (unsafe { object.get().as_ref() }) else {
        return None;
    };
    object_variable_value(object, variable_name)
}

#[inline]
pub fn form_bound_property_value(
    form: impl Into<GamePtr<TESForm>>,
    class_name: &str,
    property_name: &str,
) -> Option<Variable> {
    let Some(handle) = form_handle(form) else {
        return None;
    };
    bound_property_value(handle, class_name, property_name)
}

#[inline]
pub fn form_bound_variable_value(
    form: impl Into<GamePtr<TESForm>>,
    class_name: &str,
    variable_name: &str,
) -> Option<Variable> {
    let Some(handle) = form_handle(form) else {
        return None;
    };
    bound_variable_value(handle, class_name, variable_name)
}
