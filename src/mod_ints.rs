use std::{array, ops::{Add, Div, Mul}};
use crate::{ops::*, util:: *};
use std::marker::ConstParamTy;

// MARK: Additive Group

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U32Mod<const M: u32> (pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U64Mod<const M: u64> (pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U128Mod<const M: u128> (pub u128);

impl<const M: u32> Add<U32Mod<M>> for U32Mod<M> 
where Assert<{M <= 1 << 31}>: IsTrue {
    type Output = U32Mod<M>;

    fn add(self, other: U32Mod<M>) -> U32Mod<M> {
        U32Mod((self.0 + other.0) % M)
    }
}

impl<const M: u64> Add<U64Mod<M>> for U64Mod<M> 
where Assert<{M <= 1 << 63}>: IsTrue {
    type Output = U64Mod<M>;

    fn add(self, other: U64Mod<M>) -> U64Mod<M> {
        U64Mod((self.0 + other.0) % M)
    }
}

impl<const M: u128> Add<U128Mod<M>> for U128Mod<M> 
where Assert<{M <= 1 << 127}>: IsTrue {
    type Output = U128Mod<M>;

    fn add(self, other: U128Mod<M>) -> U128Mod<M> {
        U128Mod((self.0 + other.0) % M)
    }
}

impl<const M: u32> Inverse<ADD> for U32Mod<M> 
where Assert<{M <= 1 << 31}>: IsTrue {
    fn inverse(&self) -> U32Mod<M> {
        U32Mod(M - self.0)
    }
}

impl<const M: u64> Inverse<ADD> for U64Mod<M> 
where Assert<{M <= 1 << 63}>: IsTrue {
    fn inverse(&self) -> U64Mod<M> {
        U64Mod(M - self.0)
    }
}

impl<const M: u128> Inverse<ADD> for U128Mod<M> 
where Assert<{M <= 1 << 127}>: IsTrue {
    fn inverse(&self) -> U128Mod<M> {
        U128Mod(M - self.0)
    }
}

impl<const M: u32> Identity<ADD> for U32Mod<M> 
where Assert<{M <= 1 << 31}>: IsTrue {
    fn identity() -> U32Mod<M> {
        U32Mod(0)
    }
}

impl<const M: u64> Identity<ADD> for U64Mod<M> 
where Assert<{M <= 1 << 63}>: IsTrue {
    fn identity() -> U64Mod<M> {
        U64Mod(0)
    }
}

impl<const M: u128> Identity<ADD> for U128Mod<M> 
where Assert<{M <= 1 << 127}>: IsTrue {
    fn identity() -> U128Mod<M> {
        U128Mod(0)
    }
}


// MARK: Multiplicative Monoid
impl<const M: u32> Mul<U32Mod<M>> for U32Mod<M> 
where Assert<{M <= 1<<16}>: IsTrue {
    type Output = U32Mod<M>;

    fn mul(self, other: U32Mod<M>) -> U32Mod<M> {
        U32Mod((self.0 * other.0) % M)
    }
}

impl<const M: u64> Mul<U64Mod<M>> for U64Mod<M> 
where Assert<{M <= 1<<32}>: IsTrue {
    type Output = U64Mod<M>;

    fn mul(self, other: U64Mod<M>) -> U64Mod<M> {
        U64Mod((self.0 * other.0) % M)
    }
}

impl<const M: u128> Mul<U128Mod<M>> for U128Mod<M> 
where Assert<{M <= 1<<64}>: IsTrue {
    type Output = U128Mod<M>;

    fn mul(self, other: U128Mod<M>) -> U128Mod<M> {
        U128Mod((self.0 * other.0) % M)
    }
}

impl<const M: u32> Identity<MUL> for U32Mod<M> 
where Assert<{M <= 1<<16}>: IsTrue {
    fn identity() -> U32Mod<M> {
        U32Mod(1)
    }
}

impl<const M: u64> Identity<MUL> for U64Mod<M> 
where Assert<{M <= 1<<32}>: IsTrue {
    fn identity() -> U64Mod<M> {
        U64Mod(1)
    }
}

impl<const M: u128> Identity<MUL> for U128Mod<M> 
where Assert<{M <= 1<<64}>: IsTrue {
    fn identity() -> U128Mod<M> {
        U128Mod(1)
    }
}
// MARK: Multiplicative Group

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U32CoprimeMod<const M: u32> (u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U64CoprimeMod<const M: u64> (u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U128CoprimeMod<const M: u128> (u128);

impl<const M: u32> U32CoprimeMod<M> {
    pub fn new(n: u32) -> Self {
        todo!("Check if n is coprime to M");
        U32CoprimeMod(n)
    }
}

impl<const M: u64> U64CoprimeMod<M> {
    pub fn new(n: u64) -> Self {
        todo!("Check if n is coprime to M");
        U64CoprimeMod(n)
    }
}

impl<const M: u128> U128CoprimeMod<M> {
    pub fn new(n: u128) -> Self {
        todo!("Check if n is coprime to M");
        U128CoprimeMod(n)
    }
}

impl<const M: u32> Mul<U32CoprimeMod<M>> for U32CoprimeMod<M> 
where Assert<{M <= 1<<16}>: IsTrue {
    type Output = U32CoprimeMod<M>;

    fn mul(self, other: U32CoprimeMod<M>) -> U32CoprimeMod<M> {
        U32CoprimeMod((self.0 * other.0) % M)
    }
}

impl<const M: u64> Mul<U64CoprimeMod<M>> for U64CoprimeMod<M> 
where Assert<{M <= 1<<32}>: IsTrue {
    type Output = U64CoprimeMod<M>;

    fn mul(self, other: U64CoprimeMod<M>) -> U64CoprimeMod<M> {
        U64CoprimeMod((self.0 * other.0) % M)
    }
}

impl<const M: u128> Mul<U128CoprimeMod<M>> for U128CoprimeMod<M> 
where Assert<{M <= 1<<64}>: IsTrue {
    type Output = U128CoprimeMod<M>;

    fn mul(self, other: U128CoprimeMod<M>) -> U128CoprimeMod<M> {
        U128CoprimeMod((self.0 * other.0) % M)
    }
}

impl<const M: u32> Identity<MUL> for U32CoprimeMod<M> 
where Assert<{M <= 1<<16}>: IsTrue {
    fn identity() -> U32CoprimeMod<M> {
        U32CoprimeMod(1)
    }
}

impl<const M: u64> Identity<MUL> for U64CoprimeMod<M> 
where Assert<{M <= 1<<32}>: IsTrue {
    fn identity() -> U64CoprimeMod<M> {
        U64CoprimeMod(1)
    }
}

impl<const M: u128> Identity<MUL> for U128CoprimeMod<M> 
where Assert<{M <= 1<<64}>: IsTrue {
    fn identity() -> U128CoprimeMod<M> {
        U128CoprimeMod(1)
    }
}

impl<const M: u32> Inverse<MUL> for U32CoprimeMod<M> 
where Assert<{M <= 1<<16}>: IsTrue {
    fn inverse(&self) -> U32CoprimeMod<M> {
        todo!("Extended Euclidean Algorithm");
    }
}

impl<const M: u64> Inverse<MUL> for U64CoprimeMod<M> 
where Assert<{M <= 1<<32}>: IsTrue {
    fn inverse(&self) -> U64CoprimeMod<M> {
        todo!("Extended Euclidean Algorithm");
    }
}

impl<const M: u128> Inverse<MUL> for U128CoprimeMod<M> 
where Assert<{M <= 1<<64}>: IsTrue {
    fn inverse(&self) -> U128CoprimeMod<M> {
        todo!("Extended Euclidean Algorithm");
    }
}

// MARK: PreComped ModInts
trait PrecompedInverse<const M: u32> 
where [u32; M as usize]: {
    fn precomped_inverse(&self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct U32CoprimeModPrecomp<const M: u32> (u32);

impl<const M: u32> Mul<U32CoprimeModPrecomp<M>> for U32CoprimeModPrecomp<M> 
where Assert<{M <= 1<<16}>: IsTrue {
    type Output = U32CoprimeModPrecomp<M>;

    fn mul(self, other: U32CoprimeModPrecomp<M>) -> U32CoprimeModPrecomp<M> {
        U32CoprimeModPrecomp((self.0 * other.0) % M)
    }
}

impl<const M: u32> Identity<MUL> for U32CoprimeModPrecomp<M> 
where Assert<{M <= 1<<16}>: IsTrue {
    fn identity() -> U32CoprimeModPrecomp<M> {
        U32CoprimeModPrecomp(1)
    }
}

impl<const M: u32> Inverse<MUL> for U32CoprimeModPrecomp<M> 
where Assert<{M <= 1<<16}>: IsTrue,
      [u32; M as usize]: ,
      Self: PrecompedInverse<M> 
{
    fn inverse(&self) -> U32CoprimeModPrecomp<M> {
        self.precomped_inverse()
    }
}

// MARK: Field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// TODO: Fix all this
// Assert P is prime
pub struct U32Field<const P: u32> (pub u32) where Assert<{P <= 1 << 31}>: IsTrue;

impl <const P: u32> Add<U32Field<P>> for U32Field<P> 
where Assert<{P <= 1 << 31}>: IsTrue
{
    type Output = U32Field<P>;

    fn add(self, other: U32Field<P>) -> U32Field<P> {
        U32Field((self.0 + other.0) % P)
    }
}

impl <const P: u32> Mul<U32Field<P>> for U32Field<P> 
where Assert<{P <= 1 << 16}>: IsTrue,
      Assert<{P <= 1 << 31}>: IsTrue
{
    type Output = U32Field<P>;

    fn mul(self, other: U32Field<P>) -> U32Field<P> {
        U32Field((self.0 * other.0) % P)
    }
}

impl <const P: u32> Identity<ADD> for U32Field<P> 
where Assert<{P <= 1 << 31}>: IsTrue
{
    fn identity() -> U32Field<P> {
        U32Field(0)
    }
}

impl <const P: u32> Identity<MUL> for U32Field<P> 
where Assert<{P <= 1 << 16}>: IsTrue,
      Assert<{P <= 1 << 31}>: IsTrue
{
    fn identity() -> U32Field<P> {
        U32Field(1)
    }
}

impl <const P: u32> Inverse<ADD> for U32Field<P> 
where Assert<{P <= 1 << 31}>: IsTrue
{
    fn inverse(&self) -> U32Field<P> {
        U32Field(P - self.0)
    }
}

impl <const P: u32> Inverse<MUL> for U32Field<P> 
where Assert<{P <= 1 << 16}>: IsTrue,
      Assert<{P <= 1 << 31}>: IsTrue
{
    fn inverse(&self) -> U32Field<P> {
        todo!("Extended Euclidean Algorithm");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INVERSES_MOD_7: [u32; 6] = [1, 4, 5, 2, 3, 6];

    impl PrecompedInverse<7> for U32CoprimeModPrecomp<7> {
        fn precomped_inverse(&self) -> Self {

            Self(INVERSES_MOD_7[(self.0 - 1) as usize])
        }
    }

    fn g(x: u32) -> U32CoprimeModPrecomp<7> {
        U32CoprimeModPrecomp(x)
    }

    #[test]
    fn test_precomped_inverse() {
        let a = g(3);
        let b = g(5);
        let c = a * b;

        assert_eq!(c, g(1));
        assert_eq!(a.inverse(), b);
        assert_eq!(b.inverse(), a);
        
    }

}