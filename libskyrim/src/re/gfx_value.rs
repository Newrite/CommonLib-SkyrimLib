#![allow(non_camel_case_types)]

use core::ffi::{CStr, c_char, c_void};
use core::marker::PhantomData;

use core_util::{Enum, EnumSet};

use crate::re::{
    GColor, GFxMovieRoot, GFxStatMovieViews, GMatrix2D, GMatrix3D, GNewOverrideBase,
    GRendererCxform, GString,
};
use crate::relocation::RelocationID;

unsafe extern "C" {
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> i32;
    fn round(value: f64) -> f64;
    fn wcscoll(lhs: *const u16, rhs: *const u16) -> i32;
}

/// C++ `RE::GFxValue::ValueType`
#[libskyrim_macros::open_enum(ignore(kManagedBit, kConvertBit, kValueMask, kTypeMask))]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxValueValueType {
    kUndefined = 0x00,
    kNull = 0x01,
    kBoolean = 0x02,
    kNumber = 0x03,
    kString = 0x04,
    kStringW = 0x05,
    kObject = 0x06,
    kArray = 0x07,
    kDisplayObject = 0x08,
    kManagedBit = 1 << 6,
    kConvertBit = 1 << 7,
    kValueMask = 0x0F,
    kTypeMask = (1 << 7) | 0x0F,
    kConvertBoolean = (1 << 7) | 0x02,
    kConvertNumber = (1 << 7) | 0x03,
    kConvertString = (1 << 7) | 0x04,
    kConvertStringW = (1 << 7) | 0x05,
}
/// C++ `RE::GFxValue::DisplayInfo::Flag`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxValueDisplayInfoFlag {
    kNone = 0,
    kX = 1 << 0,
    kY = 1 << 1,
    kRotation = 1 << 2,
    kXScale = 1 << 3,
    kYScale = 1 << 4,
    kAlpha = 1 << 5,
    kVisible = 1 << 6,
    kZ = 1 << 7,
    kXRotation = 1 << 8,
    kYRotation = 1 << 9,
    kZScale = 1 << 10,
    kFOV = 1 << 11,
    kPerspMatrix3D = 1 << 12,
    kViewMatrix3D = 1 << 13,
}

core_util::impl_enumset_type!(GFxValueDisplayInfoFlag => u16);

/// C++ `RE::GFxValue::DisplayInfo`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GFxValueDisplayInfo {
    pub x: f64,                                       // 00
    pub y: f64,                                       // 08
    pub rotation: f64,                                // 10
    pub x_scale: f64,                                 // 18
    pub y_scale: f64,                                 // 20
    pub alpha: f64,                                   // 28
    pub visible: bool,                                // 30
    pub pad31: u8,                                    // 31
    pub pad32: u16,                                   // 32
    pub pad34: u32,                                   // 34
    pub z: f64,                                       // 38
    pub x_rotation: f64,                              // 40
    pub y_rotation: f64,                              // 48
    pub z_scale: f64,                                 // 50
    pub fov: f64,                                     // 58
    pub view_matrix_3d: GMatrix3D,                    // 60
    pub persp_matrix_3d: GMatrix3D,                   // A0
    pub flags: EnumSet<GFxValueDisplayInfoFlag, u16>, // E0
    pub pad_e2: u16,                                  // E2
    pub pad_e4: u32,                                  // E4
}

const _: () = assert!(core::mem::size_of::<GFxValueDisplayInfo>() == 0xE8);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, x) == 0x00);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, y) == 0x08);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, rotation) == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, x_scale) == 0x18);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, y_scale) == 0x20);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, alpha) == 0x28);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, visible) == 0x30);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, z) == 0x38);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, x_rotation) == 0x40);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, y_rotation) == 0x48);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, z_scale) == 0x50);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, fov) == 0x58);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, view_matrix_3d) == 0x60);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, persp_matrix_3d) == 0xA0);
const _: () = assert!(core::mem::offset_of!(GFxValueDisplayInfo, flags) == 0xE0);

impl Default for GFxValueDisplayInfo {
    #[inline(always)]
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            x_scale: 0.0,
            y_scale: 0.0,
            alpha: 0.0,
            visible: false,
            pad31: 0,
            pad32: 0,
            pad34: 0,
            z: 0.0,
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_scale: 0.0,
            fov: 0.0,
            view_matrix_3d: GMatrix3D::default(),
            persp_matrix_3d: GMatrix3D::default(),
            flags: EnumSet::from_underlying(GFxValueDisplayInfoFlag::kNone as u16),
            pad_e2: 0,
            pad_e4: 0,
        }
    }
}

