/// C++ `RE::ITEM_REMOVE_REASON`
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ITEM_REMOVE_REASON {
    Remove = 0,
    Steal = 1,
    Selling = 2,
    Dropping = 3,
    StoreInContainer = 4,
    StoreInTeammate = 5,
}
