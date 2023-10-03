//! `NonZero*` types that derive [`ConstParamTy`] and can be used as constant generic params.

#![allow(incomplete_features)]
#![feature(adt_const_params, rustc_attrs)]

use std::fmt;
use std::marker::ConstParamTy;

macro_rules! impl_nonzero {
    ($ty:ident, $int:ty, $std:ty) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ConstParamTy)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        #[cfg_attr(
            feature = "zerocopy",
            derive(zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)
        )]
        #[repr(transparent)]
        #[rustc_layout_scalar_valid_range_start(1)]
        #[rustc_nonnull_optimization_guaranteed]
        pub struct $ty($int);

        impl $ty {
            /// Creates a non-zero if the given value is not zero.
            #[must_use]
            #[inline]
            pub const fn new(value: $int) -> Option<Self> {
                if let Some(value) = <$std>::new(value) {
                    Some(Self::from_std(value))
                } else {
                    None
                }
            }

            /// Creates a non-zero without checking whether the value is non-zero.
            /// This results in undefined behaviour if the value is zero.
            ///
            /// # Safety
            ///
            /// The value must not be zero.
            #[must_use]
            #[inline]
            pub const unsafe fn new_unchecked(value: $int) -> Self {
                Self(value)
            }

            pub const fn get(self) -> $int {
                self.0
            }

            #[inline]
            pub const fn into_std(self) -> $std {
                // Safety: $std has the same range validity constraints as Self
                unsafe { <$std>::new_unchecked(self.0) }
            }

            #[inline]
            pub const fn from_std(value: $std) -> Self {
                // Safety: $std has the same range validity constraints as Self
                unsafe { Self(value.get()) }
            }
        }

        impl fmt::Display for $ty {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                <$int as fmt::Display>::fmt(&self.0, f)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $ty {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                <$std as serde::Deserialize<'de>>::deserialize(deserializer).map(Self::from_std)
            }
        }
    };
}

impl_nonzero!(NonZeroU8, u8, std::num::NonZeroU8);
impl_nonzero!(NonZeroU16, u16, std::num::NonZeroU16);
impl_nonzero!(NonZeroU32, u32, std::num::NonZeroU32);
impl_nonzero!(NonZeroU64, u64, std::num::NonZeroU64);
impl_nonzero!(NonZeroU128, u128, std::num::NonZeroU128);
impl_nonzero!(NonZeroUsize, usize, std::num::NonZeroUsize);
impl_nonzero!(NonZeroI8, i8, std::num::NonZeroI8);
impl_nonzero!(NonZeroI16, i16, std::num::NonZeroI16);
impl_nonzero!(NonZeroI32, i32, std::num::NonZeroI32);
impl_nonzero!(NonZeroI64, i64, std::num::NonZeroI64);
impl_nonzero!(NonZeroI128, i128, std::num::NonZeroI128);
impl_nonzero!(NonZeroIsize, isize, std::num::NonZeroIsize);
