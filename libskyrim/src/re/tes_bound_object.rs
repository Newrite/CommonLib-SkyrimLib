use core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_TESBoundObject;
use crate::offsets::offsets_vtable::VTABLE_TESBoundObject;
use crate::re::tes_object::TESObject;
use crate::re::tes_file::TESFile;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::re::bgs_destructible_object_form::BGSDestructibleObjectForm;
use crate::re::ni_av_object::NiAVObject;
use crate::re::bgs_voice_type::BGSVoiceType;
use crate::re::bs_string::BSString;
use crate::re::actor::Actor;

use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, bytemuck::Zeroable)]
pub struct NiNPShortPoint3 {
    pub x: i16, // 0
    pub y: i16, // 2
    pub z: i16, // 4
}
const _: () = assert!(core::mem::size_of::<NiNPShortPoint3>() == 0x6);

#[repr(C)]
#[derive(bytemuck::Zeroable)]
pub struct BoundData { // OBND
    pub bound_min: NiNPShortPoint3, // 0
    pub bound_max: NiNPShortPoint3, // 6
}
const _: () = assert!(core::mem::size_of::<BoundData>() == 0xC);

#[repr(C)]
pub struct TESBoundObject {
    pub base: TESObject, // 00
    pub bound_data: BoundData, // 20
    pub pad2c: u32, // 2C
}

const _: () = assert!(core::mem::size_of::<TESBoundObject>() == 0x30);

impl RttiType for TESBoundObject {
    const RTTI: VariantID = RTTI_TESBoundObject;
}

inherit!(TESBoundObject : TESObject);

impl TESBoundObject {
    pub const RTTI: VariantID = RTTI_TESBoundObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESBoundObject;

    virtual_method! {
        pub const LOAD_OBJECT_BOUND: usize = 0x26;
        pub fn load_object_bound(this: &TESBoundObject, a_mod: *mut TESFile) -> ()
    }

    virtual_method! {
        pub const IS_BOUND_OBJECT: usize = 0x27;
        pub fn is_bound_object(this: &TESBoundObject) -> bool
    }

    virtual_method! {
        pub const ACTIVATE: usize = 0x37;
        pub fn activate(this: &TESBoundObject, a_target_ref: *mut TESObjectREFR, a_activator_ref: *mut TESObjectREFR, a_arg3: u8, a_object: *mut TESBoundObject, a_target_count: i32) -> bool
    }

    virtual_method! {
        pub const CLONE_3D_OVERRIDE: usize = 0x40;
        pub fn clone_3d_override(this: &TESBoundObject, a_ref: *mut TESObjectREFR, a_arg3: bool) -> *mut NiAVObject
    }

    virtual_method! {
        pub const REPLACE_MODEL_OVERRIDE: usize = 0x44;
        pub fn replace_model_override(this: &TESBoundObject) -> bool
    }

    crate::relocation_func! {
        pub fn get_destructible_form(this: &TESBoundObject) -> *mut BGSDestructibleObjectForm => VariantID::new(14055, 14152, 0)
    }

    virtual_method! {
        pub const SET_OBJECT_VOICE_TYPE: usize = 0x48;
        pub fn set_object_voice_type(this: &TESBoundObject, voice_type: *mut BGSVoiceType) -> ()
    }

    virtual_method! {
        pub const GET_OBJECT_VOICE_TYPE: usize = 0x49;
        pub fn get_object_voice_type(this: &TESBoundObject) -> *mut BGSVoiceType
    }

    virtual_method! {
        pub const CLONE_3D: usize = 0x4A;
        pub fn clone_3d(this: &TESBoundObject, a_ref: *mut TESObjectREFR) -> *mut NiAVObject
    }

    virtual_method! {
        pub const REPLACE_MODEL: usize = 0x4B;
        pub fn replace_model(this: &TESBoundObject, a_str: *const core::ffi::c_char) -> bool
    }

    virtual_method! {
        pub const GET_ACTIVATE_TEXT: usize = 0x4C;
        pub fn get_activate_text(this: &TESBoundObject, a_activator: *mut TESObjectREFR, a_dst: *mut BSString) -> bool
    }

    virtual_method! {
        pub const CALCULATE_DO_FAVOR: usize = 0x4D;
        pub fn calculate_do_favor(this: &TESBoundObject, a_activator: *mut Actor, a_arg2: bool, a_to_activate: *mut TESObjectREFR, a_arg3: f32) -> bool
    }

    virtual_method! {
        pub const HANDLE_REMOVE_ITEM_FROM_CONTAINER: usize = 0x4E;
        pub fn handle_remove_item_from_container(this: &TESBoundObject, a_container: *mut TESObjectREFR) -> ()
    }

    virtual_method! {
        pub const ON_REMOVE_3D: usize = 0x4F;
        pub fn on_remove_3d(this: &TESBoundObject, a_obj3d: *mut NiAVObject) -> ()
    }

    virtual_method! {
        pub const ON_CHECK_MODELS: usize = 0x50;
        pub fn on_check_models(this: &TESBoundObject) -> ()
    }

    virtual_method! {
        pub const ON_COPY_REFERENCE: usize = 0x51;
        pub fn on_copy_reference(this: &TESBoundObject) -> ()
    }

    virtual_method! {
        pub const ON_FINISH_SCALE: usize = 0x52;
        pub fn on_finish_scale(this: &TESBoundObject) -> ()
    }
}