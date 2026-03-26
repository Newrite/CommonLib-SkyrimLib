use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_FirstPersonState;
use crate::offsets::offsets_vtable::VTABLE_FirstPersonState;
use crate::re::{NiAVObject, NiNode, NiPoint3, PlayerInputHandler, TESCameraState};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct FirstPersonStateData {
    pub last_position: NiPoint3,
    pub last_frame_spring_velocity: NiPoint3,
    pub dampening_offset: NiPoint3,
    pub pad54: u32,
    pub first_person_camera_obj: *mut NiAVObject,
    pub first_person_fov_control: *mut NiNode,
    pub sitting_rotation: f32,
    pub unk6c: f32,
    pub unk70: f32,
    pub current_pitch_offset: f32,
    pub target_pitch_offset: f32,
    pub unk7c: f32,
    pub unk80: u32,
    pub camera_override: bool,
    pub camera_pitch_override: bool,
    pub unk86: u16,
    pub unk88: u64,
}

#[repr(C)]
pub struct FirstPersonState {
    pub base: TESCameraState,
}

const _: () = assert!(core::mem::size_of::<FirstPersonState>() == 0x20);

impl RttiType for FirstPersonState {
    const RTTI: VariantID = RTTI_FirstPersonState;
}

inherit!(FirstPersonState : TESCameraState, base);

impl FirstPersonState {
    pub const RTTI: VariantID = RTTI_FirstPersonState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FirstPersonState;
    pub const PLAYER_INPUT_HANDLER_OFFSET: VariantOffset = VariantOffset::new(0x20, 0x20, 0x20);
    pub const DATA_OFFSET: VariantOffset = VariantOffset::new(0x30, 0x30, 0x48);

    crate::runtime_cast_accessor! {
        pub fn player_input_handler() -> PlayerInputHandler {
            offset: Self::PLAYER_INPUT_HANDLER_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn player_input_handler_mut() -> PlayerInputHandler {
            offset: Self::PLAYER_INPUT_HANDLER_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        pub fn data() -> FirstPersonStateData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> FirstPersonStateData {
            offset: Self::DATA_OFFSET
        }
    }

    pub fn unk_03(&mut self) {
        if crate::runtime::is_vr() {
            crate::relocate_virtual!(
                extern "C" fn(*mut Self),
                self as *mut Self,
                VariantOffset::new_se_ae(0x0, 0x03)
            );
        }
    }
}
