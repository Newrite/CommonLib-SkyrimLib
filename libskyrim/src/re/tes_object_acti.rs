use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESObjectACTI;
use crate::offsets::offsets_vtable::VTABLE_TESObjectACTI;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::{
    Actor, BGSDestructibleObjectForm, BGSKeywordForm, BGSLoadFormBuffer, BGSOpenCloseForm,
    BGSSaveFormBuffer, BGSSoundDescriptorForm, BSString, TESBoundAnimObject, TESBoundObject,
    TESFile, TESFullName, TESMagicTargetForm, TESModelTextureSwap, TESObjectREFR, TESWaterForm,
};
use crate::relocation::{RttiType, VariantID};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESObjectACTIFlags: u16 {
        const NONE = 0;
        const NO_DISPLACEMENT = 1 << 0;
        const IGNORED_BY_SANDBOX = 1 << 1;
        const IS_PROCEDURAL_WATER = 1 << 2;
        const IS_LOD_WATER = 1 << 3;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESObjectACTIRecordFlags: u32 {
        const DELETED = 1 << 5;
        const HAS_TREE_LOD = 1 << 6;
        const MUST_UPDATE_ANIMS = 1 << 8;
        const HIDDEN_FROM_LOCAL_MAP = 1 << 9;
        const IGNORED = 1 << 12;
        const HAS_DISTANT_LOD = 1 << 15;
        const RANDOM_ANIM_START = 1 << 16;
        const DANGEROUS = 1 << 17;
        const IGNORES_OBJECT_INTERACTION = 1 << 20;
        const IS_MARKER = 1 << 23;
        const OBSTACLE = 1 << 25;
        const NAV_MESH_GENERATION_FILTER = 1 << 26;
        const NAV_MESH_GENERATION_BOUNDING_BOX = 1 << 27;
        const CHILD_CAN_USE = 1 << 29;
        const NAV_MESH_GENERATION_GROUND = 1 << 30;
    }
}

#[repr(C)]
pub struct TESObjectACTI {
    pub base: TESBoundAnimObject,                            // 00
    pub full_name: TESFullName,                              // 30
    pub model_texture_swap: TESModelTextureSwap,             // 40
    pub destructible_object_form: BGSDestructibleObjectForm, // 78
    pub open_close_form: BGSOpenCloseForm,                   // 88
    pub keyword_form: BGSKeywordForm,                        // 90
    // `TESMagicTargetForm` is an RTTI-only marker mixin at 0xA8 under the
    // C++ ABI and does not consume standalone storage in this layout.
    pub sound_loop: *mut BGSSoundDescriptorForm,     // A8
    pub sound_activate: *mut BGSSoundDescriptorForm, // B0
    pub water_form: *mut TESWaterForm,               // B8
    pub flags: TESObjectACTIFlags,                   // C0
    pub padc2: u16,                                  // C2
    pub padc4: u32,                                  // C4
}

const _: () = assert!(core::mem::size_of::<TESObjectACTI>() == 0xC8);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, model_texture_swap) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, destructible_object_form) == 0x78);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, open_close_form) == 0x88);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, keyword_form) == 0x90);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, sound_loop) == 0xA8);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, sound_activate) == 0xB0);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, water_form) == 0xB8);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, flags) == 0xC0);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, padc2) == 0xC2);
const _: () = assert!(core::mem::offset_of!(TESObjectACTI, padc4) == 0xC4);

impl RttiType for TESObjectACTI {
    const RTTI: VariantID = RTTI_TESObjectACTI;
}

impl FormCastable for TESObjectACTI {
    const TARGET_FORM_TYPE: FormType = FormType::Activator;
}

inherit!(TESObjectACTI : TESBoundAnimObject);
inherit!(TESObjectACTI => TESFullName, full_name);
inherit!(TESObjectACTI => TESModelTextureSwap, model_texture_swap);
inherit!(TESObjectACTI => BGSDestructibleObjectForm, destructible_object_form);
inherit!(TESObjectACTI => BGSOpenCloseForm, open_close_form);
inherit!(TESObjectACTI => BGSKeywordForm, keyword_form);

impl AsRef<TESMagicTargetForm> for TESObjectACTI {
    fn as_ref(&self) -> &TESMagicTargetForm {
        // SAFETY: `TESMagicTargetForm` is a 1-byte RTTI-only marker base whose
        // ABI position coincides with the start of the trailing storage at 0xA8.
        unsafe { &*(core::ptr::from_ref(self).cast::<u8>().add(0xA8) as *const TESMagicTargetForm) }
    }
}

impl AsMut<TESMagicTargetForm> for TESObjectACTI {
    fn as_mut(&mut self) -> &mut TESMagicTargetForm {
        // SAFETY: same ABI reasoning as `AsRef`.
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<u8>().add(0xA8) as *mut TESMagicTargetForm)
        }
    }
}

impl TESObjectACTI {
    pub const RTTI: VariantID = RTTI_TESObjectACTI;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectACTI;
    pub const FORMTYPE: FormType = FormType::Activator;

