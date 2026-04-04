use alloc::boxed::Box;
use core::hash::{Hash, Hasher};

use super::{GamePtr, GameRef};

#[derive(Debug, PartialEq, Eq)]
struct BaseNode {
    value: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct DerivedNode {
    base: BaseNode,
    extra: i32,
}

impl AsRef<BaseNode> for DerivedNode {
    fn as_ref(&self) -> &BaseNode {
        &self.base
    }
}

impl AsMut<BaseNode> for DerivedNode {
    fn as_mut(&mut self) -> &mut BaseNode {
        &mut self.base
    }
}

fn pointer_hash<T: Hash>(value: T) -> u64 {
    #[derive(Default)]
    struct PointerHasher(u64);

    impl Hasher for PointerHasher {
        fn finish(&self) -> u64 {
            self.0
        }

        fn write(&mut self, bytes: &[u8]) {
            for &byte in bytes {
                self.0 = self.0.rotate_left(5) ^ u64::from(byte);
            }
        }
    }

    let mut hasher = PointerHasher::default();
    value.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn game_ref_round_trips_pointer_identity_and_base_projection() {
    let mut value = Box::new(DerivedNode {
        base: BaseNode { value: 7 },
        extra: 11,
    });
    let raw = value.as_mut() as *mut DerivedNode;

    let game_ref: GameRef<DerivedNode> = unsafe { GameRef::from_raw(raw) };
    let base_ref = game_ref.base::<BaseNode>();

    assert_eq!(game_ref.as_ptr(), raw);
    assert_eq!(game_ref.with(|node| node.extra), 11);
    assert_eq!(base_ref.value, 7);
    assert_eq!(game_ref.into_ptr().as_ptr(), raw);
}

#[test]
fn game_ref_mut_projection_updates_underlying_value() {
    let mut value = Box::new(DerivedNode {
        base: BaseNode { value: 3 },
        extra: 5,
    });
    let game_ref: GameRef<DerivedNode> = unsafe { GameRef::from_mut_unchecked(value.as_mut()) };

    unsafe {
        game_ref.with_mut_unchecked(|node| {
            node.base.value = 12;
            node.extra = 21;
        });
        game_ref
            .base_mut_unchecked::<BaseNode>()
            .with_mut_unchecked(|base| base.value += 8);
    }

    assert_eq!(value.base.value, 20);
    assert_eq!(value.extra, 21);
}

#[test]
fn game_ptr_null_behaves_like_empty_option() {
    let ptr = GamePtr::<DerivedNode>::null();

    assert!(ptr.is_null());
    assert!(!ptr.is_some());
    assert!(ptr.as_non_null().is_none());
    assert!(ptr.as_ref().is_none());
    assert!(ptr.into_option().is_none());
    assert_eq!(ptr.with(|node| node.extra), None);
    assert_eq!(ptr.map(|node| node.extra), None);
    assert_eq!(ptr.map_or(99, |node| node.extra), 99);
    assert_eq!(ptr.map_or_else(|| 42, |node| node.extra), 42);
    assert_eq!(ptr.and_then(|node| Some(node.extra)), None);
    assert!(ptr.base::<BaseNode>().is_null());
}

#[test]
fn game_ptr_maps_over_live_value_and_supports_base_projection() {
    let mut value = Box::new(DerivedNode {
        base: BaseNode { value: 14 },
        extra: 28,
    });
    let ptr = unsafe { GamePtr::from_raw(value.as_mut()) };

    assert!(ptr.is_some());
    assert_eq!(ptr.with(|node| node.extra), Some(28));
    assert_eq!(ptr.map(|node| node.base.value + node.extra), Some(42));
    assert_eq!(ptr.map_or(0, |node| node.base.value), 14);
    assert_eq!(ptr.map_or_else(|| 0, |node| node.extra), 28);
    assert_eq!(
        ptr.and_then(|node| Some(node.extra - node.base.value)),
        Some(14)
    );
    assert_eq!(ptr.base::<BaseNode>().map(|base| base.value), Some(14));
}

#[test]
fn game_ptr_mut_projection_and_pointer_identity_are_stable() {
    let mut value = Box::new(DerivedNode {
        base: BaseNode { value: 1 },
        extra: 2,
    });
    let raw = value.as_mut() as *mut DerivedNode;
    let ptr = unsafe { GamePtr::from_raw(raw) };

    unsafe {
        ptr.with_mut_unchecked(|node| {
            node.extra = 9;
        });
        ptr.base_mut_unchecked::<BaseNode>()
            .with_mut_unchecked(|base| {
                base.value = 4;
            });
    }

    assert_eq!(value.base.value, 4);
    assert_eq!(value.extra, 9);
    assert_eq!(ptr.as_ptr(), raw);
    assert_eq!(ptr.unwrap().as_ptr(), raw);
}

#[test]
fn game_ref_and_game_ptr_hash_pointer_identity() {
    let mut value = Box::new(DerivedNode {
        base: BaseNode { value: 8 },
        extra: 13,
    });
    let game_ref: GameRef<DerivedNode> = unsafe { GameRef::from_mut_unchecked(value.as_mut()) };
    let game_ptr = game_ref.into_ptr();

    assert_eq!(pointer_hash(game_ref), pointer_hash(game_ref));
    assert_eq!(pointer_hash(game_ptr), pointer_hash(game_ptr));
    assert_eq!(game_ptr, GamePtr::from(game_ref));
}
