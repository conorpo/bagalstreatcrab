mod cardinality {
    use std::cmp::Ordering;
    use std::fmt::{self, Display, Formatter};
    
    #[derive(Debug, Clone, Copy)]
    pub enum Cardinality {
        Finite(usize),
        Infinite
    }
    
    impl Display for Cardinality {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Cardinality::Finite(n) => write!(f, "{}", n),
                Cardinality::Infinite => write!(f, "∞")
            }
        }
    }
    
    
    impl PartialEq for Cardinality {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Cardinality::Finite(a), Cardinality::Finite(b)) => a == b,
                _ => false
            }
        }
    }
    
    impl PartialOrd for Cardinality {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match (self, other) {
                (Cardinality::Finite(a), Cardinality::Finite(b)) => a.partial_cmp(b),
                (Cardinality::Infinite, Cardinality::Infinite) => None,
                (Cardinality::Infinite, _) => Some(Ordering::Greater),
                (_, Cardinality::Infinite) => Some(Ordering::Less)
            }
        }
    }
    
}

use cardinality::*;

pub trait Set<T> {
    fn contains(&self, element: T) -> bool;
    fn is_empty(&self) -> bool {
        false
    }

    fn cardinality(&self) -> Cardinality;
    fn iter(&self) -> Iter<T> {
        unimplemented!("Implement this for your set type")
    }

    fn is_subset_of(&self, other: &dyn Set<T>) -> bool {
        if self.cardinality() >= other.cardinality() {
            return false;
        }
        
        self.iter().all(|x| other.contains(x))
    }
}

// TODO: Rework sets.
// fn equivalent_sets<A,B>(a: &dyn Set<A>, b: &dyn Set<B>) -> bool {
//     if a.cardinality() == 0 && b.cardinality() == 0 {
//         return true;
//     } else {
//         equivalent_sets_helper(a, b)
//     }
// }

// fn equivalent_sets_helper<A>(a: &dyn Set<A>, b: &dyn Set<A>) -> bool {
//     a.is_subset_of(b) && b.is_subset_of(a)
// }



struct Iter<T> {
    _boo: PhantomData<T>
}

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

const DEFAULT_IS_EMPTY_HINT: bool = false;

use std::marker::PhantomData;

pub use universe::Universe as UniversalSet;
pub use small_set::FiniteSet as SmallSet;
pub use predicate_set::PredicateSet as PredicateSet;
pub use hash_set::*;

mod small_set {
    use super::*;
    pub struct FiniteSet<T> {
        elements: Vec<T>
    }
    
    impl<T> FiniteSet<T> {
        pub fn new(elements: Vec<T>) -> Self {
            FiniteSet { elements }
        }
    }
    
    impl<T: Eq> Set<T> for FiniteSet<T> {
        fn contains(&self, element: T) -> bool {
            self.elements.contains(&element)
        }

        fn is_empty(&self) -> bool {
            self.elements.is_empty()
        }

        fn cardinality(&self) -> Cardinality {
            Cardinality::Finite(self.elements.len())
        }

        
    }
}


mod predicate_set {
    use std::usize;

    use super::*;
    pub struct PredicateSet<T> {
        predicate: Box<dyn Fn(T) -> bool>,
        is_empty_hint: bool
    }
    
    impl<T> PredicateSet<T> {
        pub fn new(predicate: Box<dyn Fn(T) -> bool>, is_empty_hint: Option<bool>) -> Self {
            PredicateSet { predicate, is_empty_hint: is_empty_hint.unwrap_or(DEFAULT_IS_EMPTY_HINT) }
        }
    }
    
    impl<T> Set<T> for PredicateSet<T> {
        fn contains(&self, element: T) -> bool {
            (self.predicate)(element)
        }

        fn is_empty(&self) -> bool {
            self.is_empty_hint
        }

        fn cardinality(&self) -> Cardinality {
            todo!();
            Cardinality::Infinite
        }
    }
}


mod hash_set {
    use super::*;
    use std::collections::HashSet;
    use std::hash::Hash;

    impl<T: Eq + Hash> Set<T> for HashSet<T>{
        fn contains(&self, element: T) -> bool {
            self.contains(&element)
        }

        fn is_empty(&self) -> bool {
            self.is_empty()
        }

        fn cardinality(&self) -> Cardinality {
            Cardinality::Finite(self.len())
        }
    }
}


mod universe {
    // For now this always returns true, but when used in an algebraic object, the type of T will be constrained, and it will act as a universal set for that type.
    use super::*;
    pub struct Universe<T> {
        cardinality: Cardinality,
        _boo: PhantomData<T>
    }

    impl<T> Universe<T> {
        pub fn new(cardinality: Cardinality) -> Self {
            Universe { cardinality , _boo: PhantomData }
        }
    }

    impl<T: Eq> Set<T> for Universe<T> {
        fn contains(&self, _element: T) -> bool {
            true
        }

        fn is_empty(&self) -> bool {
            false
        }

        fn cardinality(&self) -> Cardinality {
            self.cardinality
        }
    }
}


mod empty_set {
    use super::*;
    pub type EmptySet = !;


    impl<T: Eq> Set<T> for EmptySet {
        fn contains(&self, _element: T) -> bool {
            false
        }

        fn is_empty(&self) -> bool {
            true
        }

        fn cardinality(&self) -> Cardinality {
            Cardinality::Finite(0)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_small_set() {
        let set = small_set::FiniteSet::new(vec![1, 2, 3]);
        assert!(set.contains(1));
        assert!(!set.contains(4));
    }

    #[test]
    fn test_predicate_set() {
        let set = predicate_set::PredicateSet::new(Box::new(|x| x % 2 == 0), Some(false));
        assert!(set.contains(2));
        assert!(!set.contains(3));
    }

    #[test]
    fn test_hash_set() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert!(Set::contains(&set, 1));
        assert!(!Set::contains(&set, 4));
    }

    #[test]
    fn test_universe() {
        let set = UniversalSet::new(Cardinality::Infinite);
        assert!(set.contains(1));
        //assert!(set.contains("test"));
    }
}