impl GFxValueDisplayInfo {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn with_position(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            ..Self::default()
        }
    }

    #[inline(always)]
    pub fn with_rotation(rotation: f64) -> Self {
        Self {
            rotation,
            ..Self::default()
        }
    }

    #[inline(always)]
    pub fn with_visibility(visible: bool) -> Self {
        Self {
            visible,
            ..Self::default()
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.flags = EnumSet::from_underlying(GFxValueDisplayInfoFlag::kNone as u16);
    }

    #[inline(always)]
    pub fn get_alpha(&self) -> f64 {
        self.alpha
    }

    #[inline(always)]
    pub fn get_fov(&self) -> f64 {
        self.fov
    }

    #[inline(always)]
    pub fn get_rotation(&self) -> f64 {
        self.rotation
    }

    #[inline(always)]
    pub fn get_visible(&self) -> bool {
        self.visible
    }

    #[inline(always)]
    pub fn get_perspective_matrix_3d(&self) -> Option<&GMatrix3D> {
        self.is_flag_set(GFxValueDisplayInfoFlag::kPerspMatrix3D)
            .then_some(&self.persp_matrix_3d)
    }

    #[inline(always)]
    pub fn get_view_matrix_3d(&self) -> Option<&GMatrix3D> {
        self.is_flag_set(GFxValueDisplayInfoFlag::kViewMatrix3D)
            .then_some(&self.view_matrix_3d)
    }

    #[inline(always)]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    #[inline(always)]
    pub fn get_z(&self) -> f64 {
        self.z
    }

    #[inline(always)]
    pub fn get_x_rotation(&self) -> f64 {
        self.x_rotation
    }

    #[inline(always)]
    pub fn get_y_rotation(&self) -> f64 {
        self.y_rotation
    }

    #[inline(always)]
    pub fn get_x_scale(&self) -> f64 {
        self.x_scale
    }

    #[inline(always)]
    pub fn get_y_scale(&self) -> f64 {
        self.y_scale
    }

    #[inline(always)]
    pub fn get_z_scale(&self) -> f64 {
        self.z_scale
    }

    #[inline(always)]
    pub fn is_flag_set(&self, flag: GFxValueDisplayInfoFlag) -> bool {
        self.flags.all(flag)
    }

    #[inline(always)]
    pub fn initialize(
        &mut self,
        vars_set: GFxValueDisplayInfoFlag,
        x: f64,
        y: f64,
        rotation: f64,
        x_scale: f64,
        y_scale: f64,
        alpha: f64,
        visible: bool,
        z: f64,
        x_rotation: f64,
        y_rotation: f64,
        z_scale: f64,
        fov: f64,
        view_matrix_3d: &GMatrix3D,
        persp_matrix_3d: &GMatrix3D,
    ) {
        self.flags = EnumSet::from_underlying(vars_set as u16);
        self.x = x;
        self.y = y;
        self.rotation = rotation;
        self.x_scale = x_scale;
        self.y_scale = y_scale;
        self.alpha = alpha;
        self.visible = visible;
        self.z = z;
        self.x_rotation = x_rotation;
        self.y_rotation = y_rotation;
        self.z_scale = z_scale;
        self.fov = fov;
        self.view_matrix_3d = *view_matrix_3d;
        self.persp_matrix_3d = *persp_matrix_3d;
    }

    #[inline(always)]
    pub fn set_visible(&mut self, visible: bool) {
        self.set_flags(GFxValueDisplayInfoFlag::kVisible);
        self.visible = visible;
    }

    #[inline(always)]
    pub fn set_perspective_matrix_3d(&mut self, matrix: Option<&GMatrix3D>) {
        if let Some(matrix) = matrix {
            self.set_flags(GFxValueDisplayInfoFlag::kPerspMatrix3D);
            self.persp_matrix_3d = *matrix;
        } else {
            self.clear_flags(GFxValueDisplayInfoFlag::kPerspMatrix3D);
        }
    }

    #[inline(always)]
    pub fn set_2d(
        &mut self,
        x: f64,
        y: f64,
        rotation: f64,
        x_scale: f64,
        y_scale: f64,
        alpha: f64,
        visible: bool,
    ) {
        self.flags.set_many([
            GFxValueDisplayInfoFlag::kX,
            GFxValueDisplayInfoFlag::kY,
            GFxValueDisplayInfoFlag::kRotation,
            GFxValueDisplayInfoFlag::kXScale,
            GFxValueDisplayInfoFlag::kYScale,
            GFxValueDisplayInfoFlag::kAlpha,
            GFxValueDisplayInfoFlag::kVisible,
        ]);
        self.x = x;
        self.y = y;
        self.rotation = rotation;
        self.x_scale = x_scale;
        self.y_scale = y_scale;
        self.alpha = alpha;
        self.visible = visible;
    }

    #[inline(always)]
    pub fn set_3d(
        &mut self,
        x: f64,
        y: f64,
        rotation: f64,
        x_scale: f64,
        y_scale: f64,
        alpha: f64,
        visible: bool,
        z: f64,
        x_rotation: f64,
        y_rotation: f64,
        z_scale: f64,
    ) {
        self.flags.set_many([
            GFxValueDisplayInfoFlag::kX,
            GFxValueDisplayInfoFlag::kY,
            GFxValueDisplayInfoFlag::kRotation,
            GFxValueDisplayInfoFlag::kXScale,
            GFxValueDisplayInfoFlag::kYScale,
            GFxValueDisplayInfoFlag::kAlpha,
            GFxValueDisplayInfoFlag::kVisible,
            GFxValueDisplayInfoFlag::kZ,
            GFxValueDisplayInfoFlag::kXRotation,
            GFxValueDisplayInfoFlag::kYRotation,
            GFxValueDisplayInfoFlag::kZScale,
        ]);
        self.x = x;
        self.y = y;
        self.rotation = rotation;
        self.x_scale = x_scale;
        self.y_scale = y_scale;
        self.alpha = alpha;
        self.visible = visible;
        self.z = z;
        self.x_rotation = x_rotation;
        self.y_rotation = y_rotation;
        self.z_scale = z_scale;
    }

    #[inline(always)]
    pub fn set_view_matrix_3d(&mut self, matrix: Option<&GMatrix3D>) {
        if let Some(matrix) = matrix {
            self.set_flags(GFxValueDisplayInfoFlag::kViewMatrix3D);
            self.view_matrix_3d = *matrix;
        } else {
            self.clear_flags(GFxValueDisplayInfoFlag::kViewMatrix3D);
        }
    }

    #[inline(always)]
    pub fn set_alpha(&mut self, alpha: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kAlpha);
        self.alpha = alpha;
    }

    #[inline(always)]
    pub fn set_fov(&mut self, fov: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kFOV);
        self.fov = fov;
    }

    #[inline(always)]
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.flags
            .set_many([GFxValueDisplayInfoFlag::kX, GFxValueDisplayInfoFlag::kY]);
        self.x = x;
        self.y = y;
    }

    #[inline(always)]
    pub fn set_rotation(&mut self, degrees: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kRotation);
        self.rotation = degrees;
    }

    #[inline(always)]
    pub fn set_scale(&mut self, x_scale: f64, y_scale: f64) {
        self.flags.set_many([
            GFxValueDisplayInfoFlag::kXScale,
            GFxValueDisplayInfoFlag::kYScale,
        ]);
        self.x_scale = x_scale;
        self.y_scale = y_scale;
    }

    #[inline(always)]
    pub fn set_x(&mut self, x: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kX);
        self.x = x;
    }

    #[inline(always)]
    pub fn set_x_rotation(&mut self, degrees: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kXRotation);
        self.x_rotation = degrees;
    }

    #[inline(always)]
    pub fn set_x_scale(&mut self, x_scale: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kXScale);
        self.x_scale = x_scale;
    }

    #[inline(always)]
    pub fn set_y(&mut self, y: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kY);
        self.y = y;
    }

    #[inline(always)]
    pub fn set_y_rotation(&mut self, degrees: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kRotation);
        self.y_rotation = degrees;
    }

    #[inline(always)]
    pub fn set_y_scale(&mut self, y_scale: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kYScale);
        self.y_scale = y_scale;
    }

    #[inline(always)]
    pub fn set_z(&mut self, z: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kZ);
        self.z = z;
    }

    #[inline(always)]
    pub fn set_z_scale(&mut self, z_scale: f64) {
        self.set_flags(GFxValueDisplayInfoFlag::kZScale);
        self.z_scale = z_scale;
    }

    #[inline(always)]
    fn set_flags(&mut self, flags: GFxValueDisplayInfoFlag) {
        self.flags.set(flags);
    }

    #[inline(always)]
    fn clear_flags(&mut self, flags: GFxValueDisplayInfoFlag) {
        self.flags.reset(flags);
    }
}

