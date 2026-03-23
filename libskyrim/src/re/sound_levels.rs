//! Translation of `RE::SoundLevels.h`.

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SoundLevel {
    Loud = 0,
    Normal = 1,
    Silent = 2,
    VeryLoud = 3,
    Quiet = 4,
}

core_util::impl_enumset_type!(SoundLevel => u8);
core_util::impl_enumset_type!(SoundLevel => u32);

impl TryFrom<u8> for SoundLevel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(value as u32)
    }
}

impl TryFrom<u32> for SoundLevel {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::Quiet as u32 {
            Ok(unsafe { core::mem::transmute::<u32, Self>(value) })
        } else {
            Err(())
        }
    }
}

#[allow(non_camel_case_types)]
pub type SOUND_LEVEL = SoundLevel;
