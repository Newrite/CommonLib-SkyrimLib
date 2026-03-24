use core::ffi::c_void;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESSpellList;
use crate::offsets::offsets_vtable::VTABLE_TESSpellList;
use crate::re::SpellItem;
use crate::re::TESLevSpell;
use crate::re::TESShout;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct SpellData {
    pub spells: *mut *mut SpellItem,       // 00
    pub lev_spells: *mut *mut TESLevSpell, // 08
    pub shouts: *mut *mut TESShout,        // 10
    pub num_spells: u32,                   // 18
    pub num_lev_spells: u32,               // 1C
    pub num_shouts: u32,                   // 20
    pub pad24: u32,                        // 24
}

const _: () = assert!(core::mem::size_of::<SpellData>() == 0x28);
const _: () = assert!(core::mem::offset_of!(SpellData, spells) == 0x00);
const _: () = assert!(core::mem::offset_of!(SpellData, lev_spells) == 0x08);
const _: () = assert!(core::mem::offset_of!(SpellData, shouts) == 0x10);
const _: () = assert!(core::mem::offset_of!(SpellData, num_spells) == 0x18);
const _: () = assert!(core::mem::offset_of!(SpellData, num_lev_spells) == 0x1C);
const _: () = assert!(core::mem::offset_of!(SpellData, num_shouts) == 0x20);

impl Default for SpellData {
    fn default() -> Self {
        Self::new()
    }
}

impl SpellData {
    #[inline]
    pub const fn new() -> Self {
        Self {
            spells: core::ptr::null_mut(),
            lev_spells: core::ptr::null_mut(),
            shouts: core::ptr::null_mut(),
            num_spells: 0,
            num_lev_spells: 0,
            num_shouts: 0,
            pad24: 0,
        }
    }

