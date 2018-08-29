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
use std::collections::HashMap;
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
        .from_path(&opt.input)?;

    let mut dictionary = HashMap::new();

    eprint!("Reading {:?} ... ", opt.input);
    for result in reader.deserialize::<Record>() {
        let record = result?;
        if record.goshu == "記号"
            || record.pos1 == "感動詞"
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
            || (record.pos1 == "名詞"
                && (record.surface_form.contains("うぃ")
                    || record.surface_form.contains("うぇ")
                    || record.surface_form.contains("うぉ")))
            || (record.c_form == "意志推量形"
                && (record.surface_form.ends_with("はう")
                    || record.surface_form.ends_with("へよ")
                    || record.surface_form.ends_with("まう")
                    || record.surface_form.ends_with('っ')
                    || record.surface_form.ends_with('ふ')))
            || (record.c_form.ends_with("促音便") && record.surface_form.ends_with('ッ'))
            || (record.pos1 != "助詞" && record.surface_form.contains('を'))
            || record.surface_form.contains("ぁー")
            || record.surface_form.contains("ぃー")
            || record.surface_form.contains("ぅー")
            || record.surface_form.contains("ぇー")
            || record.surface_form.contains("ぉー")
            || record.surface_form.contains("っっ")
            || record.surface_form.contains("ーー")
            || record.surface_form.contains("っっ")
            || record.surface_form.chars().any(|c| {
                c.is_ascii()
                    || c == 'ぢ'
                    || c == 'ゐ'
                    || c == 'ゑ'
                    || c == 'ヂ'
                    || c == 'ヰ'
                    || c == 'ヱ'
                    || c == 'ヲ'
                    || c == '\u{2010}' // ハイフン
                    || c == '\u{2167}' // ローマ数字の8
                    || ('\u{3000}' <= c && c <= '\u{303f}' && c != '々') // 約物
                    || ('\u{309b}' <= c && c <= '\u{309e}') // 濁点/半濁点/繰り返し記号
                    || ('\u{30f7}' <= c && c <= '\u{30ff}' && c != 'ー') // (ワ/ヰ/ヱ/ヲ)の濁音/中黒/繰り返し記号/コト記号
                    || ('\u{ff00}' <= c && c <= '\u{ffef}') // 全角英数
            }) {
            continue;
        }
        dictionary.insert(
            record.lid,
            Word::new(
                &record.surface_form,
                &record.pos1,
                &record.pos2,
                &record.pos3,
                &record.pos4,
                &record.c_form,
            )?,
        );
    }
    eprintln!("done! The dictionary has {} entries.", dictionary.len());

    eprint!("Writing out into {:?} ... ", opt.output);
    let dictionary = dictionary.values().collect::<Vec<_>>();
    rmp_serde::encode::write(&mut File::create(&opt.output)?, &dictionary)?;
    eprintln!("done!");

    Ok(())
}
