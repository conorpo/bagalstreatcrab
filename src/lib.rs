
#![feature(generic_const_exprs)]
#![feature(trait_upcasting)]
#![allow(dead_code)]
#![feature(trait_alias)]
#![feature(adt_const_params)]

mod ops;
mod set;
mod algebraic_objects;
mod util;
mod mod_ints;

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
