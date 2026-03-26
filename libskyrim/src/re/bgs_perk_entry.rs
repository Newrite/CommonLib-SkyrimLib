#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_BGSPerkEntry;
use crate::offsets::offsets_vtable::VTABLE_BGSPerkEntry;
use crate::re::{Actor, BGSEntryPointEntryPoint, BGSEntryPointFunctionData, BGSPerk, TESFile};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::PERK_ENTRY_TYPE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PERK_ENTRY_TYPE {
    Quest = 0,
    Ability = 1,
    EntryPoint = 2,
}

/// C++ `RE::BGSPerkEntry::Header`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSPerkEntryHeader {
    pub rank: u8,
    pub priority: u8,
    pub unk2: u16,
    pub unk4: u32,
}

const _: () = assert!(core::mem::size_of::<BGSPerkEntryHeader>() == 0x8);

/// C++ `RE::BGSPerkEntry`
#[repr(C)]
pub struct BGSPerkEntry {
    pub vtable: *const usize,       // 00
    pub header: BGSPerkEntryHeader, // 08
}

const _: () = assert!(core::mem::size_of::<BGSPerkEntry>() == 0x10);

impl RttiType for BGSPerkEntry {
    const RTTI: VariantID = RTTI_BGSPerkEntry;
}

impl BGSPerkEntry {
    pub const RTTI: VariantID = RTTI_BGSPerkEntry;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSPerkEntry;

    crate::virtual_method! {
        pub const VFUNC_CHECK_CONDITION_FILTERS: usize = 0x00;
        pub fn check_condition_filters(
            &mut self,
            num_args: u32,
            args: *mut core::ffi::c_void
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FUNCTION: usize = 0x01;
        pub fn get_function(&mut self) -> BGSEntryPointEntryPoint
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FUNCTION_DATA: usize = 0x02;
        pub fn get_function_data() -> *mut BGSEntryPointFunctionData
    }

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x03;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x04;
        pub fn get_type() -> PERK_ENTRY_TYPE
    }

    crate::virtual_method! {
        pub const VFUNC_CLEAR_DATA: usize = 0x05;
        pub fn clear_data(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_ITEM: usize = 0x06;
        pub fn init_item(&mut self, owner: *mut TESFile)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD: usize = 0x07;
        pub fn load(&mut self, file: *mut TESFile) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SET_PARENT: usize = 0x08;
        pub fn set_parent(&mut self, parent: *mut BGSPerk)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ID: usize = 0x09;
        pub fn get_id() -> u16
    }

    crate::virtual_method! {
        pub const VFUNC_APPLY_PERK_ENTRY: usize = 0x0A;
        pub fn apply_perk_entry(&mut self, actor: *mut Actor)
    }

    crate::virtual_method! {
        pub const VFUNC_REMOVE_PERK_ENTRY: usize = 0x0B;
        pub fn remove_perk_entry(&mut self, actor: *mut Actor)
    }

    #[inline(always)]
    pub fn get_rank(&self) -> u8 {
        self.header.rank
    }

    #[inline(always)]
    pub fn get_priority(&self) -> u8 {
        self.header.priority
    }
}

pub trait BGSPerkEntryExt {
    fn check_condition_filters(&mut self, num_args: u32, args: *mut core::ffi::c_void) -> bool;
    fn dtor(&mut self);
    fn get_function(&mut self) -> BGSEntryPointEntryPoint;
    fn get_function_data(&self) -> *mut BGSEntryPointFunctionData;
    fn get_type(&self) -> PERK_ENTRY_TYPE;
    fn clear_data(&mut self);
    fn init_item(&mut self, owner: *mut TESFile);
    fn load(&mut self, file: *mut TESFile) -> bool;
    fn set_parent(&mut self, parent: *mut BGSPerk);
    fn get_id(&self) -> u16;
    fn apply_perk_entry(&mut self, actor: *mut Actor);
    fn remove_perk_entry(&mut self, actor: *mut Actor);
    fn get_rank(&self) -> u8;
    fn get_priority(&self) -> u8;
}

impl<T: AsRef<BGSPerkEntry> + AsMut<BGSPerkEntry>> BGSPerkEntryExt for T {
    #[inline(always)]
    fn check_condition_filters(&mut self, num_args: u32, args: *mut core::ffi::c_void) -> bool {
        BGSPerkEntry::check_condition_filters(self.as_mut(), num_args, args)
    }

    #[inline(always)]
    fn dtor(&mut self) {
        BGSPerkEntry::dtor(self.as_mut())
    }

    #[inline(always)]
    fn get_function(&mut self) -> BGSEntryPointEntryPoint {
        BGSPerkEntry::get_function(self.as_mut())
    }

    #[inline(always)]
    fn get_function_data(&self) -> *mut BGSEntryPointFunctionData {
        BGSPerkEntry::get_function_data(self.as_ref())
    }

    #[inline(always)]
    fn get_type(&self) -> PERK_ENTRY_TYPE {
        BGSPerkEntry::get_type(self.as_ref())
    }

    #[inline(always)]
    fn clear_data(&mut self) {
        BGSPerkEntry::clear_data(self.as_mut())
    }

    #[inline(always)]
    fn init_item(&mut self, owner: *mut TESFile) {
        BGSPerkEntry::init_item(self.as_mut(), owner)
    }

    #[inline(always)]
    fn load(&mut self, file: *mut TESFile) -> bool {
        BGSPerkEntry::load(self.as_mut(), file)
    }

    #[inline(always)]
    fn set_parent(&mut self, parent: *mut BGSPerk) {
        BGSPerkEntry::set_parent(self.as_mut(), parent)
    }

    #[inline(always)]
    fn get_id(&self) -> u16 {
        BGSPerkEntry::get_id(self.as_ref())
    }

    #[inline(always)]
    fn apply_perk_entry(&mut self, actor: *mut Actor) {
        BGSPerkEntry::apply_perk_entry(self.as_mut(), actor)
    }

    #[inline(always)]
    fn remove_perk_entry(&mut self, actor: *mut Actor) {
        BGSPerkEntry::remove_perk_entry(self.as_mut(), actor)
    }

    #[inline(always)]
    fn get_rank(&self) -> u8 {
        BGSPerkEntry::get_rank(self.as_ref())
    }

    #[inline(always)]
    fn get_priority(&self) -> u8 {
        BGSPerkEntry::get_priority(self.as_ref())
    }
}
