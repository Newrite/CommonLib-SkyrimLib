use core::ffi::c_void;

use crate::offsets::offsets_rtti::RTTI_BGSKeywordForm;
use crate::offsets::offsets_vtable::VTABLE_BGSKeywordForm;
use crate::re::BGSKeyword;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bs_container::BSContainerForEachResult;
use crate::re::tes_form::FormID;
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

    pub fn add_keyword(&mut self, keyword: *mut BGSKeyword) -> bool {
        if self.get_keyword_index(keyword).is_some() {
            return false;
        }

        let mut copied_data = self.get_keywords().to_vec();
        copied_data.push(keyword);
        self.copy_keywords(&copied_data);
        true
    }

    pub fn add_keywords(&mut self, keywords: &[*mut BGSKeyword]) -> bool {
        let mut copied_data = self.get_keywords().to_vec();
        for &keyword in keywords {
            if !copied_data.contains(&keyword) {
                copied_data.push(keyword);
            }
        }
        self.copy_keywords(&copied_data);
        true
    }

    pub fn contains_keyword_string(&self, editor_id: &str) -> bool {
        let mut result = false;
        self.for_each_keyword(|keyword| {
            let current = unsafe { &*keyword };
            if current.get_form_editor_id_as_str().contains(editor_id) {
                result = true;
                BSContainerForEachResult::Stop
            } else {
                BSContainerForEachResult::Continue
            }
        });
        result
    }

    pub fn for_each_keyword<F>(&self, mut callback: F)
    where
        F: FnMut(*mut BGSKeyword) -> BSContainerForEachResult,
    {
        if self.keywords.is_null() {
            return;
        }

        for idx in 0..self.num_keywords as usize {
            let keyword = unsafe { *self.keywords.add(idx) };
            if keyword.is_null() {
                continue;
            }

            if callback(keyword).is_stop() {
                return;
            }
        }
    }

    pub fn get_keyword_at(&self, idx: u32) -> Option<*mut BGSKeyword> {
        self.get_keywords().get(idx as usize).copied()
    }

    #[inline]
    pub fn get_keyword_ref_at(&self, idx: u32) -> Option<&BGSKeyword> {
        unsafe { self.get_keyword_at(idx)?.as_ref() }
    }

    pub fn get_keyword_index(&self, keyword: *mut BGSKeyword) -> Option<u32> {
        self.get_keywords()
            .iter()
            .position(|&current| current == keyword)
            .map(|idx| idx as u32)
    }

    #[inline]
    pub fn get_default_keyword_ref(&self) -> Option<&BGSKeyword> {
        // SAFETY: get_default_keyword returns a valid pointer to a keyword or null
        unsafe { self.get_default_keyword().as_ref() }
    }

    pub fn has_keyword_id(&self, form_id: FormID) -> bool {
        let mut result = false;
        self.for_each_keyword(|keyword| {
            if unsafe { (&*keyword).form_id } == form_id {
                result = true;
                BSContainerForEachResult::Stop
            } else {
                BSContainerForEachResult::Continue
            }
        });
        result
    }

    pub fn has_keyword_string(&self, editor_id: &str) -> bool {
        let mut result = false;
        self.for_each_keyword(|keyword| {
            let current = unsafe { &*keyword };
            if current.get_form_editor_id_as_str() == editor_id {
                result = true;
                BSContainerForEachResult::Stop
            } else {
                BSContainerForEachResult::Continue
            }
        });
        result
    }

    pub fn remove_keyword_at(&mut self, index: u32) -> bool {
        if index as usize >= self.num_keywords as usize {
            return false;
        }

        let mut copied_data = self.get_keywords().to_vec();
        copied_data.remove(index as usize);
        self.copy_keywords(&copied_data);
        true
    }

    pub fn remove_keyword(&mut self, keyword: *mut BGSKeyword) -> bool {
        match self.get_keyword_index(keyword) {
            Some(index) => self.remove_keyword_at(index),
            None => false,
        }
    }

    pub fn remove_keywords(&mut self, keywords: &[*mut BGSKeyword]) -> bool {
        let mut copied_data = self.get_keywords().to_vec();
        let old_len = copied_data.len();
        copied_data.retain(|keyword| !keywords.contains(keyword));
        if copied_data.len() != old_len {
            self.copy_keywords(&copied_data);
            true
        } else {
            false
        }
    }

    fn copy_keywords(&mut self, copied_data: &[*mut BGSKeyword]) {
        let old_data = self.keywords;
        let new_size = copied_data.len();
        let new_data = if new_size == 0 {
            core::ptr::null_mut()
        } else {
            unsafe {
                crate::ffi::commonlib_calloc(new_size, core::mem::size_of::<*mut BGSKeyword>())
                    as *mut *mut BGSKeyword
            }
        };

        if new_size != 0 {
            assert!(
                !new_data.is_null(),
                "BGSKeywordForm::copy_keywords allocation failed"
            );
            unsafe {
                core::ptr::copy_nonoverlapping(copied_data.as_ptr(), new_data, new_size);
            }
        }

        self.num_keywords = new_size as u32;
        self.keywords = new_data;

        if !old_data.is_null() {
            unsafe {
                crate::ffi::commonlib_free(old_data.cast::<c_void>());
            }
        }
    }
}

