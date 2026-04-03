use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSListForm;
use crate::offsets::offsets_vtable::VTABLE_BGSListForm;
use crate::re::bs_container::BSContainerForEachResult;
use crate::re::bs_core_types::FormID;
use crate::re::bst_array::BSTArray;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_form::TESForm;
use crate::relocation::{RelocationID, RttiType, VariantID};
use bitflags::bitflags;

bitflags! {
    /// C++ `RE::BGSListForm::ChangeFlags::ChangeFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSListFormChangeFlags: u32 {
        const ADDED_FORM = 1 << 31;
    }
}

bitflags! {
    /// C++ `RE::BGSListForm::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSListFormRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::BGSListForm`
#[repr(C)]
pub struct BGSListForm {
    pub base: TESForm,                                  // 00
    pub forms: BSTArray<*mut TESForm>,                  // 20 - LNAM
    pub script_added_temp_forms: *mut BSTArray<FormID>, // 38
    pub script_added_form_count: u32,                   // 40
    pub pad44: u32,                                     // 44
}

const _: () = assert!(core::mem::size_of::<BGSListForm>() == 0x48);
const _: () = assert!(core::mem::offset_of!(BGSListForm, forms) == 0x20);

impl RttiType for BGSListForm {
    const RTTI: VariantID = RTTI_BGSListForm;
}

impl FormCastable for BGSListForm {
    const TARGET_FORM_TYPE: FormType = FormType::FormList;
}

inherit!(BGSListForm : TESForm);

impl BGSListForm {
    pub const RTTI: VariantID = RTTI_BGSListForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSListForm;
    pub const FORMTYPE: FormType = FormType::FormList;

    // override (TESForm)
    // void ClearData() override;                         // 05
    // bool Load(TESFile* a_mod) override;                // 06
    // void SaveGame(BGSSaveFormBuffer* a_buf) override;  // 0E
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;  // 0F
    // void Revert(BGSLoadFormBuffer* a_buf) override;    // 12
    // void InitItemImpl() override;                      // 13

    // RELOCATION_ID SE: 20470, AE: 20913
    crate::relocation_func! {
        pub fn add_form(&mut self, form: *mut TESForm) => RelocationID::new(20470, 20913)
    }

    #[inline]
    pub fn forms_slice(&self) -> &[*mut TESForm] {
        unsafe { self.forms.as_slice() }
    }

    #[inline]
    pub fn script_added_temp_forms_slice(&self) -> &[FormID] {
        if self.script_added_temp_forms.is_null() {
            &[]
        } else {
            unsafe { (*self.script_added_temp_forms).as_slice() }
        }
    }

    pub fn for_each_form<F>(&self, mut callback: F)
    where
        F: FnMut(*mut TESForm) -> BSContainerForEachResult,
    {
        for &form in self.forms_slice() {
            if !form.is_null() && callback(form).is_stop() {
                return;
            }
        }

        for &added_form_id in self.script_added_temp_forms_slice() {
            let Some(added_form) = TESForm::lookup_by_id(added_form_id) else {
                continue;
            };
            if callback(added_form).is_stop() {
                return;
            }
        }
    }

    pub fn contains_only_type(&self, form_type: FormType) -> bool {
        let mut result = true;
        self.for_each_form(|form| {
            let matches = unsafe { (*form).get_form_type() == form_type };
            if !matches {
                result = false;
                BSContainerForEachResult::Stop
            } else {
                BSContainerForEachResult::Continue
            }
        });
        result
    }

    pub fn has_form(&self, form: *const TESForm) -> bool {
        if form.is_null() {
            return false;
        }

        if self
            .forms_slice()
            .iter()
            .any(|&entry| core::ptr::eq(entry as *const TESForm, form))
        {
            return true;
        }

        let form_id = unsafe { (*form).get_form_id() };
        self.script_added_temp_forms_slice().contains(&form_id)
    }

    #[inline]
    pub fn has_form_id(&self, form_id: FormID) -> bool {
        TESForm::lookup_by_id(form_id)
            .map(|form| self.has_form(form.cast_const()))
            .unwrap_or(false)
    }
}

impl AsRef<BGSListForm> for BGSListForm {
    #[inline(always)]
    fn as_ref(&self) -> &BGSListForm {
        self
    }
}

impl AsMut<BGSListForm> for BGSListForm {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut BGSListForm {
        self
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::BGSListForm;
    use crate::re::base_form_component::BaseFormComponent;
    use crate::re::bst_array::BSTArray;
    use crate::re::{FormType, InGameFormFlag, RecordFlag, TESFileContainer, TESForm};
    use core_util::EnumSet;

    fn test_form(form_id: u32, form_type: FormType) -> TESForm {
        TESForm {
            base: BaseFormComponent {
                vtable: core::ptr::null(),
            },
            source_files: TESFileContainer {
                array: core::ptr::null_mut(),
            },
            form_flags: RecordFlag::empty(),
            form_id,
            in_game_form_flags: EnumSet::<InGameFormFlag, u16>::from_underlying(0),
            form_type: EnumSet::from(form_type),
            pad1b: 0,
            pad1c: 0,
        }
    }

    fn list_from_forms(forms: impl IntoIterator<Item = *mut TESForm>) -> BGSListForm {
        BGSListForm {
            base: test_form(0, FormType::FormList),
            forms: forms.into_iter().collect::<BSTArray<*mut TESForm>>(),
            script_added_temp_forms: core::ptr::null_mut(),
            script_added_form_count: 0,
            pad44: 0,
        }
    }

    #[test]
    fn forms_slice_has_form_and_iteration_skip_null_entries() {
        let mut spell = test_form(0x300, FormType::Spell);
        let mut keyword = test_form(0x301, FormType::Keyword);
        let list = list_from_forms([
            &mut spell as *mut TESForm,
            core::ptr::null_mut(),
            &mut keyword as *mut TESForm,
        ]);

        assert_eq!(list.forms_slice().len(), 3);
        assert!(list.has_form(&spell));
        assert!(list.has_form(&keyword));

        let mut visited = Vec::new();
        list.for_each_form(|form| {
            visited.push(unsafe { (&*form).form_id });
            super::BSContainerForEachResult::Continue
        });

        assert_eq!(visited, [0x300, 0x301]);
    }

    #[test]
    fn contains_only_type_checks_live_entries() {
        let mut spell = test_form(0x400, FormType::Spell);
        let mut second_spell = test_form(0x401, FormType::Spell);
        let mut keyword = test_form(0x402, FormType::Keyword);

        let list = list_from_forms([
            &mut spell as *mut TESForm,
            core::ptr::null_mut(),
            &mut second_spell as *mut TESForm,
        ]);
        assert!(list.contains_only_type(FormType::Spell));

        let mixed = list_from_forms([&mut spell as *mut TESForm, &mut keyword as *mut TESForm]);
        assert!(!mixed.contains_only_type(FormType::Spell));
    }
}
