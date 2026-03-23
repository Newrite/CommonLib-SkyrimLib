use core::marker::PhantomData;

use crate::{EnumSetInteger, EnumSetType};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Enum<E, U> {
    value: U,
    _marker: PhantomData<fn() -> E>,
}

impl<E, U> Enum<E, U>
where
    U: EnumSetInteger,
{
    #[inline(always)]
    pub const fn from_underlying(value: U) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn underlying(self) -> U {
        self.value
    }

    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.value == U::ZERO
    }

    #[inline(always)]
    pub fn get(self) -> Option<E>
    where
        E: TryFrom<U>,
    {
        E::try_from(self.value).ok()
    }
}

impl<E, U> Default for Enum<E, U>
where
    U: EnumSetInteger,
{
    #[inline(always)]
    fn default() -> Self {
        Self::from_underlying(U::ZERO)
    }
}

impl<E, U> Enum<E, U>
where
    E: EnumSetType<U>,
    U: EnumSetInteger,
{
    #[inline(always)]
    pub fn new(value: E) -> Self {
        Self::from_underlying(value.to_underlying())
    }

    #[inline(always)]
    pub fn set(&mut self, value: E) -> &mut Self {
        self.value = value.to_underlying();
        self
    }
}

impl<E, U> From<E> for Enum<E, U>
where
    E: EnumSetType<U>,
    U: EnumSetInteger,
{
    #[inline(always)]
    fn from(value: E) -> Self {
        Self::new(value)
    }
}

impl<E, U> PartialEq<E> for Enum<E, U>
where
    E: EnumSetType<U>,
    U: EnumSetInteger,
{
    #[inline(always)]
    fn eq(&self, other: &E) -> bool {
        self.value == other.to_underlying()
    }
}

#[macro_export]
macro_rules! impl_enum_type {
    ($enum_ty:ty => $storage_ty:ty) => {
        impl $crate::EnumSetType<$storage_ty> for $enum_ty {
            #[inline(always)]
            fn to_underlying(self) -> $storage_ty {
                self as $storage_ty
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::Enum;

    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestValue {
        Zero = 0,
        One = 1,
        Two = 2,
    }

    impl TryFrom<u8> for TestValue {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Zero),
                1 => Ok(Self::One),
                2 => Ok(Self::Two),
                _ => Err(()),
            }
        }
    }

    crate::impl_enum_type!(TestValue => i32);
    crate::impl_enum_type!(TestValue => u8);

    #[test]
    fn stores_exact_underlying_value() {
        let value = Enum::<TestValue, i32>::from(TestValue::Two);
        assert_eq!(value.underlying(), 2);
        assert_eq!(value, TestValue::Two);
    }

    #[test]
    fn supports_narrower_storage_than_repr() {
        let mut value = Enum::<TestValue, u8>::default();
        value.set(TestValue::One);
        assert_eq!(value.underlying(), 1u8);
        assert_eq!(value.get(), Some(TestValue::One));
    }
}
