#![allow(non_camel_case_types)]

use core_util::Enum;
use core_util::inherit;

use crate::re::{
    GColor, GFxFunctionHandler, GFxMovieDef, GFxStatMovieViews, GFxValue, GRefCountBase,
};

/// C++ `RE::GFxMovie::PlayState`
#[libskyrim_macros::open_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxMoviePlayState {
    kPlaying = 0,
    kStopped = 1,
}

/// C++ `RE::GFxMovie::SetVarType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxMovieSetVarType {
    kNormal = 0,
    kSticky = 1,
    kPermanent = 2,
}

/// C++ `RE::GFxMovie::SetArrayType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxMovieSetArrayType {
    kInt = 0,
    kDouble = 1,
    kFloat = 2,
    kString = 3,
    kStringW = 4,
    kValue = 5,
}

/// C++ `RE::GFxMovie`
#[repr(C)]
pub struct GFxMovie {
    pub base: GRefCountBase<GFxMovie, { GFxStatMovieViews::OTHER_MEM as u32 }>, // 00
}

const _: () = assert!(core::mem::size_of::<GFxMovie>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxMovie, base) == 0x0);

inherit!(GFxMovie : GRefCountBase<GFxMovie, { GFxStatMovieViews::OTHER_MEM as u32 }>, base);

