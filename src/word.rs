//! 単語
#![allow(missing_docs)]
use failure::{err_msg, Error};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct WordBase {
    pron: String,
    surface_forms: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 代名詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 副詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_仮定形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_命令形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_意志推量形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_未然形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_終止形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_語幹_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_連体形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_連用形_イ音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_連用形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_連用形_促音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助動詞_連用形_撥音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞_係助詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞_副助詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞_接続助詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞_格助詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞_準体助詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 助詞_終助詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_仮定形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_命令形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_意志推量形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_未然形_セ(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_未然形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_終止形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_連体形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_連用形_イ音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_連用形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_連用形_促音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_一般_連用形_撥音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_仮定形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_命令形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_意志推量形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_未然形_サ(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_未然形_セ(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_未然形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_終止形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_連体形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_連用形_イ音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_連用形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_連用形_促音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 動詞_非自立可能_連用形_撥音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_助動詞語幹(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_固有名詞_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_固有名詞_人名_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_固有名詞_人名_名(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_固有名詞_人名_姓(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_固有名詞_地名_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_固有名詞_地名_国(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_数詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_普通名詞_サ変可能(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_普通名詞_サ変形状詞可能(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_普通名詞_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_普通名詞_副詞可能(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_普通名詞_助数詞可能(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 名詞_普通名詞_形状詞可能(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_一般_仮定形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_一般_意志推量形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_一般_終止形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_一般_語幹_サ(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_一般_語幹_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_一般_連体形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_一般_連用形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_一般_連用形_促音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_非自立可能_仮定形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_非自立可能_意志推量形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_非自立可能_終止形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_非自立可能_語幹_サ(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_非自立可能_語幹_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_非自立可能_連体形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_非自立可能_連用形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形容詞_非自立可能_連用形_促音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形状詞_タリ(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形状詞_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 形状詞_助動詞語幹(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_動詞的_仮定形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_動詞的_命令形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_動詞的_意志推量形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_動詞的_未然形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_動詞的_終止形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_動詞的_連体形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_動詞的_連用形_イ音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_動詞的_連用形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_動詞的_連用形_促音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_動詞的_連用形_撥音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_名詞的_サ変可能(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_名詞的_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_名詞的_副詞可能(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_名詞的_助数詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_形容詞的_仮定形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_形容詞的_意志推量形(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_形容詞的_終止形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_形容詞的_語幹_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_形容詞的_連体形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_形容詞的_連用形_一般(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_形容詞的_連用形_促音便(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接尾辞_形状詞的(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接続詞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 接頭辞(WordBase);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct 連体詞(WordBase);

/// 単語
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Word {
    代名詞(代名詞),
    副詞(副詞),
    助動詞_仮定形_一般(助動詞_仮定形_一般),
    助動詞_命令形(助動詞_命令形),
    助動詞_意志推量形(助動詞_意志推量形),
    助動詞_未然形_一般(助動詞_未然形_一般),
    助動詞_終止形_一般(助動詞_終止形_一般),
    助動詞_語幹_一般(助動詞_語幹_一般),
    助動詞_連体形_一般(助動詞_連体形_一般),
    助動詞_連用形_イ音便(助動詞_連用形_イ音便),
    助動詞_連用形_一般(助動詞_連用形_一般),
    助動詞_連用形_促音便(助動詞_連用形_促音便),
    助動詞_連用形_撥音便(助動詞_連用形_撥音便),
    助詞_係助詞(助詞_係助詞),
    助詞_副助詞(助詞_副助詞),
    助詞_接続助詞(助詞_接続助詞),
    助詞_格助詞(助詞_格助詞),
    助詞_準体助詞(助詞_準体助詞),
    助詞_終助詞(助詞_終助詞),
    動詞_一般_仮定形_一般(動詞_一般_仮定形_一般),
    動詞_一般_命令形(動詞_一般_命令形),
    動詞_一般_意志推量形(動詞_一般_意志推量形),
    動詞_一般_未然形_セ(動詞_一般_未然形_セ),
    動詞_一般_未然形_一般(動詞_一般_未然形_一般),
    動詞_一般_終止形_一般(動詞_一般_終止形_一般),
    動詞_一般_連体形_一般(動詞_一般_連体形_一般),
    動詞_一般_連用形_イ音便(動詞_一般_連用形_イ音便),
    動詞_一般_連用形_一般(動詞_一般_連用形_一般),
    動詞_一般_連用形_促音便(動詞_一般_連用形_促音便),
    動詞_一般_連用形_撥音便(動詞_一般_連用形_撥音便),
    動詞_非自立可能_仮定形_一般(動詞_非自立可能_仮定形_一般),
    動詞_非自立可能_命令形(動詞_非自立可能_命令形),
    動詞_非自立可能_意志推量形(動詞_非自立可能_意志推量形),
    動詞_非自立可能_未然形_サ(動詞_非自立可能_未然形_サ),
    動詞_非自立可能_未然形_セ(動詞_非自立可能_未然形_セ),
    動詞_非自立可能_未然形_一般(動詞_非自立可能_未然形_一般),
    動詞_非自立可能_終止形_一般(動詞_非自立可能_終止形_一般),
    動詞_非自立可能_連体形_一般(動詞_非自立可能_連体形_一般),
    動詞_非自立可能_連用形_イ音便(動詞_非自立可能_連用形_イ音便),
    動詞_非自立可能_連用形_一般(動詞_非自立可能_連用形_一般),
    動詞_非自立可能_連用形_促音便(動詞_非自立可能_連用形_促音便),
    動詞_非自立可能_連用形_撥音便(動詞_非自立可能_連用形_撥音便),
    名詞_助動詞語幹(名詞_助動詞語幹),
    名詞_固有名詞_一般(名詞_固有名詞_一般),
    名詞_固有名詞_人名_一般(名詞_固有名詞_人名_一般),
    名詞_固有名詞_人名_名(名詞_固有名詞_人名_名),
    名詞_固有名詞_人名_姓(名詞_固有名詞_人名_姓),
    名詞_固有名詞_地名_一般(名詞_固有名詞_地名_一般),
    名詞_固有名詞_地名_国(名詞_固有名詞_地名_国),
    名詞_数詞(名詞_数詞),
    名詞_普通名詞_サ変可能(名詞_普通名詞_サ変可能),
    名詞_普通名詞_サ変形状詞可能(名詞_普通名詞_サ変形状詞可能),
    名詞_普通名詞_一般(名詞_普通名詞_一般),
    名詞_普通名詞_副詞可能(名詞_普通名詞_副詞可能),
    名詞_普通名詞_助数詞可能(名詞_普通名詞_助数詞可能),
    名詞_普通名詞_形状詞可能(名詞_普通名詞_形状詞可能),
    形容詞_一般_仮定形_一般(形容詞_一般_仮定形_一般),
    形容詞_一般_意志推量形(形容詞_一般_意志推量形),
    形容詞_一般_終止形_一般(形容詞_一般_終止形_一般),
    形容詞_一般_語幹_サ(形容詞_一般_語幹_サ),
    形容詞_一般_語幹_一般(形容詞_一般_語幹_一般),
    形容詞_一般_連体形_一般(形容詞_一般_連体形_一般),
    形容詞_一般_連用形_一般(形容詞_一般_連用形_一般),
    形容詞_一般_連用形_促音便(形容詞_一般_連用形_促音便),
    形容詞_非自立可能_仮定形_一般(形容詞_非自立可能_仮定形_一般),
    形容詞_非自立可能_意志推量形(形容詞_非自立可能_意志推量形),
    形容詞_非自立可能_終止形_一般(形容詞_非自立可能_終止形_一般),
    形容詞_非自立可能_語幹_サ(形容詞_非自立可能_語幹_サ),
    形容詞_非自立可能_語幹_一般(形容詞_非自立可能_語幹_一般),
    形容詞_非自立可能_連体形_一般(形容詞_非自立可能_連体形_一般),
    形容詞_非自立可能_連用形_一般(形容詞_非自立可能_連用形_一般),
    形容詞_非自立可能_連用形_促音便(形容詞_非自立可能_連用形_促音便),
    形状詞_タリ(形状詞_タリ),
    形状詞_一般(形状詞_一般),
    形状詞_助動詞語幹(形状詞_助動詞語幹),
    接尾辞_動詞的_仮定形_一般(接尾辞_動詞的_仮定形_一般),
    接尾辞_動詞的_命令形(接尾辞_動詞的_命令形),
    接尾辞_動詞的_意志推量形(接尾辞_動詞的_意志推量形),
    接尾辞_動詞的_未然形_一般(接尾辞_動詞的_未然形_一般),
    接尾辞_動詞的_終止形_一般(接尾辞_動詞的_終止形_一般),
    接尾辞_動詞的_連体形_一般(接尾辞_動詞的_連体形_一般),
    接尾辞_動詞的_連用形_イ音便(接尾辞_動詞的_連用形_イ音便),
    接尾辞_動詞的_連用形_一般(接尾辞_動詞的_連用形_一般),
    接尾辞_動詞的_連用形_促音便(接尾辞_動詞的_連用形_促音便),
    接尾辞_動詞的_連用形_撥音便(接尾辞_動詞的_連用形_撥音便),
    接尾辞_名詞的_サ変可能(接尾辞_名詞的_サ変可能),
    接尾辞_名詞的_一般(接尾辞_名詞的_一般),
    接尾辞_名詞的_副詞可能(接尾辞_名詞的_副詞可能),
    接尾辞_名詞的_助数詞(接尾辞_名詞的_助数詞),
    接尾辞_形容詞的_仮定形_一般(接尾辞_形容詞的_仮定形_一般),
    接尾辞_形容詞的_意志推量形(接尾辞_形容詞的_意志推量形),
    接尾辞_形容詞的_終止形_一般(接尾辞_形容詞的_終止形_一般),
    接尾辞_形容詞的_語幹_一般(接尾辞_形容詞的_語幹_一般),
    接尾辞_形容詞的_連体形_一般(接尾辞_形容詞的_連体形_一般),
    接尾辞_形容詞的_連用形_一般(接尾辞_形容詞的_連用形_一般),
    接尾辞_形容詞的_連用形_促音便(接尾辞_形容詞的_連用形_促音便),
    接尾辞_形状詞的(接尾辞_形状詞的),
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
        let word = WordBase {
            pron,
            surface_forms,
        };
        Ok(match (pos1, pos2, pos3, pos4, c_form) {
            ("代名詞", "*", "*", "*", "*") => Word::代名詞(代名詞(word)),
            ("副詞", "*", "*", "*", "*") => Word::副詞(副詞(word)),
            ("助動詞", "*", "*", "*", "仮定形-一般") => {
                Word::助動詞_仮定形_一般(助動詞_仮定形_一般(word))
            }
            ("助動詞", "*", "*", "*", "命令形") => {
                Word::助動詞_命令形(助動詞_命令形(word))
            }
            ("助動詞", "*", "*", "*", "意志推量形") => {
                Word::助動詞_意志推量形(助動詞_意志推量形(word))
            }
            ("助動詞", "*", "*", "*", "未然形-一般") => {
                Word::助動詞_未然形_一般(助動詞_未然形_一般(word))
            }
            ("助動詞", "*", "*", "*", "終止形-一般") => {
                Word::助動詞_終止形_一般(助動詞_終止形_一般(word))
            }
            ("助動詞", "*", "*", "*", "語幹-一般") => {
                Word::助動詞_語幹_一般(助動詞_語幹_一般(word))
            }
            ("助動詞", "*", "*", "*", "連体形-一般") => {
                Word::助動詞_連体形_一般(助動詞_連体形_一般(word))
            }
            ("助動詞", "*", "*", "*", "連用形-イ音便") => {
                Word::助動詞_連用形_イ音便(助動詞_連用形_イ音便(word))
            }
            ("助動詞", "*", "*", "*", "連用形-一般") => {
                Word::助動詞_連用形_一般(助動詞_連用形_一般(word))
            }
            ("助動詞", "*", "*", "*", "連用形-促音便") => {
                Word::助動詞_連用形_促音便(助動詞_連用形_促音便(word))
            }
            ("助動詞", "*", "*", "*", "連用形-撥音便") => {
                Word::助動詞_連用形_撥音便(助動詞_連用形_撥音便(word))
            }
            ("助詞", "係助詞", "*", "*", "*") => {
                Word::助詞_係助詞(助詞_係助詞(word))
            }
            ("助詞", "副助詞", "*", "*", "*") => {
                Word::助詞_副助詞(助詞_副助詞(word))
            }
            ("助詞", "接続助詞", "*", "*", "*") => {
                Word::助詞_接続助詞(助詞_接続助詞(word))
            }
            ("助詞", "格助詞", "*", "*", "*") => {
                Word::助詞_格助詞(助詞_格助詞(word))
            }
            ("助詞", "準体助詞", "*", "*", "*") => {
                Word::助詞_準体助詞(助詞_準体助詞(word))
            }
            ("助詞", "終助詞", "*", "*", "*") => {
                Word::助詞_終助詞(助詞_終助詞(word))
            }
            ("動詞", "一般", "*", "*", "仮定形-一般") => {
                Word::動詞_一般_仮定形_一般(動詞_一般_仮定形_一般(word))
            }
            ("動詞", "一般", "*", "*", "命令形") => {
                Word::動詞_一般_命令形(動詞_一般_命令形(word))
            }
            ("動詞", "一般", "*", "*", "意志推量形") => {
                Word::動詞_一般_意志推量形(動詞_一般_意志推量形(word))
            }
            ("動詞", "一般", "*", "*", "未然形-セ") => {
                Word::動詞_一般_未然形_セ(動詞_一般_未然形_セ(word))
            }
            ("動詞", "一般", "*", "*", "未然形-一般") => {
                Word::動詞_一般_未然形_一般(動詞_一般_未然形_一般(word))
            }
            ("動詞", "一般", "*", "*", "終止形-一般") => {
                Word::動詞_一般_終止形_一般(動詞_一般_終止形_一般(word))
            }
            ("動詞", "一般", "*", "*", "連体形-一般") => {
                Word::動詞_一般_連体形_一般(動詞_一般_連体形_一般(word))
            }
            ("動詞", "一般", "*", "*", "連用形-イ音便") => {
                Word::動詞_一般_連用形_イ音便(動詞_一般_連用形_イ音便(word))
            }
            ("動詞", "一般", "*", "*", "連用形-一般") => {
                Word::動詞_一般_連用形_一般(動詞_一般_連用形_一般(word))
            }
            ("動詞", "一般", "*", "*", "連用形-促音便") => {
                Word::動詞_一般_連用形_促音便(動詞_一般_連用形_促音便(word))
            }
            ("動詞", "一般", "*", "*", "連用形-撥音便") => {
                Word::動詞_一般_連用形_撥音便(動詞_一般_連用形_撥音便(word))
            }
            ("動詞", "非自立可能", "*", "*", "仮定形-一般") => {
                Word::動詞_非自立可能_仮定形_一般(
                    動詞_非自立可能_仮定形_一般(word),
                )
            }
            ("動詞", "非自立可能", "*", "*", "命令形") => {
                Word::動詞_非自立可能_命令形(動詞_非自立可能_命令形(word))
            }
            ("動詞", "非自立可能", "*", "*", "意志推量形") => {
                Word::動詞_非自立可能_意志推量形(
                    動詞_非自立可能_意志推量形(word),
                )
            }
            ("動詞", "非自立可能", "*", "*", "未然形-サ") => {
                Word::動詞_非自立可能_未然形_サ(動詞_非自立可能_未然形_サ(
                    word,
                ))
            }
            ("動詞", "非自立可能", "*", "*", "未然形-セ") => {
                Word::動詞_非自立可能_未然形_セ(動詞_非自立可能_未然形_セ(
                    word,
                ))
            }
            ("動詞", "非自立可能", "*", "*", "未然形-一般") => {
                Word::動詞_非自立可能_未然形_一般(
                    動詞_非自立可能_未然形_一般(word),
                )
            }
            ("動詞", "非自立可能", "*", "*", "終止形-一般") => {
                Word::動詞_非自立可能_終止形_一般(
                    動詞_非自立可能_終止形_一般(word),
                )
            }
            ("動詞", "非自立可能", "*", "*", "連体形-一般") => {
                Word::動詞_非自立可能_連体形_一般(
                    動詞_非自立可能_連体形_一般(word),
                )
            }
            ("動詞", "非自立可能", "*", "*", "連用形-イ音便") => {
                Word::動詞_非自立可能_連用形_イ音便(
                    動詞_非自立可能_連用形_イ音便(word),
                )
            }
            ("動詞", "非自立可能", "*", "*", "連用形-一般") => {
                Word::動詞_非自立可能_連用形_一般(
                    動詞_非自立可能_連用形_一般(word),
                )
            }
            ("動詞", "非自立可能", "*", "*", "連用形-促音便") => {
                Word::動詞_非自立可能_連用形_促音便(
                    動詞_非自立可能_連用形_促音便(word),
                )
            }
            ("動詞", "非自立可能", "*", "*", "連用形-撥音便") => {
                Word::動詞_非自立可能_連用形_撥音便(
                    動詞_非自立可能_連用形_撥音便(word),
                )
            }
            ("名詞", "助動詞語幹", "*", "*", "*") => {
                Word::名詞_助動詞語幹(名詞_助動詞語幹(word))
            }
            ("名詞", "固有名詞", "一般", "*", "*") => {
                Word::名詞_固有名詞_一般(名詞_固有名詞_一般(word))
            }
            ("名詞", "固有名詞", "人名", "一般", "*") => {
                Word::名詞_固有名詞_人名_一般(名詞_固有名詞_人名_一般(word))
            }
            ("名詞", "固有名詞", "人名", "名", "*") => {
                Word::名詞_固有名詞_人名_名(名詞_固有名詞_人名_名(word))
            }
            ("名詞", "固有名詞", "人名", "姓", "*") => {
                Word::名詞_固有名詞_人名_姓(名詞_固有名詞_人名_姓(word))
            }
            ("名詞", "固有名詞", "地名", "一般", "*") => {
                Word::名詞_固有名詞_地名_一般(名詞_固有名詞_地名_一般(word))
            }
            ("名詞", "固有名詞", "地名", "国", "*") => {
                Word::名詞_固有名詞_地名_国(名詞_固有名詞_地名_国(word))
            }
            ("名詞", "数詞", "*", "*", "*") => Word::名詞_数詞(名詞_数詞(word)),
            ("名詞", "普通名詞", "サ変可能", "*", "*") => {
                Word::名詞_普通名詞_サ変可能(名詞_普通名詞_サ変可能(word))
            }
            ("名詞", "普通名詞", "サ変形状詞可能", "*", "*") => {
                Word::名詞_普通名詞_サ変形状詞可能(
                    名詞_普通名詞_サ変形状詞可能(word),
                )
            }
            ("名詞", "普通名詞", "一般", "*", "*") => {
                Word::名詞_普通名詞_一般(名詞_普通名詞_一般(word))
            }
            ("名詞", "普通名詞", "副詞可能", "*", "*") => {
                Word::名詞_普通名詞_副詞可能(名詞_普通名詞_副詞可能(word))
            }
            ("名詞", "普通名詞", "助数詞可能", "*", "*") => {
                Word::名詞_普通名詞_助数詞可能(名詞_普通名詞_助数詞可能(word))
            }
            ("名詞", "普通名詞", "形状詞可能", "*", "*") => {
                Word::名詞_普通名詞_形状詞可能(名詞_普通名詞_形状詞可能(word))
            }
            ("形容詞", "一般", "*", "*", "仮定形-一般") => {
                Word::形容詞_一般_仮定形_一般(形容詞_一般_仮定形_一般(word))
            }
            ("形容詞", "一般", "*", "*", "意志推量形") => {
                Word::形容詞_一般_意志推量形(形容詞_一般_意志推量形(word))
            }
            ("形容詞", "一般", "*", "*", "終止形-一般") => {
                Word::形容詞_一般_終止形_一般(形容詞_一般_終止形_一般(word))
            }
            ("形容詞", "一般", "*", "*", "語幹-サ") => {
                Word::形容詞_一般_語幹_サ(形容詞_一般_語幹_サ(word))
            }
            ("形容詞", "一般", "*", "*", "語幹-一般") => {
                Word::形容詞_一般_語幹_一般(形容詞_一般_語幹_一般(word))
            }
            ("形容詞", "一般", "*", "*", "連体形-一般") => {
                Word::形容詞_一般_連体形_一般(形容詞_一般_連体形_一般(word))
            }
            ("形容詞", "一般", "*", "*", "連用形-一般") => {
                Word::形容詞_一般_連用形_一般(形容詞_一般_連用形_一般(word))
            }
            ("形容詞", "一般", "*", "*", "連用形-促音便") => {
                Word::形容詞_一般_連用形_促音便(形容詞_一般_連用形_促音便(
                    word,
                ))
            }
            ("形容詞", "非自立可能", "*", "*", "仮定形-一般") => {
                Word::形容詞_非自立可能_仮定形_一般(
                    形容詞_非自立可能_仮定形_一般(word),
                )
            }
            ("形容詞", "非自立可能", "*", "*", "意志推量形") => {
                Word::形容詞_非自立可能_意志推量形(
                    形容詞_非自立可能_意志推量形(word),
                )
            }
            ("形容詞", "非自立可能", "*", "*", "終止形-一般") => {
                Word::形容詞_非自立可能_終止形_一般(
                    形容詞_非自立可能_終止形_一般(word),
                )
            }
            ("形容詞", "非自立可能", "*", "*", "語幹-サ") => {
                Word::形容詞_非自立可能_語幹_サ(形容詞_非自立可能_語幹_サ(
                    word,
                ))
            }
            ("形容詞", "非自立可能", "*", "*", "語幹-一般") => {
                Word::形容詞_非自立可能_語幹_一般(
                    形容詞_非自立可能_語幹_一般(word),
                )
            }
            ("形容詞", "非自立可能", "*", "*", "連体形-一般") => {
                Word::形容詞_非自立可能_連体形_一般(
                    形容詞_非自立可能_連体形_一般(word),
                )
            }
            ("形容詞", "非自立可能", "*", "*", "連用形-一般") => {
                Word::形容詞_非自立可能_連用形_一般(
                    形容詞_非自立可能_連用形_一般(word),
                )
            }
            ("形容詞", "非自立可能", "*", "*", "連用形-促音便") => {
                Word::形容詞_非自立可能_連用形_促音便(
                    形容詞_非自立可能_連用形_促音便(word),
                )
            }
            ("形状詞", "タリ", "*", "*", "*") => {
                Word::形状詞_タリ(形状詞_タリ(word))
            }
            ("形状詞", "一般", "*", "*", "*") => {
                Word::形状詞_一般(形状詞_一般(word))
            }
            ("形状詞", "助動詞語幹", "*", "*", "*") => {
                Word::形状詞_助動詞語幹(形状詞_助動詞語幹(word))
            }
            ("接尾辞", "動詞的", "*", "*", "仮定形-一般") => {
                Word::接尾辞_動詞的_仮定形_一般(接尾辞_動詞的_仮定形_一般(
                    word,
                ))
            }
            ("接尾辞", "動詞的", "*", "*", "命令形") => {
                Word::接尾辞_動詞的_命令形(接尾辞_動詞的_命令形(word))
            }
            ("接尾辞", "動詞的", "*", "*", "意志推量形") => {
                Word::接尾辞_動詞的_意志推量形(接尾辞_動詞的_意志推量形(word))
            }
            ("接尾辞", "動詞的", "*", "*", "未然形-一般") => {
                Word::接尾辞_動詞的_未然形_一般(接尾辞_動詞的_未然形_一般(
                    word,
                ))
            }
            ("接尾辞", "動詞的", "*", "*", "終止形-一般") => {
                Word::接尾辞_動詞的_終止形_一般(接尾辞_動詞的_終止形_一般(
                    word,
                ))
            }
            ("接尾辞", "動詞的", "*", "*", "連体形-一般") => {
                Word::接尾辞_動詞的_連体形_一般(接尾辞_動詞的_連体形_一般(
                    word,
                ))
            }
            ("接尾辞", "動詞的", "*", "*", "連用形-イ音便") => {
                Word::接尾辞_動詞的_連用形_イ音便(
                    接尾辞_動詞的_連用形_イ音便(word),
                )
            }
            ("接尾辞", "動詞的", "*", "*", "連用形-一般") => {
                Word::接尾辞_動詞的_連用形_一般(接尾辞_動詞的_連用形_一般(
                    word,
                ))
            }
            ("接尾辞", "動詞的", "*", "*", "連用形-促音便") => {
                Word::接尾辞_動詞的_連用形_促音便(
                    接尾辞_動詞的_連用形_促音便(word),
                )
            }
            ("接尾辞", "動詞的", "*", "*", "連用形-撥音便") => {
                Word::接尾辞_動詞的_連用形_撥音便(
                    接尾辞_動詞的_連用形_撥音便(word),
                )
            }
            ("接尾辞", "名詞的", "サ変可能", "*", "*") => {
                Word::接尾辞_名詞的_サ変可能(接尾辞_名詞的_サ変可能(word))
            }
            ("接尾辞", "名詞的", "一般", "*", "*") => {
                Word::接尾辞_名詞的_一般(接尾辞_名詞的_一般(word))
            }
            ("接尾辞", "名詞的", "副詞可能", "*", "*") => {
                Word::接尾辞_名詞的_副詞可能(接尾辞_名詞的_副詞可能(word))
            }
            ("接尾辞", "名詞的", "助数詞", "*", "*") => {
                Word::接尾辞_名詞的_助数詞(接尾辞_名詞的_助数詞(word))
            }
            ("接尾辞", "形容詞的", "*", "*", "仮定形-一般") => {
                Word::接尾辞_形容詞的_仮定形_一般(
                    接尾辞_形容詞的_仮定形_一般(word),
                )
            }
            ("接尾辞", "形容詞的", "*", "*", "意志推量形") => {
                Word::接尾辞_形容詞的_意志推量形(
                    接尾辞_形容詞的_意志推量形(word),
                )
            }
            ("接尾辞", "形容詞的", "*", "*", "終止形-一般") => {
                Word::接尾辞_形容詞的_終止形_一般(
                    接尾辞_形容詞的_終止形_一般(word),
                )
            }
            ("接尾辞", "形容詞的", "*", "*", "語幹-一般") => {
                Word::接尾辞_形容詞的_語幹_一般(接尾辞_形容詞的_語幹_一般(
                    word,
                ))
            }
            ("接尾辞", "形容詞的", "*", "*", "連体形-一般") => {
                Word::接尾辞_形容詞的_連体形_一般(
                    接尾辞_形容詞的_連体形_一般(word),
                )
            }
            ("接尾辞", "形容詞的", "*", "*", "連用形-一般") => {
                Word::接尾辞_形容詞的_連用形_一般(
                    接尾辞_形容詞的_連用形_一般(word),
                )
            }
            ("接尾辞", "形容詞的", "*", "*", "連用形-促音便") => {
                Word::接尾辞_形容詞的_連用形_促音便(
                    接尾辞_形容詞的_連用形_促音便(word),
                )
            }
            ("接尾辞", "形状詞的", "*", "*", "*") => {
                Word::接尾辞_形状詞的(接尾辞_形状詞的(word))
            }
            ("接続詞", "*", "*", "*", "*") => Word::接続詞(接続詞(word)),
            ("接頭辞", "*", "*", "*", "*") => Word::接頭辞(接頭辞(word)),
            ("連体詞", "*", "*", "*", "*") => Word::連体詞(連体詞(word)),
            (_, _, _, _, _) => {
                return Err(err_msg(format!(
                    "Invalid signature: {} {} {} {} {}",
                    pos1, pos2, pos3, pos4, c_form
                )))
            }
        })
    }
}
