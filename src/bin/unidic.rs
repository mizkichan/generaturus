//! Unidic の辞書 (lex.csv) をもとに Genomenon 用の辞書を作成するツール。
//! - 文語活用の除去
//! - くだけた音便形の除去
//! - 記号類の除去
//! - 固有名詞の除去
#![feature(nll, non_ascii_idents)]
extern crate csv;
extern crate failure;
extern crate genomenon;
extern crate regex;
extern crate rmp_serde;
extern crate serde_json;
extern crate structopt;
extern crate unicode_normalization;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use failure::{err_msg, Error};
use genomenon::Word;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use unicode_normalization::UnicodeNormalization;

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
    #[structopt(
        short = "f",
        long = "format",
        default_value = "msgpack",
        parse(try_from_str = "Format::from_str")
    )]
    format: Format,

    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

#[derive(Debug)]
enum Format {
    MessagePack,
    JSON,
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Format, Error> {
        match s {
            "msgpack" => Ok(Format::MessagePack),
            "json" => Ok(Format::JSON),
            _ => Err(err_msg(format!("Invalid string: {}", s))),
        }
    }
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&opt.input)?;

    let mut records = HashMap::<_, Vec<_>>::new();
    eprint!("Reading {:?} ... ", opt.input);
    for result in reader.deserialize::<Record>() {
        let record = result?;
        if is_bad_record(&record) {
            continue;
        }

        let key = (
            record.pos1,
            record.pos2,
            record.pos3,
            record.pos4,
            record.c_form,
            record.pron,
            record.lemma_id,
        );

        // Normalization
        let mut surface_form = record.surface_form;
        if !unicode_normalization::is_nfc(&surface_form) {
            surface_form = surface_form.nfc().collect();
        }

        if let Some(surface_forms) = records.get_mut(&key) {
            surface_forms.push(surface_form);
        } else {
            records.insert(key, vec![surface_form]);
        }
    }
    eprintln!("done!");

    eprint!("Collecting words ... ");
    let mut dictionary = Vec::new();
    for ((pos1, pos2, pos3, pos4, c_form, pron, ..), surface_forms) in records {
        let mut surface_forms = surface_forms.into_iter().collect::<Vec<_>>();
        surface_forms.sort_unstable();
        surface_forms.dedup();
        dictionary.push(Word::new(
            surface_forms,
            pron,
            &pos1,
            &pos2,
            &pos3,
            &pos4,
            &c_form,
        )?);
    }
    dictionary.sort_unstable();
    eprintln!("done! The dictionary has {} entries.", dictionary.len());

    eprint!("Writing out into {:?} ... ", opt.output);
    let mut file = File::create(&opt.output)?;
    match opt.format {
        Format::MessagePack => rmp_serde::encode::write(&mut file, &dictionary)?,
        Format::JSON => serde_json::to_writer_pretty(file, &dictionary)?,
    }
    eprintln!("done!");

    Ok(())
}

