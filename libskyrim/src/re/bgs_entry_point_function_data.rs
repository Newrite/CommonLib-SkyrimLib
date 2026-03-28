#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_BGSEntryPointFunctionData;
use crate::offsets::offsets_vtable::VTABLE_BGSEntryPointFunctionData;
use crate::re::{BGSPerk, BGSPerkEntry, TESFile, TESForm};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BGSEntryPointFunctionData::ENTRY_POINT_FUNCTION_DATA`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSEntryPointFunctionDataType {
    Invalid = 0,
    OneValue = 1,
    TwoValue = 2,
    LeveledList = 3,
    ActivateChoice = 4,
    SpellItem = 5,
    BooleanGraphVariable = 6,
    Text = 7,
}

/// C++ `RE::BGSEntryPointFunctionData`
#[repr(C)]
pub struct BGSEntryPointFunctionData {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BGSEntryPointFunctionData>() == 0x8);

impl RttiType for BGSEntryPointFunctionData {
    const RTTI: VariantID = RTTI_BGSEntryPointFunctionData;
}

impl BGSEntryPointFunctionData {
    pub const RTTI: VariantID = RTTI_BGSEntryPointFunctionData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSEntryPointFunctionData;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x01;
        pub fn get_type() -> BGSEntryPointFunctionDataType
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_IMPL: usize = 0x02;
        pub fn load_impl(&mut self, mod_file: *mut TESFile) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_ITEM: usize = 0x03;
        pub fn init_item(&mut self, form: *mut TESForm)
    }

    crate::virtual_method! {
        pub const VFUNC_APPLY_ON_ADD: usize = 0x04;
        pub fn apply_on_add(
            &mut self,
            form: *mut TESForm,
            perk_entry: *mut BGSPerkEntry
        )
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ID: usize = 0x05;
        pub fn get_id(&self) -> u16
    }

    crate::virtual_method! {
        pub const VFUNC_SET_PARENT_PERK: usize = 0x06;
        pub fn set_parent_perk(&mut self, perk: *mut BGSPerk)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_PARENT_PERK: usize = 0x07;
        pub fn get_parent_perk(&self) -> *mut BGSPerk
    }
}

pub trait BGSEntryPointFunctionDataExt {
    fn dtor(&mut self);
    fn get_type(&self) -> BGSEntryPointFunctionDataType;
    fn load_impl(&mut self, mod_file: *mut TESFile) -> bool;
    fn init_item(&mut self, form: *mut TESForm);
    fn apply_on_add(&mut self, form: *mut TESForm, perk_entry: *mut BGSPerkEntry);
    fn get_id(&self) -> u16;
    fn set_parent_perk(&mut self, perk: *mut BGSPerk);
    fn get_parent_perk(&self) -> *mut BGSPerk;
    fn get_parent_perk_ref(&self) -> Option<&BGSPerk>;
}

impl<T: AsRef<BGSEntryPointFunctionData> + AsMut<BGSEntryPointFunctionData>>
    BGSEntryPointFunctionDataExt for T
{
    #[inline(always)]
    fn dtor(&mut self) {
        BGSEntryPointFunctionData::dtor(self.as_mut())
    }

    #[inline(always)]
    fn get_type(&self) -> BGSEntryPointFunctionDataType {
        BGSEntryPointFunctionData::get_type(self.as_ref())
    }

    #[inline(always)]
    fn load_impl(&mut self, mod_file: *mut TESFile) -> bool {
        BGSEntryPointFunctionData::load_impl(self.as_mut(), mod_file)
    }

    #[inline(always)]
    fn init_item(&mut self, form: *mut TESForm) {
        BGSEntryPointFunctionData::init_item(self.as_mut(), form)
    }

    #[inline(always)]
    fn apply_on_add(&mut self, form: *mut TESForm, perk_entry: *mut BGSPerkEntry) {
        BGSEntryPointFunctionData::apply_on_add(self.as_mut(), form, perk_entry)
    }

    #[inline(always)]
    fn get_id(&self) -> u16 {
        BGSEntryPointFunctionData::get_id(self.as_ref())
    }

    #[inline(always)]
    fn set_parent_perk(&mut self, perk: *mut BGSPerk) {
        BGSEntryPointFunctionData::set_parent_perk(self.as_mut(), perk)
    }

    #[inline(always)]
    fn get_parent_perk(&self) -> *mut BGSPerk {
        BGSEntryPointFunctionData::get_parent_perk(self.as_ref())
    }

    #[inline(always)]
    fn get_parent_perk_ref(&self) -> Option<&BGSPerk> {
        unsafe { BGSEntryPointFunctionData::get_parent_perk(self.as_ref()).as_ref() }
    }
}
