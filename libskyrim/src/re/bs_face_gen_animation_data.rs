use core::ffi::c_void;

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_BSFaceGenAnimationData;
use crate::offsets::offsets_rtti::RTTI_BSFaceGenAnimationData;
use crate::offsets::offsets_vtable::VTABLE_BSFaceGenAnimationData;
use crate::re::BSFaceGenKeyframeMultiple;
use crate::re::BSFaceGenKeyframeMultipleExpression;
use crate::re::BSSpinLock;
use crate::re::NiExtraData;
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSFaceGenAnimationData::EyesBlinkingStage`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSFaceGenAnimationDataEyesBlinkingStage {
    BlinkDelay = 0,
    BlinkDown = 1,
    BlinkUp = 2,
    WaitForLookDown = 3,
    BlinkDownAndWait1 = 4,
    BlinkDownAndWait2 = 5,
}

/// C++ `RE::BSFaceGenAnimationData::DialogueData::Unk28`
#[repr(C)]
pub struct BSFaceGenAnimationDataDialogueDataUnk28 {
    pub unk0: i32,          // 00
    pub unk4: i32,          // 04
    pub unk8: [u8; 0x8],    // 08
    pub unk10: *mut c_void, // 10
    pub unk18: *mut c_void, // 18
    pub unk20: u8,          // 20
    pub pad21: [u8; 0x7],   // 21
}

const _: () = assert!(core::mem::size_of::<BSFaceGenAnimationDataDialogueDataUnk28>() == 0x28);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationDataDialogueDataUnk28, unk0) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationDataDialogueDataUnk28, unk4) == 0x04);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationDataDialogueDataUnk28, unk8) == 0x08);
const _: () =
    assert!(core::mem::offset_of!(BSFaceGenAnimationDataDialogueDataUnk28, unk10) == 0x10);
const _: () =
    assert!(core::mem::offset_of!(BSFaceGenAnimationDataDialogueDataUnk28, unk18) == 0x18);
const _: () =
    assert!(core::mem::offset_of!(BSFaceGenAnimationDataDialogueDataUnk28, unk20) == 0x20);

/// C++ `RE::BSFaceGenAnimationData::DialogueData`
#[repr(C)]
pub struct BSFaceGenAnimationDataDialogueData {
    pub unk0: [u8; 0xC],                                     // 00
    pub ref_count: u32,                                      // 0C
    pub unk10: [u8; 0x18],                                   // 10
    pub unk28: *mut BSFaceGenAnimationDataDialogueDataUnk28, // 28
}

const _: () = assert!(core::mem::size_of::<BSFaceGenAnimationDataDialogueData>() == 0x30);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationDataDialogueData, unk0) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationDataDialogueData, ref_count) == 0x0C);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationDataDialogueData, unk10) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationDataDialogueData, unk28) == 0x28);

/// C++ `RE::BSFaceGenAnimationData`
#[repr(C)]
pub struct BSFaceGenAnimationData {
    pub base: NiExtraData,                                            // 000
    pub transition_target_key_frame: *mut BSFaceGenKeyframeMultiple,  // 018
    pub expression_key_frame: BSFaceGenKeyframeMultiple,              // 020
    pub unk040: BSFaceGenKeyframeMultiple,                            // 040
    pub modifier_key_frame: BSFaceGenKeyframeMultiple,                // 060
    pub phenome_key_frame: BSFaceGenKeyframeMultiple,                 // 080
    pub custom_key_frame: BSFaceGenKeyframeMultiple,                  // 0A0
    pub expression3: BSFaceGenKeyframeMultiple,                       // 0C0
    pub modifier1: BSFaceGenKeyframeMultiple,                         // 0E0
    pub modifier3: BSFaceGenKeyframeMultiple,                         // 100
    pub phoneme1: BSFaceGenKeyframeMultiple,                          // 120
    pub phoneme3: BSFaceGenKeyframeMultiple,                          // 140
    pub custom1: BSFaceGenKeyframeMultiple,                           // 160
    pub custom3: BSFaceGenKeyframeMultiple,                           // 180
    pub unk1a0: f32,                                                  // 1A0
    pub unk1a4: f32,                                                  // 1A4
    pub unk1a8: f32,                                                  // 1A8
    pub unk1ac: u32,                                                  // 1AC
    pub unk1b0: u32,                                                  // 1B0
    pub nk1b4: u32,                                                   // 1B4
    pub unk1b8: u32,                                                  // 1B8
    pub eyes_heading: f32,                                            // 1BC
    pub eyes_pitch: f32,                                              // 1C0
    pub pad1c4: u32,                                                  // 1C4
    pub unk1c8: u64,                                                  // 1C8
    pub unk1d0: u8,                                                   // 1D0
    pub unk1d1: u8,                                                   // 1D1
    pub nk1d2: u8,                                                    // 1D2
    pub unk1d3: u8,                                                   // 1D3
    pub eyes_heading_base: f32,                                       // 1D4
    pub eyes_pitch_base: f32,                                         // 1D8
    pub unk1dc: u32,                                                  // 1DC
    pub unk1e0: u32,                                                  // 1E0
    pub unk1e4: u32,                                                  // 1E4
    pub unk1e8: u32,                                                  // 1E8
    pub unk1ec: u32,                                                  // 1EC
    pub unk1f0: u32,                                                  // 1F0
    pub unk1f4: u32,                                                  // 1F4
    pub unk1f8: u32,                                                  // 1F8
    pub unk1fc: u32,                                                  // 1FC
    pub eyes_blinking_stage: BSFaceGenAnimationDataEyesBlinkingStage, // 200
    pub eyes_blinking_timer: f32,                                     // 204
    pub eyes_offset_timer: f32,                                       // 208
    pub eyes_heading_offset: f32,                                     // 20C
    pub eyes_pitch_offset: f32,                                       // 210
    pub unk214: u8,                                                   // 214
    pub unk215: u8,                                                   // 215
    pub unk216: u8,                                                   // 216
    pub unk217: bool,                                                 // 217
    pub unk218: u8,                                                   // 218
    pub unk219: u8,                                                   // 219
    pub unk21a: u8,                                                   // 21A
    pub unk21b: u8,                                                   // 21B
    pub unk21c: u8,                                                   // 21C
    pub unk21d: u8,                                                   // 21D
    pub expr_override: bool,                                          // 21E
    pub unk21f: u8,                                                   // 21F
    pub lock: BSSpinLock,                                             // 220
    pub dialogue_data: *mut BSFaceGenAnimationDataDialogueData,       // 228
}

const _: () = assert!(core::mem::size_of::<BSFaceGenAnimationData>() == 0x230);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, base) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(BSFaceGenAnimationData, transition_target_key_frame) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, expression_key_frame) == 0x20);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, custom_key_frame) == 0xA0);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, expression3) == 0xC0);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, unk1a0) == 0x1A0);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, eyes_heading) == 0x1BC);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, eyes_blinking_stage) == 0x200);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, unk214) == 0x214);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, expr_override) == 0x21E);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, lock) == 0x220);
const _: () = assert!(core::mem::offset_of!(BSFaceGenAnimationData, dialogue_data) == 0x228);

impl RttiType for BSFaceGenAnimationData {
    const RTTI: VariantID = RTTI_BSFaceGenAnimationData;
}

impl crate::re::ni_ref_object::NiRef for BSFaceGenAnimationData {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(BSFaceGenAnimationData : NiExtraData);

impl BSFaceGenAnimationData {
    pub const RTTI: VariantID = RTTI_BSFaceGenAnimationData;
    pub const NI_RTTI: VariantID = NiRTTI_BSFaceGenAnimationData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSFaceGenAnimationData;

    // override (NiExtraData)
    // const NiRTTI* GetRTTI() const override;  // 02

    crate::relocation_func! {
        pub fn set_expression_override(&mut self, a_idx: u32, a_value: f32) => RelocationID::new(25980, 26594)
    }

    #[inline(always)]
    pub fn clear_expression_override(&mut self) {
        self.expr_override = false;
    }

    crate::relocation_func! {
        pub fn reset_animation_state(
            &mut self,
            a_timer: f32,
            a_reset_expression: bool,
            a_reset_modifier_and_phoneme: bool,
            a_reset_custom: bool,
            a_close_eyes: bool
        ) => RelocationID::new(25977, 26586)
    }

    #[inline]
    pub fn get_active_expression(&self) -> u32 {
        let mut expression = BSFaceGenKeyframeMultipleExpression::MOOD_NEUTRAL as u32;
        let values = self.expression3.values;
        let count = self.expression3.count;

        if values.is_null() || count <= expression {
            return expression;
        }

        for i in 0..count {
            unsafe {
                if *values.add(i as usize) > *values.add(expression as usize) {
                    expression = i;
                }
            }
        }

        expression
    }
}