    #[inline]
    pub fn spells_slice(&self) -> &[*mut SpellItem] {
        if self.spells.is_null() || self.num_spells == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.spells, self.num_spells as usize) }
        }
    }

    #[inline]
    pub fn lev_spells_slice(&self) -> &[*mut TESLevSpell] {
        if self.lev_spells.is_null() || self.num_lev_spells == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.lev_spells, self.num_lev_spells as usize) }
        }
    }

    #[inline]
    pub fn shouts_slice(&self) -> &[*mut TESShout] {
        if self.shouts.is_null() || self.num_shouts == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.shouts, self.num_shouts as usize) }
        }
    }

    fn copy_lev_spell_list(&mut self, copied_data: &[*mut TESLevSpell]) {
        let old_data = self.lev_spells;
        let new_size = copied_data.len();
        let new_data = if new_size == 0 {
            core::ptr::null_mut()
        } else {
            unsafe {
                crate::ffi::commonlib_calloc(new_size, core::mem::size_of::<*mut TESLevSpell>())
                    as *mut *mut TESLevSpell
            }
        };

        if new_size != 0 {
            assert!(
                !new_data.is_null(),
                "TESSpellList::SpellData lev spell allocation failed"
            );
            unsafe {
                core::ptr::copy_nonoverlapping(copied_data.as_ptr(), new_data, new_size);
            }
        }

        self.num_lev_spells = new_size as u32;
        self.lev_spells = new_data;

        if !old_data.is_null() {
            unsafe { crate::ffi::commonlib_free(old_data.cast::<c_void>()) };
        }
    }

    fn copy_shout_list(&mut self, copied_data: &[*mut TESShout]) {
        let old_data = self.shouts;
        let new_size = copied_data.len();
        let new_data = if new_size == 0 {
            core::ptr::null_mut()
        } else {
            unsafe {
                crate::ffi::commonlib_calloc(new_size, core::mem::size_of::<*mut TESShout>())
                    as *mut *mut TESShout
            }
        };

        if new_size != 0 {
            assert!(
                !new_data.is_null(),
                "TESSpellList::SpellData shout allocation failed"
            );
            unsafe {
                core::ptr::copy_nonoverlapping(copied_data.as_ptr(), new_data, new_size);
            }
        }

        self.num_shouts = new_size as u32;
        self.shouts = new_data;

        if !old_data.is_null() {
            unsafe { crate::ffi::commonlib_free(old_data.cast::<c_void>()) };
        }
    }

    fn copy_spell_list(&mut self, copied_data: &[*mut SpellItem]) {
        let old_data = self.spells;
        let new_size = copied_data.len();
        let new_data = if new_size == 0 {
            core::ptr::null_mut()
        } else {
            unsafe {
                crate::ffi::commonlib_calloc(new_size, core::mem::size_of::<*mut SpellItem>())
                    as *mut *mut SpellItem
            }
        };

        if new_size != 0 {
            assert!(
                !new_data.is_null(),
                "TESSpellList::SpellData spell allocation failed"
            );
            unsafe {
                core::ptr::copy_nonoverlapping(copied_data.as_ptr(), new_data, new_size);
            }
        }

        self.num_spells = new_size as u32;
        self.spells = new_data;

        if !old_data.is_null() {
            unsafe { crate::ffi::commonlib_free(old_data.cast::<c_void>()) };
        }
    }

    pub fn add_lev_spell(&mut self, lev_spell: *mut TESLevSpell) -> bool {
        if self.get_lev_spell_index(lev_spell).is_some() {
            return false;
        }

        let mut copied_data = self.lev_spells_slice().to_vec();
        copied_data.push(lev_spell);
        self.copy_lev_spell_list(&copied_data);
        true
    }

    pub fn add_lev_spells(&mut self, lev_spells: &[*mut TESLevSpell]) -> bool {
        let mut copied_data = self.lev_spells_slice().to_vec();
        for &spell in lev_spells {
            if !copied_data.contains(&spell) {
                copied_data.push(spell);
            }
        }
        self.copy_lev_spell_list(&copied_data);
        true
    }

    pub fn add_shout(&mut self, shout: *mut TESShout) -> bool {
        if self.get_shout_index(shout).is_some() {
            return false;
        }

        let mut copied_data = self.shouts_slice().to_vec();
        copied_data.push(shout);
        self.copy_shout_list(&copied_data);
        true
    }

    pub fn add_shouts(&mut self, shouts: &[*mut TESShout]) -> bool {
        let mut copied_data = self.shouts_slice().to_vec();
        for &shout in shouts {
            if !copied_data.contains(&shout) {
                copied_data.push(shout);
            }
        }
        self.copy_shout_list(&copied_data);
        true
    }

    pub fn add_spell(&mut self, spell: *mut SpellItem) -> bool {
        if self.get_spell_index(spell).is_some() {
            return false;
        }

        let mut copied_data = self.spells_slice().to_vec();
        copied_data.push(spell);
        self.copy_spell_list(&copied_data);
        true
    }

    pub fn add_spells(&mut self, spells: &[*mut SpellItem]) -> bool {
        let mut copied_data = self.spells_slice().to_vec();
        for &spell in spells {
            if !copied_data.contains(&spell) {
                copied_data.push(spell);
            }
        }
        self.copy_spell_list(&copied_data);
        true
    }

    #[inline]
    pub fn get_spell_index(&self, spell: *const SpellItem) -> Option<u32> {
        self.spells_slice()
            .iter()
            .position(|&current| core::ptr::eq(current as *const SpellItem, spell))
            .map(|index| index as u32)
    }

    #[inline]
    pub fn get_lev_spell_index(&self, lev_spell: *const TESLevSpell) -> Option<u32> {
        self.lev_spells_slice()
            .iter()
            .position(|&current| core::ptr::eq(current as *const TESLevSpell, lev_spell))
            .map(|index| index as u32)
    }

    #[inline]
    pub fn get_shout_index(&self, shout: *const TESShout) -> Option<u32> {
        self.shouts_slice()
            .iter()
            .position(|&current| core::ptr::eq(current as *const TESShout, shout))
            .map(|index| index as u32)
    }

    pub fn remove_lev_spell(&mut self, lev_spell: *mut TESLevSpell) -> bool {
        let Some(index) = self.get_lev_spell_index(lev_spell) else {
            return false;
        };

        let mut copied_data = self.lev_spells_slice().to_vec();
        copied_data.remove(index as usize);
        self.copy_lev_spell_list(&copied_data);
        true
    }

    pub fn remove_lev_spells(&mut self, lev_spells: &[*mut TESLevSpell]) -> bool {
        let mut copied_data = self.lev_spells_slice().to_vec();
        let old_len = copied_data.len();
        copied_data.retain(|spell| !lev_spells.contains(spell));
        if copied_data.len() != old_len {
            self.copy_lev_spell_list(&copied_data);
            true
        } else {
            false
        }
    }

    pub fn remove_shout(&mut self, shout: *mut TESShout) -> bool {
        let Some(index) = self.get_shout_index(shout) else {
            return false;
        };

        let mut copied_data = self.shouts_slice().to_vec();
        copied_data.remove(index as usize);
        self.copy_shout_list(&copied_data);
        true
    }

    pub fn remove_shouts(&mut self, shouts: &[*mut TESShout]) -> bool {
        let mut copied_data = self.shouts_slice().to_vec();
        let old_len = copied_data.len();
        copied_data.retain(|shout| !shouts.contains(shout));
        if copied_data.len() != old_len {
            self.copy_shout_list(&copied_data);
            true
        } else {
            false
        }
    }

    pub fn remove_spell(&mut self, spell: *mut SpellItem) -> bool {
        let Some(index) = self.get_spell_index(spell) else {
            return false;
        };

        let mut copied_data = self.spells_slice().to_vec();
        copied_data.remove(index as usize);
        self.copy_spell_list(&copied_data);
        true
    }

    pub fn remove_spells(&mut self, spells: &[*mut SpellItem]) -> bool {
        let mut copied_data = self.spells_slice().to_vec();
        let old_len = copied_data.len();
        copied_data.retain(|spell| !spells.contains(spell));
        if copied_data.len() != old_len {
            self.copy_spell_list(&copied_data);
            true
        } else {
            false
        }
    }
}