impl GFxMovie {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_GET_MOVIE_DEF: usize = 0x01; pub fn get_movie_def() -> *mut GFxMovieDef }
    crate::virtual_method! { pub const VFUNC_GET_CURRENT_FRAME: usize = 0x02; pub fn get_current_frame() -> u32 }
    crate::virtual_method! { pub const VFUNC_HAS_LOOPED: usize = 0x03; pub fn has_looped() -> bool }
    crate::virtual_method! { pub const VFUNC_GOTO_FRAME: usize = 0x04; pub fn goto_frame(frame_number: u32) }
    crate::virtual_method! { pub const VFUNC_GOTO_LABELED_FRAME: usize = 0x05; pub fn goto_labeled_frame(label: *const i8, offset: i32) -> bool }
    crate::virtual_method! { pub const VFUNC_SET_PLAY_STATE: usize = 0x06; pub fn set_play_state(state: GFxMoviePlayState) }
    crate::virtual_method! { pub const VFUNC_GET_PLAY_STATE: usize = 0x07; pub fn get_play_state_raw() -> u32 }
    crate::virtual_method! { pub const VFUNC_SET_VISIBLE: usize = 0x08; pub fn set_visible(visible: bool) }
    crate::virtual_method! { pub const VFUNC_GET_VISIBLE: usize = 0x09; pub fn get_visible() -> bool }
    crate::virtual_method! { pub const VFUNC_IS_AVAILABLE: usize = 0x0A; pub fn is_available(path_to_var: *const i8) -> bool }
    crate::virtual_method! { pub const VFUNC_CREATE_STRING: usize = 0x0B; pub fn create_string(value: *mut GFxValue, string: *const i8) }
    crate::virtual_method! { pub const VFUNC_CREATE_STRING_W: usize = 0x0C; pub fn create_string_w(value: *mut GFxValue, string: *const u16) }
    crate::virtual_method! { pub const VFUNC_CREATE_OBJECT: usize = 0x0D; pub fn create_object(value: *mut GFxValue, class_name: *const i8, args: *const GFxValue, num_args: u32) }
    crate::virtual_method! { pub const VFUNC_CREATE_ARRAY: usize = 0x0E; pub fn create_array(value: *mut GFxValue) }
    crate::virtual_method! { pub const VFUNC_CREATE_FUNCTION: usize = 0x0F; pub fn create_function(value: *mut GFxValue, fc: *mut GFxFunctionHandler, user_data: *mut core::ffi::c_void) }
    crate::virtual_method! { pub const VFUNC_SET_VARIABLE: usize = 0x10; pub fn set_variable(path_to_var: *const i8, value: &GFxValue, set_type: GFxMovieSetVarType) -> bool }
    crate::virtual_method! { pub const VFUNC_GET_VARIABLE: usize = 0x11; pub fn get_variable(value: *mut GFxValue, path_to_var: *const i8) -> bool }
    crate::virtual_method! { pub const VFUNC_SET_VARIABLE_ARRAY: usize = 0x12; pub fn set_variable_array(array_type: GFxMovieSetArrayType, path_to_var: *const i8, index: u32, data: *const core::ffi::c_void, count: u32, set_type: GFxMovieSetVarType) -> bool }
    crate::virtual_method! { pub const VFUNC_SET_VARIABLE_ARRAY_SIZE: usize = 0x13; pub fn set_variable_array_size(path_to_var: *const i8, count: u32, set_type: GFxMovieSetVarType) -> bool }
    crate::virtual_method! { pub const VFUNC_GET_VARIABLE_ARRAY_SIZE: usize = 0x14; pub fn get_variable_array_size(path_to_var: *const i8) -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_VARIABLE_ARRAY: usize = 0x15; pub fn get_variable_array(array_type: GFxMovieSetArrayType, path_to_var: *const i8, index: u32, data: *mut core::ffi::c_void, count: u32) -> bool }
    crate::virtual_method! { pub const VFUNC_INVOKE_FMT: usize = 0x16; pub fn invoke_fmt(method_name: *const i8, result: *mut GFxValue, arg_fmt: *const i8) -> bool }
    crate::virtual_method! { pub const VFUNC_INVOKE: usize = 0x17; pub fn invoke(method_name: *const i8, result: *mut GFxValue, args: *const GFxValue, num_args: u32) -> bool }
    crate::virtual_method! { pub const VFUNC_INVOKE_ARGS: usize = 0x18; pub fn invoke_args(method_name: *const i8, result: *mut GFxValue, arg_fmt: *const i8, args: *mut core::ffi::c_void) -> bool }

    #[inline(always)]
    pub const fn get_render_pixel_scale() -> f32 {
        20.0
    }

    #[inline(always)]
    pub fn get_frame_count(&self) -> u32 {
        unsafe { (*self.get_movie_def()).get_frame_count() }
    }

    #[inline(always)]
    pub fn get_frame_rate(&self) -> f32 {
        unsafe { (*self.get_movie_def()).get_frame_rate() }
    }

    #[inline(always)]
    pub fn play_state_storage(&self) -> Enum<GFxMoviePlayState, u32> {
        Enum::from_underlying(self.get_play_state_raw())
    }

    #[inline(always)]
    pub fn try_get_play_state(&self) -> Option<GFxMoviePlayState> {
        self.play_state_storage().get()
    }

    #[inline(always)]
    pub fn get_play_state(&self) -> GFxMoviePlayState {
        self.try_get_play_state()
            .unwrap_or(GFxMoviePlayState::kPlaying)
    }

    #[inline(always)]
    pub unsafe fn set_variable_c_str(
        &mut self,
        path_to_var: *const i8,
        value: *const i8,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        let value = unsafe { GFxValue::from_string(value) };
        self.set_variable(path_to_var, &value, set_type)
    }

    #[inline(always)]
    pub unsafe fn set_variable_w_str(
        &mut self,
        path_to_var: *const i8,
        value: *const u16,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        let value = unsafe { GFxValue::from_string_w(value) };
        self.set_variable(path_to_var, &value, set_type)
    }

    #[inline(always)]
    pub fn set_variable_double(
        &mut self,
        path_to_var: *const i8,
        value: f64,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        let value = GFxValue::from_number(value);
        self.set_variable(path_to_var, &value, set_type)
    }

    #[inline(always)]
    pub fn get_variable_double(&self, path_to_var: *const i8) -> f64 {
        let mut value = GFxValue::with_type(crate::re::GFxValueValueType::kNumber);
        self.get_variable(&mut value, path_to_var);
        if value.is_number() {
            value.get_number()
        } else {
            0.0
        }
    }

    #[inline(always)]
    pub fn set_variable_value_array(
        &self,
        path_to_var: *const i8,
        index: u32,
        data: *const GFxValue,
        count: u32,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        self.set_variable_array(
            GFxMovieSetArrayType::kValue,
            path_to_var,
            index,
            data.cast(),
            count,
            set_type,
        )
    }

    #[inline(always)]
    pub fn get_variable_value_array(
        &self,
        path_to_var: *const i8,
        index: u32,
        data: *mut GFxValue,
        count: u32,
    ) -> bool {
        self.get_variable_array(
            GFxMovieSetArrayType::kValue,
            path_to_var,
            index,
            data.cast(),
            count,
        )
    }

    #[inline(always)]
    pub fn set_color_tint(&self, path_to_var: *const i8, tint: &GColor) -> bool {
        let mut object = GFxValue::default();
        if !self.get_variable(&mut object, path_to_var) {
            return false;
        }
        if !object.is_display_object() {
            return false;
        }
        object.set_color_tint(tint)
    }

    #[inline(always)]
    pub fn remove_color_tint(&self, path_to_var: *const i8) -> bool {
        let mut object = GFxValue::default();
        if !self.get_variable(&mut object, path_to_var) {
            return false;
        }
        if !object.is_display_object() {
            return false;
        }
        object.remove_color_tint()
    }
}

