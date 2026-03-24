use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSSoundDescriptor;
use crate::offsets::offsets_vtable::VTABLE_BGSSoundDescriptor;
use crate::re::bgs_sound_category::BGSSoundCategory;
use crate::re::bsi_sound_descriptor::BSISoundDescriptor;
use crate::re::tes_file::TESFile;
use crate::re::tes_form::{FormID, TESForm};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BGSSoundDescriptor`
#[repr(C)]
pub struct BGSSoundDescriptor {
    pub base: BSISoundDescriptor,        // 00
    pub category: *mut BGSSoundCategory, // 08
    pub alternate_sound_form_id: FormID, // 10
    pub pad14: u32,                      // 14
}

const _: () = assert!(core::mem::size_of::<BGSSoundDescriptor>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSSoundDescriptor, category) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSSoundDescriptor, alternate_sound_form_id) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSSoundDescriptor, pad14) == 0x14);

impl RttiType for BGSSoundDescriptor {
    const RTTI: VariantID = RTTI_BGSSoundDescriptor;
}

inherit!(BGSSoundDescriptor : BSISoundDescriptor);

impl BGSSoundDescriptor {
    pub const RTTI: VariantID = RTTI_BGSSoundDescriptor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSSoundDescriptor;

    // override (BSISoundDescriptor)
    // bool DoResolve(Resolution& a_resolution) override = 0;  // 01
    // void Unk_02(void) override = 0;                         // 02

    virtual_method! {
        pub const VFUNC_INIT_SOUND: usize = 0x03;
        pub fn init_sound(src: *mut TESForm)
    }

    virtual_method! {
        pub const VFUNC_LOAD_SOUND: usize = 0x04;
        pub fn load_sound(mod_file: *mut TESFile) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x05;
        pub fn get_type() -> u32
    }

    virtual_method! {
        pub const VFUNC_GET_MAX_AUDIBLE_DISTANCE: usize = 0x06;
        pub fn get_max_audible_distance() -> f32
    }

    #[inline(always)]
    pub const fn get_category(&self) -> *mut BGSSoundCategory {
        self.category
    }

    #[inline(always)]
    pub const fn get_alternate_sound_form_id(&self) -> FormID {
        self.alternate_sound_form_id
    }
}

pub trait BGSSoundDescriptorExt {
    fn init_sound(&mut self, src: *mut TESForm);
    fn load_sound(&mut self, mod_file: *mut TESFile) -> bool;
    fn get_type(&self) -> u32;
    fn get_max_audible_distance(&self) -> f32;
    fn get_category(&self) -> *mut BGSSoundCategory;
    fn get_alternate_sound_form_id(&self) -> FormID;
}

impl<T: AsRef<BGSSoundDescriptor> + AsMut<BGSSoundDescriptor>> BGSSoundDescriptorExt for T {
    fn init_sound(&mut self, src: *mut TESForm) {
        self.as_mut().init_sound(src)
    }

    fn load_sound(&mut self, mod_file: *mut TESFile) -> bool {
        self.as_mut().load_sound(mod_file)
    }

    fn get_type(&self) -> u32 {
        self.as_ref().get_type()
    }

    fn get_max_audible_distance(&self) -> f32 {
        self.as_ref().get_max_audible_distance()
    }

    fn get_category(&self) -> *mut BGSSoundCategory {
        self.as_ref().get_category()
    }

    fn get_alternate_sound_form_id(&self) -> FormID {
        self.as_ref().get_alternate_sound_form_id()
    }
}
