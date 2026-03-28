use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESObjectBOOK;
use crate::offsets::offsets_vtable::VTABLE_TESObjectBOOK;
use crate::re::actor_values::ActorValue;
use crate::re::bgs_destructible_object_form::BGSDestructibleObjectForm;
use crate::re::bgs_keyword::BGSKeyword;
use crate::re::bgs_keyword_form::BGSKeywordForm;
use crate::re::bgs_load_form_buffer::BGSLoadFormBuffer;
use crate::re::bgs_message_icon::BGSMessageIcon;
use crate::re::bgs_pickup_putdown_sounds::BGSPickupPutdownSounds;
use crate::re::bgs_save_form_buffer::BGSSaveFormBuffer;
use crate::re::bs_string::BSString;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::spell_item::SpellItem;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_description::TESDescription;
use crate::re::tes_file::TESFile;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_icon::TESIcon;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::re::tes_object_stat::TESObjectSTAT;
use crate::re::tes_value_form::TESValueForm;
use crate::re::tes_weight_form::TESWeightForm;
use crate::relocation::{RelocationID, RttiType, VariantID};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESObjectBOOKFlag {
    None = 0,
    AdvancesActorValue = 1 << 0,
    CantTake = 1 << 1,
    TeachesSpell = 1 << 2,
    HasBeenRead = 1 << 3,
}

core_util::impl_enumset_type!(TESObjectBOOKFlag => u8);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESObjectBOOKType {
    BookTome = 0x00,
    NoteScroll = 0xFF,
}

core_util::impl_enumset_type!(TESObjectBOOKType => u8);

#[repr(C)]
pub union TESObjectBOOKTeaches {
    pub actor_value_to_advance: ActorValue,
    pub spell: *mut SpellItem,
}

impl Copy for TESObjectBOOKTeaches {}
impl Clone for TESObjectBOOKTeaches {
    fn clone(&self) -> Self {
        *self
    }
}

const _: () = assert!(core::mem::size_of::<TESObjectBOOKTeaches>() == 0x8);

#[repr(C)]
pub struct TESObjectBOOKData {
    pub flags: EnumSet<TESObjectBOOKFlag, u8>, // 00
    pub type_: EnumSet<TESObjectBOOKType, u8>, // 01
    pub pad02: u16,                            // 02
    pub pad04: u32,                            // 04
    pub teaches: TESObjectBOOKTeaches,         // 08
}

const _: () = assert!(core::mem::size_of::<TESObjectBOOKData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOKData, flags) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOKData, type_) == 0x01);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOKData, teaches) == 0x08);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESObjectBOOKChangeFlags: u32 {
        const TEACHES_SKILL = 1 << 5;
        const READ = 1 << 6;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESObjectBOOKRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct TESObjectBOOK {
    pub base: TESBoundObject,                                // 000
    pub full_name: TESFullName,                              // 030
    pub model_texture_swap: TESModelTextureSwap,             // 040
    pub icon: TESIcon,                                       // 078
    pub value_form: TESValueForm,                            // 088
    pub weight_form: TESWeightForm,                          // 098
    pub description: TESDescription,                         // 0A8
    pub destructible_object_form: BGSDestructibleObjectForm, // 0B8
    pub message_icon: BGSMessageIcon,                        // 0C8
    pub pickup_putdown_sounds: BGSPickupPutdownSounds,       // 0E0
    pub keyword_form: BGSKeywordForm,                        // 0F8
    pub data: TESObjectBOOKData,                             // 110
    pub inventory_model: *mut TESObjectSTAT,                 // 120
    pub item_card_description: TESDescription,               // 128
}

const _: () = assert!(core::mem::size_of::<TESObjectBOOK>() == 0x138);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, model_texture_swap) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, icon) == 0x78);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, value_form) == 0x88);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, weight_form) == 0x98);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, description) == 0xA8);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, destructible_object_form) == 0xB8);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, message_icon) == 0xC8);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, pickup_putdown_sounds) == 0xE0);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, keyword_form) == 0xF8);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, data) == 0x110);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, inventory_model) == 0x120);
const _: () = assert!(core::mem::offset_of!(TESObjectBOOK, item_card_description) == 0x128);