impl AsRef<GFxMovie> for GFxMovie {
    #[inline(always)]
    fn as_ref(&self) -> &GFxMovie {
        self
    }
}

impl AsMut<GFxMovie> for GFxMovie {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GFxMovie {
        self
    }
}

pub trait GFxMovieExt: AsRef<GFxMovie> + AsMut<GFxMovie> {
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn get_movie_def(&self) -> *mut GFxMovieDef {
        self.as_ref().get_movie_def()
    }

    #[inline(always)]
    fn get_current_frame(&self) -> u32 {
        self.as_ref().get_current_frame()
    }

    #[inline(always)]
    fn has_looped(&self) -> bool {
        self.as_ref().has_looped()
    }

    #[inline(always)]
    fn goto_frame(&mut self, frame_number: u32) {
        self.as_mut().goto_frame(frame_number)
    }

    #[inline(always)]
    fn goto_labeled_frame(&mut self, label: *const i8, offset: i32) -> bool {
        self.as_mut().goto_labeled_frame(label, offset)
    }

    #[inline(always)]
    fn set_play_state(&mut self, state: GFxMoviePlayState) {
        self.as_mut().set_play_state(state)
    }

    #[inline(always)]
    fn play_state_storage(&self) -> Enum<GFxMoviePlayState, u32> {
        self.as_ref().play_state_storage()
    }

    #[inline(always)]
    fn try_get_play_state(&self) -> Option<GFxMoviePlayState> {
        self.as_ref().try_get_play_state()
    }

    #[inline(always)]
    fn get_play_state(&self) -> GFxMoviePlayState {
        self.as_ref().get_play_state()
    }

    #[inline(always)]
    fn set_visible(&mut self, visible: bool) {
        self.as_mut().set_visible(visible)
    }

    #[inline(always)]
    fn get_visible(&self) -> bool {
        self.as_ref().get_visible()
    }

    #[inline(always)]
    fn is_available(&self, path_to_var: *const i8) -> bool {
        self.as_ref().is_available(path_to_var)
    }

    #[inline(always)]
    fn create_string(&mut self, value: *mut GFxValue, string: *const i8) {
        self.as_mut().create_string(value, string)
    }

    #[inline(always)]
    fn create_string_w(&mut self, value: *mut GFxValue, string: *const u16) {
        self.as_mut().create_string_w(value, string)
    }

    #[inline(always)]
    fn create_object(
        &mut self,
        value: *mut GFxValue,
        class_name: *const i8,
        args: *const GFxValue,
        num_args: u32,
    ) {
        self.as_mut()
            .create_object(value, class_name, args, num_args)
    }

    #[inline(always)]
    fn create_array(&mut self, value: *mut GFxValue) {
        self.as_mut().create_array(value)
    }

    #[inline(always)]
    fn create_function(
        &mut self,
        value: *mut GFxValue,
        fc: *mut GFxFunctionHandler,
        user_data: *mut core::ffi::c_void,
    ) {
        self.as_mut().create_function(value, fc, user_data)
    }

    #[inline(always)]
    fn set_variable(
        &mut self,
        path_to_var: *const i8,
        value: &GFxValue,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        self.as_mut().set_variable(path_to_var, value, set_type)
    }

    #[inline(always)]
    fn get_variable(&self, value: *mut GFxValue, path_to_var: *const i8) -> bool {
        self.as_ref().get_variable(value, path_to_var)
    }

