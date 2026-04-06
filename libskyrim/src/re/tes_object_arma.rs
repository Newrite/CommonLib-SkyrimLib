use core::fmt::Write;
use core_util::{StringBuffer, inherit};

use crate::offsets::offsets_rtti::RTTI_TESObjectARMA;
use crate::offsets::offsets_vtable::VTABLE_TESObjectARMA;
use crate::re::bgs_art_object::BGSArtObject;
use crate::re::bgs_biped_object_form::BGSBipedObjectForm;
use crate::re::bgs_footstep_set::BGSFootstepSet;
use crate::re::bgs_list_form::BGSListForm;
use crate::re::bgs_texture_set::BGSTextureSet;
use crate::re::biped_anim::BipedAnim;
use crate::re::bst_array::BSTArray;
use crate::re::bst_smart_pointer::BSTSmartPointer;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::sexes::{SEX, SEXES_TOTAL};
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::re::tes_npc::TESNPC;
use crate::re::tes_object::TESObject;
use crate::re::tes_object_armo::TESObjectARMO;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::re::tes_race::TESRace;
use crate::re::tes_race_form::TESRaceForm;
use crate::re::{TESFile, TESForm};
use crate::relocation::{RelocationID, RttiType, VariantID, skyrim_cast_const};
use crate::virtual_method;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct TESObjectARMARecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OBJ_ARMA {
    pub priorities: [i8; SEXES_TOTAL],  // 00
    pub model_range: [i8; SEXES_TOTAL], // 02
    pub unused: [i8; SEXES_TOTAL],      // 04
    pub detection_sound_value: i8,      // 06
    pub pad07: u8,                      // 07
    pub weapon_adjust: f32,             // 08
}

const _: () = assert!(core::mem::size_of::<OBJ_ARMA>() == 0x0C);

#[repr(C)]
pub struct TESObjectARMA {
    pub base: TESObject,                                             // 00
    pub race_form: TESRaceForm,                                      // 20
    pub biped_object_form: BGSBipedObjectForm,                       // 30
    pub data: OBJ_ARMA,                                              // 40 - DNAM
    pub pad04c: u32,                                                 // 4C
    pub biped_models: [TESModelTextureSwap; SEXES_TOTAL],            // 50
    pub biped_model_1st_persons: [TESModelTextureSwap; SEXES_TOTAL], // C0
    pub skin_textures: [*mut BGSTextureSet; SEXES_TOTAL],            // 130 - NAM0/NAM1
    pub skin_texture_swap_lists: [*mut BGSListForm; SEXES_TOTAL],    // 140 - NAM2/NAM3
    pub additional_races: BSTArray<*mut TESRace>,                    // 150 - MODL
    pub footstep_set: *mut BGSFootstepSet,                           // 168 - SNDD
    pub art_object: *mut BGSArtObject,                               // 170 - ONAM
}

const _: () = assert!(core::mem::size_of::<TESObjectARMA>() == 0x178);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, race_form) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, biped_object_form) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, data) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, pad04c) == 0x4C);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, biped_models) == 0x50);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, biped_model_1st_persons) == 0xC0);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, skin_textures) == 0x130);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, skin_texture_swap_lists) == 0x140);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, additional_races) == 0x150);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, footstep_set) == 0x168);
const _: () = assert!(core::mem::offset_of!(TESObjectARMA, art_object) == 0x170);

impl RttiType for TESObjectARMA {
    const RTTI: VariantID = RTTI_TESObjectARMA;
}

impl FormCastable for TESObjectARMA {
    const TARGET_FORM_TYPE: FormType = FormType::Armature;
}

inherit!(TESObjectARMA : TESObject);
inherit!(TESObjectARMA => TESRaceForm, race_form);
inherit!(TESObjectARMA => BGSBipedObjectForm, biped_object_form);

impl TESObjectARMA {
    pub const RTTI: VariantID = RTTI_TESObjectARMA;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectARMA;
    pub const FORMTYPE: FormType = FormType::Armature;

