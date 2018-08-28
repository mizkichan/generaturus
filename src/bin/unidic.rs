//! Unidic の辞書 (lex.csv) をもとに Genomenon 用の辞書を作成するツール。
//! - 文語活用の除去
//! - くだけた音便形の除去
//! - 記号類の除去
//! - 固有名詞の除去
extern crate csv;
extern crate failure;
extern crate genomenon;
extern crate rmp_serde;
extern crate structopt;
#[macro_use]
extern crate serde_derive;

use failure::Error;
use genomenon::Word;
use std::fs::File;
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

    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(opt.input)?;

    let mut dictionary = Vec::new();
    for result in reader.deserialize::<Record>() {
        let record = result?;
        if record.goshu == "記号"
            || record.pos2 == "固有名詞"
            || record.c_type.starts_with("文語")
            || record.c_form == "仮定形-融合"
            || record.c_form == "未然形-撥音便"
            || record.c_form == "終止形-促音便"
            || record.c_form == "終止形-撥音便"
            || record.c_form == "終止形-融合"
            || record.c_form == "連体形-ウ音便"
            || record.c_form == "連体形-撥音便"
            || record.c_form == "連体形-省略"
            || record.c_form == "連体形-補助"
            || record.c_form == "連用形-ウ音便"
            || record.c_form == "連用形-ニ"
            || record.c_form == "連用形-省略"
            || record.c_form == "連用形-融合"
        {
            continue;
        }
        dictionary.push(Word::new(
            &record.surface_form,
            &record.pos1,
            &record.pos2,
            &record.pos3,
            &record.pos4,
            &record.c_form,
        )?);
    }

    rmp_serde::encode::write(&mut File::create(opt.output)?, &dictionary)?;

    Ok(())
}
