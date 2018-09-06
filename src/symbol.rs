use Word;

/// 記号
pub trait Symbol {
    /// 生成するもの
    type Output;

    /// 生成します。
    fn generate(&self, dict: &[Word]) -> Self::Output;
}