    // override (TESObject)
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA: usize = 0x04;
        pub fn initialize_data(&mut self)
    }

    virtual_method! {
        pub const VFUNC_CLEAR_DATA: usize = 0x05;
        pub fn clear_data(&mut self)
    }

    virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_: *mut TESFile) -> bool
    }

    virtual_method! {
        pub const VFUNC_INIT_ITEM_IMPL: usize = 0x13;
        pub fn init_item_impl(&mut self)
    }

    #[inline]
    pub fn is_valid_race(&self, source_race: *mut TESRace) -> bool {
        let Some(source_race) = (unsafe { source_race.as_ref() }) else {
            return false;
        };

        #[inline]
        fn has_armor_race(source_race: &TESRace, target_race: *const TESRace) -> bool {
            if core::ptr::eq(source_race, unsafe {
                target_race.as_ref().unwrap_unchecked()
            }) {
                return true;
            }

            let mut armor_race = source_race.armor_parent_race;
            while let Some(current) = unsafe { armor_race.as_ref() } {
                if core::ptr::eq(current, unsafe { target_race.as_ref().unwrap_unchecked() }) {
                    return true;
                }
                armor_race = current.armor_parent_race;
            }

            false
        }

        if !self.race_form.race.is_null() && has_armor_race(source_race, self.race_form.race) {
            return true;
        }

        unsafe { self.additional_races.as_slice() }
            .iter()
            .copied()
            .filter(|race| !race.is_null())
            .any(|target_race| has_armor_race(source_race, target_race))
    }

    pub fn get_node_name(
        &self,
        dst_buff: *mut core::ffi::c_char,
        refr: *const TESObjectREFR,
        armor: *const TESObjectARMO,
        weight_override: f32,
    ) {
        if dst_buff.is_null() {
            return;
        }

        assert!(
            !refr.is_null(),
            "TESObjectARMA::get_node_name requires non-null TESObjectREFR"
        );

        let mut weight;
        let mut sex = 0u32;
        let base_obj = unsafe { (*refr).get_base_object() };

        if !base_obj.is_null() {
            let mut npc =
                unsafe { skyrim_cast_const::<TESForm, TESNPC>(base_obj.cast::<TESForm>()) };
            if !npc.is_null() {
                npc = unsafe { (*npc).get_root_face_npc_const() };
                weight = unsafe { (*npc).weight };
                sex = unsafe { (*npc).get_sex() as u32 };
            } else {
                weight = unsafe { (*refr).get_weight() };
            }
        } else {
            weight = unsafe { (*refr).get_weight() };
        }

        if weight_override >= 0.0 {
            weight = weight_override * 100.0;
        }

        let armor_form_id = if armor.is_null() {
            0
        } else {
            unsafe { (*armor).get_form_id() }
        };

        let mut buffer = StringBuffer::<48>::new();
        let _ = write!(
            &mut buffer,
            " ({:08X})[{}]/ ({:08X}) [{:2.0}%]",
            self.get_form_id(),
            sex,
            armor_form_id,
            weight
        );
        let bytes = buffer.as_c_str().to_bytes_with_nul();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr().cast(), dst_buff, bytes.len());
        }
    }

    // RELOCATION_ID SE: 17361, AE: 17759
    crate::relocation_func! {
        pub fn init_worn_armor_addon(this: &TESObjectARMA, armor: *mut TESObjectARMO, biped: *mut BSTSmartPointer<BipedAnim>, sex: SEX) => RelocationID::new(17361, 17759)
    }
}

pub trait TESObjectARMAExt {
    fn dtor(&mut self);
    fn initialize_data(&mut self);
    fn clear_data(&mut self);
    fn load(&mut self, mod_: *mut TESFile) -> bool;
    fn init_item_impl(&mut self);
    fn is_valid_race(&self, source_race: *mut TESRace) -> bool;
    fn get_node_name(
        &self,
        dst_buff: *mut core::ffi::c_char,
        refr: *const TESObjectREFR,
        armor: *const TESObjectARMO,
        weight_override: f32,
    );
    fn init_worn_armor_addon(
        &self,
        armor: *mut TESObjectARMO,
        biped: *mut BSTSmartPointer<BipedAnim>,
        sex: SEX,
    );
}

impl<T: AsRef<TESObjectARMA> + AsMut<TESObjectARMA>> TESObjectARMAExt for T {
    fn dtor(&mut self) {
        TESObjectARMA::dtor(self.as_mut())
    }

    fn initialize_data(&mut self) {
        TESObjectARMA::initialize_data(self.as_mut())
    }

    fn clear_data(&mut self) {
        TESObjectARMA::clear_data(self.as_mut())
    }

    fn load(&mut self, mod_: *mut TESFile) -> bool {
        TESObjectARMA::load(self.as_mut(), mod_)
    }

    fn init_item_impl(&mut self) {
        TESObjectARMA::init_item_impl(self.as_mut())
    }

    fn is_valid_race(&self, source_race: *mut TESRace) -> bool {
        TESObjectARMA::is_valid_race(self.as_ref(), source_race)
    }

    fn get_node_name(
        &self,
        dst_buff: *mut core::ffi::c_char,
        refr: *const TESObjectREFR,
        armor: *const TESObjectARMO,
        weight_override: f32,
    ) {
        TESObjectARMA::get_node_name(self.as_ref(), dst_buff, refr, armor, weight_override)
    }

    fn init_worn_armor_addon(
        &self,
        armor: *mut TESObjectARMO,
        biped: *mut BSTSmartPointer<BipedAnim>,
        sex: SEX,
    ) {
        TESObjectARMA::init_worn_armor_addon(self.as_ref(), armor, biped, sex)
    }
}
