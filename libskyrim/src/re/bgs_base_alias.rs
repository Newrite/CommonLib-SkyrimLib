use core_util::EnumSet;

use crate::offsets::offsets_rtti::RTTI_BGSBaseAlias;
use crate::offsets::offsets_vtable::VTABLE_BGSBaseAlias;
use crate::re::{BSFixedString, TESFile, TESForm, TESQuest, VMTypeID};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BGSBaseAlias::FLAGS`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSBaseAliasFlags {
    None = 0,
    Reserves = 1 << 0,
    Optional = 1 << 1,
    QuestObject = 1 << 2,
    AllowReuse = 1 << 3,
    AllowDead = 1 << 4,
    LoadedOnly = 1 << 5,
    Essential = 1 << 6,
    AllowDisabled = 1 << 7,
    StoreName = 1 << 8,
    AllowReserved = 1 << 9,
    Protected = 1 << 10,
    ForcedFromAlias = 1 << 11,
    AllowDestroyed = 1 << 12,
    FindPlayerClosest = 1 << 13,
    UsesNames = 1 << 14,
    InitiallyDisabled = 1 << 15,
    AllowCleared = 1 << 16,
    ClearNameOnRemove = 1 << 17,
    ActorsOnly = 1 << 18,
    Transient = 1 << 19,
    ExternalLink = 1 << 20,
    NoPickpocket = 1 << 21,
    DataAlias = 1 << 22,
    SceneOptional = 1 << 24,
    CreateIn = 0x8000_0000,
}

core_util::impl_enumset_type!(BGSBaseAliasFlags => u32);

/// C++ `RE::BGSBaseAlias::FILL_TYPE`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSBaseAliasFillType {
    Conditions = 0,
    Forced = 1,
    FromAlias = 2,
    FromEvent = 3,
    Created = 4,
    FromExternal = 5,
    UniqueActor = 6,
    NearAlias = 7,
}

core_util::impl_enumset_type!(BGSBaseAliasFillType => u16);

/// C++ `RE::BGSBaseAlias`
#[repr(C)]
pub struct BGSBaseAlias {
    pub vtable: *const usize,                          // 00
    pub alias_name: BSFixedString,                     // 08
    pub owning_quest: *mut TESQuest,                   // 10
    pub alias_id: u32,                                 // 18
    pub flags: EnumSet<BGSBaseAliasFlags, u32>,        // 1C
    pub fill_type: EnumSet<BGSBaseAliasFillType, u16>, // 20
    pub pad22: u16,                                    // 22
    pub pad24: u32,                                    // 24
}

const _: () = assert!(core::mem::size_of::<BGSBaseAlias>() == 0x28);
const _: () = assert!(core::mem::offset_of!(BGSBaseAlias, alias_name) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSBaseAlias, owning_quest) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSBaseAlias, alias_id) == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSBaseAlias, flags) == 0x1C);
const _: () = assert!(core::mem::offset_of!(BGSBaseAlias, fill_type) == 0x20);

impl RttiType for BGSBaseAlias {
    const RTTI: VariantID = RTTI_BGSBaseAlias;
}

impl BGSBaseAlias {
    pub const RTTI: VariantID = RTTI_BGSBaseAlias;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSBaseAlias;
    pub const VM_TYPE_ID: VMTypeID = 139;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_LOAD: usize = 0x01;
        pub fn load(&mut self, mod_file: *mut TESFile) -> bool
    }

    virtual_method! {
        pub const VFUNC_INIT_ITEM: usize = 0x02;
        pub fn init_item(&mut self, form: *mut TESForm)
    }

    virtual_method! {
        pub const VFUNC_Q_TYPE: usize = 0x03;
        fn q_type_ptr(&self) -> *const BSFixedString
    }

    #[inline(always)]
    pub fn q_type(&self) -> &BSFixedString {
        let q_type = self.q_type_ptr();
        debug_assert!(!q_type.is_null());
        unsafe { &*q_type }
    }

    #[inline(always)]
    pub fn get_type_string(&self) -> &BSFixedString {
        self.q_type()
    }

    #[inline(always)]
    pub fn get_vm_type_id(&self) -> VMTypeID {
        match self
            .get_type_string()
            .as_c_str()
            .map(|string| string.to_bytes())
        {
            Some(b"Loc") => crate::re::BGSLocAlias::VM_TYPE_ID,
            Some(b"Ref") => crate::re::BGSRefAlias::VM_TYPE_ID,
            _ => Self::VM_TYPE_ID,
        }
    }

    #[inline(always)]
    pub fn is_essential(&self) -> bool {
        self.flags.all(BGSBaseAliasFlags::Essential)
    }

    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        self.flags.all(BGSBaseAliasFlags::Protected)
    }

    #[inline(always)]
    pub fn is_quest_object(&self) -> bool {
        self.flags.all(BGSBaseAliasFlags::QuestObject)
    }

    #[inline(always)]
    pub fn set_essential(&mut self, set: bool) {
        if set {
            self.flags.set(BGSBaseAliasFlags::Essential);
        } else {
            self.flags.reset(BGSBaseAliasFlags::Essential);
        }
    }

    #[inline(always)]
    pub fn set_protected(&mut self, set: bool) {
        if set {
            self.flags.set(BGSBaseAliasFlags::Protected);
        } else {
            self.flags.reset(BGSBaseAliasFlags::Protected);
        }
    }
}

pub trait BGSBaseAliasExt {
    fn dtor(&mut self);
    fn load(&mut self, mod_file: *mut TESFile) -> bool;
    fn init_item(&mut self, form: *mut TESForm);
    fn q_type(&self) -> &BSFixedString;
    fn get_type_string(&self) -> &BSFixedString;
    fn get_vm_type_id(&self) -> VMTypeID;
    fn is_essential(&self) -> bool;
    fn is_protected(&self) -> bool;
    fn is_quest_object(&self) -> bool;
    fn set_essential(&mut self, set: bool);
    fn set_protected(&mut self, set: bool);
}

impl<T: AsRef<BGSBaseAlias> + AsMut<BGSBaseAlias>> BGSBaseAliasExt for T {
    #[inline(always)]
    fn dtor(&mut self) {
        BGSBaseAlias::dtor(self.as_mut())
    }

    #[inline(always)]
    fn load(&mut self, mod_file: *mut TESFile) -> bool {
        BGSBaseAlias::load(self.as_mut(), mod_file)
    }

    #[inline(always)]
    fn init_item(&mut self, form: *mut TESForm) {
        BGSBaseAlias::init_item(self.as_mut(), form)
    }

    #[inline(always)]
    fn q_type(&self) -> &BSFixedString {
        BGSBaseAlias::q_type(self.as_ref())
    }

    #[inline(always)]
    fn get_type_string(&self) -> &BSFixedString {
        BGSBaseAlias::get_type_string(self.as_ref())
    }

    #[inline(always)]
    fn get_vm_type_id(&self) -> VMTypeID {
        BGSBaseAlias::get_vm_type_id(self.as_ref())
    }

    #[inline(always)]
    fn is_essential(&self) -> bool {
        BGSBaseAlias::is_essential(self.as_ref())
    }

    #[inline(always)]
    fn is_protected(&self) -> bool {
        BGSBaseAlias::is_protected(self.as_ref())
    }

    #[inline(always)]
    fn is_quest_object(&self) -> bool {
        BGSBaseAlias::is_quest_object(self.as_ref())
    }

    #[inline(always)]
    fn set_essential(&mut self, set: bool) {
        BGSBaseAlias::set_essential(self.as_mut(), set)
    }

    #[inline(always)]
    fn set_protected(&mut self, set: bool) {
        BGSBaseAlias::set_protected(self.as_mut(), set)
    }
}
