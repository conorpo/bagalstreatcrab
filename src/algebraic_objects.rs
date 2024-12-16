use crate::{ops::*, set::{self, *, Cardinality}, util::*};
use std::rc::Rc;
use std::marker::PhantomData;


pub trait HasSet<T> {
    fn get_set(&self) -> Rc<dyn SetT<T>>;
    
    fn is_subset_of(&self, other: &dyn HasSet<T>) -> bool {
        let set = self.get_set();
        let other_set = other.get_set();
        unimplemented!("Implement this function")
    }
}

pub trait GroupLike<O: OpFlag, T>: HasSet<T> + Sized
where T: Op<O>
{
    fn try_new(set: Rc<dyn SetT<T>>) -> Result<Self, &'static str>;
    fn new_unchecked(set: Rc<dyn SetT<T>>) -> Self;
    fn new_from_generators(generators: Rc<dyn SetT<T>>) -> Result<Self, &'static str>;
}


use magma::*;
use semigroup::*;
use monoid::*;
use group::*;

pub use group::{Group, GroupT};

// MARK: MAGMA
mod magma {
    use super::*;

    pub struct Magma<O: OpFlag, T>
    where T: Op<O>
    {
        set: Rc<dyn SetT<T>>,
        _boo: PhantomData<O>
    }

    impl<O: OpFlag, T> GroupLike<O, T> for Magma<O, T>
    where T: Op<O>
    {
        fn try_new(set: Rc<dyn SetT<T>>) -> Result<Self, &'static str> {
            Ok(Magma { set, _boo: PhantomData })
        }

        fn new_unchecked(set: Rc<dyn SetT<T>>) -> Self {
            Magma { set, _boo: PhantomData }
        }

        fn new_from_generators(generators: Rc<dyn SetT<T>>) -> Result<Self, &'static str> {
            unimplemented!();
        }
    }

    pub trait MagmaT<O: OpFlag, T>: HasSet<T>
    where T: Op<O>
    {
        fn as_magma(&self) -> Magma<O, T> {
            Magma::new_unchecked(self.get_set())
        }

        fn is_submagma_of(&self, other: &dyn MagmaT<O, T>) -> bool {
            // Reasoning: Both are closed under the same operation, so A subset of B implies A is a submagma of B
            self.is_subset_of(other)
        }
    }
    
    impl<O: OpFlag, T> HasSet<T> for Magma<O, T> 
    where T: Op<O>
    {
        fn get_set(&self) -> Rc<dyn SetT<T>> {
            self.set.clone()
        }
    }
    
    impl<O: OpFlag, T> MagmaT<O, T> for Magma<O, T> where T: Op<O> {}
}


// MARK: SEMIGROUP
mod semigroup {
    use super::*;

    pub trait SemigroupElement<O: OpFlag> = Associative<O> + PartialEq;

    pub struct Semigroup<O: OpFlag, T>
    where T: SemigroupElement<O>
    {
        set: Rc<dyn SetT<T>>,
        _boo: PhantomData<O>
    }

    impl<O: OpFlag, T> GroupLike<O, T> for Semigroup<O, T>
    where T: SemigroupElement<O>
    {
        fn try_new(set: Rc<dyn SetT<T>>) -> Result<Self, &'static str> {
            Ok(Semigroup { set, _boo: PhantomData })
        }

        fn new_unchecked(set: Rc<dyn SetT<T>>) -> Self {
            Semigroup { set, _boo: PhantomData }
        }

