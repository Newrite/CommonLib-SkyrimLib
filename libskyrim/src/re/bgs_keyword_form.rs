use crate::offsets::offsets_rtti::RTTI_BGSKeywordForm;
use crate::offsets::offsets_vtable::VTABLE_BGSKeywordForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::BGSKeyword;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

/// C++ `RE::BGSKeywordForm`
#[repr(C)]
pub struct BGSKeywordForm {
    pub base: BaseFormComponent,        // 0x00
    pub keywords: *mut *mut BGSKeyword, // 0x08 - KWDA
    pub num_keywords: u32,              // 0x10 - KSIZ
    pub pad14: u32,                     // 0x14
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
        pub fn has_keyword(keyword: *const BGSKeyword) -> bool
    }

    virtual_method! {
        pub const GET_DEFAULT_KEYWORD: usize = 0x05;
        pub fn get_default_keyword() -> *mut BGSKeyword
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

    #[inline]
    pub fn get_default_keyword_ref(&self) -> Option<&BGSKeyword> {
        // SAFETY: get_default_keyword returns a valid pointer to a keyword or null
        unsafe { self.get_default_keyword().as_ref() }
    }
}

pub trait BGSKeywordFormExt {
    fn has_keyword(&self, keyword: *const BGSKeyword) -> bool;
    fn get_default_keyword_ref(&self) -> Option<&BGSKeyword>;
    fn get_num_keywords(&self) -> u32;
    fn get_keywords(&self) -> &[*mut BGSKeyword];
}

impl<T: AsRef<BGSKeywordForm>> BGSKeywordFormExt for T {
    fn has_keyword(&self, keyword: *const BGSKeyword) -> bool {
        self.as_ref().has_keyword(keyword)
    }

    fn get_default_keyword_ref(&self) -> Option<&BGSKeyword> {
        self.as_ref().get_default_keyword_ref()
    }

    fn get_num_keywords(&self) -> u32 {
        self.as_ref().get_num_keywords()
    }

    fn get_keywords(&self) -> &[*mut BGSKeyword] {
        self.as_ref().get_keywords()
    }
}

inherit!(BGSKeywordForm : BaseFormComponent);