    #[inline(always)]
    fn set_variable_array(
        &self,
        array_type: GFxMovieSetArrayType,
        path_to_var: *const i8,
        index: u32,
        data: *const core::ffi::c_void,
        count: u32,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        self.as_ref()
            .set_variable_array(array_type, path_to_var, index, data, count, set_type)
    }

    #[inline(always)]
    fn set_variable_array_size(
        &mut self,
        path_to_var: *const i8,
        count: u32,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        self.as_mut()
            .set_variable_array_size(path_to_var, count, set_type)
    }

    #[inline(always)]
    fn get_variable_array_size(&mut self, path_to_var: *const i8) -> u32 {
        self.as_mut().get_variable_array_size(path_to_var)
    }

    #[inline(always)]
    fn get_variable_array(
        &mut self,
        array_type: GFxMovieSetArrayType,
        path_to_var: *const i8,
        index: u32,
        data: *mut core::ffi::c_void,
        count: u32,
    ) -> bool {
        self.as_mut()
            .get_variable_array(array_type, path_to_var, index, data, count)
    }

    #[inline(always)]
    fn invoke_fmt(
        &mut self,
        method_name: *const i8,
        result: *mut GFxValue,
        arg_fmt: *const i8,
    ) -> bool {
        self.as_mut().invoke_fmt(method_name, result, arg_fmt)
    }

    #[inline(always)]
    fn invoke(
        &mut self,
        method_name: *const i8,
        result: *mut GFxValue,
        args: *const GFxValue,
        num_args: u32,
    ) -> bool {
        self.as_mut().invoke(method_name, result, args, num_args)
    }

    #[inline(always)]
    fn invoke_args(
        &mut self,
        method_name: *const i8,
        result: *mut GFxValue,
        arg_fmt: *const i8,
        args: *mut core::ffi::c_void,
    ) -> bool {
        self.as_mut()
            .invoke_args(method_name, result, arg_fmt, args)
    }

    #[inline(always)]
    fn get_frame_count(&self) -> u32 {
        self.as_ref().get_frame_count()
    }

    #[inline(always)]
    fn get_frame_rate(&self) -> f32 {
        self.as_ref().get_frame_rate()
    }

    #[inline(always)]
    unsafe fn set_variable_c_str(
        &mut self,
        path_to_var: *const i8,
        value: *const i8,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        unsafe {
            self.as_mut()
                .set_variable_c_str(path_to_var, value, set_type)
        }
    }

    #[inline(always)]
    unsafe fn set_variable_w_str(
        &mut self,
        path_to_var: *const i8,
        value: *const u16,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        unsafe {
            self.as_mut()
                .set_variable_w_str(path_to_var, value, set_type)
        }
    }

    #[inline(always)]
    fn set_variable_double(
        &mut self,
        path_to_var: *const i8,
        value: f64,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        self.as_mut()
            .set_variable_double(path_to_var, value, set_type)
    }

    #[inline(always)]
    fn get_variable_double(&self, path_to_var: *const i8) -> f64 {
        self.as_ref().get_variable_double(path_to_var)
    }

    #[inline(always)]
    fn set_variable_value_array(
        &self,
        path_to_var: *const i8,
        index: u32,
        data: *const GFxValue,
        count: u32,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        self.as_ref()
            .set_variable_value_array(path_to_var, index, data, count, set_type)
    }

    #[inline(always)]
    fn get_variable_value_array(
        &self,
        path_to_var: *const i8,
        index: u32,
        data: *mut GFxValue,
        count: u32,
    ) -> bool {
        self.as_ref()
            .get_variable_value_array(path_to_var, index, data, count)
    }

    #[inline(always)]
    fn set_color_tint(&self, path_to_var: *const i8, tint: &GColor) -> bool {
        self.as_ref().set_color_tint(path_to_var, tint)
    }

    #[inline(always)]
    fn remove_color_tint(&self, path_to_var: *const i8) -> bool {
        self.as_ref().remove_color_tint(path_to_var)
    }
}

impl<T> GFxMovieExt for T where T: AsRef<GFxMovie> + AsMut<GFxMovie> {}
