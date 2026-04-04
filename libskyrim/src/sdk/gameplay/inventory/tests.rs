use alloc::boxed::Box;

use super::{InventoryEntrySnapshot, InventoryQueryOptions};
use crate::re::InventoryEntryData;
use crate::sdk::core::GamePtr;

#[test]
fn query_options_default_to_initializing_inventory() {
    assert_eq!(InventoryQueryOptions::default().no_init, false);
    assert!(InventoryQueryOptions::new().no_init(true).no_init);
}

#[test]
fn null_snapshot_reports_safe_defaults() {
    let mut snapshot = InventoryEntrySnapshot {
        object: GamePtr::null(),
        count: 3,
        entry: Box::new(InventoryEntryData::default()),
    };

    assert_eq!(snapshot.form_id(), 0);
    assert_eq!(snapshot.form_editor_id(), "");
    assert_eq!(snapshot.display_name(), "");
    assert_eq!(snapshot.weight(), -1.0);
    assert_eq!(snapshot.stack_weight(), 0.0);
    assert!(!snapshot.is_armor());
    assert!(!snapshot.is_worn());
    assert!(!snapshot.is_favorited());
}
