#![allow(non_camel_case_types)]

use core_util::EnumSet;

use crate::re::hkHalf;

/// C++ `RE::hkpMaterial::ResponseType`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpMaterialResponseType {
    Invalid = 0,
    SimpleContact = 1,
    Reporting = 2,
    None = 3,
    Total = 4,
}

core_util::impl_enumset_type!(hkpMaterialResponseType => u8);

/// C++ `RE::hkpMaterial`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkpMaterial {
    pub response_type: EnumSet<hkpMaterialResponseType, u8>, // 00
    pub pad01: u8,                                           // 01
    pub rolling_friction_multiplier: hkHalf,                 // 02
    pub friction: f32,                                       // 04
    pub restitution: f32,                                    // 08
}

const _: () = assert!(core::mem::size_of::<hkpMaterial>() == 0x0C);
