//! `NonZero*` types that derive [`ConstParamTy`] and can be used as constant generic params.

#![expect(internal_features)]
#![feature(
    adt_const_params,
    const_option_ops,
    const_param_ty_unchecked,
    const_trait_impl,
    nonzero_internals
)]

use std::fmt;
use std::marker::ConstParamTy;
use std::num::ZeroablePrimitive;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ConstParamTy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(
    feature = "zerocopy",
    derive(zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)
)]
#[repr(transparent)]
pub struct NonZero<T: ZeroablePrimitive>(T);

impl<T: ZeroablePrimitive> NonZero<T> {
    #[doc = r" Creates a non-zero if the given value is not zero."]
    #[must_use]
    #[inline(always)]
    pub const fn new(value: T) -> Option<Self> {
        <std::num::NonZero<T>>::new(value).map(Self::from_std)
    }

    #[doc = r" Creates a non-zero without checking whether the value is non-zero."]
    #[doc = r" This results in undefined behaviour if the value is zero."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" The value must not be zero."]
    #[must_use]
    #[inline(always)]
    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self::from_std(unsafe { std::num::NonZero::new_unchecked(value) })
    }

    #[must_use]
    #[inline(always)]
    pub const fn get(self) -> T {
        self.into_std().get()
    }

    #[must_use]
    #[inline(always)]
    pub const fn into_std(self) -> std::num::NonZero<T> {
        unsafe { <std::num::NonZero<T>>::new_unchecked(self.0) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_std(value: std::num::NonZero<T>) -> Self {
        Self(value.get())
    }
}

impl<T: ZeroablePrimitive + fmt::Display> fmt::Display for NonZero<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <T as fmt::Display>::fmt(&self.0, f)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: ZeroablePrimitive + serde::Deserialize<'de>> serde::Deserialize<'de> for NonZero<T> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = <T as serde::Deserialize<'de>>::deserialize(deserializer)?;
        Self::new(v).ok_or_else(|| serde::de::Error::custom("expected nonzero integer"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        struct A<const NZ: NonZero<u8>>;

        impl<const NZ: NonZero<u8>> A<NZ> {
            fn get(self) -> u8 {
                NZ.get()
            }

            fn get_static() -> u8 {
                NZ.get()
            }
        }

        assert_eq!(A::<{ NonZero::new(1).unwrap() }>.get(), 1);
        assert_eq!(A::<{ NonZero::new(1).unwrap() }>::get_static(), 1);
    }
}
