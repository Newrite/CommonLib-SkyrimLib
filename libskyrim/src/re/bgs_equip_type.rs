use crate::offsets::offsets_rtti::RTTI_BGSEquipType;
use crate::offsets::offsets_vtable::VTABLE_BGSEquipType;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bgs_equip_slot::BGSEquipSlot;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

#[repr(C)]
pub struct BGSEquipType {
    pub base: BaseFormComponent,       // 00
    pub equip_slot: *mut BGSEquipSlot, // 08 - ETYP
}

const _: () = assert!(core::mem::size_of::<BGSEquipType>() == 0x10);

impl RttiType for BGSEquipType {
    const RTTI: VariantID = RTTI_BGSEquipType;
}

inherit!(BGSEquipType : BaseFormComponent);

impl BGSEquipType {
    pub const RTTI: VariantID = RTTI_BGSEquipType;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSEquipType;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03

    virtual_method! {
        pub const GET_EQUIP_SLOT: usize = 0x04;
        pub fn get_equip_slot() -> *mut BGSEquipSlot
    }

    virtual_method! {
        pub const SET_EQUIP_SLOT: usize = 0x05;
        pub fn set_equip_slot(&mut self, slot: *mut BGSEquipSlot)
    }

    #[inline(always)]
    pub fn get_equip_slot_ref(&self) -> Option<&BGSEquipSlot> {
        unsafe { self.get_equip_slot().as_ref() }
    }
}

pub trait BGSEquipTypeExt {
    fn get_equip_slot(&self) -> *mut BGSEquipSlot;
    fn get_equip_slot_ref(&self) -> Option<&BGSEquipSlot>;
    fn set_equip_slot(&mut self, slot: *mut BGSEquipSlot);
}

impl<T: AsRef<BGSEquipType> + AsMut<BGSEquipType>> BGSEquipTypeExt for T {
    fn get_equip_slot(&self) -> *mut BGSEquipSlot {
        self.as_ref().get_equip_slot()
    }

    fn get_equip_slot_ref(&self) -> Option<&BGSEquipSlot> {
        self.as_ref().get_equip_slot_ref()
    }

    fn set_equip_slot(&mut self, slot: *mut BGSEquipSlot) {
        self.as_mut().set_equip_slot(slot)
    }
}
