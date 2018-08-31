//! 単語
#![allow(missing_docs)]
use failure::{err_msg, Error};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 代名詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 副詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞仮定形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞命令形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞意志推量形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞未然形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞終止形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞語幹一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞連体形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞連用形イ音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞連用形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞連用形促音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞連用形撥音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞係助詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞副助詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞接続助詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞格助詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞準体助詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞終助詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般仮定形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般命令形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般意志推量形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般未然形セ {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般未然形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般終止形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般連体形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般連用形イ音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般連用形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般連用形促音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞一般連用形撥音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能仮定形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能命令形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能意志推量形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能未然形サ {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能未然形セ {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能未然形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能終止形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能連体形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能連用形イ音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能連用形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能連用形促音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞非自立可能連用形撥音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞助動詞語幹 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞固有名詞一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞固有名詞人名一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞固有名詞人名名 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞固有名詞人名姓 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞固有名詞地名一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞固有名詞地名国 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞数詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞普通名詞サ変可能 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞普通名詞サ変形状詞可能 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞普通名詞一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞普通名詞副詞可能 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞普通名詞助数詞可能 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞普通名詞形状詞可能 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞一般仮定形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞一般意志推量形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞一般終止形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞一般語幹サ {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞一般語幹一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞一般連体形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞一般連用形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞一般連用形促音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞非自立可能仮定形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞非自立可能意志推量形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞非自立可能終止形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞非自立可能語幹サ {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞非自立可能語幹一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞非自立可能連体形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞非自立可能連用形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞非自立可能連用形促音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形状詞タリ {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形状詞一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形状詞助動詞語幹 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞動詞的仮定形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞動詞的命令形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞動詞的意志推量形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞動詞的未然形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞動詞的終止形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞動詞的連体形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞動詞的連用形イ音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞動詞的連用形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞動詞的連用形促音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞動詞的連用形撥音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞名詞的サ変可能 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞名詞的一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞名詞的副詞可能 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞名詞的助数詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞形容詞的仮定形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞形容詞的意志推量形 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞形容詞的終止形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞形容詞的語幹一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞形容詞的連体形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞形容詞的連用形一般 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞形容詞的連用形促音便 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞形状詞的 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接続詞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接頭辞 {
    pron: String,
    surface_forms: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 連体詞 {
    pron: String,
    surface_forms: Vec<String>,
}

/// 単語
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Word {
    代名詞(代名詞),
    副詞(副詞),
    助動詞仮定形一般(助動詞仮定形一般),
    助動詞命令形(助動詞命令形),
    助動詞意志推量形(助動詞意志推量形),
    助動詞未然形一般(助動詞未然形一般),
    助動詞終止形一般(助動詞終止形一般),
    助動詞語幹一般(助動詞語幹一般),
    助動詞連体形一般(助動詞連体形一般),
    助動詞連用形イ音便(助動詞連用形イ音便),
    助動詞連用形一般(助動詞連用形一般),
    助動詞連用形促音便(助動詞連用形促音便),
    助動詞連用形撥音便(助動詞連用形撥音便),
    助詞係助詞(助詞係助詞),
    助詞副助詞(助詞副助詞),
    助詞接続助詞(助詞接続助詞),
    助詞格助詞(助詞格助詞),
    助詞準体助詞(助詞準体助詞),
    助詞終助詞(助詞終助詞),
    動詞一般仮定形一般(動詞一般仮定形一般),
    動詞一般命令形(動詞一般命令形),
    動詞一般意志推量形(動詞一般意志推量形),
    動詞一般未然形セ(動詞一般未然形セ),
    動詞一般未然形一般(動詞一般未然形一般),
    動詞一般終止形一般(動詞一般終止形一般),
    動詞一般連体形一般(動詞一般連体形一般),
    動詞一般連用形イ音便(動詞一般連用形イ音便),
    動詞一般連用形一般(動詞一般連用形一般),
    動詞一般連用形促音便(動詞一般連用形促音便),
    動詞一般連用形撥音便(動詞一般連用形撥音便),
    動詞非自立可能仮定形一般(動詞非自立可能仮定形一般),
    動詞非自立可能命令形(動詞非自立可能命令形),
    動詞非自立可能意志推量形(動詞非自立可能意志推量形),
    動詞非自立可能未然形サ(動詞非自立可能未然形サ),
    動詞非自立可能未然形セ(動詞非自立可能未然形セ),
    動詞非自立可能未然形一般(動詞非自立可能未然形一般),
    動詞非自立可能終止形一般(動詞非自立可能終止形一般),
    動詞非自立可能連体形一般(動詞非自立可能連体形一般),
    動詞非自立可能連用形イ音便(動詞非自立可能連用形イ音便),
    動詞非自立可能連用形一般(動詞非自立可能連用形一般),
    動詞非自立可能連用形促音便(動詞非自立可能連用形促音便),
    動詞非自立可能連用形撥音便(動詞非自立可能連用形撥音便),
    名詞助動詞語幹(名詞助動詞語幹),
    名詞固有名詞一般(名詞固有名詞一般),
    名詞固有名詞人名一般(名詞固有名詞人名一般),
    名詞固有名詞人名名(名詞固有名詞人名名),
    名詞固有名詞人名姓(名詞固有名詞人名姓),
    名詞固有名詞地名一般(名詞固有名詞地名一般),
    名詞固有名詞地名国(名詞固有名詞地名国),
    名詞数詞(名詞数詞),
    名詞普通名詞サ変可能(名詞普通名詞サ変可能),
    名詞普通名詞サ変形状詞可能(名詞普通名詞サ変形状詞可能),
    名詞普通名詞一般(名詞普通名詞一般),
    名詞普通名詞副詞可能(名詞普通名詞副詞可能),
    名詞普通名詞助数詞可能(名詞普通名詞助数詞可能),
    名詞普通名詞形状詞可能(名詞普通名詞形状詞可能),
    形容詞一般仮定形一般(形容詞一般仮定形一般),
    形容詞一般意志推量形(形容詞一般意志推量形),
    形容詞一般終止形一般(形容詞一般終止形一般),
    形容詞一般語幹サ(形容詞一般語幹サ),
    形容詞一般語幹一般(形容詞一般語幹一般),
    形容詞一般連体形一般(形容詞一般連体形一般),
    形容詞一般連用形一般(形容詞一般連用形一般),
    形容詞一般連用形促音便(形容詞一般連用形促音便),
    形容詞非自立可能仮定形一般(形容詞非自立可能仮定形一般),
    形容詞非自立可能意志推量形(形容詞非自立可能意志推量形),
    形容詞非自立可能終止形一般(形容詞非自立可能終止形一般),
    形容詞非自立可能語幹サ(形容詞非自立可能語幹サ),
    形容詞非自立可能語幹一般(形容詞非自立可能語幹一般),
    形容詞非自立可能連体形一般(形容詞非自立可能連体形一般),
    形容詞非自立可能連用形一般(形容詞非自立可能連用形一般),
    形容詞非自立可能連用形促音便(形容詞非自立可能連用形促音便),
    形状詞タリ(形状詞タリ),
    形状詞一般(形状詞一般),
    形状詞助動詞語幹(形状詞助動詞語幹),
    接尾辞動詞的仮定形一般(接尾辞動詞的仮定形一般),
    接尾辞動詞的命令形(接尾辞動詞的命令形),
    接尾辞動詞的意志推量形(接尾辞動詞的意志推量形),
    接尾辞動詞的未然形一般(接尾辞動詞的未然形一般),
    接尾辞動詞的終止形一般(接尾辞動詞的終止形一般),
    接尾辞動詞的連体形一般(接尾辞動詞的連体形一般),
    接尾辞動詞的連用形イ音便(接尾辞動詞的連用形イ音便),
    接尾辞動詞的連用形一般(接尾辞動詞的連用形一般),
    接尾辞動詞的連用形促音便(接尾辞動詞的連用形促音便),
    接尾辞動詞的連用形撥音便(接尾辞動詞的連用形撥音便),
    接尾辞名詞的サ変可能(接尾辞名詞的サ変可能),
    接尾辞名詞的一般(接尾辞名詞的一般),
    接尾辞名詞的副詞可能(接尾辞名詞的副詞可能),
    接尾辞名詞的助数詞(接尾辞名詞的助数詞),
    接尾辞形容詞的仮定形一般(接尾辞形容詞的仮定形一般),
    接尾辞形容詞的意志推量形(接尾辞形容詞的意志推量形),
    接尾辞形容詞的終止形一般(接尾辞形容詞的終止形一般),
    接尾辞形容詞的語幹一般(接尾辞形容詞的語幹一般),
    接尾辞形容詞的連体形一般(接尾辞形容詞的連体形一般),
    接尾辞形容詞的連用形一般(接尾辞形容詞的連用形一般),
    接尾辞形容詞的連用形促音便(接尾辞形容詞的連用形促音便),
    接尾辞形状詞的(接尾辞形状詞的),
    接続詞(接続詞),
    接頭辞(接頭辞),
    連体詞(連体詞),
}

impl Word {
    /// 表層形、品詞情報、変化形から単語を作成します。
    pub fn new(
        surface_forms: Vec<String>,
        pron: String,
        pos1: &str,
        pos2: &str,
        pos3: &str,
        pos4: &str,
        c_form: &str,
    ) -> Result<Word, Error> {
        Ok(match (pos1, pos2, pos3, pos4, c_form) {
            ("代名詞", "*", "*", "*", "*") => Word::代名詞(代名詞 {
                pron,
                surface_forms,
            }),
            ("副詞", "*", "*", "*", "*") => Word::副詞(副詞 {
                pron,
                surface_forms,
            }),
            ("助動詞", "*", "*", "*", "仮定形-一般") => {
                Word::助動詞仮定形一般(助動詞仮定形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("助動詞", "*", "*", "*", "命令形") => {
                Word::助動詞命令形(助動詞命令形 {
                    pron,
                    surface_forms,
                })
            }
            ("助動詞", "*", "*", "*", "意志推量形") => {
                Word::助動詞意志推量形(助動詞意志推量形 {
                    pron,
                    surface_forms,
                })
            }
            ("助動詞", "*", "*", "*", "未然形-一般") => {
                Word::助動詞未然形一般(助動詞未然形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("助動詞", "*", "*", "*", "終止形-一般") => {
                Word::助動詞終止形一般(助動詞終止形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("助動詞", "*", "*", "*", "語幹-一般") => {
                Word::助動詞語幹一般(助動詞語幹一般 {
                    pron,
                    surface_forms,
                })
            }
            ("助動詞", "*", "*", "*", "連体形-一般") => {
                Word::助動詞連体形一般(助動詞連体形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("助動詞", "*", "*", "*", "連用形-イ音便") => {
                Word::助動詞連用形イ音便(助動詞連用形イ音便 {
                    pron,
                    surface_forms,
                })
            }
            ("助動詞", "*", "*", "*", "連用形-一般") => {
                Word::助動詞連用形一般(助動詞連用形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("助動詞", "*", "*", "*", "連用形-促音便") => {
                Word::助動詞連用形促音便(助動詞連用形促音便 {
                    pron,
                    surface_forms,
                })
            }
            ("助動詞", "*", "*", "*", "連用形-撥音便") => {
                Word::助動詞連用形撥音便(助動詞連用形撥音便 {
                    pron,
                    surface_forms,
                })
            }
            ("助詞", "係助詞", "*", "*", "*") => Word::助詞係助詞(助詞係助詞 {
                pron,
                surface_forms,
            }),
            ("助詞", "副助詞", "*", "*", "*") => Word::助詞副助詞(助詞副助詞 {
                pron,
                surface_forms,
            }),
            ("助詞", "接続助詞", "*", "*", "*") => {
                Word::助詞接続助詞(助詞接続助詞 {
                    pron,
                    surface_forms,
                })
            }
            ("助詞", "格助詞", "*", "*", "*") => Word::助詞格助詞(助詞格助詞 {
                pron,
                surface_forms,
            }),
            ("助詞", "準体助詞", "*", "*", "*") => {
                Word::助詞準体助詞(助詞準体助詞 {
                    pron,
                    surface_forms,
                })
            }
            ("助詞", "終助詞", "*", "*", "*") => Word::助詞終助詞(助詞終助詞 {
                pron,
                surface_forms,
            }),
            ("動詞", "一般", "*", "*", "仮定形-一般") => {
                Word::動詞一般仮定形一般(動詞一般仮定形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "一般", "*", "*", "命令形") => {
                Word::動詞一般命令形(動詞一般命令形 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "一般", "*", "*", "意志推量形") => {
                Word::動詞一般意志推量形(動詞一般意志推量形 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "一般", "*", "*", "未然形-セ") => {
                Word::動詞一般未然形セ(動詞一般未然形セ {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "一般", "*", "*", "未然形-一般") => {
                Word::動詞一般未然形一般(動詞一般未然形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "一般", "*", "*", "終止形-一般") => {
                Word::動詞一般終止形一般(動詞一般終止形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "一般", "*", "*", "連体形-一般") => {
                Word::動詞一般連体形一般(動詞一般連体形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "一般", "*", "*", "連用形-イ音便") => {
                Word::動詞一般連用形イ音便(動詞一般連用形イ音便 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "一般", "*", "*", "連用形-一般") => {
                Word::動詞一般連用形一般(動詞一般連用形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "一般", "*", "*", "連用形-促音便") => {
                Word::動詞一般連用形促音便(動詞一般連用形促音便 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "一般", "*", "*", "連用形-撥音便") => {
                Word::動詞一般連用形撥音便(動詞一般連用形撥音便 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "非自立可能", "*", "*", "仮定形-一般") => {
                Word::動詞非自立可能仮定形一般(動詞非自立可能仮定形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "非自立可能", "*", "*", "命令形") => {
                Word::動詞非自立可能命令形(動詞非自立可能命令形 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "非自立可能", "*", "*", "意志推量形") => {
                Word::動詞非自立可能意志推量形(動詞非自立可能意志推量形 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "非自立可能", "*", "*", "未然形-サ") => {
                Word::動詞非自立可能未然形サ(動詞非自立可能未然形サ {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "非自立可能", "*", "*", "未然形-セ") => {
                Word::動詞非自立可能未然形セ(動詞非自立可能未然形セ {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "非自立可能", "*", "*", "未然形-一般") => {
                Word::動詞非自立可能未然形一般(動詞非自立可能未然形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "非自立可能", "*", "*", "終止形-一般") => {
                Word::動詞非自立可能終止形一般(動詞非自立可能終止形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "非自立可能", "*", "*", "連体形-一般") => {
                Word::動詞非自立可能連体形一般(動詞非自立可能連体形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "非自立可能", "*", "*", "連用形-イ音便") => {
                Word::動詞非自立可能連用形イ音便(
                    動詞非自立可能連用形イ音便 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("動詞", "非自立可能", "*", "*", "連用形-一般") => {
                Word::動詞非自立可能連用形一般(動詞非自立可能連用形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("動詞", "非自立可能", "*", "*", "連用形-促音便") => {
                Word::動詞非自立可能連用形促音便(
                    動詞非自立可能連用形促音便 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("動詞", "非自立可能", "*", "*", "連用形-撥音便") => {
                Word::動詞非自立可能連用形撥音便(
                    動詞非自立可能連用形撥音便 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("名詞", "助動詞語幹", "*", "*", "*") => {
                Word::名詞助動詞語幹(名詞助動詞語幹 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "固有名詞", "一般", "*", "*") => {
                Word::名詞固有名詞一般(名詞固有名詞一般 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "固有名詞", "人名", "一般", "*") => {
                Word::名詞固有名詞人名一般(名詞固有名詞人名一般 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "固有名詞", "人名", "名", "*") => {
                Word::名詞固有名詞人名名(名詞固有名詞人名名 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "固有名詞", "人名", "姓", "*") => {
                Word::名詞固有名詞人名姓(名詞固有名詞人名姓 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "固有名詞", "地名", "一般", "*") => {
                Word::名詞固有名詞地名一般(名詞固有名詞地名一般 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "固有名詞", "地名", "国", "*") => {
                Word::名詞固有名詞地名国(名詞固有名詞地名国 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "数詞", "*", "*", "*") => Word::名詞数詞(名詞数詞 {
                pron,
                surface_forms,
            }),
            ("名詞", "普通名詞", "サ変可能", "*", "*") => {
                Word::名詞普通名詞サ変可能(名詞普通名詞サ変可能 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "普通名詞", "サ変形状詞可能", "*", "*") => {
                Word::名詞普通名詞サ変形状詞可能(
                    名詞普通名詞サ変形状詞可能 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("名詞", "普通名詞", "一般", "*", "*") => {
                Word::名詞普通名詞一般(名詞普通名詞一般 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "普通名詞", "副詞可能", "*", "*") => {
                Word::名詞普通名詞副詞可能(名詞普通名詞副詞可能 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "普通名詞", "助数詞可能", "*", "*") => {
                Word::名詞普通名詞助数詞可能(名詞普通名詞助数詞可能 {
                    pron,
                    surface_forms,
                })
            }
            ("名詞", "普通名詞", "形状詞可能", "*", "*") => {
                Word::名詞普通名詞形状詞可能(名詞普通名詞形状詞可能 {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "一般", "*", "*", "仮定形-一般") => {
                Word::形容詞一般仮定形一般(形容詞一般仮定形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "一般", "*", "*", "意志推量形") => {
                Word::形容詞一般意志推量形(形容詞一般意志推量形 {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "一般", "*", "*", "終止形-一般") => {
                Word::形容詞一般終止形一般(形容詞一般終止形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "一般", "*", "*", "語幹-サ") => {
                Word::形容詞一般語幹サ(形容詞一般語幹サ {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "一般", "*", "*", "語幹-一般") => {
                Word::形容詞一般語幹一般(形容詞一般語幹一般 {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "一般", "*", "*", "連体形-一般") => {
                Word::形容詞一般連体形一般(形容詞一般連体形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "一般", "*", "*", "連用形-一般") => {
                Word::形容詞一般連用形一般(形容詞一般連用形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "一般", "*", "*", "連用形-促音便") => {
                Word::形容詞一般連用形促音便(形容詞一般連用形促音便 {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "非自立可能", "*", "*", "仮定形-一般") => {
                Word::形容詞非自立可能仮定形一般(
                    形容詞非自立可能仮定形一般 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("形容詞", "非自立可能", "*", "*", "意志推量形") => {
                Word::形容詞非自立可能意志推量形(
                    形容詞非自立可能意志推量形 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("形容詞", "非自立可能", "*", "*", "終止形-一般") => {
                Word::形容詞非自立可能終止形一般(
                    形容詞非自立可能終止形一般 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("形容詞", "非自立可能", "*", "*", "語幹-サ") => {
                Word::形容詞非自立可能語幹サ(形容詞非自立可能語幹サ {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "非自立可能", "*", "*", "語幹-一般") => {
                Word::形容詞非自立可能語幹一般(形容詞非自立可能語幹一般 {
                    pron,
                    surface_forms,
                })
            }
            ("形容詞", "非自立可能", "*", "*", "連体形-一般") => {
                Word::形容詞非自立可能連体形一般(
                    形容詞非自立可能連体形一般 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("形容詞", "非自立可能", "*", "*", "連用形-一般") => {
                Word::形容詞非自立可能連用形一般(
                    形容詞非自立可能連用形一般 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("形容詞", "非自立可能", "*", "*", "連用形-促音便") => {
                Word::形容詞非自立可能連用形促音便(
                    形容詞非自立可能連用形促音便 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("形状詞", "タリ", "*", "*", "*") => Word::形状詞タリ(形状詞タリ {
                pron,
                surface_forms,
            }),
            ("形状詞", "一般", "*", "*", "*") => Word::形状詞一般(形状詞一般 {
                pron,
                surface_forms,
            }),
            ("形状詞", "助動詞語幹", "*", "*", "*") => {
                Word::形状詞助動詞語幹(形状詞助動詞語幹 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "動詞的", "*", "*", "仮定形-一般") => {
                Word::接尾辞動詞的仮定形一般(接尾辞動詞的仮定形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "動詞的", "*", "*", "命令形") => {
                Word::接尾辞動詞的命令形(接尾辞動詞的命令形 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "動詞的", "*", "*", "意志推量形") => {
                Word::接尾辞動詞的意志推量形(接尾辞動詞的意志推量形 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "動詞的", "*", "*", "未然形-一般") => {
                Word::接尾辞動詞的未然形一般(接尾辞動詞的未然形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "動詞的", "*", "*", "終止形-一般") => {
                Word::接尾辞動詞的終止形一般(接尾辞動詞的終止形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "動詞的", "*", "*", "連体形-一般") => {
                Word::接尾辞動詞的連体形一般(接尾辞動詞的連体形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "動詞的", "*", "*", "連用形-イ音便") => {
                Word::接尾辞動詞的連用形イ音便(接尾辞動詞的連用形イ音便 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "動詞的", "*", "*", "連用形-一般") => {
                Word::接尾辞動詞的連用形一般(接尾辞動詞的連用形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "動詞的", "*", "*", "連用形-促音便") => {
                Word::接尾辞動詞的連用形促音便(接尾辞動詞的連用形促音便 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "動詞的", "*", "*", "連用形-撥音便") => {
                Word::接尾辞動詞的連用形撥音便(接尾辞動詞的連用形撥音便 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "名詞的", "サ変可能", "*", "*") => {
                Word::接尾辞名詞的サ変可能(接尾辞名詞的サ変可能 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "名詞的", "一般", "*", "*") => {
                Word::接尾辞名詞的一般(接尾辞名詞的一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "名詞的", "副詞可能", "*", "*") => {
                Word::接尾辞名詞的副詞可能(接尾辞名詞的副詞可能 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "名詞的", "助数詞", "*", "*") => {
                Word::接尾辞名詞的助数詞(接尾辞名詞的助数詞 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "形容詞的", "*", "*", "仮定形-一般") => {
                Word::接尾辞形容詞的仮定形一般(接尾辞形容詞的仮定形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "形容詞的", "*", "*", "意志推量形") => {
                Word::接尾辞形容詞的意志推量形(接尾辞形容詞的意志推量形 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "形容詞的", "*", "*", "終止形-一般") => {
                Word::接尾辞形容詞的終止形一般(接尾辞形容詞的終止形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "形容詞的", "*", "*", "語幹-一般") => {
                Word::接尾辞形容詞的語幹一般(接尾辞形容詞的語幹一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "形容詞的", "*", "*", "連体形-一般") => {
                Word::接尾辞形容詞的連体形一般(接尾辞形容詞的連体形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "形容詞的", "*", "*", "連用形-一般") => {
                Word::接尾辞形容詞的連用形一般(接尾辞形容詞的連用形一般 {
                    pron,
                    surface_forms,
                })
            }
            ("接尾辞", "形容詞的", "*", "*", "連用形-促音便") => {
                Word::接尾辞形容詞的連用形促音便(
                    接尾辞形容詞的連用形促音便 {
                        pron,
                        surface_forms,
                    },
                )
            }
            ("接尾辞", "形状詞的", "*", "*", "*") => {
                Word::接尾辞形状詞的(接尾辞形状詞的 {
                    pron,
                    surface_forms,
                })
            }
            ("接続詞", "*", "*", "*", "*") => Word::接続詞(接続詞 {
                pron,
                surface_forms,
            }),
            ("接頭辞", "*", "*", "*", "*") => Word::接頭辞(接頭辞 {
                pron,
                surface_forms,
            }),
            ("連体詞", "*", "*", "*", "*") => Word::連体詞(連体詞 {
                pron,
                surface_forms,
            }),
            (_, _, _, _, _) => {
                return Err(err_msg(format!(
                    "Invalid signature: {} {} {} {} {}",
                    pos1, pos2, pos3, pos4, c_form
                )))
            }
        })
    }
}
