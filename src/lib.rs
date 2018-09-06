#![warn(missing_docs)]
#![feature(non_ascii_idents, box_syntax)]
//! Γενόμενον
extern crate failure;
#[macro_use]
extern crate serde_derive;

mod word;

pub use word::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
