use core::ffi::{CStr, c_char};
use core::fmt;

/// Creates a `CStr` literal from a Rust string literal.
#[macro_export]
macro_rules! cstr {
    ( $str:literal ) => {
        $crate::core::ffi::CStr::from_bytes_until_nul($crate::core::concat!($str, "\0").as_bytes())
            .unwrap()
    };
}

/// Creates a UTF-16 `WideStr` literal from a Rust string literal.
#[macro_export]
macro_rules! wcstr {
    ( $str:literal ) => {{
        const SIZE: usize = $crate::get_utf16_len($str) + 1;
        $crate::WideStr::from_slice(&$crate::create_utf16_string::<SIZE>($str))
    }};
}

/// A UTF-16 C-style string slice. The underlying buffer must be NUL-terminated.
#[repr(transparent)]
pub struct WideStr([u16]);

impl WideStr {
    /// Creates a `WideStr` from a UTF-16 slice with a trailing NUL terminator.
    pub const fn from_slice(s: &[u16]) -> &Self {
        assert!(!s.is_empty());
        assert!(s[s.len() - 1] == 0);

        let mut i = 0;
        while i < s.len() - 1 {
            assert!(s[i] != 0);
            i += 1;
        }

        unsafe { &*(s as *const [u16] as *const Self) }
    }

    /// Creates a `WideStr` from a NUL-terminated UTF-16 pointer.
    ///
    /// # Safety
    /// `s` must point to a valid NUL-terminated UTF-16 string.
    pub unsafe fn from_ptr<'a>(s: *const u16) -> &'a Self {
        let mut wchars = 0;
        while unsafe { *s.add(wchars) } != 0 {
            wchars += 1;
        }

        Self::from_slice(unsafe { core::slice::from_raw_parts::<'a, u16>(s, wchars + 1) })
    }

    /// Returns the underlying UTF-16 pointer.
    pub const fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
}

/// A fixed-capacity UTF-16 formatting buffer for FFI.
pub struct WideStringBuffer<const SIZE: usize> {
    buf: [u16; SIZE],
    len: usize,
}

impl<const SIZE: usize> WideStringBuffer<SIZE> {
    /// Creates an empty buffer.
    pub const fn new() -> Self {
        Self {
            buf: [0; SIZE],
            len: 0,
        }
    }

    /// Returns the written portion as a `WideStr`.
    pub fn as_w_str(&self) -> &WideStr {
        WideStr::from_slice(self.buf.split_at(self.len + 1).0)
    }

    /// Appends a `WideStr` to the buffer.
    pub fn write_w_str(&mut self, s: &WideStr) -> Result<(), fmt::Error> {
        if s.0.len() + self.len > SIZE - 1 {
            return Err(fmt::Error);
        }

        self.buf
            .split_at_mut(self.len)
            .1
            .split_at_mut(s.0.len())
            .0
            .copy_from_slice(&s.0);
        self.len += s.0.len() - 1;
        Ok(())
    }
}

impl<const SIZE: usize> Default for WideStringBuffer<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> fmt::Write for WideStringBuffer<SIZE> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if s.encode_utf16().count() >= SIZE - self.len {
            return Err(fmt::Error);
        }

        for w in s.encode_utf16() {
            self.buf[self.len] = w;
            self.len += 1;
        }
        self.buf[self.len] = 0;

        Ok(())
    }
}

/// A fixed-capacity UTF-8 formatting buffer for FFI.
pub struct StringBuffer<const SIZE: usize> {
    buf: [u8; SIZE],
    len: usize,
}

impl<const SIZE: usize> StringBuffer<SIZE> {
    /// Creates an empty buffer.
    pub const fn new() -> Self {
        Self {
            buf: [0; SIZE],
            len: 0,
        }
    }

    /// Returns the written portion as a `CStr`.
    pub fn as_c_str(&self) -> &CStr {
        CStr::from_bytes_with_nul(self.buf.split_at(self.len + 1).0).unwrap()
    }
}

impl<const SIZE: usize> Default for StringBuffer<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> fmt::Write for StringBuffer<SIZE> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if s.len() + self.len > SIZE - 1 {
            return Err(fmt::Error);
        }

        self.buf
            .split_at_mut(self.len)
            .1
            .split_at_mut(s.len())
            .0
            .copy_from_slice(s.as_bytes());
        self.len += s.len();
        self.buf[self.len] = 0;
        Ok(())
    }
}

#[inline]
pub fn ptr_to_str<'a>(ptr: *const c_char) -> &'a str {
    if ptr.is_null() {
        "<null>"
    } else {
        unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .unwrap_or("<invalid utf8>")
    }
}

#[doc(hidden)]
pub const fn create_utf16_string<const DIM: usize>(s: &'static str) -> [u16; DIM] {
    let b = s.as_bytes();
    let mut ret = [0; DIM];

    let mut b_i: usize = 0;
    let mut w_i: usize = 0;
    while b_i < b.len() {
        if b[b_i] & 0x80 == 0 {
            ret[w_i] = b[b_i] as u16;
        } else if b[b_i] & 0xE0 == 0xC0 {
            assert!(b[b_i + 1] & 0xC0 == 0x80);
            ret[w_i] = (((b[b_i] & 0x1F) as u16) << 6) | ((b[b_i + 1] & 0x3F) as u16);
            b_i += 1;
        } else if b[b_i] & 0xF0 == 0xE0 {
            assert!(b[b_i + 1] & 0xC0 == 0x80);
            assert!(b[b_i + 2] & 0xC0 == 0x80);
            ret[w_i] = (((b[b_i] & 0x0F) as u16) << 12)
                | (((b[b_i + 1] & 0x3F) as u16) << 6)
                | ((b[b_i + 2] & 0x3F) as u16);
            assert!(ret[w_i] & 0xF800 != 0xD8);
            b_i += 2;
        } else {
            assert!(b[b_i] & 0xF8 == 0xF0);
            assert!(b[b_i + 1] & 0xC0 == 0x80);
            assert!(b[b_i + 2] & 0xC0 == 0x80);
            assert!(b[b_i + 3] & 0xC0 == 0x80);
            ret[w_i] = 0xD800
                | (((b[b_i] & 0x03) as u16) << 8)
                | (((b[b_i + 1] & 0x3F) as u16) << 2)
                | (((b[b_i + 2] & 0x30) as u16) >> 4);
            ret[w_i + 1] =
                0xDC00 | (((b[b_i + 2] & 0x0F) as u16) << 6) | ((b[b_i + 3] & 0x3F) as u16);
            w_i += 1;
            b_i += 3;
        }

        w_i += 1;
        b_i += 1;
    }

    assert!(w_i == DIM - 1);
    ret
}

#[doc(hidden)]
pub const fn get_utf16_len(s: &'static str) -> usize {
    let b = s.as_bytes();

    let mut code_points: usize = 0;
    let mut i: usize = 0;
    while i < b.len() {
        if b[i] & 0x80 == 0 {
            code_points += 1;
        } else if b[i] & 0xE0 == 0xC0 {
            code_points += 1;
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
        } else if b[i] & 0xF0 == 0xE0 {
            code_points += 1;
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
        } else {
            assert!(b[i] & 0xF8 == 0xF0);
            code_points += 1;
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
            i += 1;
            assert!(b[i] & 0xC0 == 0x80);
        }
        i += 1;
    }

    code_points
}
