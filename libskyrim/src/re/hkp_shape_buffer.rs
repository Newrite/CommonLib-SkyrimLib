#![allow(non_camel_case_types)]

use core::ops::{Index, IndexMut};

/// C++ `RE::hkpShapeBuffer`
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct hkpShapeBuffer {
    pub buf: [u8; 512], // 00
}

const _: () = assert!(core::mem::size_of::<hkpShapeBuffer>() == 0x200);
const _: () = assert!(core::mem::align_of::<hkpShapeBuffer>() == 0x10);
const _: () = assert!(core::mem::offset_of!(hkpShapeBuffer, buf) == 0x00);

impl Default for hkpShapeBuffer {
    #[inline(always)]
    fn default() -> Self {
        Self { buf: [0; 512] }
    }
}

impl Index<usize> for hkpShapeBuffer {
    type Output = u8;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl IndexMut<usize> for hkpShapeBuffer {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}
