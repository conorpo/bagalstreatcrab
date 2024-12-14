#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(nonzero_internals)]
#![feature(trait_upcasting)]
#![feature(never_type)]

mod ops;
mod set;
mod algebraic_objects;
mod util;

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