/// C++ `RE::GFxValue::ObjectInterface::ObjVisitor`
#[repr(C)]
pub struct GFxValueObjVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GFxValueObjVisitor>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GFxValueObjVisitor, vtable) == 0x00);

impl GFxValueObjVisitor {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_VISIT: usize = 0x01;
        pub fn visit(name: *const c_char, value: &GFxValue)
    }
}

/// C++ `RE::GFxValue::ObjectInterface::ArrVisitor`
#[repr(C)]
pub struct GFxValueArrVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GFxValueArrVisitor>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GFxValueArrVisitor, vtable) == 0x00);

impl GFxValueArrVisitor {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_VISIT: usize = 0x01;
        pub fn visit(index: u32, value: &GFxValue)
    }
}

pub type GFxValueObjectVisitor = GFxValueObjVisitor;
pub type GFxValueArrayVisitor = GFxValueArrVisitor;

/// C++ `RE::GFxValue::ObjectInterface`
#[repr(C)]
pub struct GFxValueObjectInterface {
    pub base: GNewOverrideBase<{ GFxStatMovieViews::OTHER_MEM as u32 }>,
    pub movie_root: *mut GFxMovieRoot, // 00
}

const _: () = assert!(core::mem::size_of::<GFxValueObjectInterface>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GFxValueObjectInterface, movie_root) == 0x00);

impl GFxValueObjectInterface {
    #[inline(always)]
    pub const fn new(movie_root: *mut GFxMovieRoot) -> Self {
        Self {
            base: GNewOverrideBase {
                _marker: PhantomData,
            },
            movie_root,
        }
    }

    crate::relocation_func! {
        pub fn object_add_ref(&self, value: *mut GFxValue, object: *mut c_void) => RelocationID::new(80244, 82269)
    }

    crate::relocation_func! {
        pub fn object_release(&self, value: *mut GFxValue, object: *mut c_void) => RelocationID::new(80245, 82270)
    }

    crate::relocation_func! {
        pub fn has_member(&self, data: *mut c_void, name: *const c_char, is_display_object: bool) -> bool => RelocationID::new(80231, 82254)
    }

    crate::relocation_func! {
        pub fn get_member(&self, data: *mut c_void, name: *const c_char, value: *mut GFxValue, is_display_object: bool) -> bool => RelocationID::new(80222, 82245)
    }

    crate::relocation_func! {
        pub fn set_member(&self, data: *mut c_void, name: *const c_char, value: &GFxValue, is_display_object: bool) -> bool => RelocationID::new(80268, 82292)
    }