pub trait BGSKeywordFormExt {
    fn add_keyword(&mut self, keyword: *mut BGSKeyword) -> bool;
    fn add_keywords(&mut self, keywords: &[*mut BGSKeyword]) -> bool;
    fn contains_keyword_string(&self, editor_id: &str) -> bool;
    fn for_each_keyword<F>(&self, callback: F)
    where
        F: FnMut(*mut BGSKeyword) -> BSContainerForEachResult;
    fn get_default_keyword(&self) -> *mut BGSKeyword;
    fn get_keyword_at(&self, idx: u32) -> Option<*mut BGSKeyword>;
    fn get_keyword_ref_at(&self, idx: u32) -> Option<&BGSKeyword>;
    fn get_keyword_index(&self, keyword: *mut BGSKeyword) -> Option<u32>;
    fn has_keyword(&self, keyword: *const BGSKeyword) -> bool;
    fn has_keyword_id(&self, form_id: FormID) -> bool;
    fn has_keyword_string(&self, editor_id: &str) -> bool;
    fn get_default_keyword_ref(&self) -> Option<&BGSKeyword>;
    fn get_num_keywords(&self) -> u32;
    fn get_keywords(&self) -> &[*mut BGSKeyword];
    fn remove_keyword_at(&mut self, index: u32) -> bool;
    fn remove_keyword(&mut self, keyword: *mut BGSKeyword) -> bool;
    fn remove_keywords(&mut self, keywords: &[*mut BGSKeyword]) -> bool;
}

impl<T: AsRef<BGSKeywordForm> + AsMut<BGSKeywordForm>> BGSKeywordFormExt for T {
    fn add_keyword(&mut self, keyword: *mut BGSKeyword) -> bool {
        self.as_mut().add_keyword(keyword)
    }

    fn add_keywords(&mut self, keywords: &[*mut BGSKeyword]) -> bool {
        self.as_mut().add_keywords(keywords)
    }

    fn contains_keyword_string(&self, editor_id: &str) -> bool {
        self.as_ref().contains_keyword_string(editor_id)
    }

    fn for_each_keyword<F>(&self, callback: F)
    where
        F: FnMut(*mut BGSKeyword) -> BSContainerForEachResult,
    {
        self.as_ref().for_each_keyword(callback)
    }

    fn get_default_keyword(&self) -> *mut BGSKeyword {
        self.as_ref().get_default_keyword()
    }

    fn get_keyword_at(&self, idx: u32) -> Option<*mut BGSKeyword> {
        self.as_ref().get_keyword_at(idx)
    }

    fn get_keyword_ref_at(&self, idx: u32) -> Option<&BGSKeyword> {
        self.as_ref().get_keyword_ref_at(idx)
    }

    fn get_keyword_index(&self, keyword: *mut BGSKeyword) -> Option<u32> {
        self.as_ref().get_keyword_index(keyword)
    }

    fn has_keyword(&self, keyword: *const BGSKeyword) -> bool {
        self.as_ref().has_keyword(keyword)
    }

    fn has_keyword_id(&self, form_id: FormID) -> bool {
        self.as_ref().has_keyword_id(form_id)
    }

    fn has_keyword_string(&self, editor_id: &str) -> bool {
        self.as_ref().has_keyword_string(editor_id)
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

    fn remove_keyword_at(&mut self, index: u32) -> bool {
        self.as_mut().remove_keyword_at(index)
    }

    fn remove_keyword(&mut self, keyword: *mut BGSKeyword) -> bool {
        self.as_mut().remove_keyword(keyword)
    }

    fn remove_keywords(&mut self, keywords: &[*mut BGSKeyword]) -> bool {
        self.as_mut().remove_keywords(keywords)
    }
}

inherit!(BGSKeywordForm : BaseFormComponent);
