use crate::relocation_variable;
use crate::relocation::VariantID;
use crate::re::FormType;

#[repr(C)]
pub struct FormEnumString {
    pub form_type: FormType,    // 00
    pub form_string: *const i8, // 08 (char*)
    pub form_code: u32,         // 10
}

const _: () = assert!(core::mem::size_of::<FormEnumString>() == 0x18);

impl FormEnumString {
    relocation_variable! {
        pub fn get_form_enum_string_array() -> &'static [FormEnumString; 138]
            => VariantID::new(501008, 359120, 0)
    }

    #[inline]
    pub const fn get_code_bytes(&self) -> [u8; 4] {
        self.form_code.to_le_bytes()
    }

    pub fn format_code(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = self.get_code_bytes();
        for &b in &bytes {
            // Если байт печатаемый — выводим как символ, иначе — как точку
            let c = if b >= 32 && b <= 126 { b as char } else { '.' };
            write!(f, "{}", c)?;
        }
        Ok(())
    }

    #[inline]
    pub fn get_all() -> &'static [FormEnumString] {
        Self::get_form_enum_string_array().as_slice()
    }
}

// Чтобы можно было просто писать: println!("{}", entry);
impl core::fmt::Display for FormEnumString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::ffi::CStr;
        let name = unsafe {
            if self.form_string.is_null() {
                "None"
            } else {
                CStr::from_ptr(self.form_string).to_str().unwrap_or("Invalid")
            }
        };

        write!(f, "[{}: ", name)?;
        self.format_code(f)?;
        write!(f, "]")
    }
}