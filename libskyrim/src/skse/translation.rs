use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec::Vec;

/// Thin Rust facade over `SKSE::Translation`.
pub struct Translation;

impl Translation {
    #[inline(always)]
    pub fn parse_translation(name: &str) {
        let Ok(name) = CString::new(name) else {
            return;
        };

        unsafe {
            crate::ffi::commonlib_skse_translation_parse_translation(name.as_ptr());
        }
    }

    #[inline(always)]
    pub fn translate(key: &str) -> Option<String> {
        let key = CString::new(key).ok()?;
        let required = unsafe {
            crate::ffi::commonlib_skse_translation_translate(key.as_ptr(), core::ptr::null_mut(), 0)
        };
        if required == 0 {
            return None;
        }

        let mut buffer = Vec::<u8>::with_capacity(required);
        let written = unsafe {
            crate::ffi::commonlib_skse_translation_translate(
                key.as_ptr(),
                buffer.as_mut_ptr().cast(),
                required,
            )
        };
        if written == 0 {
            return None;
        }

        unsafe {
            buffer.set_len(written.saturating_sub(1));
        }

        String::from_utf8(buffer).ok()
    }
}

#[inline(always)]
pub fn parse_translation(name: &str) {
    Translation::parse_translation(name);
}

#[inline(always)]
pub fn translate(key: &str) -> Option<String> {
    Translation::translate(key)
}
