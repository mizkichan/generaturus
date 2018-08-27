extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate structopt;

use failure::Error;
use std::collections::BTreeSet;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Deserialize)]
struct Record {
    surface_form: String,
    left_context_id: u16,
    right_context_id: u16,
    cost: i16,
    pos1: String,
    pos2: String,
    pos3: String,
    pos4: String,
    c_type: String,
    c_form: String,
    l_form: String,
    lemma: String,
    orth: String,
    pron: String,
    orth_base: String,
    pron_base: String,
    goshu: String,
    i_type: String,
    i_form: String,
    f_type: String,
    f_form: String,
    i_con_type: String,
    f_con_type: String,
    type_: String,
    kana: String,
    kana_base: String,
    form: String,
    form_base: String,
    a_type: String,
    a_con_type: String,
    a_mod_type: String,
    lid: u64,
    lemma_id: u32,
}

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(opt.input)?;

    let mut set = BTreeSet::new();
    for result in reader.deserialize::<Record>() {
        let record = result?;
        set.insert((
            record.pos1,
            record.pos2,
            record.pos3,
            record.pos4,
            record.c_form,
        ));
    }

    for value in set {
        let (pos1, pos2, pos3, pos4, c_form) = value;
        let (c_form1, c_form2) = c_form
            .find('-')
            .map(|i| c_form.split_at(i))
            .map(|t| (t.0, Some(&t.1[1..])))
            .unwrap_or((&c_form, None));
        print!("{}\t{}\t{}\t{}\t{}", pos1, pos2, pos3, pos4, c_form1,);
        if let Some(c_form2) = c_form2 {
            print!("\t{}", c_form2);
        }
        println!();
    }

    Ok(())
}
