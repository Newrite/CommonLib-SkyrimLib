use alloc::string::String;
use alloc::vec::Vec;

use core::ffi::{CStr, c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr::NonNull;

use core_util::WideStr;

core_util::abstract_type! {
    /// The opaque C standard library file handle.
    type FILE;
}

#[link(name = "msvcrt")]
unsafe extern "C" {
    fn fopen(filename: *const c_char, mode: *const c_char) -> Option<NonNull<FILE>>;
    fn _wfopen(filename: *const u16, mode: *const u16) -> Option<NonNull<FILE>>;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: usize, count: usize, stream: *mut FILE) -> usize;
    fn fwrite(ptr: *const c_void, size: usize, count: usize, stream: *mut FILE) -> usize;
    fn fgets(ptr: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
    fn fseek(stream: *mut FILE, offset: c_long, origin: c_int) -> c_int;
    fn ftell(stream: *mut FILE) -> c_long;
    fn ferror(stream: *mut FILE) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
}

/// A non-null wrapper around a C `FILE*`.
#[repr(transparent)]
pub struct File(NonNull<FILE>);

/// Seek origin and offset used by [`File::seek`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seek {
    Set(i64),
    Current(i64),
    End(i64),
}

/// The UTF-16 little-endian byte order mark.
pub const UTF16LE_BOM: [u8; 2] = [0xff, 0xfe];

/// The UTF-8 byte order mark.
pub const UTF8_BOM: [u8; 3] = [0xef, 0xbb, 0xbf];

impl File {
    /// Opens a file using narrow C strings for both path and mode.
    pub fn open(path: &CStr, mode: &CStr) -> Result<File, ()> {
        unsafe { fopen(path.as_ptr(), mode.as_ptr()).map(Self).ok_or(()) }
    }

    /// Opens a file using UTF-16 path and mode strings.
    pub fn wopen(path: &WideStr, mode: &WideStr) -> Result<File, ()> {
        unsafe { _wfopen(path.as_ptr(), mode.as_ptr()).map(Self).ok_or(()) }
    }

    /// Reads elements into `data` and returns the number of elements read.
    pub fn read<T: Copy>(&mut self, data: &mut [T]) -> Result<usize, ()> {
        unsafe {
            let ret = fread(
                data.as_mut_ptr().cast(),
                size_of::<T>(),
                data.len(),
                self.0.as_ptr(),
            );

            if ferror(self.0.as_ptr()) == 0 {
                Ok(ret)
            } else {
                Err(())
            }
        }
    }

    /// Writes elements from `data`.
    ///
    /// On failure, returns the number of elements successfully written.
    pub fn write<T: Copy>(&mut self, data: &[T]) -> Result<(), usize> {
        unsafe {
            let ret = fwrite(
                data.as_ptr().cast(),
                size_of::<T>(),
                data.len(),
                self.0.as_ptr(),
            );
            if ret != data.len() { Err(ret) } else { Ok(()) }
        }
    }

    /// Reads a single line into the provided byte buffer.
    pub fn gets(&mut self, data: &mut [u8]) -> Result<(), ()> {
        unsafe {
            let ret = fgets(
                data.as_mut_ptr().cast(),
                data.len().try_into().unwrap(),
                self.0.as_ptr(),
            );
            if ret.is_null() { Err(()) } else { Ok(()) }
        }
    }

    /// Seeks to a new file position.
    pub fn seek(&mut self, seek: Seek) -> Result<(), ()> {
        const SEEK_SET: c_int = 0;
        const SEEK_CUR: c_int = 1;
        const SEEK_END: c_int = 2;

        unsafe {
            if 0 == match seek {
                Seek::Set(off) => fseek(self.0.as_ptr(), off.try_into().unwrap(), SEEK_SET),
                Seek::Current(off) => fseek(self.0.as_ptr(), off.try_into().unwrap(), SEEK_CUR),
                Seek::End(off) => fseek(self.0.as_ptr(), off.try_into().unwrap(), SEEK_END),
            } {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    /// Returns the current stream position.
    pub fn pos(&mut self) -> Result<i64, ()> {
        unsafe {
            let ret = ftell(self.0.as_ptr()) as i64;
            if ret < 0 { Err(()) } else { Ok(ret) }
        }
    }

    /// Flushes the stream.
    pub fn flush(&mut self) -> Result<(), ()> {
        unsafe {
            if fflush(self.0.as_ptr()) == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    /// Reads the entire file into a `Vec<u8>`.
    pub fn into_vec(mut self) -> Result<Vec<u8>, ()> {
        let mut ret = Vec::new();
        self.seek(Seek::End(0))?;
        ret.resize(self.pos()? as usize, 0);
        self.seek(Seek::Set(0))?;
        self.read(ret.as_mut_slice())?;
        ret.shrink_to_fit();
        Ok(ret)
    }

    /// Reads the entire file into a `String`.
    pub fn into_string(self) -> Result<String, ()> {
        let bytes = self.into_vec()?;
        if bytes.len() >= UTF16LE_BOM.len() && bytes[0..UTF16LE_BOM.len()] == UTF16LE_BOM {
            Ok(String::from_utf16_lossy(unsafe {
                core::slice::from_raw_parts(
                    bytes.as_ptr().cast::<u16>().add(1),
                    bytes.len() / 2 - 1,
                )
            }))
        } else if bytes.len() >= UTF8_BOM.len() && bytes[0..UTF8_BOM.len()] == UTF8_BOM {
            Ok(String::from(String::from_utf8_lossy(
                &bytes[UTF8_BOM.len()..],
            )))
        } else {
            match String::from_utf8(bytes) {
                Ok(s) => Ok(s),
                Err(err) => {
                    let bytes = err.as_bytes();
                    String::from_utf16(unsafe {
                        core::slice::from_raw_parts(bytes.as_ptr().cast(), bytes.len() / 2)
                    })
                    .map_err(|_| ())
                }
            }
        }
    }
}

impl core::fmt::Write for File {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes()).map_err(|_| core::fmt::Error)?;
        self.flush().map_err(|_| core::fmt::Error)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            assert!(fclose(self.0.as_ptr()) == 0);
        }
    }
}
