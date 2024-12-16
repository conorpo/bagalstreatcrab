use std::borrow::BorrowMut;
use std::rc::Rc;
use std::sync::{LazyLock, Arc, RwLock};
use crate::ops::*;
use crate::set::{UniversalSet, Cardinality};
use std::collections::HashMap;
use crate::algebraic_objects::*;
use itertools::Itertools;

#[derive(Clone, Eq)]
pub struct Permutation<const N: usize> {
    perm: Arc<Vec<usize>>,
}


impl<const N: usize> Permutation<N> {
    pub fn try_new(mut perm: Vec<usize>) -> Result<Permutation<N>, ()> {
        let n = perm.len();

        if n != N {
            return Err(());
        }

        let mut visited = vec![false; n];


        for &i in perm.iter() {
            if i >= n {
                return Err(());
            }

            if visited[i] {
                return Err(());
            }
            
            visited[i] = true;
        }

        Ok(Permutation {
            perm: Arc::new(perm)
        })
    }

    pub fn new_unchecked(perm: Vec<usize>) -> Permutation<N> {
        Permutation {
            perm: Arc::new(perm),
        }
    }

    pub fn new_from_disjoint(cycles: Vec<Vec<usize>>) -> Permutation<N> {
        let id: Permutation<N> = self::Identity::identity();
        let mut res = (*id.perm).clone();
        let mut visited = vec![false; N];
        for cycle in cycles {
            for i in 0..cycle.len() {
                if visited[cycle[i] - 1] {
                    panic!("Cycle has repeated elements");
                }
                res[cycle[i] - 1] = cycle[(i + 1) % cycle.len()] - 1;
                visited[cycle[i] - 1] = true;
            }
        }

        Permutation {
            perm: Arc::new(res),
        }
    }
}


impl<const N: usize> PartialEq for Permutation<N> {
    fn eq(&self, other: &Self) -> bool {
        self.perm == other.perm
    }
}


impl<const N: usize> Op<OP<1>> for Permutation<N> {
    fn op(&self, other: Permutation<N>) -> Self {
        let perm = self.perm.iter().map(|&x| other.perm[x]).collect();
        Permutation {
            perm: Arc::new(perm),
        }
    }
}

impl<const N: usize> Associative<OP<1>> for Permutation<N> {}




static IDENTITY_MAP: LazyLock<RwLock<HashMap<usize, Arc<Vec<usize>>>>> = LazyLock::new(|| { RwLock::new(HashMap::new())});


impl<const N: usize> Identity<OP<1>> for Permutation<N> {
    fn identity() -> Self {
        Permutation {
            perm: (LazyLock::force(&IDENTITY_MAP)).write().expect("Could not get lock").entry(N).or_insert_with(|| Arc::new((0..N).collect())).clone()
        }
    }
}

impl<const N: usize> Inverse<OP<1>> for Permutation<N> {
    fn inverse(&self) -> Self {
        let mut inv = vec![0; N];
        for (i, &x) in self.perm.iter().enumerate() {
            inv[x] = i;
        }

        Permutation {
            perm: Arc::new(inv),
        }
    }
}

pub fn symmetric_group<const N: usize>() -> Group<OP<1>, Permutation<N>> {
    let set = UniversalSet::new(Cardinality::Finite(N));
    Group::new_unchecked(Rc::new(set))
}



