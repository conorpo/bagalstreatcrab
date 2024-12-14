use crate::{ops::*, set::{self, *}, util::*};
use std::rc::Rc;


trait HasSet<T> {
    fn get_set(&self) -> Rc<dyn Set<T>>;
    
    fn is_subset_of(&self, other: Box<dyn HasSet<T>>) -> bool {
        let set = self.get_set();
        let other_set = other.get_set();
        set.is_subset_of(&*other_set)
    }
}

use magma::*;
use semigroup::*;
use monoid::*;

mod magma {
    use super::*;


    struct Magma<const OP: OpFlag, T>
    where T: Op<{OP}>
    {
        set: Rc<dyn Set<T>>,
    }

    pub trait MagmaT<const OP: OpFlag, T>: HasSet<T>
    where T: Op<{OP}>
    {
        fn as_magma(&self) -> Magma<OP, T> {
            Magma::new(self.get_set())
        }

        fn is_submagma_of(&self, other: Box<dyn MagmaT<OP, T>>) -> bool {
            // Reasoning: Both are closed under the same operation, so A subset of B implies A is a submagma of B
            self.is_subset_of(other)
        }
    }
    
    impl<const OP: OpFlag, T> HasSet<T> for Magma<OP, T> 
    where T: Op<{OP}>
    {
        fn get_set(&self) -> Rc<dyn Set<T>> {
            self.set.clone()
        }
    }
    
    impl<const OP: OpFlag, T> Magma<OP, T> 
    where T: Op<{OP}>
    {
        fn new(set: Rc<dyn Set<T>>) -> Self {
            Magma { set }
        }
    }
    
    impl<const OP: OpFlag, T> MagmaT<OP, T> for Magma<OP, T> where T: Op<{OP}> {}
}


mod semigroup {
    use super::*;
    pub struct Semigroup<const OP: OpFlag, T>
    where T: Associative<{OP}>
    {
        set: Rc<dyn Set<T>>,
    }

    pub trait SemigroupT<const OP: OpFlag, T>: MagmaT<OP, T>
    where T: Associative<{OP}>
    {
        fn as_semigroup(&self) -> Semigroup<OP, T> {
            Semigroup::new(self.get_set())
        }

        fn is_subsemigroup_of(&self, other: Box<dyn SemigroupT<OP, T>>) -> bool {
            // Reasoning: Both are closed under the same operation, so A subset of B implies A is a subsemigroup of B
            self.is_subset_of(other)
        }
    }

    impl<const OP: OpFlag, T> HasSet<T> for Semigroup<OP, T> 
    where T: Associative<{OP}>
    {
        fn get_set(&self) -> Rc<dyn Set<T>> {
            self.set.clone()
        }
    }

    impl<const OP: OpFlag, T> Semigroup<OP, T> 
    where T: Associative<{OP}>
    {
        pub fn new(set: Rc<dyn Set<T>>) -> Self {
            Semigroup { set }
        }
    }

    impl<const OP: OpFlag, T> MagmaT<OP, T> for Semigroup<OP, T> where T: Associative<{OP}> {}
    impl<const OP: OpFlag, T> SemigroupT<OP, T> for Semigroup<OP, T> where T: Associative<{OP}> {}
}


mod monoid {
    use super::*;
    pub struct Monoid<const OP: OpFlag, T>
    where T: Identity<{OP}> + Associative<{OP}>
    {
        set: Rc<dyn Set<T>>,
    }

    pub trait MonoidT<const OP: OpFlag, T>: SemigroupT<OP, T>
    where T: Identity<{OP}> + Associative<{OP}>
    {
        fn as_monoid(&self) -> Monoid<OP, T> {
            Monoid::new(self.get_set())
        }

        fn is_submonoid_of(&self, other: Box<dyn MonoidT<OP, T>>) -> bool {
            other.get_set().contains(T::identity()) && self.is_submagma_of(other)
        }  
    }

    impl<const OP: OpFlag, T> HasSet<T> for Monoid<OP, T> 
    where T: Identity<{OP}> + Associative<{OP}>
    {
        fn get_set(&self) -> Rc<dyn Set<T>> {
            self.set.clone()
        }
    }

    impl<const OP: OpFlag, T> Monoid<OP, T> 
    where T: Identity<{OP}> + Associative<{OP}>
    {
        fn new(set: Rc<dyn Set<T>>) -> Self {
            assert!(set.contains(T::identity()));
            Monoid { set }
        }
 
    } 


    impl<const OP: OpFlag, T> MagmaT<OP, T> for Monoid<OP, T> where T: Identity<{OP}> + Associative<{OP}> {}
    impl<const OP: OpFlag, T> SemigroupT<OP, T> for Monoid<OP, T> where T: Identity<{OP}> + Associative<{OP}> {}
    impl<const OP: OpFlag, T> MonoidT<OP, T> for Monoid<OP, T> where T: Identity<{OP}> + Associative<{OP}> {}

    
}


#[cfg(test)]
mod tests {
    use super::*;
}