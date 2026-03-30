//! Translation of `RE::BSTTuple.h`.
//!
//! `BSTTuple<T1, T2>` is the engine's equivalent of `std::pair`.
//! Used as the value type in `BSTHashMap<K, V>` (stores `BSTTuple<const K, V>`).

use core::cmp::Ordering;

/// C++ `RE::BSTTuple<T1, T2>`.
///
/// A simple pair struct matching the engine's layout.
/// Equivalent to `std::pair<T1, T2>` in C++.
#[repr(C)]
pub struct BSTTuple<T1, T2> {
    pub first: T1,  // 0x00
    pub second: T2, // sizeof(T1) (with alignment)
}

impl<T1, T2> BSTTuple<T1, T2> {
    /// Creates a new tuple.
    #[must_use]
    #[inline(always)]
    pub const fn new(first: T1, second: T2) -> Self {
        Self { first, second }
    }

    /// Swaps the pair contents with another tuple.
    #[inline]
    pub fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self, other);
    }
}

impl<T1: Default, T2: Default> Default for BSTTuple<T1, T2> {
    fn default() -> Self {
        Self {
            first: T1::default(),
            second: T2::default(),
        }
    }
}

impl<T1: Clone, T2: Clone> Clone for BSTTuple<T1, T2> {
    fn clone(&self) -> Self {
        Self {
            first: self.first.clone(),
            second: self.second.clone(),
        }
    }
}

impl<T1: Copy, T2: Copy> Copy for BSTTuple<T1, T2> {}

impl<T1: PartialEq, T2: PartialEq> PartialEq for BSTTuple<T1, T2> {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

impl<T1: Eq, T2: Eq> Eq for BSTTuple<T1, T2> {}

impl<T1: PartialOrd, T2: PartialOrd> PartialOrd for BSTTuple<T1, T2> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.first.partial_cmp(&other.first)? {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => self.second.partial_cmp(&other.second),
        }
    }
}

impl<T1: Ord, T2: Ord> Ord for BSTTuple<T1, T2> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.first.cmp(&other.first) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.second.cmp(&other.second),
        }
    }
}

impl<T1, T2> From<(T1, T2)> for BSTTuple<T1, T2> {
    fn from(value: (T1, T2)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T1, T2> From<BSTTuple<T1, T2>> for (T1, T2) {
    fn from(value: BSTTuple<T1, T2>) -> Self {
        (value.first, value.second)
    }
}

/// C++ `RE::make_pair`.
#[must_use]
#[inline(always)]
pub fn make_pair<T1, T2>(first: T1, second: T2) -> BSTTuple<T1, T2> {
    BSTTuple::new(first, second)
}

/// C++ `RE::make_tuple`.
#[must_use]
#[inline(always)]
pub fn make_tuple<T1, T2>(first: T1, second: T2) -> BSTTuple<T1, T2> {
    BSTTuple::new(first, second)
}

/// C++ `swap(BSTTuple&, BSTTuple&)`.
#[inline(always)]
pub fn swap<T1, T2>(lhs: &mut BSTTuple<T1, T2>, rhs: &mut BSTTuple<T1, T2>) {
    lhs.swap(rhs);
}

type BSTTupleRepresentativeU8U32 = BSTTuple<u8, u32>;
type BSTTupleRepresentativeU32U32 = BSTTuple<u32, u32>;
type BSTTupleRepresentativePtrPtr = BSTTuple<*const u8, *const u8>;

const _: () = assert!(core::mem::size_of::<BSTTupleRepresentativeU8U32>() == 0x08);
const _: () = assert!(core::mem::offset_of!(BSTTupleRepresentativeU8U32, first) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSTTupleRepresentativeU8U32, second) == 0x04);
const _: () = assert!(core::mem::size_of::<BSTTupleRepresentativeU32U32>() == 0x08);
const _: () = assert!(core::mem::offset_of!(BSTTupleRepresentativeU32U32, second) == 0x04);
const _: () = assert!(core::mem::size_of::<BSTTupleRepresentativePtrPtr>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSTTupleRepresentativePtrPtr, second) == 0x08);
