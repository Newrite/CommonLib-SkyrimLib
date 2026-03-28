use crate::offsets::offsets_rtti::RTTI_TESRaceForm;
use crate::offsets::offsets_vtable::VTABLE_TESRaceForm;

use crate::re::BaseFormComponent;
use crate::re::TESRace;
use crate::virtual_method;
use core_util::inherit;

#[repr(C)]
pub struct TESRaceForm {
    pub base: BaseFormComponent, // 00
    pub race: *mut TESRace,      // 08 - RNAM
}
const _: () = assert!(core::mem::size_of::<TESRaceForm>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESRaceForm, race) == 0x08);

impl crate::relocation::RttiType for TESRaceForm {
    const RTTI: crate::relocation::VariantID = RTTI_TESRaceForm;
}

inherit!(TESRaceForm : BaseFormComponent);

impl TESRaceForm {
    pub const RTTI: crate::relocation::VariantID = RTTI_TESRaceForm;
    pub const VTABLE: &'static [crate::relocation::VariantID] = &VTABLE_TESRaceForm;

    // override (BaseFormComponent)
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA_COMPONENT: usize = 0x01;
        pub fn initialize_data_component()
    }

    virtual_method! {
        pub const VFUNC_CLEAR_DATA_COMPONENT: usize = 0x02;
        pub fn clear_data_component()
    }

    virtual_method! {
        pub const VFUNC_COPY_COMPONENT: usize = 0x03;
        pub fn copy_component(rhs: *mut BaseFormComponent)
    }

    #[inline(always)]
    pub const fn get_race(&self) -> *mut TESRace {
        self.race
    }

    #[inline(always)]
    pub fn set_race(&mut self, race: *mut TESRace) {
        self.race = race;
    }
}

pub trait TESRaceFormExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
    fn get_race(&self) -> *mut TESRace;
    fn set_race(&mut self, race: *mut TESRace);
}

impl<T: AsRef<TESRaceForm> + AsMut<TESRaceForm>> TESRaceFormExt for T {
    #[inline(always)]
    fn dtor(&mut self) {
        TESRaceForm::dtor(self.as_mut())
    }

    #[inline(always)]
    fn initialize_data_component(&mut self) {
        TESRaceForm::initialize_data_component(self.as_mut())
    }

    #[inline(always)]
    fn clear_data_component(&mut self) {
        TESRaceForm::clear_data_component(self.as_mut())
    }

    #[inline(always)]
    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        TESRaceForm::copy_component(self.as_mut(), rhs)
    }

    #[inline(always)]
    fn get_race(&self) -> *mut TESRace {
        TESRaceForm::get_race(self.as_ref())
    }

    #[inline(always)]
    fn set_race(&mut self, race: *mut TESRace) {
        TESRaceForm::set_race(self.as_mut(), race)
    }
}
