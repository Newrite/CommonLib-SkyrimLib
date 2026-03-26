/// C++ `RE::NiRect<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NiRect<T> {
    pub left: T,   // 00
    pub right: T,  // 04
    pub top: T,    // 08
    pub bottom: T, // 0C
}

const _: () = assert!(core::mem::size_of::<NiRect<f32>>() == 0x10);

impl<T> NiRect<T>
where
    T: Copy + PartialOrd + core::ops::Sub<Output = T>,
{
    #[inline(always)]
    pub const fn new(left: T, right: T, top: T, bottom: T) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    #[inline(always)]
    pub fn get_width(&self) -> T {
        if self.right > self.left {
            self.right - self.left
        } else {
            self.left - self.right
        }
    }

    #[inline(always)]
    pub fn get_height(&self) -> T {
        if self.top > self.bottom {
            self.top - self.bottom
        } else {
            self.bottom - self.top
        }
    }
}
