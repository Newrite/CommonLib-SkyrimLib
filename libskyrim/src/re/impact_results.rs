/// C++ `RE::ImpactResult`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImpactResult {
    None = 0,
    Destroy = 1,
    Bounce = 2,
    Impale = 3,
    Stick = 4,
}

core_util::impl_enumset_type!(ImpactResult => u8);

impl TryFrom<u8> for ImpactResult {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= Self::Stick as u8 {
            Ok(unsafe { core::mem::transmute::<i32, Self>(value as i32) })
        } else {
            Err(())
        }
    }
}
