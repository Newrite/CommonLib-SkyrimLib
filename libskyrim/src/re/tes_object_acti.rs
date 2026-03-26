use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESObjectACTI;
use crate::offsets::offsets_vtable::VTABLE_TESObjectACTI;
use crate::re::bgs_destructible_object_form::BGSDestructibleObjectForm;
use crate::re::bgs_keyword_form::BGSKeywordForm;
use crate::re::bgs_open_close_form::BGSOpenCloseForm;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_bound_anim_object::TESBoundAnimObject;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_magic_target_form::TESMagicTargetForm;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::re::tes_water_form::TESWaterForm;
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
        pub fn dtor()
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

    #[inline(always)]
    pub const fn get_water_type(&self) -> *mut TESWaterForm {
        self.water_form
    }

    #[inline(always)]
    pub fn get_ignored_by_sandbox(&self) -> bool {
        self.flags.contains(TESObjectACTIFlags::IGNORED_BY_SANDBOX)
    }

    #[inline(always)]
    pub fn is_water(&self) -> bool {
        !self.water_form.is_null()
    }

    // void          InitializeData() override;                                                                // 04
    // void          ClearData() override;                                                                     // 05
    // bool          Load(TESFile* a_mod) override;                                                            // 06
    // void          SaveGame(BGSSaveFormBuffer* a_buf) override;                                              // 0E
    // void          LoadGame(BGSLoadFormBuffer* a_buf) override;                                              // 0F
    // void          InitItemImpl() override;                                                                  // 13
    // bool          GetIgnoredBySandbox() const override;                                                     // 22
    // bool          IsWater() const override;                                                                 // 2A
    // bool          Activate(...) override;                                                                   // 37
    // TESWaterForm* GetWaterType() const override;                                                            // 3D
    // bool          GetActivateText(TESObjectREFR* a_activator, BSString& a_dst) override;                   // 4C
    // bool          CalculateDoFavor(Actor* a_activator, bool a_arg2, TESObjectREFR* a_toActivate, float) override;  // 4D
}

pub trait TESObjectACTIExt {
    fn dtor(&mut self);
    fn get_sound_loop(&self) -> *mut BGSSoundDescriptorForm;
    fn get_sound_activate(&self) -> *mut BGSSoundDescriptorForm;
    fn get_water_form(&self) -> *mut TESWaterForm;
    fn get_water_type(&self) -> *mut TESWaterForm;
    fn get_ignored_by_sandbox(&self) -> bool;
    fn is_water(&self) -> bool;
}

impl<T: AsRef<TESObjectACTI> + AsMut<TESObjectACTI>> TESObjectACTIExt for T {
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    fn get_sound_loop(&self) -> *mut BGSSoundDescriptorForm {
        self.as_ref().get_sound_loop()
    }

    fn get_sound_activate(&self) -> *mut BGSSoundDescriptorForm {
        self.as_ref().get_sound_activate()
    }

    fn get_water_form(&self) -> *mut TESWaterForm {
        self.as_ref().get_water_form()
    }

    fn get_water_type(&self) -> *mut TESWaterForm {
        self.as_ref().get_water_type()
    }

    fn get_ignored_by_sandbox(&self) -> bool {
        self.as_ref().get_ignored_by_sandbox()
    }

    fn is_water(&self) -> bool {
        self.as_ref().is_water()
    }
}
