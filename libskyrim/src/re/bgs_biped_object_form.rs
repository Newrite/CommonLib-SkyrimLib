use crate::offsets::offsets_rtti::RTTI_BGSBipedObjectForm;
use crate::offsets::offsets_vtable::VTABLE_BGSBipedObjectForm;

use crate::core_util::inherit;
use crate::re::BaseFormComponent;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct BipedObjectSlot: u32 {
        const kNone = 0;
        const kHead = 1 << 0;
        const kHair = 1 << 1;
        const kBody = 1 << 2;
        const kHands = 1 << 3;
        const kForearms = 1 << 4;
        const kAmulet = 1 << 5;
        const kRing = 1 << 6;
        const kFeet = 1 << 7;
        const kCalves = 1 << 8;
        const kShield = 1 << 9;
        const kTail = 1 << 10;
        const kLongHair = 1 << 11;
        const kCirclet = 1 << 12;
        const kEars = 1 << 13;
        const kModMouth = 1 << 14;
        const kModNeck = 1 << 15;
        const kModChestPrimary = 1 << 16;
        const kModBack = 1 << 17;
        const kModMisc1 = 1 << 18;
        const kModPelvisPrimary = 1 << 19;
        const kDecapitateHead = 1 << 20;
        const kDecapitate = 1 << 21;
        const kModPelvisSecondary = 1 << 22;
        const kModLegRight = 1 << 23;
        const kModLegLeft = 1 << 24;
        const kModFaceJewelry = 1 << 25;
        const kModChestSecondary = 1 << 26;
        const kModShoulder = 1 << 27;
        const kModArmLeft = 1 << 28;
        const kModArmRight = 1 << 29;
        const kModMisc2 = 1 << 30;
        const kFX01 = 1 << 31;
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmorType {
    LightArmor = 0,
    HeavyArmor = 1,
    Clothing = 2,
}

#[repr(C)]
pub struct BIPED_MODEL {
    pub biped_object_slots: BipedObjectSlot, // 0x00
    pub armor_type: ArmorType,               // 0x04
}
const _: () = assert!(core::mem::size_of::<BIPED_MODEL>() == 0x8);

#[repr(C)]
pub struct BGSBipedObjectForm {
    pub base: BaseFormComponent,       // 0x00
    pub biped_model_data: BIPED_MODEL, // 0x08 - BOD2
}
const _: () = assert!(core::mem::size_of::<BGSBipedObjectForm>() == 0x10);

impl crate::relocation::RttiType for BGSBipedObjectForm {
    const RTTI: crate::relocation::VariantID = RTTI_BGSBipedObjectForm;
}

inherit!(BGSBipedObjectForm : BaseFormComponent);

impl BGSBipedObjectForm {
    pub const RTTI: crate::relocation::VariantID = RTTI_BGSBipedObjectForm;
    pub const VTABLE: &'static [crate::relocation::VariantID] = &VTABLE_BGSBipedObjectForm;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02 - { return; }
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03

    pub fn add_slot_to_mask(&mut self, a_slot: BipedObjectSlot) -> BipedObjectSlot {
        self.biped_model_data.biped_object_slots.insert(a_slot);
        self.biped_model_data.biped_object_slots
    }

    pub fn get_armor_type(&self) -> ArmorType {
        self.biped_model_data.armor_type
    }

    pub fn get_slot_mask(&self) -> BipedObjectSlot {
        self.biped_model_data.biped_object_slots
    }

    pub fn has_part_of(&self, a_flag: BipedObjectSlot) -> bool {
        self.biped_model_data.biped_object_slots.contains(a_flag)
    }

    pub fn is_clothing(&self) -> bool {
        self.biped_model_data.armor_type == ArmorType::Clothing
    }

    pub fn is_heavy_armor(&self) -> bool {
        self.biped_model_data.armor_type == ArmorType::HeavyArmor
    }

    pub fn is_light_armor(&self) -> bool {
        self.biped_model_data.armor_type == ArmorType::LightArmor
    }

    pub fn is_shield(&self) -> bool {
        self.biped_model_data
            .biped_object_slots
            .contains(BipedObjectSlot::kShield)
    }

    pub fn remove_slot_from_mask(&mut self, a_slot: BipedObjectSlot) -> BipedObjectSlot {
        if a_slot != BipedObjectSlot::kNone {
            self.biped_model_data.biped_object_slots.remove(a_slot);
        }
        self.biped_model_data.biped_object_slots
    }

    pub fn set_slot_mask(&mut self, a_mask: BipedObjectSlot) {
        self.biped_model_data.biped_object_slots = a_mask;
    }
}

pub trait BGSBipedObjectFormExt {
    fn add_slot_to_mask(&mut self, a_slot: BipedObjectSlot) -> BipedObjectSlot;
    fn get_armor_type(&self) -> ArmorType;
    fn get_slot_mask(&self) -> BipedObjectSlot;
    fn has_part_of(&self, a_flag: BipedObjectSlot) -> bool;
    fn is_clothing(&self) -> bool;
    fn is_heavy_armor(&self) -> bool;
    fn is_light_armor(&self) -> bool;
    fn is_shield(&self) -> bool;
    fn remove_slot_from_mask(&mut self, a_slot: BipedObjectSlot) -> BipedObjectSlot;
    fn set_slot_mask(&mut self, a_mask: BipedObjectSlot);
}

impl<T: AsRef<BGSBipedObjectForm> + AsMut<BGSBipedObjectForm>> BGSBipedObjectFormExt for T {
    fn add_slot_to_mask(&mut self, a_slot: BipedObjectSlot) -> BipedObjectSlot {
        self.as_mut().add_slot_to_mask(a_slot)
    }

    fn get_armor_type(&self) -> ArmorType {
        self.as_ref().get_armor_type()
    }

    fn get_slot_mask(&self) -> BipedObjectSlot {
        self.as_ref().get_slot_mask()
    }

    fn has_part_of(&self, a_flag: BipedObjectSlot) -> bool {
        self.as_ref().has_part_of(a_flag)
    }

    fn is_clothing(&self) -> bool {
        self.as_ref().is_clothing()
    }

    fn is_heavy_armor(&self) -> bool {
        self.as_ref().is_heavy_armor()
    }

    fn is_light_armor(&self) -> bool {
        self.as_ref().is_light_armor()
    }

    fn is_shield(&self) -> bool {
        self.as_ref().is_shield()
    }

    fn remove_slot_from_mask(&mut self, a_slot: BipedObjectSlot) -> BipedObjectSlot {
        self.as_mut().remove_slot_from_mask(a_slot)
    }

    fn set_slot_mask(&mut self, a_mask: BipedObjectSlot) {
        self.as_mut().set_slot_mask(a_mask)
    }
}