    crate::relocation_func! {
        pub fn invoke(&self, data: *mut c_void, result: *mut GFxValue, name: *const c_char, args: *const GFxValue, num_args: usize, is_display_object: bool) -> bool => RelocationID::new(80233, 82256)
    }

    crate::relocation_func! {
        pub fn delete_member(&self, data: *mut c_void, name: *const c_char, is_display_object: bool) -> bool => RelocationID::new(80207, 82230)
    }

    crate::relocation_func! {
        pub fn visit_members(&self, data: *mut c_void, visitor: *mut GFxValueObjectVisitor, is_display_object: bool) => RelocationID::new(80279, 82302)
    }

    crate::relocation_func! {
        pub fn get_array_size(&self, data: *mut c_void) -> u32 => RelocationID::new(80214, 82237)
    }

    crate::relocation_func! {
        pub fn set_array_size(&self, data: *mut c_void, size: u32) -> bool => RelocationID::new(80261, 82285)
    }

    crate::relocation_func! {
        pub fn get_element(&self, data: *mut c_void, index: u32, value: *mut GFxValue) -> bool => RelocationID::new(80218, 82241)
    }

    crate::relocation_func! {
        pub fn set_element(&self, data: *mut c_void, index: u32, value: &GFxValue) -> bool => RelocationID::new(80265, 82289)
    }

    crate::relocation_func! {
        pub fn push_back(&self, data: *mut c_void, value: &GFxValue) -> bool => RelocationID::new(80248, 82273)
    }

    crate::relocation_func! {
        pub fn remove_elements(&self, data: *mut c_void, index: u32, count: i32) -> bool => RelocationID::new(80252, 82280)
    }

    crate::relocation_func! {
        pub fn get_display_info(&self, data: *mut c_void, info: *mut GFxValueDisplayInfo) -> bool => RelocationID::new(80216, 82239)
    }

    crate::relocation_func! {
        pub fn set_display_info(&self, data: *mut c_void, info: &GFxValueDisplayInfo) -> bool => RelocationID::new(80263, 82287)
    }

    crate::relocation_func! {
        pub fn get_display_matrix(&self, data: *mut c_void, matrix: *mut GMatrix2D) -> bool => RelocationID::new(80217, 82240)
    }

    crate::relocation_func! {
        pub fn set_display_matrix(&self, data: *mut c_void, matrix: &GMatrix2D) -> bool => RelocationID::new(80264, 82288)
    }

    crate::relocation_func! {
        pub fn get_cxform(&self, data: *mut c_void, cxform: *mut GRendererCxform) -> bool => RelocationID::new(80215, 82238)
    }

    crate::relocation_func! {
        pub fn set_cxform(&self, data: *mut c_void, cxform: &GRendererCxform) -> bool => RelocationID::new(80262, 82286)
    }

    crate::relocation_func! {
        pub fn set_text(&self, data: *mut c_void, text: *const c_char, is_html: bool) -> bool => RelocationID::new(80270, 82293)
    }

    crate::relocation_func! {
        pub fn attach_movie(&self, data: *mut c_void, movie_clip: *mut GFxValue, symbol_name: *const c_char, instance_name: *const c_char, depth: i32, init_obj: *const GFxValue) -> bool => RelocationID::new(80197, 82219)
    }

    crate::relocation_func! {
        pub fn create_empty_movie_clip(&self, data: *mut c_void, movie_clip: *mut GFxValue, instance_name: *const c_char, depth: i32) -> bool => RelocationID::new(80201, 82224)
    }

    crate::relocation_func! {
        pub fn goto_and_play(&self, data: *mut c_void, frame: *const c_char, stop: bool) -> bool => RelocationID::new(80230, 82253)
    }

    #[inline(always)]
    pub fn is_same_context(&self, rhs: &Self) -> bool {
        self.movie_root == rhs.movie_root
    }
}

/// C++ `RE::GFxValue::ValueUnion`
#[repr(C)]
#[derive(Clone, Copy)]
pub union GFxValueValueUnion {
    pub number: f64,
    pub boolean: bool,
    pub string: *const c_char,
    pub managed_string: *mut *const c_char,
    pub wide_string: *const u16,
    pub managed_wide_string: *mut *const u16,
    pub object: *mut c_void,
}

const _: () = assert!(core::mem::size_of::<GFxValueValueUnion>() == 0x8);

impl Default for GFxValueValueUnion {
    #[inline(always)]
    fn default() -> Self {
        Self {
            object: core::ptr::null_mut(),
        }
    }
}

/// C++ `RE::GFxValue`
#[repr(C)]
pub struct GFxValue {
    pub object_interface: *mut GFxValueObjectInterface, // 00
    pub type_: EnumSet<GFxValueValueType, u32>,         // 08
    pub pad0c: u32,                                     // 0C
    pub value: GFxValueValueUnion,                      // 10
}

const _: () = assert!(core::mem::size_of::<GFxValue>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GFxValue, object_interface) == 0x00);
const _: () = assert!(core::mem::offset_of!(GFxValue, type_) == 0x08);
const _: () = assert!(core::mem::offset_of!(GFxValue, pad0c) == 0x0C);
const _: () = assert!(core::mem::offset_of!(GFxValue, value) == 0x10);

