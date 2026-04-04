use alloc::vec;
use alloc::vec::Vec;
use core::ops::ControlFlow;

use super::{
    ContiguousSequence, ContiguousSequenceIterationOptions, contiguous_sequence_bounds,
    for_each_contiguous_sequence, snapshot_contiguous_cloned, snapshot_contiguous_copied,
};

struct TestSequence<T> {
    len: u32,
    data: *const T,
    capacity: Option<u32>,
}

impl<T> crate::sdk::core::sealed::Sealed for TestSequence<T> {}

impl<T> ContiguousSequence<T> for TestSequence<T> {
    fn len(&self) -> u32 {
        self.len
    }

    fn data(&self) -> *const T {
        self.data
    }

    fn capacity_hint(&self) -> Option<u32> {
        self.capacity
    }
}

#[test]
fn bounds_reject_empty_or_invalid_sequences() {
    let empty = TestSequence::<u32> {
        len: 0,
        data: core::ptr::null(),
        capacity: Some(0),
    };
    assert!(
        contiguous_sequence_bounds(&empty, ContiguousSequenceIterationOptions::new()).is_none()
    );

    let invalid_capacity = TestSequence {
        len: 3,
        data: [1u32, 2, 3].as_ptr(),
        capacity: Some(2),
    };
    assert!(
        contiguous_sequence_bounds(&invalid_capacity, ContiguousSequenceIterationOptions::new())
            .is_none()
    );

    let null_data = TestSequence::<u32> {
        len: 1,
        data: core::ptr::null(),
        capacity: Some(1),
    };
    assert!(
        contiguous_sequence_bounds(&null_data, ContiguousSequenceIterationOptions::new()).is_none()
    );
}

#[test]
fn bounds_accept_reasonable_sequences() {
    let values = [10u32, 20, 30];
    let sequence = TestSequence {
        len: 3,
        data: values.as_ptr(),
        capacity: Some(3),
    };

    let bounds = contiguous_sequence_bounds(&sequence, ContiguousSequenceIterationOptions::new());
    assert_eq!(bounds, Some((values.as_ptr(), 3)));
}

#[test]
fn iteration_stops_when_visitor_requests_break() {
    let values = [1u32, 2, 3, 4];
    let sequence = TestSequence {
        len: values.len() as u32,
        data: values.as_ptr(),
        capacity: Some(values.len() as u32),
    };
    let mut visited = Vec::new();

    let flow = for_each_contiguous_sequence(
        &sequence,
        ContiguousSequenceIterationOptions::new(),
        |value| {
            visited.push(*value);
            if *value == 3 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        },
    );

    assert_eq!(flow, ControlFlow::Break(()));
    assert_eq!(visited, vec![1, 2, 3]);
}

#[test]
fn snapshots_copy_and_clone_elements() {
    let copied_values = [4u32, 5, 6];
    let copied = TestSequence {
        len: copied_values.len() as u32,
        data: copied_values.as_ptr(),
        capacity: Some(copied_values.len() as u32),
    };
    assert_eq!(
        snapshot_contiguous_copied(&copied, ContiguousSequenceIterationOptions::new()),
        vec![4, 5, 6]
    );

    let cloned_values = [StringLike("a"), StringLike("b"), StringLike("c")];
    let cloned = TestSequence {
        len: cloned_values.len() as u32,
        data: cloned_values.as_ptr(),
        capacity: Some(cloned_values.len() as u32),
    };
    assert_eq!(
        snapshot_contiguous_cloned(&cloned, ContiguousSequenceIterationOptions::new()),
        vec![StringLike("a"), StringLike("b"), StringLike("c")]
    );
}

#[test]
fn max_reasonable_len_guard_filters_suspicious_sequences() {
    let values = [1u32, 2, 3, 4];
    let sequence = TestSequence {
        len: values.len() as u32,
        data: values.as_ptr(),
        capacity: Some(values.len() as u32),
    };

    assert!(
        contiguous_sequence_bounds(
            &sequence,
            ContiguousSequenceIterationOptions::new().with_max_reasonable_len(2),
        )
        .is_none()
    );
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StringLike(&'static str);
