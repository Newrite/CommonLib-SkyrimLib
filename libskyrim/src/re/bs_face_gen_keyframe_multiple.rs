use alloc::string::String;
use core::ffi::c_char;

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BSFaceGenKeyframeMultiple;
use crate::offsets::offsets_vtable::VTABLE_BSFaceGenKeyframeMultiple;
use crate::re::BSFaceGenKeyframe;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::{relocation_func, virtual_method};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSFaceGenKeyframeMultipleExpression {
    DialogueAnger = 0,
    DialogueFear = 1,
    DialogueHappy = 2,
    DialogueSad = 3,
    DialogueSurprise = 4,
    DialoguePuzzled = 5,
    DialogueDisgusted = 6,
    MoodNeutral = 7,
    MoodAnger = 8,
    MoodFear = 9,
    MoodHappy = 10,
    MoodSad = 11,
    MoodSurprise = 12,
    MoodPuzzled = 13,
    MoodDisgusted = 14,
    CombatAnger = 15,
    CombatShout = 16,
}

impl BSFaceGenKeyframeMultipleExpression {
    pub const DIALOGUE_ANGER: usize = Self::DialogueAnger as usize;
    pub const DIALOGUE_FEAR: usize = Self::DialogueFear as usize;
    pub const DIALOGUE_HAPPY: usize = Self::DialogueHappy as usize;
    pub const DIALOGUE_SAD: usize = Self::DialogueSad as usize;
    pub const DIALOGUE_SURPRISE: usize = Self::DialogueSurprise as usize;
    pub const DIALOGUE_PUZZLED: usize = Self::DialoguePuzzled as usize;
    pub const DIALOGUE_DISGUSTED: usize = Self::DialogueDisgusted as usize;
    pub const MOOD_NEUTRAL: usize = Self::MoodNeutral as usize;
    pub const MOOD_ANGER: usize = Self::MoodAnger as usize;
    pub const MOOD_FEAR: usize = Self::MoodFear as usize;
    pub const MOOD_HAPPY: usize = Self::MoodHappy as usize;
    pub const MOOD_SAD: usize = Self::MoodSad as usize;
    pub const MOOD_SURPRISE: usize = Self::MoodSurprise as usize;
    pub const MOOD_PUZZLED: usize = Self::MoodPuzzled as usize;
    pub const MOOD_DISGUSTED: usize = Self::MoodDisgusted as usize;
    pub const COMBAT_ANGER: usize = Self::CombatAnger as usize;
    pub const COMBAT_SHOUT: usize = Self::CombatShout as usize;
    pub const TOTAL: usize = 17;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSFaceGenKeyframeMultipleModifier {
    BlinkLeft = 0,
    BlinkRight = 1,
    BrowDownLeft = 2,
    BrowDownRight = 3,
    BrowInLeft = 4,
    BrowInRight = 5,
    BrowUpLeft = 6,
    BrowUpRight = 7,
    LookDown = 8,
    LookLeft = 9,
    LookRight = 10,
    LookUp = 11,
    SquintLeft = 12,
    SquintRight = 13,
    HeadPitch = 14,
    HeadRoll = 15,
    HeadYaw = 16,
}

impl BSFaceGenKeyframeMultipleModifier {
    pub const BLINK_LEFT: usize = Self::BlinkLeft as usize;
    pub const BLINK_RIGHT: usize = Self::BlinkRight as usize;
    pub const BROW_DOWN_LEFT: usize = Self::BrowDownLeft as usize;
    pub const BROW_DOWN_RIGHT: usize = Self::BrowDownRight as usize;
    pub const BROW_IN_LEFT: usize = Self::BrowInLeft as usize;
    pub const BROW_IN_RIGHT: usize = Self::BrowInRight as usize;
    pub const BROW_UP_LEFT: usize = Self::BrowUpLeft as usize;
    pub const BROW_UP_RIGHT: usize = Self::BrowUpRight as usize;
    pub const LOOK_DOWN: usize = Self::LookDown as usize;
    pub const LOOK_LEFT: usize = Self::LookLeft as usize;
    pub const LOOK_RIGHT: usize = Self::LookRight as usize;
    pub const LOOK_UP: usize = Self::LookUp as usize;
    pub const SQUINT_LEFT: usize = Self::SquintLeft as usize;
    pub const SQUINT_RIGHT: usize = Self::SquintRight as usize;
    pub const HEAD_PITCH: usize = Self::HeadPitch as usize;
    pub const HEAD_ROLL: usize = Self::HeadRoll as usize;
    pub const HEAD_YAW: usize = Self::HeadYaw as usize;
    pub const TOTAL: usize = 17;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSFaceGenKeyframeMultiplePhoneme {
    Aah = 0,
    BigAah = 1,
    BMP = 2,
    ChJSh = 3,
    DST = 4,
    Eee = 5,
    Eh = 6,
    FV = 7,
    I = 8,
    K = 9,
    N = 10,
    Oh = 11,
    OohQ = 12,
    R = 13,
    Th = 14,
    W = 15,
}

impl BSFaceGenKeyframeMultiplePhoneme {
    pub const AAH: usize = Self::Aah as usize;
    pub const BIG_AAH: usize = Self::BigAah as usize;
    pub const BMP: usize = Self::BMP as usize;
    pub const CH_J_SH: usize = Self::ChJSh as usize;
    pub const DST: usize = Self::DST as usize;
    pub const EEE: usize = Self::Eee as usize;
    pub const EH: usize = Self::Eh as usize;
    pub const FV: usize = Self::FV as usize;
    pub const I: usize = Self::I as usize;
    pub const K: usize = Self::K as usize;
    pub const N: usize = Self::N as usize;
    pub const OH: usize = Self::Oh as usize;
    pub const OOH_Q: usize = Self::OohQ as usize;
    pub const R: usize = Self::R as usize;
    pub const TH: usize = Self::Th as usize;
    pub const W: usize = Self::W as usize;
    pub const TOTAL: usize = 16;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSFaceGenKeyframeMultipleCustom {
    SkinnyMorph = 0,
}

impl BSFaceGenKeyframeMultipleCustom {
    pub const SKINNY_MORPH: usize = Self::SkinnyMorph as usize;
    pub const TOTAL: usize = 1;
}

/// C++ `RE::BSFaceGenKeyframeMultiple`
#[repr(C)]
pub struct BSFaceGenKeyframeMultiple {
    pub base: BSFaceGenKeyframe, // 00
    pub values: *mut f32,        // 10
    pub count: u32,              // 18
    pub is_updated: bool,        // 1C
    pub pad1d: u8,               // 1D
    pub pad1e: u16,              // 1E
}

const _: () = assert!(core::mem::size_of::<BSFaceGenKeyframeMultiple>() == 0x20);
const _: () = assert!(core::mem::offset_of!(BSFaceGenKeyframeMultiple, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSFaceGenKeyframeMultiple, values) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSFaceGenKeyframeMultiple, count) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSFaceGenKeyframeMultiple, is_updated) == 0x1C);

impl RttiType for BSFaceGenKeyframeMultiple {
    const RTTI: VariantID = RTTI_BSFaceGenKeyframeMultiple;
}

inherit!(BSFaceGenKeyframeMultiple : BSFaceGenKeyframe);

impl BSFaceGenKeyframeMultiple {
    pub const RTTI: VariantID = RTTI_BSFaceGenKeyframeMultiple;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSFaceGenKeyframeMultiple;

