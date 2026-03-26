/// Scopes the question mark operator within a temporary closure.
///
/// This macro can be used as:
/// `attempt! {{ /* body */ }}`
///
/// Or with an inline `map_err` handler:
/// `attempt! {{ /* body */ } catch(err) { /* map */ }}`
#[macro_export]
macro_rules! attempt {
    ( $try:block ) => {
        (|| $try)()
    };

    ( $try:block catch($arg:ident) $catch:block ) => {
        $crate::core::result::Result::map_err((|| $try)(), |$arg| $catch)
    };
}

/// Declares a static array and optionally captures its computed length in a
/// companion constant.
#[macro_export]
macro_rules! disarray {
    ( $(#[$meta:meta])* $scope:vis static $arr:ident: [$type:ty; $size:ident] = [
        $($items:expr),*
    ]; ) => {
        $scope const $size: usize = $crate::disarray!(@maybe_count $($items),*);
        $(#[$meta])* $scope static $arr: [$type; $size] = [ $($items),* ];
    };

    ( $(#[$meta:meta])* $scope:vis static $arr:ident: [$type:ty] = [
        $($items:expr),*
    ]; ) => {
        $(#[$meta])* $scope static $arr: [$type; $crate::disarray!(@maybe_count $($items),*)] = [
            $($items),*
        ];
    };

    (@maybe_count) => { 0 };
    (@maybe_count $($items:expr),+ ) => { [ $($crate::disarray!(@count $items)),* ].len() };
    (@count $item:expr ) => { 0 };
}

/// Declares abstract FFI-safe types whose internal layout is intentionally
/// unknown to Rust.
#[macro_export]
macro_rules! abstract_type {
    ( $( $(#[$meta:meta])* $scope:vis type $name:ident );+; ) => {
        $($(#[$meta])* #[repr(C)] $scope struct $name {
            _private: [u8; 0],
            _marker: $crate::core::marker::PhantomData<
                (*mut u8, $crate::core::marker::PhantomPinned)
            >,
        })*
    };
}

/// Generates inheritance-style forwarding impls for `Deref`, `DerefMut`,
/// `AsRef`, and `AsMut`.
#[macro_export]
macro_rules! inherit {
    (for[$($impl_generics:tt)+] $derived:ty : $base:ty $(where $($where_clause:tt)+)? ) => {
        impl<$($impl_generics)+> core::ops::Deref for $derived
        $(where $($where_clause)+)?
        {
            type Target = $base;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl<$($impl_generics)+> core::ops::DerefMut for $derived
        $(where $($where_clause)+)?
        {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }

        impl<$($impl_generics)+> AsRef<$base> for $derived
        $(where $($where_clause)+)?
        {
            #[inline(always)]
            fn as_ref(&self) -> &$base {
                &self.base
            }
        }
    };

    (for[$($impl_generics:tt)+] $derived:ty : $base:ty, $field:ident $(where $($where_clause:tt)+)? ) => {
        impl<$($impl_generics)+> core::ops::Deref for $derived
        $(where $($where_clause)+)?
        {
            type Target = $base;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<$($impl_generics)+> core::ops::DerefMut for $derived
        $(where $($where_clause)+)?
        {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }

        impl<$($impl_generics)+> AsRef<$base> for $derived
        $(where $($where_clause)+)?
        {
            #[inline(always)]
            fn as_ref(&self) -> &$base {
                &self.$field
            }
        }

        impl<$($impl_generics)+> AsMut<$base> for $derived
        $(where $($where_clause)+)?
        {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $base {
                &mut self.$field
            }
        }
    };

    (for[$($impl_generics:tt)+] $derived:ty => $base:ty, $field:ident $(where $($where_clause:tt)+)? ) => {
        impl<$($impl_generics)+> AsRef<$base> for $derived
        $(where $($where_clause)+)?
        {
            #[inline(always)]
            fn as_ref(&self) -> &$base {
                &self.$field
            }
        }

        impl<$($impl_generics)+> AsMut<$base> for $derived
        $(where $($where_clause)+)?
        {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $base {
                &mut self.$field
            }
        }
    };

    ($derived:ident : $base:ty) => {
        impl core::ops::Deref for $derived {
            type Target = $base;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl core::ops::DerefMut for $derived {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }

        impl AsRef<$base> for $derived {
            #[inline(always)]
            fn as_ref(&self) -> &$base {
                &self.base
            }
        }
    };

    ($derived:ident : $base:ty, $field:ident) => {
        impl core::ops::Deref for $derived {
            type Target = $base;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl core::ops::DerefMut for $derived {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }

        impl AsRef<$base> for $derived {
            #[inline(always)]
            fn as_ref(&self) -> &$base {
                &self.$field
            }
        }

        impl AsMut<$base> for $derived {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $base {
                &mut self.$field
            }
        }
    };

    ($derived:ident => $base:ty, $field:ident) => {
        impl AsRef<$base> for $derived {
            #[inline(always)]
            fn as_ref(&self) -> &$base {
                &self.$field
            }
        }

        impl AsMut<$base> for $derived {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $base {
                &mut self.$field
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[repr(C)]
    struct GenericBase<T> {
        value: T,
    }

    #[repr(C)]
    struct GenericRoot<T> {
        base: GenericBase<T>,
    }

    crate::inherit!(for[T] GenericRoot<T> : GenericBase<T>);

    #[repr(C)]
    struct GenericField<T, const N: usize> {
        values: [T; N],
    }

    #[repr(C)]
    struct GenericOwner<T, const N: usize> {
        inner: GenericField<T, N>,
    }

    crate::inherit!(for[T, const N: usize] GenericOwner<T, N> => GenericField<T, N>, inner);

    #[test]
    fn inherit_supports_generic_types() {
        let root = GenericRoot {
            base: GenericBase { value: 7u32 },
        };
        assert_eq!(root.value, 7);

        let mut owner = GenericOwner {
            inner: GenericField {
                values: [1u32, 2, 3],
            },
        };
        let field_ref: &GenericField<u32, 3> = owner.as_ref();
        assert_eq!(field_ref.values[1], 2);

        let field_mut: &mut GenericField<u32, 3> = owner.as_mut();
        field_mut.values[2] = 9;
        assert_eq!(owner.inner.values[2], 9);
    }
}
