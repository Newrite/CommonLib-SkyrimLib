#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_hkbVariableValueSet;
use crate::offsets::offsets_vtable::VTABLE_hkbVariableValueSet;
use crate::re::{hkArray, hkRefVariant, hkReferencedObject, hkVector4};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::hkbVariableValue`
#[repr(C)]
#[derive(Clone, Copy)]
pub union hkbVariableValue {
    pub b: bool,
    pub i: i32,
    pub f: f32,
}

const _: () = assert!(core::mem::size_of::<hkbVariableValue>() == 0x04);

/// C++ `RE::hkbVariableBounds`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct hkbVariableBounds {
    pub min: hkbVariableValue, // 00
    pub max: hkbVariableValue, // 04
}

const _: () = assert!(core::mem::size_of::<hkbVariableBounds>() == 0x08);

/// Partial C++ `RE::hkbVariableValueSet`
#[repr(C)]
pub struct hkbVariableValueSet {
    pub base: hkReferencedObject,                        // 00
    pub word_variable_values: hkArray<hkbVariableValue>, // 10
    pub quad_variable_values: hkArray<hkVector4>,        // 20
    pub variant_variable_values: hkArray<hkRefVariant>,  // 30
}

const _: () = assert!(core::mem::size_of::<hkbVariableValueSet>() == 0x40);
const _: () = assert!(core::mem::offset_of!(hkbVariableValueSet, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkbVariableValueSet, word_variable_values) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkbVariableValueSet, quad_variable_values) == 0x20);
const _: () = assert!(core::mem::offset_of!(hkbVariableValueSet, variant_variable_values) == 0x30);

impl RttiType for hkbVariableValueSet {
    const RTTI: VariantID = RTTI_hkbVariableValueSet;
}

inherit!(hkbVariableValueSet : hkReferencedObject, base);

impl hkbVariableValueSet {
    pub const RTTI: VariantID = RTTI_hkbVariableValueSet;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkbVariableValueSet;
}