impl Default for GFxValue {
    #[inline(always)]
    fn default() -> Self {
        Self {
            object_interface: core::ptr::null_mut(),
            type_: EnumSet::from_underlying(GFxValueValueType::kUndefined as u32),
            pad0c: 0,
            value: GFxValueValueUnion::default(),
        }
    }
}

impl Clone for GFxValue {
    #[inline(always)]
    fn clone(&self) -> Self {
        let mut value = Self {
            object_interface: core::ptr::null_mut(),
            type_: self.type_,
            pad0c: 0,
            value: self.value,
        };
        if self.is_managed_value() {
            value.acquire_managed_value(self);
        }
        value
    }

    #[inline(always)]
    fn clone_from(&mut self, source: &Self) {
        self.assign(source);
    }
}

impl Drop for GFxValue {
    #[inline(always)]
    fn drop(&mut self) {
        if self.is_managed_value() {
            self.release_managed_value();
        }
    }
}

impl PartialEq for GFxValue {
    fn eq(&self, other: &Self) -> bool {
        if self.type_ != other.type_ {
            return false;
        }

        match self.get_type() {
            GFxValueValueType::kBoolean => self.get_bool() == other.get_bool(),
            GFxValueValueType::kNumber => self.get_number() == other.get_number(),
            GFxValueValueType::kString => unsafe {
                strcmp(self.get_string(), other.get_string()) == 0
            },
            GFxValueValueType::kStringW => unsafe {
                wcscoll(self.get_string_w(), other.get_string_w()) == 0
            },
            _ => unsafe { self.value.object == other.value.object },
        }
    }
}

impl Eq for GFxValue {}

impl core::fmt::Debug for GFxValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = f.debug_struct("GFxValue");
        debug.field("type", &self.get_type());
        match self.get_type() {
            GFxValueValueType::kBoolean => {
                debug.field("value", &self.get_bool());
            }
            GFxValueValueType::kNumber => {
                debug.field("value", &self.get_number());
            }
            GFxValueValueType::kString => {
                debug.field("value", &self.get_string_c_str().map(CStr::to_bytes));
            }
            GFxValueValueType::kStringW => {
                debug.field("wide_string", &self.get_string_w());
            }
            _ => unsafe {
                debug.field("object", &self.value.object);
            },
        }
        debug.finish()
    }
}

macro_rules! impl_gfx_value_from_int {
    ($($ty:ty),* $(,)?) => {
        $(
            impl From<$ty> for GFxValue {
                #[inline(always)]
                fn from(value: $ty) -> Self {
                    Self::from_number(value as f64)
                }
            }
        )*
    };
}

impl_gfx_value_from_int!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

impl From<f64> for GFxValue {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self::from_number(value)
    }
}

impl From<bool> for GFxValue {
    #[inline(always)]
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl GFxValue {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn with_type(value_type: GFxValueValueType) -> Self {
        assert!(
            value_type != GFxValueValueType::kObject
                && value_type != GFxValueValueType::kArray
                && value_type != GFxValueValueType::kDisplayObject
        );

        Self {
            object_interface: core::ptr::null_mut(),
            type_: EnumSet::from_underlying(value_type as u32),
            pad0c: 0,
            value: GFxValueValueUnion {
                string: core::ptr::null(),
            },
        }
    }

    #[inline(always)]
    pub fn from_null() -> Self {
        let mut value = Self::default();
        value.set_null();
        value
    }

    #[inline(always)]
    pub fn from_number(number: f64) -> Self {
        Self {
            object_interface: core::ptr::null_mut(),
            type_: EnumSet::from_underlying(GFxValueValueType::kNumber as u32),
            pad0c: 0,
            value: GFxValueValueUnion { number },
        }
    }

    #[inline(always)]
    pub fn from_bool(boolean: bool) -> Self {
        Self {
            object_interface: core::ptr::null_mut(),
            type_: EnumSet::from_underlying(GFxValueValueType::kBoolean as u32),
            pad0c: 0,
            value: GFxValueValueUnion { boolean },
        }
    }

    #[inline(always)]
    pub unsafe fn from_string(string: *const c_char) -> Self {
        Self {
            object_interface: core::ptr::null_mut(),
            type_: EnumSet::from_underlying(GFxValueValueType::kString as u32),
            pad0c: 0,
            value: GFxValueValueUnion { string },
        }
    }

    #[inline(always)]
    pub unsafe fn from_string_w(wide_string: *const u16) -> Self {
        Self {
            object_interface: core::ptr::null_mut(),
            type_: EnumSet::from_underlying(GFxValueValueType::kStringW as u32),
            pad0c: 0,
            value: GFxValueValueUnion { wide_string },
        }
    }

    #[inline(always)]
    pub fn to_g_string(&self) -> GString {
        unsafe { GString::from_c_str(self.get_string()) }
    }

    #[inline(always)]
    pub fn value_type_storage(&self) -> Enum<GFxValueValueType, u32> {
        Enum::from_underlying(self.type_.underlying() & (GFxValueValueType::kTypeMask as u32))
    }

    #[inline(always)]
    pub fn try_get_type(&self) -> Option<GFxValueValueType> {
        self.value_type_storage().get()
    }

    #[inline(always)]
    pub fn get_type(&self) -> GFxValueValueType {
        self.try_get_type().unwrap_or(GFxValueValueType::kUndefined)
    }

    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        self.get_type() == GFxValueValueType::kUndefined
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.get_type() == GFxValueValueType::kNull
    }

