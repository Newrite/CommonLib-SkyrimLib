use crate::offsets::offsets_rtti::RTTI_BSISoundOutputModel;
use crate::offsets::offsets_vtable::VTABLE_BSISoundOutputModel;
use crate::re::bsi_attenuation_characteristics::BSIAttenuationCharacteristics;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BSISoundOutputModel`
#[repr(C)]
pub struct BSISoundOutputModel {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSISoundOutputModel>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSISoundOutputModel, vtable) == 0x00);

impl RttiType for BSISoundOutputModel {
    const RTTI: VariantID = RTTI_BSISoundOutputModel;
}

impl BSISoundOutputModel {
    pub const RTTI: VariantID = RTTI_BSISoundOutputModel;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSISoundOutputModel;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_DO_GET_USES_HRTF: usize = 0x01;
        pub fn do_get_uses_hrtf() -> bool
    }

    virtual_method! {
        pub const VFUNC_DO_GET_HAS_SPEAKER_BIAS: usize = 0x02;
        pub fn do_get_has_speaker_bias() -> bool
    }

    virtual_method! {
        pub const VFUNC_DO_GET_SPEAKER_BIAS: usize = 0x03;
        pub fn do_get_speaker_bias(arg1: u32, arg2: u32, out_bias: *mut [f32; 8]) -> bool
    }

    virtual_method! {
        pub const VFUNC_DO_GET_ATTENUATES_WITH_DISTANCE: usize = 0x04;
        pub fn do_get_attenuates_with_distance() -> bool
    }

    virtual_method! {
        pub const VFUNC_DO_GET_AUDIBILITY: usize = 0x05;
        pub fn do_get_audibility(distance: f32) -> bool
    }

    virtual_method! {
        pub const VFUNC_DO_GET_SUPPORTED_INPUT_CHANNELS: usize = 0x06;
        pub fn do_get_supported_input_channels() -> u32
    }

    virtual_method! {
        pub const VFUNC_DO_GET_ATTENUATION: usize = 0x07;
        pub fn do_get_attenuation() -> *const BSIAttenuationCharacteristics
    }

    virtual_method! {
        pub const VFUNC_DO_GET_REVERB_SEND_LEVEL: usize = 0x08;
        pub fn do_get_reverb_send_level() -> f32
    }

    virtual_method! {
        pub const VFUNC_DO_GET_SUPPORTS_MONITOR: usize = 0x09;
        pub fn do_get_supports_monitor(arg1: u32) -> bool
    }

    #[inline(always)]
    pub fn get_attenuation(&self) -> Option<&BSIAttenuationCharacteristics> {
        unsafe { self.do_get_attenuation().as_ref() }
    }

    #[inline(always)]
    pub fn get_speaker_bias_into(&self, arg1: u32, arg2: u32, out_bias: &mut [f32; 8]) -> bool {
        self.do_get_speaker_bias(arg1, arg2, out_bias as *mut [f32; 8])
    }

    #[inline(always)]
    pub fn get_speaker_bias(&self, arg1: u32, arg2: u32) -> Option<[f32; 8]> {
        let mut out_bias = [0.0; 8];
        if self.get_speaker_bias_into(arg1, arg2, &mut out_bias) {
            Some(out_bias)
        } else {
            None
        }
    }
}

pub trait BSISoundOutputModelExt {
    fn do_get_uses_hrtf(&self) -> bool;
    fn do_get_has_speaker_bias(&self) -> bool;
    fn do_get_speaker_bias(&self, arg1: u32, arg2: u32, out_bias: *mut [f32; 8]) -> bool;
    fn do_get_attenuates_with_distance(&self) -> bool;
    fn do_get_audibility(&self, distance: f32) -> bool;
    fn do_get_supported_input_channels(&self) -> u32;
    fn do_get_reverb_send_level(&self) -> f32;
    fn do_get_supports_monitor(&self, arg1: u32) -> bool;
    fn get_attenuation(&self) -> Option<&BSIAttenuationCharacteristics>;
    fn get_speaker_bias_into(&self, arg1: u32, arg2: u32, out_bias: &mut [f32; 8]) -> bool;
    fn get_speaker_bias(&self, arg1: u32, arg2: u32) -> Option<[f32; 8]>;
}

impl<T: AsRef<BSISoundOutputModel>> BSISoundOutputModelExt for T {
    fn do_get_uses_hrtf(&self) -> bool {
        self.as_ref().do_get_uses_hrtf()
    }

    fn do_get_has_speaker_bias(&self) -> bool {
        self.as_ref().do_get_has_speaker_bias()
    }

    fn do_get_speaker_bias(&self, arg1: u32, arg2: u32, out_bias: *mut [f32; 8]) -> bool {
        self.as_ref().do_get_speaker_bias(arg1, arg2, out_bias)
    }

    fn do_get_attenuates_with_distance(&self) -> bool {
        self.as_ref().do_get_attenuates_with_distance()
    }

    fn do_get_audibility(&self, distance: f32) -> bool {
        self.as_ref().do_get_audibility(distance)
    }

    fn do_get_supported_input_channels(&self) -> u32 {
        self.as_ref().do_get_supported_input_channels()
    }

    fn do_get_reverb_send_level(&self) -> f32 {
        self.as_ref().do_get_reverb_send_level()
    }

    fn do_get_supports_monitor(&self, arg1: u32) -> bool {
        self.as_ref().do_get_supports_monitor(arg1)
    }

    fn get_attenuation(&self) -> Option<&BSIAttenuationCharacteristics> {
        self.as_ref().get_attenuation()
    }

    fn get_speaker_bias_into(&self, arg1: u32, arg2: u32, out_bias: &mut [f32; 8]) -> bool {
        self.as_ref().get_speaker_bias_into(arg1, arg2, out_bias)
    }

    fn get_speaker_bias(&self, arg1: u32, arg2: u32) -> Option<[f32; 8]> {
        self.as_ref().get_speaker_bias(arg1, arg2)
    }
}
