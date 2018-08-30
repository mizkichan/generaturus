//! Unidic の辞書 (lex.csv) をもとに Genomenon 用の辞書を作成するツール。
//! - 文語活用の除去
//! - くだけた音便形の除去
//! - 記号類の除去
//! - 固有名詞の除去
extern crate csv;
extern crate failure;
extern crate genomenon;
extern crate regex;
extern crate rmp_serde;
extern crate structopt;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use failure::Error;
use genomenon::Word;
use regex::Regex;
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
        if is_bad_record(&record) {
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

fn is_bad_record(
    &Record {
        ref surface_form,
        ref pos1,
        ref pos2,
        ref c_type,
        ref c_form,
        ref goshu,
        ..
    }: &Record,
) -> bool {
    lazy_static! {
        static ref filter_pattern: Regex = Regex::new(
            r"(?x)
            [[:ascii:]]
            | (?:う[ぁ-ぉ])
            | (?:[ぁ-ぉ][ぁ-ぉ])
            | [ぢゐゑヂヰヱヲヵ゛゜ゝゞヽヾ]
            | [^\u{30a1}-\u{30ff}]+[＝・ー][^\u{30a1}-\u{30ff}]*  # カタカナ以外と一緒に使われる [＝・ー]

            | \u{2010}  # ハイフン
            | [\u{2150}-\u{218f}]  # ローマ数字など
            | [\u{3000}-\u{303f}--々]  # 約物 ('々' を除く)
            | [\u{ff00}-\u{ffef}]  # 全角英数, 半角カナ等
        ",
        ).unwrap();
    }

    goshu == "記号"
        || pos1 == "感動詞"
        || pos2 == "固有名詞"
        || c_type.starts_with("文語")
        || c_form == "仮定形-融合"
        || c_form == "未然形-撥音便"
        || c_form == "終止形-促音便"
        || c_form == "終止形-撥音便"
        || c_form == "終止形-融合"
        || c_form == "連体形-ウ音便"
        || c_form == "連体形-撥音便"
        || c_form == "連体形-省略"
        || c_form == "連体形-補助"
        || c_form == "連用形-ウ音便"
        || c_form == "連用形-ニ"
        || c_form == "連用形-省略"
        || c_form == "連用形-融合"
        || (c_form == "意志推量形"
            && (surface_form.ends_with("はう")
                || surface_form.ends_with("へよ")
                || surface_form.ends_with("まう")
                || surface_form.ends_with('っ')
                || surface_form.ends_with('ふ')))
        || (pos1 != "助詞" && surface_form.contains('を'))
        || (pos1 != "名詞" && surface_form.contains('ヶ'))
        || filter_pattern.is_match(&surface_form)
        || ((pos1 == "副詞" || pos1 == "接尾辞") && {
            let l = surface_form.chars().count();
            let ci = surface_form.char_indices();
            l >= 3
                && (0..=l - 3)
                    .map(|i| {
                        let begin = ci.clone().nth(i).unwrap().0;
                        let end = ci
                            .clone()
                            .nth(i + 3)
                            .map(|t| t.0)
                            .unwrap_or(surface_form.len());
                        &surface_form[begin..end]
                    }).any(|s| s.chars().zip(s.chars().skip(1)).all(|(a, b)| a == b))
        })
}
