use Word;
/// 生成規則
pub trait Rule<T> {
    /// 生成します。
    fn generate(&self, dict: &[Word]) -> T;
}
