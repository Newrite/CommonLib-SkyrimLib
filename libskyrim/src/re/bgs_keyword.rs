use crate::offsets::offsets_rtti::RTTI_BGSKeyword;
use crate::offsets::offsets_vtable::VTABLE_BGSKeyword;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::i_form_factory::IFormFactory;
use crate::re::tes_form::TESForm;
use crate::relocation::{RttiType, VariantID};
use bitflags::bitflags;
use core_util::inherit;

/// C++ `RE::BGSKeyword`
#[repr(C)]
pub struct BGSKeyword {
    pub base: TESForm,                 // 00
    pub form_editor_id: BSFixedString, // 20
}

const _: () = assert!(core::mem::size_of::<BGSKeyword>() == 0x28);

impl RttiType for BGSKeyword {
    const RTTI: VariantID = RTTI_BGSKeyword;
}

impl FormCastable for BGSKeyword {
    const TARGET_FORM_TYPE: FormType = FormType::Keyword;
}

bitflags! {
    /// BGSKeyword::RecordFlags
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct KeywordRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

inherit!(BGSKeyword : TESForm);

impl BGSKeyword {
    pub const RTTI: VariantID = RTTI_BGSKeyword;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSKeyword;

    #[inline(always)]
    pub fn get_form_editor_id_as_str(&self) -> &str {
        self.form_editor_id.as_str()
    }

    #[inline(always)]
    pub fn create_keyword_from_str(form_editor_id: &str) -> *mut BGSKeyword {
        Self::create_keyword(&BSFixedString::from_str(form_editor_id))
    }

    pub fn create_keyword(form_editor_id: &BSFixedString) -> *mut BGSKeyword {
        let factory = IFormFactory::get_form_factory_by_type(FormType::Keyword);
        if !factory.is_null() {
            unsafe {
                let form = (*factory).create_impl();
                if !form.is_null() {
                    let keyword = form as *mut BGSKeyword;
                    (*keyword).form_editor_id = form_editor_id.clone();
                    return keyword;
                }
            }
        }
        core::ptr::null_mut()
    }
}
