lang_generator::generate_language!();

/// 异步翻译器特征
///
/// 定义了翻译器的通用接口，支持单文本翻译和多文本批量翻译
/// 所有翻译器实现都必须实现此特征
#[async_trait::async_trait]
pub trait AsyncTranslator: Send + Sync {
    /// 判断是否为本地翻译器
    ///
    /// 本地翻译器不需要调用远程API，可以直接在本地进行翻译
    /// 远程翻译器需要调用外部API进行翻译
    ///
    /// # 返回值
    /// - `true`: 本地翻译器
    /// - `false`: 远程翻译器
    fn local(&self) -> bool;

    /// 翻译单个文本
    ///
    /// 将指定的文本从源语言翻译到目标语言
    ///
    /// # 参数
    /// - `query`: 待翻译的文本
    /// - `from`: 源语言，None表示自动检测语言
    /// - `to`: 目标语言
    ///
    /// # 返回值
    /// 翻译结果，包含翻译后的文本和检测到的语言
    #[allow(dead_code)]
    async fn translate(
        &self,
        query: &str,
        from: Option<Language>,
        to: &Language,
    ) -> anyhow::Result<TranslationOutput>;

    /// 翻译多个文本
    ///
    /// 批量将多个文本从源语言翻译到目标语言
    ///
    /// # 参数
    /// - `query`: 待翻译的文本数组
    /// - `from`: 源语言，None表示自动检测语言
    /// - `to`: 目标语言
    ///
    /// # 返回值
    /// 翻译结果列表，包含翻译后的文本数组和检测到的语言
    #[allow(dead_code)]
    async fn translate_vec(
        &self,
        query: &[String],
        from: Option<Language>,
        to: &Language,
    ) -> anyhow::Result<TranslationListOutput>;
}

/// 单文本翻译结果
///
/// 包含翻译后的文本和检测到的语言信息
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TranslationOutput {
    /// 翻译后的文本
    pub text: String,
    /// 文本语言
    pub lang: Option<Language>,
}

/// 多文本翻译结果
///
/// 包含翻译后的文本数组和检测到的语言信息
#[derive(Clone, Debug)]
pub struct TranslationListOutput {
    /// 翻译后的文本数组
    pub text: Vec<String>,
    /// 文本语言
    pub lang: Option<Language>,
}