    #[inline(always)]
    pub fn is_bool(&self) -> bool {
        self.get_type() == GFxValueValueType::kBoolean
    }

    #[inline(always)]
    pub fn is_number(&self) -> bool {
        self.get_type() == GFxValueValueType::kNumber
    }

    #[inline(always)]
    pub fn is_string(&self) -> bool {
        self.get_type() == GFxValueValueType::kString
    }

    #[inline(always)]
    pub fn is_string_w(&self) -> bool {
        self.get_type() == GFxValueValueType::kStringW
    }

    #[inline(always)]
    pub fn is_object(&self) -> bool {
        matches!(
            self.get_type(),
            GFxValueValueType::kObject
                | GFxValueValueType::kArray
                | GFxValueValueType::kDisplayObject
        )
    }

    #[inline(always)]
    pub fn is_array(&self) -> bool {
        self.get_type() == GFxValueValueType::kArray
    }

    #[inline(always)]
    pub fn is_display_object(&self) -> bool {
        self.get_type() == GFxValueValueType::kDisplayObject
    }

    #[inline(always)]
    pub fn get_bool(&self) -> bool {
        assert!(self.is_bool());
        unsafe { self.value.boolean }
    }

    #[inline(always)]
    pub fn get_number(&self) -> f64 {
        assert!(self.is_number());
        unsafe { self.value.number }
    }

    #[inline(always)]
    pub fn get_sint(&self) -> isize {
        Self::round_to_isize(self.get_number())
    }

    #[inline(always)]
    pub fn get_uint(&self) -> usize {
        Self::round_to_isize(self.get_number()) as usize
    }

    #[inline(always)]
    pub fn get_string(&self) -> *const c_char {
        assert!(self.is_string());
        unsafe {
            if self.is_managed_value() {
                *self.value.managed_string
            } else {
                self.value.string
            }
        }
    }

    #[inline(always)]
    pub fn get_string_c_str(&self) -> Option<&CStr> {
        let string = self.get_string();
        (!string.is_null()).then(|| unsafe { CStr::from_ptr(string) })
    }

    #[inline(always)]
    pub fn get_string_w(&self) -> *const u16 {
        assert!(self.is_string_w());
        unsafe {
            if self.is_managed_value() {
                *self.value.managed_wide_string
            } else {
                self.value.wide_string
            }
        }
    }

    #[inline(always)]
    pub fn set_undefined(&mut self) {
        self.change_type(GFxValueValueType::kUndefined);
    }

    #[inline(always)]
    pub fn set_null(&mut self) {
        self.change_type(GFxValueValueType::kNull);
    }

    #[inline(always)]
    pub fn set_boolean(&mut self, value: bool) {
        self.change_type(GFxValueValueType::kBoolean);
        self.value = GFxValueValueUnion { boolean: value };
    }

    #[inline(always)]
    pub fn set_number(&mut self, value: f64) {
        self.change_type(GFxValueValueType::kNumber);
        self.value = GFxValueValueUnion { number: value };
    }

    #[inline(always)]
    pub unsafe fn set_string(&mut self, string: *const c_char) {
        self.change_type(GFxValueValueType::kString);
        self.value = GFxValueValueUnion { string };
    }

    #[inline(always)]
    pub unsafe fn set_string_w(&mut self, wide_string: *const u16) {
        self.change_type(GFxValueValueType::kStringW);
        self.value = GFxValueValueUnion { wide_string };
    }

    #[inline(always)]
    pub fn set_convert_boolean(&mut self) {
        self.change_type(GFxValueValueType::kConvertBoolean);
    }

    #[inline(always)]
    pub fn set_convert_number(&mut self) {
        self.change_type(GFxValueValueType::kConvertNumber);
    }

    #[inline(always)]
    pub fn set_convert_string(&mut self) {
        self.change_type(GFxValueValueType::kConvertString);
    }

    #[inline(always)]
    pub fn set_convert_string_w(&mut self) {
        self.change_type(GFxValueValueType::kConvertStringW);
    }

    #[inline(always)]
    pub unsafe fn has_member_raw(&self, name: *const c_char) -> bool {
        assert!(self.is_object());
        self.object_interface_ref().has_member(
            unsafe { self.value.object },
            name,
            self.is_display_object(),
        )
    }

    #[inline(always)]
    pub fn has_member_c_str(&self, name: &CStr) -> bool {
        unsafe { self.has_member_raw(name.as_ptr()) }
    }

    #[inline(always)]
    pub unsafe fn get_member_raw(&self, name: *const c_char, value: *mut GFxValue) -> bool {
        assert!(self.is_object());
        self.object_interface_ref().get_member(
            unsafe { self.value.object },
            name,
            value,
            self.is_display_object(),
        )
    }

    #[inline(always)]
    pub fn get_member_c_str(&self, name: &CStr, value: &mut GFxValue) -> bool {
        unsafe { self.get_member_raw(name.as_ptr(), value) }
    }

    #[inline(always)]
    pub unsafe fn set_member_raw(&self, name: *const c_char, value: &GFxValue) -> bool {
        assert!(self.is_object());
        self.object_interface_ref().set_member(
            unsafe { self.value.object },
            name,
            value,
            self.is_display_object(),
        )
    }

