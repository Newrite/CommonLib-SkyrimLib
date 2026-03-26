//! Translation of `RE::InventoryEvent.h`.

/// C++ `RE::INVENTORY_EVENT`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum INVENTORY_EVENT {
    kWeaponChanged = 0,
    kDraw = 1,
    kSheath = 2,
}
