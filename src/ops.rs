use std::ops::{Add, Mul, Div};
use std::num::NonZero;

pub trait OpFlag {}


pub struct ADD {}
pub struct MUL {}
pub struct OP<const T: usize> {}

impl OpFlag for ADD {}
impl OpFlag for MUL {}
impl<const T: usize> OpFlag for OP<{T}> {}

// ? Op implies the the type is closed under the operation, checking if a subset of a type is closed under the operation is difficult, so instead implement explicit methods to check closure, or alternatively disallow construction of the type if it is not closed under the operation
pub trait Op<O: OpFlag> : Sized {
    fn op(&self, other: Self) -> Self;
}

impl<T> Op<ADD> for T where T: Add<Output=T> + Copy {
    fn op(&self, other: Self) -> Self {
        self.add(other)
    }
}

impl<T> Op<MUL> for T where T: Mul<Output=T> + Copy{
       fn op(&self, other: Self) -> Self {
        self.mul(other)
    }
}

pub trait Associative<O: OpFlag> : Op<O> {
    fn repeated_op(&self, n: NonZero<usize>) -> Self 
    where Self: Copy,
    {
        let mut n = n.get();
        let mut mult = *self;
        let mut result = *self;
        n -= 1;

        while n > 0 {
            if n & 1 == 1 {
                result = result.op(mult.clone());
            }
            n /= 2;
            mult = mult.op(mult.clone());
        }

        result
    }
}

impl<T> Associative<ADD> for T where T: Add<Output=T> + Copy {}
impl<T> Associative<MUL> for T where T: Mul<Output=T> + Copy {}

// ? Only implies an identity element for the whole type with the operation, checking if a subset has the identity would just require checking during construction
pub trait Identity<O: OpFlag> : Op<O> {
    fn identity() -> Self;
}

// TODO: Make blanket implementaitons make more sense, macro?
impl<T> Identity<ADD> for T 
where T: Op<ADD> + From<u8>
{
    fn identity() -> Self {
        T::from(0)
    }
}

impl<T> Identity<MUL> for T 
where T: Div + Op<MUL> + From<u8>
{
    fn identity() -> Self {
        T::from(1)
    }
}


// ? Split into invertibility / divisibility?
pub trait Inverse<O: OpFlag> : Op<O> {
    fn inverse(&self) -> Self;
}

pub type BinaryOperation<T> = fn(T, T) -> T;
