//! Translation of `RE::BSSimpleList.h`.
//!
//! C++ layout:
//! - `BSSimpleList<T>::Node { T item; Node* next; }`
//! - `BSSimpleList<T> { Node _listHead; }`

use core::marker::PhantomData;
use core::ptr;

/// C++ `RE::BSSimpleList<T>::Node`.
#[repr(C)]
#[derive(Debug)]
pub struct BSSimpleListNode<T> {
    pub item: T,                        // 00
    pub next: *mut BSSimpleListNode<T>, // 08 (for pointer-sized `T`)
}

const _: () = assert!(core::mem::size_of::<BSSimpleListNode<*const u8>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSSimpleListNode<*const u8>, item) == 0x0);
const _: () = assert!(core::mem::offset_of!(BSSimpleListNode<*const u8>, next) == 0x8);

/// C++ `RE::BSSimpleList<T>` (forward list with embedded head node).
#[repr(C)]
#[derive(Debug)]
pub struct BSSimpleList<T> {
    pub list_head: BSSimpleListNode<T>, // 00
}

const _: () = assert!(core::mem::size_of::<BSSimpleList<*const u8>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSSimpleList<*const u8>, list_head) == 0x0);

/// Mirrors C++ `static_cast<bool>(item)` checks used by `BSSimpleList::empty()`.
pub trait BSSimpleListValue {
    fn bs_has_value(&self) -> bool;
}

impl<T> BSSimpleListValue for *const T {
    #[inline(always)]
    fn bs_has_value(&self) -> bool {
        !self.is_null()
    }
}

impl<T> BSSimpleListValue for *mut T {
    #[inline(always)]
    fn bs_has_value(&self) -> bool {
        !self.is_null()
    }
}

macro_rules! impl_bssimple_list_value_int {
    ($($ty:ty),* $(,)?) => {
        $(
            impl BSSimpleListValue for $ty {
                #[inline(always)]
                fn bs_has_value(&self) -> bool {
                    *self != 0
                }
            }
        )*
    };
}

impl_bssimple_list_value_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl BSSimpleListValue for bool {
    #[inline(always)]
    fn bs_has_value(&self) -> bool {
        *self
    }
}

impl<T: BSSimpleListValue> BSSimpleList<T> {
    /// C++ `empty()`.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.list_head.next.is_null() && !self.list_head.item.bs_has_value()
    }

    /// C++ `empty()`.
    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.is_empty()
    }

    /// C++ `size()`.
    #[inline(always)]
    pub fn len(&self) -> u32 {
        if self.is_empty() {
            return 0;
        }

        let mut count = 0u32;
        let mut node = self.head();
        while !node.is_null() {
            count += 1;
            // SAFETY: `node` comes from the embedded chain; null terminates.
            node = unsafe { (*node).next };
        }

        count
    }

    /// C++ `size()`.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        self.len()
    }

    #[inline(always)]
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.list_head.item)
        }
    }

    #[inline(always)]
    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let mut node = self.head();
        while let Some(next) =
            unsafe { node.as_ref() }.and_then(|node| (!node.next.is_null()).then_some(node.next))
        {
            node = next;
        }

        unsafe { node.as_ref() }.map(|node| &node.item)
    }

    #[inline(always)]
    pub fn iter(&self) -> BSSimpleListIter<'_, T> {
        let current = if self.is_empty() {
            ptr::null()
        } else {
            self.head()
        };

        BSSimpleListIter {
            current,
            _marker: PhantomData,
        }
    }
}

impl<T> BSSimpleList<T> {
    #[inline(always)]
    pub const fn head(&self) -> *const BSSimpleListNode<T> {
        &self.list_head
    }

    #[inline(always)]
    pub fn head_mut(&mut self) -> *mut BSSimpleListNode<T> {
        &mut self.list_head
    }
}

impl<T: Default> Default for BSSimpleListNode<T> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            item: T::default(),
            next: ptr::null_mut(),
        }
    }
}

impl<T: Default> Default for BSSimpleList<T> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            list_head: BSSimpleListNode::default(),
        }
    }
}

pub struct BSSimpleListIter<'a, T> {
    current: *const BSSimpleListNode<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for BSSimpleListIter<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        // SAFETY: iterator traverses a null-terminated engine list.
        let node = unsafe { &*self.current };
        self.current = node.next;
        Some(&node.item)
    }
}