    // override (BSFaceGenKeyframe)
    // bool               Unk_01(BSTArray<BSFaceGenKeyframe*>& a_arr) override;  // 01
    // bool               Interpolate(...) override;                              // 02
    // bool               Interpolate(...) override;                              // 03
    // void               Reset(bool a_initToZero) override;                      // 04
    // BSFaceGenKeyframe* Clone() override;                                       // 05
    // void               Copy(BSFaceGenKeyframe* a_keyframe) override;           // 06
    // bool               NotEqual(BSFaceGenKeyframe* a_keyframe) override;       // 07
    // float              GetMaxValue() override;                                 // 08
    // bool               IsZero() override;                                      // 09
    // bool               TransitionUpdate(...) override;                         // 0A
    // bool               NotZero() override;                                     // 0B
    // bool               IsKeyframeMultiple() override;                          // 0C
    // bool               IsKeyframeExclusive() override;                         // 0D

    virtual_method! {
        pub const VFUNC_IS_VALUE_VALID: usize = 0x0E;
        pub fn is_value_valid(a_idx: u32) -> bool
    }

    virtual_method! {
        pub const VFUNC_ALLOCATE: usize = 0x0F;
        pub fn allocate(a_count: u32, a_init_to_zero: bool)
    }

    relocation_func! {
        pub fn get_expression_name(a_expression: u32) -> *const c_char => RelocationID::new(26428, 27007)
    }

    relocation_func! {
        pub fn get_modifier_name(a_modifier: u32) -> *const c_char => RelocationID::new(26429, 27008)
    }

    relocation_func! {
        pub fn get_phoneme_name(a_phoneme: u32) -> *const c_char => RelocationID::new(26430, 27009)
    }

    relocation_func! {
        pub fn get_custom_name(a_custom: u32) -> *const c_char => RelocationID::new(26431, 27010)
    }

    #[inline]
    pub fn get_value_name(keyframe_type: crate::re::BSFaceGenKeyframeType, idx: u32) -> String {
        let name = match keyframe_type {
            crate::re::BSFaceGenKeyframeType::Expression => Self::get_expression_name(idx),
            crate::re::BSFaceGenKeyframeType::Modifier => Self::get_modifier_name(idx),
            crate::re::BSFaceGenKeyframeType::Phoneme => Self::get_phoneme_name(idx),
            crate::re::BSFaceGenKeyframeType::Custom => Self::get_custom_name(idx),
            crate::re::BSFaceGenKeyframeType::Undefined => core::ptr::null(),
        };

        if name.is_null() {
            String::new()
        } else {
            String::from(core_util::ptr_to_str(name))
        }
    }

    #[inline]
    pub fn set_value(&mut self, idx: u32, value: f32) {
        assert!(idx < self.count);
        assert!(!self.values.is_null());
        unsafe {
            *self.values.add(idx as usize) = value;
        }
        self.is_updated = false;
    }

    #[inline]
    pub fn reset_values(&mut self) {
        if !self.values.is_null() && self.count != 0 {
            unsafe {
                core::ptr::write_bytes(self.values, 0, self.count as usize);
            }
        }
        self.is_updated = false;
    }
}
