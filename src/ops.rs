use std::ops::{Add, Mul, Div};
use std::marker::ConstParamTy;
use std::num::{NonZero, ZeroablePrimitive};

#[derive(ConstParamTy, PartialEq, Eq)]
pub enum OpFlag {
    Addition,
    Multiplication,
    Arbitrary(usize),
}

// ? Op implies the the type is closed under the operation, checking if a subset of a type is closed under the operation is difficult, so instead implement explicit methods to check closure, or alternatively disallow construction of the type if it is not closed under the operation
pub trait Op<const ID: OpFlag> : Sized {
    fn op(&self, other: Self) -> Self;
}

impl<T> Op<{OpFlag::Addition}> for T where T: Add<Output=T> + Copy {
    fn op(&self, other: Self) -> Self {
        self.add(other)
    }
}

impl<T> Op<{OpFlag::Multiplication}> for T where T: Mul<Output=T> + Copy{
    fn op(&self, other: Self) -> Self {
        self.mul(other)
    }
}

pub trait Associative<const ID: OpFlag> : Op<{ID}> {
    fn repeated_op(&self, n: NonZero<usize>) -> Self 
    where Self: Copy
    {
        let mut n = n.get();
        let mut mult = *self;
        let mut result = *self;
        n -= 1;

        while n > 0 {
            if n & 1 == 1 {
                result = result.op(mult);
            }
            n /= 2;
            mult = mult.op(mult);
        }

        result
    }
}

impl<T> Associative<{OpFlag::Addition}> for T where T: Add<Output=T> + Copy {}
impl<T> Associative<{OpFlag::Multiplication}> for T where T: Mul<Output=T> + Copy {}

// ? Only implies an identity element for the whole type with the operation, checking if a subset has the identity would just require checking during construction
pub trait Identity<const ID: OpFlag> : Op<{ID}> {
    fn identity() -> Self;
}

// TODO: Make blanket implementaitons make more sense, macro?
impl<T> Identity<{OpFlag::Addition}> for T 
where T: ZeroablePrimitive + Op<{OpFlag::Addition}> + From<u8>
{
    fn identity() -> Self {
        T::from(0)
    }
}

impl<T> Identity<{OpFlag::Multiplication}> for T 
where T: Div + Op<{OpFlag::Multiplication}> + From<u8>
{
    fn identity() -> Self {
        T::from(1)
    }
}


// ? Split into invertibility / divisibility?
pub trait Inverse<const ID: OpFlag> : Op<{ID}> {
    fn inverse(&self) -> Self;
}

pub type BinaryOperation<T> = fn(T, T) -> T;