fn is_bad_record(
    &Record {
        ref surface_form,
        ref pos1,
        ref c_type,
        ref c_form,
        ref goshu,
        ref kana,
        ref form,
        ref pron,
        ..
    }: &Record,
) -> bool {
    lazy_static! {
        static ref allowed_characters: Regex = Regex::new(
            r"(?x)^[
                \u{4e00}-\u{9fef}  # CJK統合漢字
                \u{3400}-\u{4db5}  # CJK統合漢字拡張A
                \u{f900}-\u{faff}  # CJK互換漢字 (後で正規化すること)
                \u{3040}-\u{309f}  # 平仮名
                \u{30a0}-\u{30ff}  # 片仮名
                \u{3005}  # 々
                &&[^
                    \u{308e}  # ゎ
                    \u{3090}  # ゐ
                    \u{3091}  # ゑ
                    \u{3094}-\u{309f}  # ゔ/ゕ/ゖ/゛/゜ゝ/ゞ/ゟ
                    \u{30c2}  # ヂ
                    \u{30ee}  # ヮ
                    \u{30f0}-\u{30f2}  # ヰ/ヱ/ヲ
                    \u{30f5}-\u{30fa}  # ヵ/ヶ/ヷ/ヸ/ヹ/ヺ
                    \u{30fd}-\u{30ff}  # ヽ/ヾ/ヿ
                ]
            ]+$"
        ).unwrap();
    }
    lazy_static! {
        static ref all_kanji: Regex =
            Regex::new(r"^[\u{4e00}-\u{9fef}\u{3400}-\u{4db5}\u{f900}-\u{faff}]$").unwrap();
    }
    lazy_static! {
        static ref ugly_patterns: Regex = Regex::new(r"(?x)
            [^\u{30a0}-\u{30ff}][＝・ー]|[＝・ー][^\u{30a0}-\u{30ff}]  # カタカナ以外と一緒に使われる二重ハイフン/中黒/長音符
            | っっ
            | ーー
        ").unwrap();
    }
    lazy_static! {
        static ref 意志推量形仮名フィルタ: Regex = Regex::new(
            r"(?x)
            | (?:[アサカタナハマヤラワガザダナバパ][ウフ]$)
            | (?:[オコソトノホモヨロヲゴゾドノボポ]フ$)
        "
        ).unwrap();
    }
    lazy_static! {
        static ref 意志推量形発音フィルタ: Regex =
            Regex::new(r"[オコソトノホモヨロヲゴゾドノボポォョッン]$").unwrap();
    }

    if !allowed_characters.is_match(surface_form) {
        return true;
    }

    if ugly_patterns.is_match(surface_form) {
        return true;
    }

    if goshu == "記号" {
        return true;
    }

    if pos1 == "感動詞" {
        return true;
    }

    if c_type.starts_with("文語") {
        return true;
    }

    if c_form == "仮定形-融合"
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
    {
        return true;
    }

    match (
        surface_form == kana,
        surface_form == form,
        surface_form == pron,
        kana == form,
        kana == pron,
        form == pron,
    ) {
        //  全一致のカタカナ語
        (true, true, true, true, true, true) => (),
        // 仮名出現形のみ不一致のカタカナ語。
        // 仮名出現形の転写ミスで、↑と同一視してよい
        (false, true, true, false, false, true) => (),
        //  表層形のみ不一致
        (false, false, false, true, true, true) => {
            // 汚いハックだけど辞書が間違ってんだから仕方ない
            if c_form == "命令形"
                && ((kana != "ヘヨ" && kana.ends_with("ヘヨ"))
                    || (kana != "ヘロ" && kana.ends_with("ヘロ")))
            {
                return true;
            }
        }
        // 発音のみ不一致のカタカナ語
        (true, true, false, true, false, false) => (),
        // 表層形と語形出現形のみ一致 (「アロウズ」「ノーマライゼイション」のみ)
        // 仮名出現形の転写ミスで、↑と同一視してよい
        (false, true, false, false, false, false) => (),
        // 仮名出現形と語形出現形のみ一致
        (false, false, false, true, false, false) => {
            // 汚いハックだけど辞書が間違ってんだから仕方ない
            if (c_form == "終止形-一般" || c_form == "連体形-一般")
                && kana.ends_with('フ')
            {
                return true;
            }

            if c_form == "意志推量形"
                && pron.ends_with('ー')
                && 意志推量形仮名フィルタ.is_match(kana)
            {
                return true;
            }

            // 悲惨だけど仕方ない
            if surface_form == "出やう" {
                return true;
            }
        }

        // 表層形と仮名出現形、語形出現形と発音がそれぞれ一致のカタカナ語
        // 表層形と仮名出現形のみ一致のカタカナ語
        (true, false, false, false, false, true) | (true, false, false, false, false, false) => {
            // 「クラヴィア」「クラビア」のように許容可能なものと、
            // 「センチユリー」のように許容不能なものがある
        }

        // 語形出現形と発音のみ一致
        (false, false, false, false, false, true) => {
            // 旧仮名遣いがほとんどだが、表層形が漢字のみの語彙については許容可能
            // もっとましな方法ないの
            if !all_kanji.is_match(surface_form) {
                return true;
            }
        }

        // 語形出現形のみ不一致のカタカナ語
        // 表層形と発音、仮名出現形と語形出現形がそれぞれ一致のカタカナ語
        // (仮名出現形の転写ミスで、同一視してよい)
        (true, false, true, false, true, false) | (false, false, true, true, false, false) => {
            // 「ビョーキ」のようなものが多く許容しない方が良いかも知れない
            return true;
        }

        // 仮名出現形と発音のみ一致
        // 全て不一致
        (false, false, false, false, true, false) | (false, false, false, false, false, false) => {
            // たぶん許容不能
            return true;
        }

        otherwise => panic!("Unknown pattern: {:?}", otherwise),
    }

    if c_form == "意志推量形" && 意志推量形発音フィルタ.is_match(pron) {
        return true;
    }

    false
}
