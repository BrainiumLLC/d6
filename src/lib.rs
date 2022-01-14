use en::{num_traits::Zero, Max as _, Min as _};
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    prelude::*,
};
use std::ops::Neg;

pub fn range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    rand::thread_rng().gen_range(range)
}

pub fn radius<T>(radius: T) -> T
where
    T: Copy + Neg<Output = T> + PartialOrd + SampleUniform,
{
    let inv = -radius;
    range(radius.min(inv)..radius.max(inv))
}

pub fn zero_to_excl<T>(max: T) -> T
where
    T: PartialOrd + SampleUniform + Zero,
{
    let zero = T::zero();
    assert!(max > zero, "max must be greater than zero");
    range(zero..max)
}

pub fn zero_to_incl<T>(max: T) -> T
where
    T: PartialOrd + SampleUniform + Zero,
{
    let zero = T::zero();
    assert!(max > zero, "max must be greater than zero");
    range(zero..=max)
}
