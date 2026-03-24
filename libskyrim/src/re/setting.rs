use core::ffi::{c_char, c_void};
use core::ptr;

use crate::offsets::offsets_rtti::RTTI_Setting;
use crate::offsets::offsets_vtable::VTABLE_Setting;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::Setting::Type`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingType {
    Unknown = -1,
    Bool = 0,
    Character = 1,
    UnsignedCharacter = 2,
    Integer = 3,
    UnsignedInteger = 4,
    Float = 5,
    String = 6,
    ColorRGB = 7,
    ColorRGBA = 8,
}

/// C++ `RE::Setting::Data`
#[repr(C)]
#[derive(Clone, Copy)]
pub union SettingData {
    pub b: bool,
    pub c: i8,
    pub h: u8,
    pub i: i32,
    pub u: u32,
    pub f: f32,
    pub s: *mut c_char,
    pub r: u32,
    pub a: u32,
}

const _: () = assert!(core::mem::size_of::<SettingData>() == 0x8);

/// C++ `RE::Setting`
#[repr(C)]
pub struct Setting {
    pub vtable: *const usize, // 00
    pub data: SettingData,    // 08
    name: *mut c_char,        // 10
}

const _: () = assert!(core::mem::size_of::<Setting>() == 0x18);
const _: () = assert!(core::mem::offset_of!(Setting, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(Setting, data) == 0x08);
const _: () = assert!(core::mem::offset_of!(Setting, name) == 0x10);

impl RttiType for Setting {
    const RTTI: VariantID = RTTI_Setting;
}

impl AsRef<Setting> for Setting {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Setting> for Setting {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Setting {
    pub const RTTI: VariantID = RTTI_Setting;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Setting;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_IS_PREFERENCE_SETTING: usize = 0x01;
        pub fn is_preference_setting() -> bool
    }

    #[inline(always)]
    pub fn new_name(name: &str) -> Self {
        Self {
            vtable: Self::VTABLE[0].address() as *const usize,
            data: SettingData { s: ptr::null_mut() },
            name: alloc_c_string(name),
        }
    }

    #[inline(always)]
    pub fn new_bool(name: &str, value: bool) -> Self {
        let mut setting = Self::new_name(name);
        setting.set_bool(value);
        setting
    }

    #[inline(always)]
    pub fn new_string(name: &str, value: &str) -> Self {
        let mut setting = Self::new_name(name);
        setting.set_string(value);
        setting
    }

    #[inline(always)]
    pub fn new_float(name: &str, value: f32) -> Self {
        let mut setting = Self::new_name(name);
        setting.set_float(value);
        setting
    }

    #[inline(always)]
    pub fn new_character(name: &str, value: i8) -> Self {
        let mut setting = Self::new_name(name);
        setting.set_character(value);
        setting
    }

    #[inline(always)]
    pub fn new_integer(name: &str, value: i32) -> Self {
        let mut setting = Self::new_name(name);
        setting.set_integer(value);
        setting
    }

    #[inline(always)]
    pub fn new_unsigned_character(name: &str, value: u8) -> Self {
        let mut setting = Self::new_name(name);
        setting.set_unsigned_character(value);
        setting
    }

    #[inline(always)]
    pub fn new_unsigned_integer(name: &str, value: u32) -> Self {
        let mut setting = Self::new_name(name);
        setting.set_unsigned_integer(value);
        setting
    }

    pub fn free_managed_strings(&mut self) {
        if self.is_managed() {
            free_c_string(self.name);
            let string_ptr = unsafe { self.data.s };
            free_c_string(string_ptr);
        }
        self.name = ptr::null_mut();
        self.data = SettingData { s: ptr::null_mut() };
    }

    #[inline(always)]
    pub fn is_managed(&self) -> bool {
        !self.name.is_null() && unsafe { *self.name.cast::<u8>() } == b'S'
    }

    pub fn get_type(&self) -> SettingType {
        if self.name.is_null() {
            return SettingType::Unknown;
        }

        match unsafe { *self.name.cast::<u8>() } {
            b'b' => SettingType::Bool,
            b'c' => SettingType::Character,
            b'h' => SettingType::UnsignedCharacter,
            b'f' => SettingType::Float,
            b'i' => SettingType::Integer,
            b'r' => SettingType::ColorRGB,
            b'a' => SettingType::ColorRGBA,
            b'S' | b's' => SettingType::String,
            b'u' => SettingType::UnsignedInteger,
            _ => SettingType::Unknown,
        }
    }

    #[inline(always)]
    pub fn get_name(&self) -> *const c_char {
        if self.name.is_null() {
            c"".as_ptr()
        } else {
            self.name
        }
    }

    #[inline(always)]
    pub fn get_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_name())
    }

    #[inline(always)]
    pub fn get_bool(&self) -> bool {
        unsafe { self.data.b }
    }

    #[inline(always)]
    pub fn get_character(&self) -> i8 {
        unsafe { self.data.c }
    }

    #[inline(always)]
    pub fn get_float(&self) -> f32 {
        unsafe { self.data.f }
    }

    #[inline(always)]
    pub fn get_integer(&self) -> i32 {
        unsafe { self.data.i }
    }

    #[inline(always)]
    pub fn get_string(&self) -> *const c_char {
        unsafe { self.data.s }
    }

    #[inline(always)]
    pub fn get_string_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_string())
    }

    #[inline(always)]
    pub fn get_unsigned_character(&self) -> u8 {
        unsafe { self.data.h }
    }

    #[inline(always)]
    pub fn get_unsigned_integer(&self) -> u32 {
        unsafe { self.data.u }
    }

    #[inline(always)]
    pub fn get_color(&self) -> u32 {
        unsafe { self.data.r }
    }

    #[inline(always)]
    pub fn get_color_a(&self) -> u32 {
        unsafe { self.data.a }
    }

    #[inline(always)]
    pub fn set_bool(&mut self, value: bool) {
        self.data = SettingData { b: value };
    }

    #[inline(always)]
    pub fn set_character(&mut self, value: i8) {
        self.data = SettingData { c: value };
    }

    #[inline(always)]
    pub fn set_float(&mut self, value: f32) {
        self.data = SettingData { f: value };
    }

    #[inline(always)]
    pub fn set_integer(&mut self, value: i32) {
        self.data = SettingData { i: value };
    }

    pub fn set_string(&mut self, value: &str) {
        let old = unsafe { self.data.s };
        free_c_string(old);
        self.data = SettingData {
            s: alloc_c_string(value),
        };
    }

    #[inline(always)]
    pub fn set_unsigned_character(&mut self, value: u8) {
        self.data = SettingData { h: value };
    }

    #[inline(always)]
    pub fn set_unsigned_integer(&mut self, value: u32) {
        self.data = SettingData { u: value };
    }

    #[inline(always)]
    pub fn set_color(&mut self, value: u32) {
        self.data = SettingData { r: value };
    }

    #[inline(always)]
    pub fn set_color_a(&mut self, value: u32) {
        self.data = SettingData { a: value };
    }
}

pub trait SettingExt {
    fn dtor(&mut self);
    fn is_preference_setting(&self) -> bool;
    fn free_managed_strings(&mut self);
    fn is_managed(&self) -> bool;
    fn get_type(&self) -> SettingType;
    fn get_name(&self) -> *const c_char;
    fn get_name_as_str(&self) -> &str;
    fn get_bool(&self) -> bool;
    fn get_character(&self) -> i8;
    fn get_float(&self) -> f32;
    fn get_integer(&self) -> i32;
    fn get_string(&self) -> *const c_char;
    fn get_string_as_str(&self) -> &str;
    fn get_unsigned_character(&self) -> u8;
    fn get_unsigned_integer(&self) -> u32;
    fn get_color(&self) -> u32;
    fn get_color_a(&self) -> u32;
    fn set_bool(&mut self, value: bool);
    fn set_character(&mut self, value: i8);
    fn set_float(&mut self, value: f32);
    fn set_integer(&mut self, value: i32);
    fn set_string(&mut self, value: &str);
    fn set_unsigned_character(&mut self, value: u8);
    fn set_unsigned_integer(&mut self, value: u32);
    fn set_color(&mut self, value: u32);
    fn set_color_a(&mut self, value: u32);
}

impl<T: AsRef<Setting> + AsMut<Setting>> SettingExt for T {
    fn dtor(&mut self) {
        Setting::dtor(self.as_mut())
    }

    fn is_preference_setting(&self) -> bool {
        Setting::is_preference_setting(self.as_ref())
    }

    fn free_managed_strings(&mut self) {
        Setting::free_managed_strings(self.as_mut())
    }

    fn is_managed(&self) -> bool {
        Setting::is_managed(self.as_ref())
    }

    fn get_type(&self) -> SettingType {
        Setting::get_type(self.as_ref())
    }

    fn get_name(&self) -> *const c_char {
        Setting::get_name(self.as_ref())
    }

    fn get_name_as_str(&self) -> &str {
        Setting::get_name_as_str(self.as_ref())
    }

    fn get_bool(&self) -> bool {
        Setting::get_bool(self.as_ref())
    }

    fn get_character(&self) -> i8 {
        Setting::get_character(self.as_ref())
    }

    fn get_float(&self) -> f32 {
        Setting::get_float(self.as_ref())
    }

    fn get_integer(&self) -> i32 {
        Setting::get_integer(self.as_ref())
    }

    fn get_string(&self) -> *const c_char {
        Setting::get_string(self.as_ref())
    }

    fn get_string_as_str(&self) -> &str {
        Setting::get_string_as_str(self.as_ref())
    }

    fn get_unsigned_character(&self) -> u8 {
        Setting::get_unsigned_character(self.as_ref())
    }

    fn get_unsigned_integer(&self) -> u32 {
        Setting::get_unsigned_integer(self.as_ref())
    }

    fn get_color(&self) -> u32 {
        Setting::get_color(self.as_ref())
    }

    fn get_color_a(&self) -> u32 {
        Setting::get_color_a(self.as_ref())
    }

    fn set_bool(&mut self, value: bool) {
        Setting::set_bool(self.as_mut(), value)
    }

    fn set_character(&mut self, value: i8) {
        Setting::set_character(self.as_mut(), value)
    }

    fn set_float(&mut self, value: f32) {
        Setting::set_float(self.as_mut(), value)
    }

    fn set_integer(&mut self, value: i32) {
        Setting::set_integer(self.as_mut(), value)
    }

    fn set_string(&mut self, value: &str) {
        Setting::set_string(self.as_mut(), value)
    }

    fn set_unsigned_character(&mut self, value: u8) {
        Setting::set_unsigned_character(self.as_mut(), value)
    }

    fn set_unsigned_integer(&mut self, value: u32) {
        Setting::set_unsigned_integer(self.as_mut(), value)
    }

    fn set_color(&mut self, value: u32) {
        Setting::set_color(self.as_mut(), value)
    }

    fn set_color_a(&mut self, value: u32) {
        Setting::set_color_a(self.as_mut(), value)
    }
}

fn alloc_c_string(value: &str) -> *mut c_char {
    let bytes = value.as_bytes();
    let len = bytes.len() + 1;
    let ptr = unsafe { crate::ffi::commonlib_malloc(len) }.cast::<u8>();
    assert!(!ptr.is_null(), "Setting string allocation failed");
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr.cast()
}

fn free_c_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { crate::ffi::commonlib_free(ptr.cast::<c_void>()) };
    }
}