        fn new_from_generators(generators: Rc<dyn SetT<T>>) -> Result<Self, &'static str> {
            unimplemented!();
        }
    }

    pub trait SemigroupT<O: OpFlag, T>: MagmaT<O, T>
    where T: SemigroupElement<O>
    {
        fn as_semigroup(&self) -> Semigroup<O, T> {
            Semigroup::new_unchecked(self.get_set())
        }

        fn is_subsemigroup_of(&self, other: &dyn SemigroupT<O, T>) -> bool {
            // Reasoning: Both are closed under the same operation, so A subset of B implies A is a subsemigroup of B
            self.is_subset_of(other)
        }
    }

    impl<O: OpFlag, T> HasSet<T> for Semigroup<O, T> 
    where T: SemigroupElement<O>
    {
        fn get_set(&self) -> Rc<dyn SetT<T>> {
            self.set.clone()
        }
    }

    impl<O: OpFlag, T> MagmaT<O, T> for Semigroup<O, T> where T: SemigroupElement<O> {}
    impl<O: OpFlag, T> SemigroupT<O, T> for Semigroup<O, T> where T: SemigroupElement<O> {}
}


// MARK: QUASIGROUP
mod quasigroup {
    use super::*;

    pub trait QuasigroupElement<O: OpFlag> = Inverse<O> + PartialEq;

    pub struct Quasigroup<O: OpFlag, T>
    where T: QuasigroupElement<O>
    {
        set: Rc<dyn SetT<T>>,
        _boo: PhantomData<O>
    }

    impl<O: OpFlag, T> GroupLike<O, T> for Quasigroup<O, T>
    where T: QuasigroupElement<O>
    {
        fn try_new(set: Rc<dyn SetT<T>>) -> Result<Self, &'static str> {
            Ok(Quasigroup { set, _boo: PhantomData })
        }

        fn new_unchecked(set: Rc<dyn SetT<T>>) -> Self {
            Quasigroup { set, _boo: PhantomData }
        }

        fn new_from_generators(generators: Rc<dyn SetT<T>>) -> Result<Self, &'static str> {
            unimplemented!();
        }
    }

    pub trait QuasigroupT<O: OpFlag, T>: MagmaT<O, T>
    where T: QuasigroupElement<O>
    {
        fn test_divisibility(&self, a: T, b: T) -> bool {
            unimplemented!();
        }

        fn as_quasigroup(&self) -> Quasigroup<O, T> {
            Quasigroup::new_unchecked(self.get_set())
        }

        fn is_subquasigroup_of(&self, other: &dyn QuasigroupT<O, T>) -> bool {
            unimplemented!();
            self.is_subset_of(other)
        }
    }

    impl<O: OpFlag, T> HasSet<T> for Quasigroup<O, T> 
    where T: QuasigroupElement<O>
    {
        fn get_set(&self) -> Rc<dyn SetT<T>> {
            self.set.clone()
        }
    }

    impl<O: OpFlag, T> MagmaT<O, T> for Quasigroup<O, T> where T: QuasigroupElement<O> {}
    impl<O: OpFlag, T> QuasigroupT<O, T> for Quasigroup<O, T> where T: QuasigroupElement<O> {}
}


// MARK: MONOID
mod monoid {
    use super::*;

    pub trait MonoidElement<O: OpFlag> = Identity<O> + Associative<O> + PartialEq;

    pub struct Monoid<O: OpFlag, T>
    where T: MonoidElement<O>
    {
        set: Rc<dyn SetT<T>>,
        _boo: PhantomData<O>
    }

    impl<O: OpFlag, T> GroupLike<O, T> for Monoid<O, T>
    where T: MonoidElement<O>
    {
        fn try_new(set: Rc<dyn SetT<T>>) -> Result<Self, &'static str> {
            assert!(set.contains(&T::identity()));
            Ok(Monoid { set, _boo: PhantomData })
        }

        fn new_unchecked(set: Rc<dyn SetT<T>>) -> Self {
            Monoid { set, _boo: PhantomData }
        }

