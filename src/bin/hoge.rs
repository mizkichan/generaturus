#![feature(custom_attribute, non_ascii_idents)]
extern crate failure;
extern crate genomenon;
extern crate rmp_serde;
extern crate structopt;

use failure::Error;
use genomenon::*;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let dict = rmp_serde::from_read::<_, Vec<Word>>(File::open(opt.input)?)?;

    for _ in 0..10 {
        println!("{:?}", RuleKind::格助詞句.generate(&dict))
    }

    Ok(())
}
