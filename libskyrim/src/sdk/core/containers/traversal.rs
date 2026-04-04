use core::ops::ControlFlow;

use super::types::{ContiguousSequence, ContiguousSequenceIterationOptions};

#[inline(always)]
pub fn contiguous_sequence_bounds<T, S>(
    sequence: &S,
    options: ContiguousSequenceIterationOptions,
) -> Option<(*const T, u32)>
where
    S: ContiguousSequence<T>,
{
    contiguous_sequence_bounds_named(sequence, core::any::type_name::<S>(), options)
}

#[inline(always)]
pub fn contiguous_sequence_bounds_named<T, S>(
    sequence: &S,
    _caller: &'static str,
    options: ContiguousSequenceIterationOptions,
) -> Option<(*const T, u32)>
where
    S: ContiguousSequence<T>,
{
    let len = sequence.len();
    if len == 0 {
        return None;
    }

    if let Some(capacity) = sequence.capacity_hint() {
        if options.require_capacity_at_least_len && capacity < len {
            crate::defensive_sdk_warn!(
                "{} observed {} with len={} > capacity={}",
                _caller,
                core::any::type_name::<S>(),
                len,
                capacity
            );
            return None;
        }
    }

    if let Some(max_reasonable_len) = options.max_reasonable_len {
        if len > max_reasonable_len {
            crate::defensive_sdk_warn!(
                "{} observed suspicious {} len={} (max={})",
                _caller,
                core::any::type_name::<S>(),
                len,
                max_reasonable_len
            );
            return None;
        }
    }

    let data = sequence.data();
    if data.is_null() {
        crate::defensive_sdk_warn!(
            "{} observed {}<{}> with non-zero len={} and null data",
            _caller,
            core::any::type_name::<S>(),
            core::any::type_name::<T>(),
            len
        );
        return None;
    }

    let _ = (data as usize).checked_add(len as usize * core::mem::size_of::<T>())?;
    Some((data, len))
}

#[inline(always)]
pub fn for_each_contiguous_sequence<T, S>(
    sequence: &S,
    options: ContiguousSequenceIterationOptions,
    visit: impl FnMut(&T) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    S: ContiguousSequence<T>,
{
    for_each_contiguous_sequence_named(sequence, core::any::type_name::<S>(), options, visit)
}

#[inline(always)]
pub fn for_each_contiguous_sequence_named<T, S>(
    sequence: &S,
    _caller: &'static str,
    options: ContiguousSequenceIterationOptions,
    mut visit: impl FnMut(&T) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    S: ContiguousSequence<T>,
{
    let Some((data, len)) = contiguous_sequence_bounds_named(sequence, _caller, options) else {
        return ControlFlow::Continue(());
    };

    for index in 0..len as usize {
        let item = unsafe { &*data.add(index) };
        if let ControlFlow::Break(()) = visit(item) {
            return ControlFlow::Break(());
        }
    }

    ControlFlow::Continue(())
}
