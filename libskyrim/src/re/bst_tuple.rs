//! Translation of `RE::BSTTuple.h`.
//!
//! `BSTTuple<T1, T2>` is the engine's equivalent of `std::pair`.
//! Used as the value type in `BSTHashMap<K, V>` (stores `BSTTuple<const K, V>`).

/// C++ `RE::BSTTuple<T1, T2>`.
///
/// A simple pair struct matching the engine's layout.
/// Equivalent to `std::pair<T1, T2>` in C++.
#[repr(C)]
pub struct BSTTuple<T1, T2> {
    pub first: T1,   // 0x00
    pub second: T2,  // sizeof(T1) (with alignment)
}

impl<T1, T2> BSTTuple<T1, T2> {
    /// Creates a new tuple.
    #[inline(always)]
    pub const fn new(first: T1, second: T2) -> Self {
        Self { first, second }
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
