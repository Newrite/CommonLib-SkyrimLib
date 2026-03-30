//! Translation of `RE::InventoryEvent.h`.

/// C++ `RE::INVENTORY_EVENT`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InventoryEvent {
    WeaponChanged = 0,
    Draw = 1,
    Sheath = 2,
}