    #[inline(always)]
    pub fn set_member_c_str(&self, name: &CStr, value: &GFxValue) -> bool {
        unsafe { self.set_member_raw(name.as_ptr(), value) }
    }

    #[inline(always)]
    pub unsafe fn invoke_raw(
        &self,
        name: *const c_char,
        result: *mut GFxValue,
        args: *const GFxValue,
        num_args: usize,
    ) -> bool {
        assert!(self.is_object());
        self.object_interface_ref().invoke(
            unsafe { self.value.object },
            result,
            name,
            args,
            num_args,
            self.is_display_object(),
        )
    }

    #[inline(always)]
    pub fn invoke_c_str(
        &self,
        name: &CStr,
        result: Option<&mut GFxValue>,
        args: &[GFxValue],
    ) -> bool {
        unsafe {
            self.invoke_raw(
                name.as_ptr(),
                result.map_or(core::ptr::null_mut(), core::ptr::from_mut),
                args.as_ptr(),
                args.len(),
            )
        }
    }

    #[inline(always)]
    pub fn invoke_no_args_c_str(&self, name: &CStr, result: Option<&mut GFxValue>) -> bool {
        unsafe {
            self.invoke_raw(
                name.as_ptr(),
                result.map_or(core::ptr::null_mut(), core::ptr::from_mut),
                core::ptr::null(),
                0,
            )
        }
    }

    #[inline(always)]
    pub unsafe fn delete_member_raw(&self, name: *const c_char) -> bool {
        assert!(self.is_object());
        self.object_interface_ref().delete_member(
            unsafe { self.value.object },
            name,
            self.is_display_object(),
        )
    }

    #[inline(always)]
    pub fn delete_member_c_str(&self, name: &CStr) -> bool {
        unsafe { self.delete_member_raw(name.as_ptr()) }
    }

    #[inline(always)]
    pub fn visit_members(&self, visitor: *mut GFxValueObjectVisitor) {
        assert!(self.is_object());
        self.object_interface_ref().visit_members(
            unsafe { self.value.object },
            visitor,
            self.is_display_object(),
        );
    }

    // TODO: CommonLib also exposes `VisitMembers(ObjectVisitFn&&)` by building a
    // stack-local `ObjVisitor` wrapper in C++. Keep the raw `ObjVisitor*`
    // entrypoint until the shared ABI-safe bridge layer can synthesize that
    // transient visitor from a Rust closure without inventing a Rust-only vtable
    // contract for `GFxValue::ObjectInterface::ObjVisitor`.

    #[inline(always)]
    pub fn get_array_size(&self) -> u32 {
        assert!(self.is_array());
        self.object_interface_ref()
            .get_array_size(unsafe { self.value.object })
    }

    #[inline(always)]
    pub fn set_array_size(&self, size: u32) -> bool {
        assert!(self.is_array());
        self.object_interface_ref()
            .set_array_size(unsafe { self.value.object }, size)
    }

    #[inline(always)]
    pub fn get_element(&self, index: u32, value: &mut GFxValue) -> bool {
        assert!(self.is_array());
        self.object_interface_ref()
            .get_element(unsafe { self.value.object }, index, value)
    }

    #[inline(always)]
    pub fn set_element(&self, index: u32, value: &GFxValue) -> bool {
        assert!(self.is_array());
        self.object_interface_ref()
            .set_element(unsafe { self.value.object }, index, value)
    }

    #[inline(always)]
    pub fn push_back(&self, value: &GFxValue) -> bool {
        assert!(self.is_array());
        self.object_interface_ref()
            .push_back(unsafe { self.value.object }, value)
    }

    #[inline(always)]
    pub fn remove_elements(&self, index: u32, count: i32) -> bool {
        assert!(self.is_array());
        self.object_interface_ref()
            .remove_elements(unsafe { self.value.object }, index, count)
    }

    #[inline(always)]
    pub fn remove_element(&self, index: u32) -> bool {
        self.remove_elements(index, 1)
    }

    #[inline(always)]
    pub fn clear_elements(&self) -> bool {
        self.remove_elements(0, -1)
    }

