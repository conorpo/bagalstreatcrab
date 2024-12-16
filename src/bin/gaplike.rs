use bagalstreatcrab::permutations::*;
use bagalstreatcrab::ops::*;
use bagalstreatcrab::set::*;
use bagalstreatcrab::algebraic_objects::*;

pub fn main() {
    let r = Permutation::<6>::new_from_disjoint(vec![vec![1,3,4,5,6]]);
    let s = Permutation::<6>::new_from_disjoint(vec![vec![1,3,2]]);

    let group = symmetric_group::<6>();
    assert!(group.get_set().contains(&r));
    assert!(group.get_set().contains(&s));
    assert!(group.order_of_element(r) == 5);
    assert!(group.order_of_element(s) == 3);

}