        fn new_from_generators(generators: Rc<dyn SetT<T>>) -> Result<Self, &'static str> {
            unimplemented!();
        }
    }


    pub trait MonoidT<O: OpFlag, T>: SemigroupT<O, T>
    where T: MonoidElement<O>
    {
        fn as_monoid(&self) -> Monoid<O, T> {
            Monoid::new_unchecked(self.get_set())
        }

        fn is_submonoid_of(&self, other: &dyn MonoidT<O, T>) -> bool {
            other.get_set().contains(&T::identity()) && self.is_submagma_of(other)
        }  
    }

    impl<O: OpFlag, T> HasSet<T> for Monoid<O, T> 
    where T: MonoidElement<O>
    {
        fn get_set(&self) -> Rc<dyn SetT<T>> {
            self.set.clone()
        }
    }

    impl<O: OpFlag, T> MagmaT<O, T> for Monoid<O, T> where T: MonoidElement<O> {}
    impl<O: OpFlag, T> SemigroupT<O, T> for Monoid<O, T> where T: MonoidElement<O> {}
    impl<O: OpFlag, T> MonoidT<O, T> for Monoid<O, T> where T: MonoidElement<O> {}    
}


// MARK: GROUP
mod group {
    use super::*;

    pub trait GroupElement<O:OpFlag> = Inverse<O> + Identity<O> + Associative<O> + PartialEq + Clone;

    pub struct Group<O: OpFlag, T>
    where T: GroupElement<O>
    {
        set: Rc<dyn SetT<T>>,
        _boo: PhantomData<O>
    }

    impl<O: OpFlag, T> GroupLike<O, T> for Group<O, T>
    where T: GroupElement<O>
    {
        fn try_new(set: Rc<dyn SetT<T>>) -> Result<Self, &'static str> {
            assert!(set.contains(&T::identity()));
            Ok(Group { set, _boo: PhantomData })
        }

        fn new_unchecked(set: Rc<dyn SetT<T>>) -> Self {
            Group { set, _boo: PhantomData }
        }

        fn new_from_generators(generators: Rc<dyn SetT<T>>) -> Result<Self, &'static str> {
            unimplemented!();
        }
    }

    pub trait GroupT<O: OpFlag, T>: MonoidT<O, T>
    where T: GroupElement<O>
    {
        fn as_group(&self) -> Group<O, T> {
            Group::new_unchecked(self.get_set())
        }

        fn is_subgroup_of(&self, other: &dyn GroupT<O, T>) -> bool {
            self.is_submonoid_of(other)
        }

        fn order(&self) -> Cardinality {
            self.get_set().cardinality()
        }

        fn order_of_element(&self, element: T) -> usize {
            let mut cur = element.clone();
            let mut count = 1;
            while cur != T::identity() {
                cur = cur.op(element.clone());
                count += 1;
            }
            count
        }
    }

    impl<O: OpFlag, T> HasSet<T> for Group<O, T> 
    where T: GroupElement<O>
    {
        fn get_set(&self) -> Rc<dyn SetT<T>> {
            self.set.clone()
        }
    }

    impl<O: OpFlag, T> MagmaT<O, T> for Group<O, T> where T: GroupElement<O> {}
    impl<O: OpFlag, T> SemigroupT<O, T> for Group<O, T> where T: GroupElement<O> {}
    impl<O: OpFlag, T> MonoidT<O, T> for Group<O, T> where T: GroupElement<O> {}
    impl<O: OpFlag, T> GroupT<O, T> for Group<O, T> where T: GroupElement<O> {}
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::group::*;
    use super::set::*;
    use super::monoid::*;
    use crate::ops::*;

    mod test_subgroup {
        use super::*;
        use crate::ops::*;
        use std::collections::HashSet;
        use std::iter::FromIterator;

        impl Inverse<ADD> for u8 {
            fn inverse(&self) -> Self {
                0 - *self
            }
        }
        

        #[test]
        fn test_order() {
 
            let G = Group::<ADD, u8>::new_unchecked(Rc::new(UniversalSet::new(Cardinality::Finite(256))));
            let H = Group::<ADD, u8>::new_unchecked(Rc::new(SmallSet::new(vec![0,128])));
            
            
            assert!(G.order_of_element(0) == 1);
            assert!(G.order_of_element(128) == 2);
            assert!(G.order_of_element(1) == 256);
            assert!(G.order_of_element(2) == 128);
            assert!(G.order_of_element(3) == 256);
            assert!(G.order_of_element(4) == 64);
        }
    }
}