#[repr(C)]
pub struct TESSpellList {
    pub base: BaseFormComponent,       // 00
    pub actor_effects: *mut SpellData, // 08
}

const _: () = assert!(core::mem::size_of::<TESSpellList>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESSpellList, actor_effects) == 0x08);

impl RttiType for TESSpellList {
    const RTTI: VariantID = RTTI_TESSpellList;
}

impl AsRef<TESSpellList> for TESSpellList {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TESSpellList> for TESSpellList {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(TESSpellList : BaseFormComponent);

impl TESSpellList {
    pub const RTTI: VariantID = RTTI_TESSpellList;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESSpellList;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03

    #[inline]
    pub fn spell_data(&self) -> Option<&SpellData> {
        unsafe { self.actor_effects.as_ref() }
    }

    #[inline]
    pub fn spell_data_mut(&mut self) -> Option<&mut SpellData> {
        unsafe { self.actor_effects.as_mut() }
    }
}

pub trait TESSpellListExt {
    fn spell_data(&self) -> Option<&SpellData>;
    fn spell_data_mut(&mut self) -> Option<&mut SpellData>;
}

impl<T: AsRef<TESSpellList> + AsMut<TESSpellList>> TESSpellListExt for T {
    #[inline(always)]
    fn spell_data(&self) -> Option<&SpellData> {
        self.as_ref().spell_data()
    }

    #[inline(always)]
    fn spell_data_mut(&mut self) -> Option<&mut SpellData> {
        TESSpellList::spell_data_mut(self.as_mut())
    }
}
