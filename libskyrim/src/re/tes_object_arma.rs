use core_util::inherit;

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
use crate::re::tes_object::TESObject;
use crate::re::tes_object_armo::TESObjectARMO;
use crate::re::tes_race::TESRace;
use crate::re::tes_race_form::TESRaceForm;
use crate::relocation::{RelocationID, RttiType, VariantID};

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
    // void InitializeData() override;      // 04
    // void ClearData() override;           // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13

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

    // RELOCATION_ID SE: 17361, AE: 17759
    crate::relocation_func! {
        pub fn init_worn_armor_addon(this: &TESObjectARMA, armor: *mut TESObjectARMO, biped: *mut BSTSmartPointer<BipedAnim>, sex: SEX) => RelocationID::new(17361, 17759)
    }
}

pub trait TESObjectARMAExt {
    fn is_valid_race(&self, source_race: *mut TESRace) -> bool;
    fn init_worn_armor_addon(
        &self,
        armor: *mut TESObjectARMO,
        biped: *mut BSTSmartPointer<BipedAnim>,
        sex: SEX,
    );
}

impl<T: AsRef<TESObjectARMA>> TESObjectARMAExt for T {
    fn is_valid_race(&self, source_race: *mut TESRace) -> bool {
        self.as_ref().is_valid_race(source_race)
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
