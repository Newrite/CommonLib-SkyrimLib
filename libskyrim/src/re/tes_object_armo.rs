use crate::offsets::offsets_rtti::RTTI_TESObjectARMO;
use crate::offsets::offsets_vtable::VTABLE_TESObjectARMO;

use crate::core_util::inherit;
use crate::re::BGSBipedObjectForm;
use crate::re::BGSBlockBashData;
use crate::re::BGSDestructibleObjectForm;
use crate::re::BGSEquipType;
use crate::re::BGSKeywordForm;
use crate::re::BGSPickupPutdownSounds;
use crate::re::BSTArray;
use crate::re::TESBipedModelForm;
use crate::re::TESBoundObject;
use crate::re::TESDescription;
use crate::re::TESEnchantableForm;
use crate::re::TESFullName;
use crate::re::TESObjectARMA;
use crate::re::TESRaceForm;
use crate::re::TESValueForm;
use crate::re::TESWeightForm;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct ARMORecordFlags: u32 {
        const kNonPlayable = 1 << 2;
        const kDeleted = 1 << 5;
        const kShield = 1 << 6;
        const kIgnored = 1 << 12;
    }
}

#[repr(C)]
pub struct TESObjectARMO {
    pub base: TESBoundObject,                                // 000
    pub full_name: TESFullName,                              // 030
    pub race_form: TESRaceForm,                              // 040
    pub enchantable_form: TESEnchantableForm,                // 050
    pub value_form: TESValueForm,                            // 068
    pub weight_form: TESWeightForm,                          // 078
    pub destructible_object_form: BGSDestructibleObjectForm, // 088
    pub pickup_putdown_sounds: BGSPickupPutdownSounds,       // 098
    pub biped_model_form: TESBipedModelForm,                 // 0B0
    pub equip_type: BGSEquipType,                            // 1A0
    pub biped_object_form: BGSBipedObjectForm,               // 1B0
    pub block_bash_data: BGSBlockBashData,                   // 1C0
    pub keyword_form: BGSKeywordForm,                        // 1D8
    pub description: TESDescription,                         // 1F0
    pub armor_rating: u32,                                   // 200 - DNAM
    pub pad204: u32,                                         // 204
    pub armor_addons: BSTArray<*mut TESObjectARMA>,          // 208
    pub template_armor: *mut TESObjectARMO,                  // 220 - TNAM
}
// Size and offset checks
const _: () = assert!(core::mem::size_of::<TESObjectARMO>() == 0x228);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, race_form) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, enchantable_form) == 0x50);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, value_form) == 0x68);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, weight_form) == 0x78);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, destructible_object_form) == 0x88);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, pickup_putdown_sounds) == 0x98);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, biped_model_form) == 0xB0);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, equip_type) == 0x1A0);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, biped_object_form) == 0x1B0);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, block_bash_data) == 0x1C0);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, keyword_form) == 0x1D8);
const _: () = assert!(core::mem::offset_of!(TESObjectARMO, description) == 0x1F0);

impl crate::relocation::RttiType for TESObjectARMO { const RTTI: crate::relocation::VariantID = RTTI_TESObjectARMO; }

inherit!(TESObjectARMO => TESFullName, full_name);
inherit!(TESObjectARMO => TESRaceForm, race_form);
inherit!(TESObjectARMO => TESEnchantableForm, enchantable_form);
inherit!(TESObjectARMO => TESValueForm, value_form);
inherit!(TESObjectARMO => TESWeightForm, weight_form);
inherit!(TESObjectARMO => BGSDestructibleObjectForm, destructible_object_form);
inherit!(TESObjectARMO => BGSPickupPutdownSounds, pickup_putdown_sounds);
inherit!(TESObjectARMO => TESBipedModelForm, biped_model_form);
inherit!(TESObjectARMO => BGSEquipType, equip_type);
inherit!(TESObjectARMO => BGSBipedObjectForm, biped_object_form);
inherit!(TESObjectARMO => BGSBlockBashData, block_bash_data);
inherit!(TESObjectARMO => BGSKeywordForm, keyword_form);
inherit!(TESObjectARMO => TESDescription, description);

impl crate::re::FormCastable for TESObjectARMO {
    const TARGET_FORM_TYPE: crate::re::FormType = crate::re::FormType::Armor;
}

impl TESObjectARMO {
    pub const RTTI: crate::relocation::VariantID = RTTI_TESObjectARMO;
    pub const VTABLE: &'static [crate::relocation::VariantID] = &VTABLE_TESObjectARMO;
    pub const FORMTYPE: crate::re::FormType = crate::re::FormType::Armor;

    // override (TESBoundObject)
    // void     InitializeData() override;                    // 04
    // bool     Load(TESFile* a_mod) override;                // 06
    // void     SaveGame(BGSSaveFormBuffer* a_buf) override;  // 0E
    // void     LoadGame(BGSLoadFormBuffer* a_buf) override;  // 0F
    // void     InitItemImpl() override;                      // 13
    // TESFile* GetDescriptionOwnerFile() const override;     // 14 - { return templateArmor ? templateArmor->GetFile(-1) : GetFile(-1); }
    // void     Copy(TESForm* a_srcForm) override;            // 2F

    // override (BGSKeywordForm)
    // [[nodiscard]] BGSKeyword* GetDefaultKeyword() const override;  // 05

    pub fn get_armor_rating(&self) -> f32 {
        self.armor_rating as f32 / 100.0
    }

    pub fn get_armor_addon(&self, _a_race: *mut crate::re::TESRace) -> *mut TESObjectARMA {
        // TODO: VERIFY - requires TESObjectARMA method IsValidRace to translate properly
        todo!("Requires TESObjectARMA full translation")
    }

    pub fn get_armor_addon_by_mask(&self, _a_race: *mut crate::re::TESRace, _a_slot: crate::re::BipedObjectSlot) -> *mut TESObjectARMA {
        // TODO: VERIFY - requires TESObjectARMA method IsValidRace and HasPartOf to translate properly
        todo!("Requires TESObjectARMA full translation")
    }

    // RELOCATION_ID SE: 24232, AE: 24736
    crate::relocation_func! {
        pub fn init_worn_armor(this: &mut TESObjectARMO, a_actor: *mut crate::re::Actor, a_biped: *mut core::ffi::c_void) => crate::relocation::VariantID::new(24232, 24736, 0)
    }
}
