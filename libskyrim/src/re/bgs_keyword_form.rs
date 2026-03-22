use crate::offsets::offsets_rtti::RTTI_BGSKeywordForm;
use crate::offsets::offsets_vtable::VTABLE_BGSKeywordForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::BGSKeyword;
use crate::relocation::{VariantID, RttiType};
use core_util::inherit;
use crate::virtual_method;

/// C++ `RE::BGSKeywordForm`
#[repr(C)]
pub struct BGSKeywordForm {
    pub base: BaseFormComponent,  // 00
    pub keywords: *mut *mut BGSKeyword, // 08 - KWDA
    pub num_keywords: u32,        // 10 - KSIZ
    pub pad14: u32,               // 14
}

const _: () = assert!(core::mem::size_of::<BGSKeywordForm>() == 0x18);

impl RttiType for BGSKeywordForm {
    const RTTI: VariantID = RTTI_BGSKeywordForm;
}

impl BGSKeywordForm {
    pub const RTTI: VariantID = RTTI_BGSKeywordForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSKeywordForm;

    virtual_method! {
        pub const HAS_KEYWORD: usize = 0x04;
        pub fn has_keyword(this: &BGSKeywordForm, keyword: *const BGSKeyword) -> bool
    }

    virtual_method! {
        pub const GET_DEFAULT_KEYWORD: usize = 0x05;
        pub fn get_default_keyword(this: &BGSKeywordForm) -> *mut BGSKeyword
    }
    
    pub fn get_num_keywords(&self) -> u32 {
        self.num_keywords
    }
    
    pub fn get_keywords(&self) -> &[*mut BGSKeyword] {
        if self.keywords.is_null() || self.num_keywords == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.keywords, self.num_keywords as usize) }
        }
    }
}

inherit!(BGSKeywordForm : BaseFormComponent);