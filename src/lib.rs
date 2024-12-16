
#![feature(generic_const_exprs)]
#![feature(trait_upcasting)]
#![allow(dead_code)]
#![feature(trait_alias)]
#![feature(adt_const_params)]
#![feature(lazy_get)]

pub mod ops;
pub mod set;
pub mod algebraic_objects;
mod util;
pub mod mod_ints;
pub mod permutations;

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
