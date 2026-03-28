use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESObjectMISC;
use crate::offsets::offsets_vtable::VTABLE_TESObjectMISC;
use crate::re::bgs_destructible_object_form::BGSDestructibleObjectForm;
use crate::re::bgs_keyword::BGSKeyword;
use crate::re::bgs_keyword_form::BGSKeywordForm;
use crate::re::bgs_load_form_buffer::BGSLoadFormBuffer;
use crate::re::bgs_message_icon::BGSMessageIcon;
use crate::re::bgs_pickup_putdown_sounds::BGSPickupPutdownSounds;
use crate::re::bgs_save_form_buffer::BGSSaveFormBuffer;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_file::TESFile;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_icon::TESIcon;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::re::tes_value_form::TESValueForm;
use crate::re::tes_weight_form::TESWeightForm;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESObjectMISCRecordFlags: u32 {
        const NON_PLAYABLE = 1 << 2;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::TESObjectMISC`
#[repr(C)]
pub struct TESObjectMISC {
    pub base: TESBoundObject,                                // 000
    pub full_name: TESFullName,                              // 030
    pub model_texture_swap: TESModelTextureSwap,             // 040
    pub icon: TESIcon,                                       // 078
    pub value_form: TESValueForm,                            // 088
    pub weight_form: TESWeightForm,                          // 098
    pub destructible_object_form: BGSDestructibleObjectForm, // 0A8
    pub message_icon: BGSMessageIcon,                        // 0B8
    pub pickup_putdown_sounds: BGSPickupPutdownSounds,       // 0D0
    pub keyword_form: BGSKeywordForm,                        // 0E8
}

const _: () = assert!(core::mem::size_of::<TESObjectMISC>() == 0x100);
const _: () = assert!(core::mem::offset_of!(TESObjectMISC, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESObjectMISC, full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESObjectMISC, model_texture_swap) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESObjectMISC, icon) == 0x78);
const _: () = assert!(core::mem::offset_of!(TESObjectMISC, value_form) == 0x88);
const _: () = assert!(core::mem::offset_of!(TESObjectMISC, weight_form) == 0x98);
const _: () = assert!(core::mem::offset_of!(TESObjectMISC, destructible_object_form) == 0xA8);
const _: () = assert!(core::mem::offset_of!(TESObjectMISC, message_icon) == 0xB8);
const _: () = assert!(core::mem::offset_of!(TESObjectMISC, pickup_putdown_sounds) == 0xD0);
const _: () = assert!(core::mem::offset_of!(TESObjectMISC, keyword_form) == 0xE8);

impl RttiType for TESObjectMISC {
    const RTTI: VariantID = RTTI_TESObjectMISC;
}

impl FormCastable for TESObjectMISC {
    const TARGET_FORM_TYPE: FormType = FormType::Misc;
}

inherit!(TESObjectMISC : TESBoundObject);
inherit!(TESObjectMISC => TESFullName, full_name);
inherit!(TESObjectMISC => TESModelTextureSwap, model_texture_swap);
inherit!(TESObjectMISC => TESIcon, icon);
inherit!(TESObjectMISC => TESValueForm, value_form);
inherit!(TESObjectMISC => TESWeightForm, weight_form);
inherit!(TESObjectMISC => BGSDestructibleObjectForm, destructible_object_form);
inherit!(TESObjectMISC => BGSMessageIcon, message_icon);
inherit!(TESObjectMISC => BGSPickupPutdownSounds, pickup_putdown_sounds);
inherit!(TESObjectMISC => BGSKeywordForm, keyword_form);

impl TESObjectMISC {
    pub const RTTI: VariantID = RTTI_TESObjectMISC;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectMISC;
    pub const FORMTYPE: FormType = FormType::Misc;

    // override (TESBoundObject)
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_: *mut TESFile) -> bool
    }

    virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x0E;
        pub fn save_game(&mut self, buf: *mut BGSSaveFormBuffer)
    }

    virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x0F;
        pub fn load_game(&mut self, buf: *mut BGSLoadFormBuffer)
    }

    virtual_method! {
        pub const VFUNC_INIT_ITEM_IMPL: usize = 0x13;
        pub fn init_item_impl(&mut self)
    }

    // override (BGSKeywordForm)
    #[inline(always)]
    pub fn get_default_keyword(&self) -> *mut BGSKeyword {
        self.keyword_form.get_default_keyword()
    }

    // add
    virtual_method! {
        pub const SAVE_IMPL: usize = 0x53;
        pub fn save_impl(&mut self)
    }

    virtual_method! {
        pub const LOAD_IMPL: usize = 0x54;
        pub fn load_impl(&mut self, a_mod: *mut crate::re::tes_file::TESFile, a_chunk_id: u32)
    }

    virtual_method! {
        pub const INIT_IMPL: usize = 0x55;
        pub fn init_impl(&mut self)
    }
}

impl AsRef<TESObjectMISC> for TESObjectMISC {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TESObjectMISC> for TESObjectMISC {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub trait TESObjectMISCExt {
    fn dtor(&mut self);
    fn load(&mut self, mod_: *mut TESFile) -> bool;
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer);
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn init_item_impl(&mut self);
    fn get_default_keyword(&self) -> *mut BGSKeyword;
    fn save_impl(&mut self);
    fn load_impl(&mut self, a_mod: *mut TESFile, a_chunk_id: u32);
    fn init_impl(&mut self);
}

impl<T: AsRef<TESObjectMISC> + AsMut<TESObjectMISC>> TESObjectMISCExt for T {
    #[inline(always)]
    fn dtor(&mut self) {
        TESObjectMISC::dtor(self.as_mut())
    }

    #[inline(always)]
    fn load(&mut self, mod_: *mut TESFile) -> bool {
        TESObjectMISC::load(self.as_mut(), mod_)
    }

    #[inline(always)]
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer) {
        TESObjectMISC::save_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        TESObjectMISC::load_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn init_item_impl(&mut self) {
        TESObjectMISC::init_item_impl(self.as_mut())
    }

    #[inline(always)]
    fn get_default_keyword(&self) -> *mut BGSKeyword {
        TESObjectMISC::get_default_keyword(self.as_ref())
    }

    #[inline(always)]
    fn save_impl(&mut self) {
        TESObjectMISC::save_impl(self.as_mut())
    }

    #[inline(always)]
    fn load_impl(&mut self, a_mod: *mut TESFile, a_chunk_id: u32) {
        TESObjectMISC::load_impl(self.as_mut(), a_mod, a_chunk_id)
    }

    #[inline(always)]
    fn init_impl(&mut self) {
        TESObjectMISC::init_impl(self.as_mut())
    }
}
