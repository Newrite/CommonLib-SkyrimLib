/// C++ `RE::BSContainer::ForEachResult`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSContainerForEachResult {
    Stop = 0,
    Continue = 1,
}

impl BSContainerForEachResult {
    #[inline]
    pub const fn is_stop(self) -> bool {
        matches!(self, Self::Stop)
    }

    #[inline]
    pub const fn is_continue(self) -> bool {
        matches!(self, Self::Continue)
    }
}