impl RttiType for TESObjectBOOK {
    const RTTI: VariantID = RTTI_TESObjectBOOK;
}

impl FormCastable for TESObjectBOOK {
    const TARGET_FORM_TYPE: FormType = FormType::Book;
}

inherit!(TESObjectBOOK : TESBoundObject);
inherit!(TESObjectBOOK => TESFullName, full_name);
inherit!(TESObjectBOOK => TESModelTextureSwap, model_texture_swap);
inherit!(TESObjectBOOK => TESIcon, icon);
inherit!(TESObjectBOOK => TESValueForm, value_form);
inherit!(TESObjectBOOK => TESWeightForm, weight_form);
inherit!(TESObjectBOOK => TESDescription, description);
inherit!(TESObjectBOOK => BGSDestructibleObjectForm, destructible_object_form);
inherit!(TESObjectBOOK => BGSMessageIcon, message_icon);
inherit!(TESObjectBOOK => BGSPickupPutdownSounds, pickup_putdown_sounds);
inherit!(TESObjectBOOK => BGSKeywordForm, keyword_form);

impl TESObjectBOOKData {
    #[inline(always)]
    pub fn get_sanitized_type(&self) -> TESObjectBOOKFlag {
        if self.flags.all(TESObjectBOOKFlag::TeachesSpell) {
            TESObjectBOOKFlag::TeachesSpell
        } else if self.flags.all(TESObjectBOOKFlag::AdvancesActorValue) {
            TESObjectBOOKFlag::AdvancesActorValue
        } else {
            TESObjectBOOKFlag::None
        }
    }
}

impl TESObjectBOOK {
    pub const RTTI: VariantID = RTTI_TESObjectBOOK;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectBOOK;
    pub const FORMTYPE: FormType = FormType::Book;

    // override (TESBoundObject)
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
    pub fn teaches_skill(&self) -> bool {
        self.data.flags.all(TESObjectBOOKFlag::AdvancesActorValue)
    }

    #[inline(always)]
    pub fn teaches_spell(&self) -> bool {
        self.data.flags.all(TESObjectBOOKFlag::TeachesSpell)
    }

    #[inline(always)]
    pub fn is_read(&self) -> bool {
        self.data.flags.all(TESObjectBOOKFlag::HasBeenRead)
    }

    #[inline(always)]
    pub fn can_be_taken(&self) -> bool {
        self.data.flags.none(TESObjectBOOKFlag::CantTake)
    }

    #[inline(always)]
    pub fn is_book_tome(&self) -> bool {
        self.data.type_ == EnumSet::from_underlying(TESObjectBOOKType::BookTome as u8)
    }

    #[inline(always)]
    pub fn is_note_scroll(&self) -> bool {
        self.data.type_ == EnumSet::from_underlying(TESObjectBOOKType::NoteScroll as u8)
    }

    #[inline(always)]
    pub fn get_skill(&self) -> ActorValue {
        if self.teaches_skill() {
            unsafe { self.data.teaches.actor_value_to_advance }
        } else {
            ActorValue::None
        }
    }

    #[inline(always)]
    pub fn get_spell(&self) -> *mut SpellItem {
        if self.teaches_spell() {
            unsafe { self.data.teaches.spell }
        } else {
            core::ptr::null_mut()
        }
    }

    crate::relocation_func! {
        pub fn read(this: &mut TESObjectBOOK, reader: *mut TESObjectREFR) -> bool => RelocationID::new(17439, 17842)
    }

    crate::virtual_method! {
        pub const VFUNC_ACTIVATE: usize = 0x37;
        pub fn activate(&mut self, target_ref: *mut TESObjectREFR, activator_ref: *mut TESObjectREFR, arg3: u8, object: *mut TESBoundObject, target_count: i32) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ACTIVATE_TEXT: usize = 0x4C;
        pub fn get_activate_text(&mut self, activator: *mut TESObjectREFR, dst: *mut BSString) -> bool
    }

    // override (BGSKeywordForm)
    #[inline(always)]
    pub fn get_default_keyword(&self) -> *mut BGSKeyword {
        self.keyword_form.get_default_keyword()
    }
}
