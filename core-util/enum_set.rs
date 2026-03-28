use core::marker::PhantomData;
use core::ops::{Add, BitAnd, BitOr, BitXor, Not, Shl, Shr, Sub};

pub trait EnumSetInteger:
    Copy
    + Eq
    + Ord
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Shl<Self, Output = Self>
    + Shr<Self, Output = Self>
{
    const ZERO: Self;
}

macro_rules! impl_enum_set_integer {
    ($($ty:ty),* $(,)?) => {
        $(
            impl EnumSetInteger for $ty {
                const ZERO: Self = 0;
            }
        )*
    };
}

impl_enum_set_integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

pub trait EnumSetType<U: EnumSetInteger>: Copy {
    fn to_underlying(self) -> U;
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumSet<E, U> {
    value: U,
    _marker: PhantomData<fn() -> E>,
}

impl<E, U> EnumSet<E, U>
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
    pub fn clear(&mut self) -> &mut Self {
        self.value = U::ZERO;
        self
    }

    #[inline(always)]
    pub fn get(self) -> Option<E>
    where
        E: TryFrom<U>,
    {
        E::try_from(self.value).ok()
    }
}

impl<E, U> EnumSet<E, U>
where
    E: EnumSetType<U>,
    U: EnumSetInteger,
{
    #[inline(always)]
    pub fn new(value: E) -> Self {
        Self::from_underlying(value.to_underlying())
    }
}

macro_rules! impl_enum_set_const_storage {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<E> EnumSet<E, $ty> {
                #[inline(always)]
                pub const fn is_empty(self) -> bool {
                    self.value == 0
                }

                #[inline(always)]
                pub const fn any_set(self, other: Self) -> bool {
                    (self.value & other.value) != 0
                }

                #[inline(always)]
                pub const fn all_set(self, other: Self) -> bool {
                    (self.value & other.value) == other.value
                }

                #[inline(always)]
                pub const fn none_set(self, other: Self) -> bool {
                    (self.value & other.value) == 0
                }

                #[inline(always)]
                pub const fn any_underlying(self, value: $ty) -> bool {
                    (self.value & value) != 0
                }

                #[inline(always)]
                pub const fn all_underlying(self, value: $ty) -> bool {
                    (self.value & value) == value
                }

                #[inline(always)]
                pub const fn none_underlying(self, value: $ty) -> bool {
                    (self.value & value) == 0
                }
            }
        )*
    };
}

impl_enum_set_const_storage!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

impl<E, U> Default for EnumSet<E, U>
where
    U: EnumSetInteger,
{
    #[inline(always)]
    fn default() -> Self {
        Self::from_underlying(U::ZERO)
    }
}

impl<E, U> EnumSet<E, U>
where
    E: EnumSetType<U>,
    U: EnumSetInteger,
{
    #[inline(always)]
    pub fn set(&mut self, value: E) -> &mut Self {
        self.value = self.value | value.to_underlying();
        self
    }

    #[inline(always)]
    pub fn set_many<const N: usize>(&mut self, values: [E; N]) -> &mut Self {
        for value in values {
            self.set(value);
        }
        self
    }

    #[inline(always)]
    pub fn set_enabled(&mut self, enabled: bool, value: E) -> &mut Self {
        if enabled {
            self.set(value);
        } else {
            self.reset(value);
        }
        self
    }

    #[inline(always)]
    pub fn reset(&mut self, value: E) -> &mut Self {
        self.value = self.value & !value.to_underlying();
        self
    }

    #[inline(always)]
    pub fn reset_many<const N: usize>(&mut self, values: [E; N]) -> &mut Self {
        for value in values {
            self.reset(value);
        }
        self
    }

    #[inline(always)]
    pub fn any(self, value: E) -> bool {
        (self.value & value.to_underlying()) != U::ZERO
    }

    #[inline(always)]
    pub fn all(self, value: E) -> bool {
        (self.value & value.to_underlying()) == value.to_underlying()
    }

    #[inline(always)]
    pub fn none(self, value: E) -> bool {
        (self.value & value.to_underlying()) == U::ZERO
    }
}

impl<E, U> From<E> for EnumSet<E, U>
where
    E: EnumSetType<U>,
    U: EnumSetInteger,
{
    #[inline(always)]
    fn from(value: E) -> Self {
        Self::from_underlying(value.to_underlying())
    }
}

impl<E, U> PartialEq<E> for EnumSet<E, U>
where
    E: EnumSetType<U>,
    U: EnumSetInteger,
{
    #[inline(always)]
    fn eq(&self, other: &E) -> bool {
        self.value == other.to_underlying()
    }
}