    // override (TESBoundAnimObject)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_INITIALIZE_DATA: usize = 0x04;
        pub fn initialize_data(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_CLEAR_DATA: usize = 0x05;
        pub fn clear_data(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_: *mut TESFile) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x0E;
        pub fn save_game(&mut self, buf: *mut BGSSaveFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x0F;
        pub fn load_game(&mut self, buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_ITEM_IMPL: usize = 0x13;
        pub fn init_item_impl(&mut self)
    }

    #[inline(always)]
    pub const fn get_sound_loop(&self) -> *mut BGSSoundDescriptorForm {
        self.sound_loop
    }

    #[inline(always)]
    pub const fn get_sound_activate(&self) -> *mut BGSSoundDescriptorForm {
        self.sound_activate
    }

    #[inline(always)]
    pub const fn get_water_form(&self) -> *mut TESWaterForm {
        self.water_form
    }

    crate::virtual_method! {
        pub const VFUNC_GET_IGNORED_BY_SANDBOX: usize = 0x22;
        pub fn get_ignored_by_sandbox(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_IS_WATER: usize = 0x2A;
        pub fn is_water(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_ACTIVATE: usize = 0x37;
        pub fn activate(target_ref: *mut TESObjectREFR, activator_ref: *mut TESObjectREFR, arg3: u8, object: *mut TESBoundObject, target_count: i32) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_WATER_TYPE: usize = 0x3D;
        pub fn get_water_type(&self) -> *mut TESWaterForm
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ACTIVATE_TEXT: usize = 0x4C;
        pub fn get_activate_text(activator: *mut TESObjectREFR, dst: *mut BSString) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_CALCULATE_DO_FAVOR: usize = 0x4D;
        pub fn calculate_do_favor(activator: *mut Actor, arg2: bool, to_activate: *mut TESObjectREFR, arg3: f32) -> bool
    }
}

pub trait TESObjectACTIExt {
    fn dtor(&mut self);
    fn initialize_data(&mut self);
    fn clear_data(&mut self);
    fn load(&mut self, mod_: *mut TESFile) -> bool;
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer);
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn init_item_impl(&mut self);
    fn get_sound_loop(&self) -> *mut BGSSoundDescriptorForm;
    fn get_sound_activate(&self) -> *mut BGSSoundDescriptorForm;
    fn get_water_form(&self) -> *mut TESWaterForm;
    fn get_water_type(&self) -> *mut TESWaterForm;
    fn get_ignored_by_sandbox(&self) -> bool;
    fn is_water(&self) -> bool;
    fn activate(
        &mut self,
        target_ref: *mut TESObjectREFR,
        activator_ref: *mut TESObjectREFR,
        arg3: u8,
        object: *mut TESBoundObject,
        target_count: i32,
    ) -> bool;
    fn get_activate_text(&self, activator: *mut TESObjectREFR, dst: *mut BSString) -> bool;
    fn calculate_do_favor(
        &self,
        activator: *mut Actor,
        arg2: bool,
        to_activate: *mut TESObjectREFR,
        arg3: f32,
    ) -> bool;
}

impl<T: AsRef<TESObjectACTI> + AsMut<TESObjectACTI>> TESObjectACTIExt for T {
    fn dtor(&mut self) {
        TESObjectACTI::dtor(self.as_mut())
    }

    fn initialize_data(&mut self) {
        TESObjectACTI::initialize_data(self.as_mut())
    }

    fn clear_data(&mut self) {
        TESObjectACTI::clear_data(self.as_mut())
    }

    fn load(&mut self, mod_: *mut TESFile) -> bool {
        TESObjectACTI::load(self.as_mut(), mod_)
    }

    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer) {
        TESObjectACTI::save_game(self.as_mut(), buf)
    }

    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        TESObjectACTI::load_game(self.as_mut(), buf)
    }

    fn init_item_impl(&mut self) {
        TESObjectACTI::init_item_impl(self.as_mut())
    }

    fn get_sound_loop(&self) -> *mut BGSSoundDescriptorForm {
        TESObjectACTI::get_sound_loop(self.as_ref())
    }

    fn get_sound_activate(&self) -> *mut BGSSoundDescriptorForm {
        TESObjectACTI::get_sound_activate(self.as_ref())
    }

    fn get_water_form(&self) -> *mut TESWaterForm {
        TESObjectACTI::get_water_form(self.as_ref())
    }

    fn get_water_type(&self) -> *mut TESWaterForm {
        TESObjectACTI::get_water_type(self.as_ref())
    }

    fn get_ignored_by_sandbox(&self) -> bool {
        TESObjectACTI::get_ignored_by_sandbox(self.as_ref())
    }

    fn is_water(&self) -> bool {
        TESObjectACTI::is_water(self.as_ref())
    }

    fn activate(
        &mut self,
        target_ref: *mut TESObjectREFR,
        activator_ref: *mut TESObjectREFR,
        arg3: u8,
        object: *mut TESBoundObject,
        target_count: i32,
    ) -> bool {
        TESObjectACTI::activate(
            self.as_mut(),
            target_ref,
            activator_ref,
            arg3,
            object,
            target_count,
        )
    }

    fn get_activate_text(&self, activator: *mut TESObjectREFR, dst: *mut BSString) -> bool {
        TESObjectACTI::get_activate_text(self.as_ref(), activator, dst)
    }

    fn calculate_do_favor(
        &self,
        activator: *mut Actor,
        arg2: bool,
        to_activate: *mut TESObjectREFR,
        arg3: f32,
    ) -> bool {
        TESObjectACTI::calculate_do_favor(self.as_ref(), activator, arg2, to_activate, arg3)
    }
}
