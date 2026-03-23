#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sex {
    None = u32::MAX,
    Male = 0,
    Female = 1,
}

core_util::impl_enumset_type!(Sex => u32);

pub type SEX = Sex;

pub const SEXES_TOTAL: usize = 2;
