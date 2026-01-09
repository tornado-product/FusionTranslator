use crate::fusion_translator::async_translator::{
    AsyncTranslator, Language, TranslationListOutput, TranslationOutput,
};
use crate::fusion_translator::translator_error::TranslatorError;
use reqwest::Client;
use serde_json::Value;

/// 阿里翻译器实现
///
/// 通过调用阿里巴巴翻译API实现文本翻译功能
pub struct AlibabaTranslator {
    /// 文本翻译的最大长度限制
    input_limit: u32,
    /// API请求地址
    host: String,
    /// HTTP客户端
    client: Client,
}

/// 默认实现
impl Default for AlibabaTranslator {
    fn default() -> Self {
        AlibabaTranslator::new()
    }
}

/// 检查输入文本长度是否超出限制
///
/// # 参数
/// - `query`: 待检查的文本
/// - `input_limit`: 最大长度限制
///
/// # 返回值
/// - `Ok(())` - 长度在限制范围内
/// - `Err(TranslatorError::RequestToLong)` - 长度超出限制
pub fn input_limit_checker(query: &str, input_limit: u32) -> Result<(), TranslatorError> {
    if query.len() > input_limit as usize {
        return Err(TranslatorError::RequestToLong(
            query.len() as u32,
            input_limit,
        ));
    }
    Ok(())
}

#[async_trait::async_trait]
impl AsyncTranslator for AlibabaTranslator {
    /// 判断是否为本地翻译器
    ///
    /// 阿里翻译器需要调用远程API，返回false
    fn local(&self) -> bool {
        false
    }

    /// 翻译单个文本
    ///
    /// # 参数
    /// - `query`: 待翻译的文本
    /// - `from`: 源语言，None表示自动检测
    /// - `to`: 目标语言
    ///
    /// # 返回值
    /// 翻译结果
    async fn translate(
        &self,
        query: &str,
        from: Option<Language>,
        to: &Language,
    ) -> anyhow::Result<TranslationOutput> {
        input_limit_checker(query, self.input_limit)?;
        let _from_orig = from;
        let _from = match _from_orig {
            Some(lang) => lang
                .to_mymemory_short()
                .ok_or(TranslatorError::UnknownLanguage(lang))?,
            None => "auto",
        };

        let url = format!(
            "{}?domain={}&query={}&srcLang={}&tgtLang={}",
            self.host,
            "general",
            query,
            _from,
            to.to_mymemory_short()
                .ok_or(TranslatorError::UnknownLanguage(*to))?
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(TranslatorError::RequestFailed(response.status().as_u16()).into());
        }

        let resp: Value = response.json().await?;
        let resp = &resp["data"];
        let _lang = resp["detectLanguage"].to_string();
        let mut text = resp["translateText"].to_string();

        if text == "null" {
            return Err(TranslatorError::NoResponse.into());
        }

        if text.starts_with('"') && text.ends_with('"') {
            text = text[1..text.len() - 1].to_string();
        }

        Ok(TranslationOutput {
            text,
            lang: Some(*to),
        })
    }

    /// 翻译多个文本
    ///
    /// # 参数
    /// - `query`: 待翻译的文本数组
    /// - `from`: 源语言，None表示自动检测
    /// - `to`: 目标语言
    ///
    /// # 返回值
    /// 翻译结果数组
    async fn translate_vec(
        &self,
        query: &[String],
        from: Option<Language>,
        to: &Language,
    ) -> anyhow::Result<TranslationListOutput> {
        let t = self.translate(&query.join("_._._"), from, to).await?;
        Ok(TranslationListOutput {
            text: t
                .text
                .split("_._._")
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            lang: t.lang,
        })
    }
}

impl AlibabaTranslator {
    /// 创建新的阿里翻译器实例
    ///
    /// # 返回值
    /// 新的翻译器实例
    #[allow(dead_code)]
    pub fn new() -> Self {
        AlibabaTranslator {
            client: Client::new(),
            input_limit: 500,
            host: "https://translate.alibaba.com/api/translate/text".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fusion_translator::alibaba_translator::AlibabaTranslator;
    use crate::fusion_translator::async_translator::AsyncTranslator;

    /// 测试创建翻译器实例
    #[tokio::test]
    async fn test_create_translator() {
        let translator = AlibabaTranslator::new();
        assert!(!translator.local());
    }

    /// 测试输入长度检查
    #[test]
    fn test_input_limit_checker() {
        use crate::fusion_translator::alibaba_translator::input_limit_checker;

        let result = input_limit_checker("short text", 500);
        assert!(result.is_ok());

        let long_text = "a".repeat(600);
        let result = input_limit_checker(&long_text, 500);
        assert!(result.is_err());
    }

    /// 测试默认实现
    #[test]
    fn test_default() {
        let translator = AlibabaTranslator::default();
        assert!(!translator.local());
    }
}
