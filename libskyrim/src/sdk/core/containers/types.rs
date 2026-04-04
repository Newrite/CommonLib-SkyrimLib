use super::super::sealed;

#[derive(Debug, Clone, Copy)]
pub struct ContiguousSequenceIterationOptions {
    pub max_reasonable_len: Option<u32>,
    pub require_capacity_at_least_len: bool,
}

impl ContiguousSequenceIterationOptions {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            max_reasonable_len: None,
            require_capacity_at_least_len: true,
        }
    }

    #[inline(always)]
    pub const fn with_max_reasonable_len(mut self, max_reasonable_len: u32) -> Self {
        self.max_reasonable_len = Some(max_reasonable_len);
        self
    }

    #[inline(always)]
    pub const fn with_capacity_check(mut self, require_capacity_at_least_len: bool) -> Self {
        self.require_capacity_at_least_len = require_capacity_at_least_len;
        self
    }
}

impl Default for ContiguousSequenceIterationOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

pub trait ContiguousSequence<T>: sealed::Sealed {
    fn len(&self) -> u32;
    fn data(&self) -> *const T;
    fn capacity_hint(&self) -> Option<u32>;
}
