#![allow(non_camel_case_types)]

use crate::re::{
    BGSTextureSet, NiAVObject, NiNode, NiPoint3, NiPointer, ObjectRefHandle, TESObjectCELL,
    bst_array::BSTArray,
};

/// C++ `RE::DECAL_CREATION_DATA`
#[repr(C)]
pub struct BGSDecalGroupDecalCreationData {
    pub origin: NiPoint3,                // 00
    pub direction: NiPoint3,             // 0C
    pub surface_normal: NiPoint3,        // 18
    pub obj_ref: ObjectRefHandle,        // 24
    pub av_obj: NiPointer<NiAVObject>,   // 28
    pub clone: *mut NiNode,              // 30
    pub tex_set: *mut BGSTextureSet,     // 38
    pub tex_set2: *mut BGSTextureSet,    // 40
    pub unk48: u64,                      // 48
    pub unk50: u64,                      // 50
    pub unk58: f32,                      // 58
    pub unk5c: u32,                      // 5C
    pub unk60: u64,                      // 60
    pub unk68: f32,                      // 68
    pub unk6c: u32,                      // 6C
    pub unk70: u64,                      // 70
    pub unk78: f32,                      // 78
    pub unk7c: u32,                      // 7C
    pub parent_cell: *mut TESObjectCELL, // 80
    pub unk88: u64,                      // 88
    pub unk90: u64,                      // 90
    pub unk98: f32,                      // 98
    pub unk9c: f32,                      // 9C
    pub unk_a0: f32,                     // A0
    pub unk_a4: f32,                     // A4
    pub unk_a8: u32,                     // A8
    pub unk_ac: f32,                     // AC
    pub unk_b0: u32,                     // B0
    pub unk_b4: u16,                     // B4
    pub unk_b6: u8,                      // B6
    pub unk_b7: u8,                      // B7
    pub unk_b8: u16,                     // B8
    pub unk_ba: u8,                      // BA
    pub unk_bb: u8,                      // BB
    pub unk_bc: u16,                     // BC
    pub unk_be: u8,                      // BE
    pub unk_c0: u32,                     // C0
    pub pad_c4: u32,                     // C4
}

const _: () = assert!(core::mem::size_of::<BGSDecalGroupDecalCreationData>() == 0xC8);
const _: () = assert!(core::mem::offset_of!(BGSDecalGroupDecalCreationData, obj_ref) == 0x24);
const _: () = assert!(core::mem::offset_of!(BGSDecalGroupDecalCreationData, av_obj) == 0x28);
const _: () = assert!(core::mem::offset_of!(BGSDecalGroupDecalCreationData, clone) == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSDecalGroupDecalCreationData, parent_cell) == 0x80);

/// C++ `RE::BGSDecalGroup`
#[repr(C)]
pub struct BGSDecalGroup {
    pub permanent_group: bool,                                         // 00
    pub manual_save_load: bool,                                        // 01
    pub pad02: u16,                                                    // 02
    pub pad04: u32,                                                    // 04
    pub decal_groups: BSTArray<u32>,                                   // 08
    pub pending_decals: BSTArray<*mut BGSDecalGroupDecalCreationData>, // 20
}

const _: () = assert!(core::mem::size_of::<BGSDecalGroup>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BGSDecalGroup, decal_groups) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSDecalGroup, pending_decals) == 0x20);
