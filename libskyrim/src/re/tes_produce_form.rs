#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_TESProduceForm;
use crate::offsets::offsets_vtable::VTABLE_TESProduceForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::tes_bound_object::TESBoundObject;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

pub struct SEASON;

impl SEASON {
    pub const NONE: u32 = u32::MAX;
    pub const SPRING: usize = 0;
    pub const SUMMER: usize = 1;
    pub const FALL: usize = 2;
    pub const WINTER: usize = 3;
    pub const TOTAL: usize = 4;
}

#[repr(C)]
pub struct TESProduceForm {
    pub base: BaseFormComponent,                    // 00
    pub harvest_sound: *mut BGSSoundDescriptorForm, // 08
    pub produce_item: *mut TESBoundObject,          // 10
    pub produce_chance: [i8; SEASON::TOTAL],        // 18
    pub pad1c: u32,                                 // 1C
}

const _: () = assert!(core::mem::size_of::<TESProduceForm>() == 0x20);
const _: () = assert!(core::mem::offset_of!(TESProduceForm, harvest_sound) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESProduceForm, produce_item) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESProduceForm, produce_chance) == 0x18);
const _: () = assert!(core::mem::offset_of!(TESProduceForm, pad1c) == 0x1C);

impl RttiType for TESProduceForm {
    const RTTI: VariantID = RTTI_TESProduceForm;
}

inherit!(TESProduceForm : BaseFormComponent);

impl TESProduceForm {
    pub const RTTI: VariantID = RTTI_TESProduceForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESProduceForm;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    #[inline(always)]
    pub const fn get_harvest_sound(&self) -> *mut BGSSoundDescriptorForm {
        self.harvest_sound
    }

    #[inline(always)]
    pub const fn get_produce_item(&self) -> *mut TESBoundObject {
        self.produce_item
    }

    #[inline(always)]
    pub const fn get_produce_chance(&self) -> &[i8; SEASON::TOTAL] {
        &self.produce_chance
    }

    #[inline(always)]
    pub const fn get_produce_chance_for_season(&self, season: usize) -> Option<i8> {
        if season < SEASON::TOTAL {
            Some(self.produce_chance[season])
        } else {
            None
        }
    }

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;              // 01
    // void ClearDataComponent() override;                   // 02
    // void CopyComponent(BaseFormComponent* rhs) override;  // 03
}

pub trait TESProduceFormExt {
    fn dtor(&mut self);
    fn get_harvest_sound(&self) -> *mut BGSSoundDescriptorForm;
    fn get_produce_item(&self) -> *mut TESBoundObject;
    fn get_produce_chance(&self) -> &[i8; SEASON::TOTAL];
    fn get_produce_chance_for_season(&self, season: usize) -> Option<i8>;
}

impl<T: AsRef<TESProduceForm> + AsMut<TESProduceForm>> TESProduceFormExt for T {
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    fn get_harvest_sound(&self) -> *mut BGSSoundDescriptorForm {
        self.as_ref().get_harvest_sound()
    }

    fn get_produce_item(&self) -> *mut TESBoundObject {
        self.as_ref().get_produce_item()
    }

    fn get_produce_chance(&self) -> &[i8; SEASON::TOTAL] {
        self.as_ref().get_produce_chance()
    }

    fn get_produce_chance_for_season(&self, season: usize) -> Option<i8> {
        self.as_ref().get_produce_chance_for_season(season)
    }
}
