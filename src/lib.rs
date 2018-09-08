#![warn(missing_docs)]
#![feature(non_ascii_idents, box_syntax)]
//! Γενόμενον
extern crate failure;
extern crate rand;

#[macro_use]
extern crate serde_derive;

mod phrase;
mod rule;
mod word;

pub use phrase::*;
pub use rule::*;
pub use word::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
