use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSListForm;
use crate::offsets::offsets_vtable::VTABLE_BGSListForm;
use crate::re::bst_array::BSTArray;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_form::{FormID, TESForm};
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
        F: FnMut(*mut TESForm) -> bool,
    {
        for &form in self.forms_slice() {
            if !form.is_null() && !callback(form) {
                return;
            }
        }

        for &added_form_id in self.script_added_temp_forms_slice() {
            let Some(added_form) = TESForm::lookup_by_id(added_form_id) else {
                continue;
            };
            if !callback(added_form) {
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
            }
            matches
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