macro_rules! impl_binary_ops {
    ($trait:ident, $method:ident) => {
        impl<E, U> core::ops::$trait for EnumSet<E, U>
        where
            U: EnumSetInteger,
        {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: Self) -> Self::Output {
                Self::from_underlying(self.value.$method(rhs.value))
            }
        }

        impl<E, U> core::ops::$trait<E> for EnumSet<E, U>
        where
            E: EnumSetType<U>,
            U: EnumSetInteger,
        {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: E) -> Self::Output {
                Self::from_underlying(self.value.$method(rhs.to_underlying()))
            }
        }
    };
}

macro_rules! impl_assign_ops {
    ($trait:ident, $method:ident, $op_method:ident) => {
        impl<E, U> core::ops::$trait for EnumSet<E, U>
        where
            U: EnumSetInteger,
        {
            #[inline(always)]
            fn $method(&mut self, rhs: Self) {
                self.value = self.value.$op_method(rhs.value);
            }
        }

        impl<E, U> core::ops::$trait<E> for EnumSet<E, U>
        where
            E: EnumSetType<U>,
            U: EnumSetInteger,
        {
            #[inline(always)]
            fn $method(&mut self, rhs: E) {
                self.value = self.value.$op_method(rhs.to_underlying());
            }
        }
    };
}

impl_binary_ops!(BitAnd, bitand);
impl_binary_ops!(BitOr, bitor);
impl_binary_ops!(BitXor, bitxor);
impl_binary_ops!(Add, add);
impl_binary_ops!(Sub, sub);
impl_binary_ops!(Shl, shl);
impl_binary_ops!(Shr, shr);

impl_assign_ops!(BitAndAssign, bitand_assign, bitand);
impl_assign_ops!(BitOrAssign, bitor_assign, bitor);
impl_assign_ops!(BitXorAssign, bitxor_assign, bitxor);
impl_assign_ops!(AddAssign, add_assign, add);
impl_assign_ops!(SubAssign, sub_assign, sub);
impl_assign_ops!(ShlAssign, shl_assign, shl);
impl_assign_ops!(ShrAssign, shr_assign, shr);

impl<E, U> Not for EnumSet<E, U>
where
    U: EnumSetInteger,
{
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self::from_underlying(!self.value)
    }
}

#[macro_export]
macro_rules! impl_enumset_type {
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
    use super::EnumSet;

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestFlags {
        A = 1 << 0,
        B = 1 << 1,
        C = 1 << 2,
    }

    crate::impl_enumset_type!(TestFlags => u8);

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

    crate::impl_enumset_type!(TestValue => i32);
    crate::impl_enumset_type!(TestValue => u8);

    #[test]
    fn flags_roundtrip_underlying() {
        let mut set = EnumSet::<TestFlags, u8>::default();
        set.set(TestFlags::A).set(TestFlags::C);
        assert_eq!(set.underlying(), 0b101);
        assert!(set.any(TestFlags::A));
        assert!(set.all(TestFlags::C));
        assert!(set.none(TestFlags::B));

        set.reset(TestFlags::A);
        assert_eq!(set.underlying(), 0b100);
    }

    #[test]
    fn supports_narrower_storage_than_enum_repr() {
        let value = EnumSet::<TestValue, u8>::from(TestValue::Two);
        assert_eq!(value.underlying(), 2u8);
        assert_eq!(value.get(), Some(TestValue::Two));
    }

    #[test]
    fn supports_new_from_enum_value() {
        let value = EnumSet::<TestValue, i32>::new(TestValue::One);
        assert_eq!(value.underlying(), 1);
    }

    const CONST_FLAGS: EnumSet<TestFlags, u8> = EnumSet::from_underlying(0b101);
    const CONST_HAS_A: bool = CONST_FLAGS.all_underlying(TestFlags::A as u8);
    const CONST_LACKS_B: bool = CONST_FLAGS.none_underlying(TestFlags::B as u8);
    const CONST_HAS_ANY_C: bool = CONST_FLAGS.any_underlying(TestFlags::C as u8);
    const CONST_NOT_EMPTY: bool = !CONST_FLAGS.is_empty();
    const CONST_HAS_AC_SET: bool = CONST_FLAGS.all_set(EnumSet::from_underlying(0b101));

    #[test]
    fn supports_const_underlying_queries() {
        assert!(CONST_HAS_A);
        assert!(CONST_LACKS_B);
        assert!(CONST_HAS_ANY_C);
        assert!(CONST_NOT_EMPTY);
        assert!(CONST_HAS_AC_SET);
    }
}
