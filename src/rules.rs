use rand;
use rand::seq::IteratorRandom;
use Word;
use WordKind;

use std::fmt;

/// 生成規則
pub trait Rule<T> {
    /// 生成します。
    fn generate(&self, dict: &[Word]) -> T;
}

/// 終端/非終端記号
pub trait Symbol: fmt::Debug {}

/// 句構造
#[derive(Debug)]
pub struct Phrase {
    kind: RuleKind,
    children: Vec<Box<dyn Symbol>>,
}

impl Phrase {
    /// 生成規則と子要素から句構造を作成します。
    pub fn new(kind: RuleKind, children: Vec<Box<dyn Symbol>>) -> Phrase {
        Phrase { kind, children }
    }
}

impl Symbol for Phrase {}

/// 生成規則
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum RuleKind {
    名詞,
    格助詞句,
}

impl Rule<Phrase> for RuleKind {
    fn generate(&self, dict: &[Word]) -> Phrase {
        let mut rng = rand::thread_rng();
        let children = match self {
            RuleKind::名詞 => vec![
                box [
                    WordKind::名詞_助動詞語幹,
                    WordKind::名詞_固有名詞_一般,
                    WordKind::名詞_固有名詞_人名_一般,
                    WordKind::名詞_固有名詞_人名_名,
                    WordKind::名詞_固有名詞_人名_姓,
                    WordKind::名詞_固有名詞_地名_一般,
                    WordKind::名詞_固有名詞_地名_国,
                    WordKind::名詞_数詞,
                    WordKind::名詞_普通名詞_サ変可能,
                    WordKind::名詞_普通名詞_サ変形状詞可能,
                    WordKind::名詞_普通名詞_一般,
                    WordKind::名詞_普通名詞_副詞可能,
                    WordKind::名詞_普通名詞_助数詞可能,
                    WordKind::名詞_普通名詞_形状詞可能,
                ]
                    .iter()
                    .choose(&mut rng)
                    .unwrap()
                    .generate(dict) as Box<dyn Symbol>,
            ],

            RuleKind::格助詞句 => vec![
                box RuleKind::名詞.generate(dict) as Box<dyn Symbol>,
                box WordKind::助詞_格助詞.generate(dict) as Box<dyn Symbol>,
            ],
        };

        Phrase::new(self.clone(), children)
    }
}
