use crate::re::bgs_keyword_form::BGSKeywordForm;
use crate::re::bgs_keyword::BGSKeyword;

pub trait BGSKeywordFormExt {
    fn has_keyword(&self, keyword: *const BGSKeyword) -> bool;
    fn get_default_keyword(&self) -> *mut BGSKeyword;
}

impl<T: AsRef<BGSKeywordForm>> BGSKeywordFormExt for T {
    fn has_keyword(&self, keyword: *const BGSKeyword) -> bool {
        self.as_ref().has_keyword(self.as_ref(), keyword)
    }

    fn get_default_keyword(&self) -> *mut BGSKeyword {
        self.as_ref().get_default_keyword(self.as_ref())
    }
}