    #[inline(always)]
    pub fn get_display_info(&self, info: &mut GFxValueDisplayInfo) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref()
            .get_display_info(unsafe { self.value.object }, info)
    }

    #[inline(always)]
    pub fn set_display_info(&self, info: &GFxValueDisplayInfo) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref()
            .set_display_info(unsafe { self.value.object }, info)
    }

    #[inline(always)]
    pub fn get_display_matrix(&self, matrix: &mut GMatrix2D) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref()
            .get_display_matrix(unsafe { self.value.object }, matrix)
    }

    #[inline(always)]
    pub fn set_display_matrix(&self, matrix: &GMatrix2D) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref()
            .set_display_matrix(unsafe { self.value.object }, matrix)
    }

    #[inline(always)]
    pub fn get_cxform(&self, cxform: &mut GRendererCxform) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref()
            .get_cxform(unsafe { self.value.object }, cxform)
    }

    #[inline(always)]
    pub fn set_cxform(&self, cxform: &GRendererCxform) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref()
            .set_cxform(unsafe { self.value.object }, cxform)
    }

    #[inline(always)]
    pub fn set_text_c_str(&self, text: &CStr) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref()
            .set_text(unsafe { self.value.object }, text.as_ptr(), false)
    }

    #[inline(always)]
    pub fn set_text_html_c_str(&self, html: &CStr) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref()
            .set_text(unsafe { self.value.object }, html.as_ptr(), true)
    }

    #[inline(always)]
    pub fn attach_movie_c_str(
        &self,
        movie_clip: &mut GFxValue,
        symbol_name: &CStr,
        instance_name: &CStr,
        depth: i32,
        init_obj: Option<&GFxValue>,
    ) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref().attach_movie(
            unsafe { self.value.object },
            movie_clip,
            symbol_name.as_ptr(),
            instance_name.as_ptr(),
            depth,
            init_obj.map_or(core::ptr::null(), core::ptr::from_ref),
        )
    }

    #[inline(always)]
    pub fn create_empty_movie_clip_c_str(
        &self,
        movie_clip: &mut GFxValue,
        instance_name: &CStr,
        depth: i32,
    ) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref().create_empty_movie_clip(
            unsafe { self.value.object },
            movie_clip,
            instance_name.as_ptr(),
            depth,
        )
    }

    #[inline(always)]
    pub fn goto_and_play_c_str(&self, frame: &CStr) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref().goto_and_play(
            unsafe { self.value.object },
            frame.as_ptr(),
            false,
        )
    }

    #[inline(always)]
    pub fn goto_and_stop_c_str(&self, frame: &CStr) -> bool {
        assert!(self.is_display_object());
        self.object_interface_ref().goto_and_play(
            unsafe { self.value.object },
            frame.as_ptr(),
            true,
        )
    }

    #[inline(always)]
    pub fn set_color_tint(&self, tint: &GColor) -> bool {
        assert!(self.is_display_object());

        let mut color_transform = GRendererCxform::default();
        if !self.get_cxform(&mut color_transform) {
            return false;
        }

        let normalized_red = tint.red() as f32 / 255.0;
        let normalized_green = tint.green() as f32 / 255.0;
        let normalized_blue = tint.blue() as f32 / 255.0;
        let intensity = tint.alpha() as f32 / 255.0;

        color_transform.matrix[GRendererCxform::R][GRendererCxform::MULT] =
            1.0 - (intensity * (1.0 - normalized_red));
        color_transform.matrix[GRendererCxform::G][GRendererCxform::MULT] =
            1.0 - (intensity * (1.0 - normalized_green));
        color_transform.matrix[GRendererCxform::B][GRendererCxform::MULT] =
            1.0 - (intensity * (1.0 - normalized_blue));
        color_transform.matrix[GRendererCxform::A][GRendererCxform::MULT] = 1.0;

        color_transform.matrix[GRendererCxform::R][GRendererCxform::ADD] =
            intensity * normalized_red * 0.5;
        color_transform.matrix[GRendererCxform::G][GRendererCxform::ADD] =
            intensity * normalized_green * 0.5;
        color_transform.matrix[GRendererCxform::B][GRendererCxform::ADD] =
            intensity * normalized_blue * 0.5;
        color_transform.matrix[GRendererCxform::A][GRendererCxform::ADD] = 0.0;

        self.set_cxform(&color_transform)
    }

    #[inline(always)]
    pub fn remove_color_tint(&self) -> bool {
        assert!(self.is_display_object());

        let mut color_transform = GRendererCxform::default();
        if !self.get_cxform(&mut color_transform) {
            return false;
        }
        color_transform.set_identity();

        self.set_cxform(&color_transform)
    }

    #[inline(always)]
    pub fn is_managed_value(&self) -> bool {
        self.type_.all(GFxValueValueType::kManagedBit)
    }

    #[inline(always)]
    pub fn assign(&mut self, rhs: &Self) {
        if core::ptr::eq(self, rhs) {
            return;
        }

        if self.is_managed_value() {
            self.release_managed_value();
        }

        self.type_ = rhs.type_;
        self.value = rhs.value;

        if rhs.is_managed_value() {
            self.acquire_managed_value(rhs);
        }
    }

    #[inline(always)]
    fn acquire_managed_value(&mut self, rhs: &Self) {
        let object = unsafe { self.value.object };
        assert!(!object.is_null());
        assert!(!rhs.object_interface.is_null());

        self.object_interface = rhs.object_interface;
        unsafe {
            (*self.object_interface).object_add_ref(self, object);
        }
    }

    #[inline(always)]
    fn release_managed_value(&mut self) {
        let object = unsafe { self.value.object };
        assert!(!object.is_null());
        assert!(!self.object_interface.is_null());

        unsafe {
            (*self.object_interface).object_release(self, object);
        }
        self.object_interface = core::ptr::null_mut();
    }

    #[inline(always)]
    fn change_type(&mut self, value_type: GFxValueValueType) {
        if self.is_managed_value() {
            self.release_managed_value();
        }
        self.type_ = EnumSet::from_underlying(value_type as u32);
    }

    #[inline(always)]
    fn object_interface_ref(&self) -> &GFxValueObjectInterface {
        let object_interface = self.object_interface;
        assert!(!object_interface.is_null());
        unsafe { &*object_interface }
    }

    #[inline(always)]
    fn round_to_isize(value: f64) -> isize {
        unsafe { round(value) as isize }
    }
}
