use crate::offsets::offsets_rtti::RTTI_BSFaceGenKeyframe;
use crate::offsets::offsets_vtable::VTABLE_BSFaceGenKeyframe;
use crate::re::BSTArray;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BSFaceGenKeyframe::Type`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSFaceGenKeyframeType {
    Undefined = -1,
    Phoneme = 0,
    Expression = 1,
    Modifier = 2,
    Custom = 3,
}

/// C++ `RE::BSFaceGenKeyframe`
#[repr(C)]
pub struct BSFaceGenKeyframe {
    pub vtable: *const usize,                 // 00
    pub keyframe_type: BSFaceGenKeyframeType, // 08
    pub unk0c: f32,                           // 0C
}

const _: () = assert!(core::mem::size_of::<BSFaceGenKeyframe>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSFaceGenKeyframe, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSFaceGenKeyframe, keyframe_type) == 0x08);
const _: () = assert!(core::mem::offset_of!(BSFaceGenKeyframe, unk0c) == 0x0C);

impl RttiType for BSFaceGenKeyframe {
    const RTTI: VariantID = RTTI_BSFaceGenKeyframe;
}

impl AsRef<BSFaceGenKeyframe> for BSFaceGenKeyframe {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSFaceGenKeyframe> for BSFaceGenKeyframe {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSFaceGenKeyframe {
    pub const RTTI: VariantID = RTTI_BSFaceGenKeyframe;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSFaceGenKeyframe;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_UNK_01: usize = 0x01;
        pub fn unk_01(a_arr: *mut BSTArray<*mut BSFaceGenKeyframe>) -> bool
    }

    virtual_method! {
        pub const VFUNC_INTERPOLATE: usize = 0x02;
        pub fn interpolate_between(
            a_keyframe1: *mut BSFaceGenKeyframe,
            a_keyframe2: *mut BSFaceGenKeyframe,
            a_k: f32
        ) -> bool
    }

    virtual_method! {
        pub const VFUNC_INTERPOLATE_SINGLE: usize = 0x03;
        pub fn interpolate_single(
            a_keyframe: *mut BSFaceGenKeyframe,
            a_k: f32,
            a_ignore_small: bool,
            a_copy_big: bool
        ) -> bool
    }

    virtual_method! {
        pub const VFUNC_RESET: usize = 0x04;
        pub fn reset(a_init_to_zero: bool)
    }

    virtual_method! {
        pub const VFUNC_CLONE: usize = 0x05;
        pub fn clone_keyframe() -> *mut BSFaceGenKeyframe
    }

    virtual_method! {
        pub const VFUNC_COPY: usize = 0x06;
        pub fn copy_from(a_keyframe: *mut BSFaceGenKeyframe)
    }

    virtual_method! {
        pub const VFUNC_NOT_EQUAL: usize = 0x07;
        pub fn not_equal(a_keyframe: *mut BSFaceGenKeyframe) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_MAX_VALUE: usize = 0x08;
        pub fn get_max_value() -> f32
    }

    virtual_method! {
        pub const VFUNC_IS_ZERO: usize = 0x09;
        pub fn is_zero() -> bool
    }

    virtual_method! {
        pub const VFUNC_TRANSITION_UPDATE: usize = 0x0A;
        pub fn transition_update(
            a_time_delta: f32,
            a_keyframe: *mut BSFaceGenKeyframe
        ) -> bool
    }

    virtual_method! {
        pub const VFUNC_NOT_ZERO: usize = 0x0B;
        pub fn not_zero() -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_KEYFRAME_MULTIPLE: usize = 0x0C;
        pub fn is_keyframe_multiple() -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_KEYFRAME_EXCLUSIVE: usize = 0x0D;
        pub fn is_keyframe_exclusive() -> bool
    }
}

pub trait BSFaceGenKeyframeExt {
    fn unk_01(&self, a_arr: *mut BSTArray<*mut BSFaceGenKeyframe>) -> bool;
    fn interpolate_between(
        &self,
        a_keyframe1: *mut BSFaceGenKeyframe,
        a_keyframe2: *mut BSFaceGenKeyframe,
        a_k: f32,
    ) -> bool;
    fn interpolate_single(
        &self,
        a_keyframe: *mut BSFaceGenKeyframe,
        a_k: f32,
        a_ignore_small: bool,
        a_copy_big: bool,
    ) -> bool;
    fn reset(&self, a_init_to_zero: bool);
    fn clone_keyframe(&self) -> *mut BSFaceGenKeyframe;
    fn copy_from(&self, a_keyframe: *mut BSFaceGenKeyframe);
    fn not_equal(&self, a_keyframe: *mut BSFaceGenKeyframe) -> bool;
    fn get_max_value(&self) -> f32;
    fn is_zero(&self) -> bool;
    fn transition_update(&self, a_time_delta: f32, a_keyframe: *mut BSFaceGenKeyframe) -> bool;
    fn not_zero(&self) -> bool;
    fn is_keyframe_multiple(&self) -> bool;
    fn is_keyframe_exclusive(&self) -> bool;
}

impl<T: AsRef<BSFaceGenKeyframe>> BSFaceGenKeyframeExt for T {
    #[inline(always)]
    fn unk_01(&self, a_arr: *mut BSTArray<*mut BSFaceGenKeyframe>) -> bool {
        self.as_ref().unk_01(a_arr)
    }

    #[inline(always)]
    fn interpolate_between(
        &self,
        a_keyframe1: *mut BSFaceGenKeyframe,
        a_keyframe2: *mut BSFaceGenKeyframe,
        a_k: f32,
    ) -> bool {
        self.as_ref()
            .interpolate_between(a_keyframe1, a_keyframe2, a_k)
    }

    #[inline(always)]
    fn interpolate_single(
        &self,
        a_keyframe: *mut BSFaceGenKeyframe,
        a_k: f32,
        a_ignore_small: bool,
        a_copy_big: bool,
    ) -> bool {
        self.as_ref()
            .interpolate_single(a_keyframe, a_k, a_ignore_small, a_copy_big)
    }

    #[inline(always)]
    fn reset(&self, a_init_to_zero: bool) {
        self.as_ref().reset(a_init_to_zero)
    }

    #[inline(always)]
    fn clone_keyframe(&self) -> *mut BSFaceGenKeyframe {
        self.as_ref().clone_keyframe()
    }

    #[inline(always)]
    fn copy_from(&self, a_keyframe: *mut BSFaceGenKeyframe) {
        self.as_ref().copy_from(a_keyframe)
    }

    #[inline(always)]
    fn not_equal(&self, a_keyframe: *mut BSFaceGenKeyframe) -> bool {
        self.as_ref().not_equal(a_keyframe)
    }

    #[inline(always)]
    fn get_max_value(&self) -> f32 {
        self.as_ref().get_max_value()
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.as_ref().is_zero()
    }

    #[inline(always)]
    fn transition_update(&self, a_time_delta: f32, a_keyframe: *mut BSFaceGenKeyframe) -> bool {
        self.as_ref().transition_update(a_time_delta, a_keyframe)
    }

    #[inline(always)]
    fn not_zero(&self) -> bool {
        self.as_ref().not_zero()
    }

    #[inline(always)]
    fn is_keyframe_multiple(&self) -> bool {
        self.as_ref().is_keyframe_multiple()
    }

    #[inline(always)]
    fn is_keyframe_exclusive(&self) -> bool {
        self.as_ref().is_keyframe_exclusive()
    }
}
