#![warn(missing_docs)]
#![feature(non_ascii_idents, box_syntax)]
//! Γενόμενον
extern crate failure;
extern crate rand;

#[macro_use]
extern crate serde_derive;

mod symbol;
mod word;

pub use symbol::*;
pub use word::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
