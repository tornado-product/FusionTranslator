use crate::fusion_translator::async_translator::{AsyncTranslator, Language, TranslationListOutput, TranslationOutput};
use crate::fusion_translator::translator_error::TranslatorError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// 彩云翻译器API请求结构
///
/// 用于构建发送给彩云翻译API的JSON请求
#[derive(Serialize)]
#[allow(dead_code)]
struct CaiyunRequest<'a> {
    /// 翻译类型，格式为"源语言2目标语言"
    trans_type: String,
    /// 待翻译的文本数组
    source: &'a [String],
    /// 是否自动检测语言
    #[serde(skip_serializing_if = "Option::is_none")]
    detect: Option<bool>,
    /// 请求ID
    request_id: &'a str,
}

/// 彩云翻译器API响应结构
///
/// 用于解析彩云翻译API返回的JSON响应
#[derive(Deserialize)]
#[allow(dead_code)]
struct CaiyunResponse {
    /// 翻译结果数组
    target: Option<Vec<String>>,
}

/// 彩云翻译器实现
///
/// 通过调用彩云科技翻译API实现文本翻译功能
pub struct CaiyunTranslator {
    /// HTTP客户端
    client: Client,
    /// API访问令牌
    token: String,
    /// 请求ID
    request_id: String,
}

#[async_trait::async_trait]
impl AsyncTranslator for CaiyunTranslator {
    /// 判断是否为本地翻译器
    ///
    /// 彩云翻译器需要调用远程API，返回false
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
        let mut v = self
            .translate_vec(&[query.to_owned()], from, to)
            .await?;
        Ok(TranslationOutput {
            text: v.text.remove(0),
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
        let f = from;
        let from = match from {
            Some(from) => from.to_caiyun().ok_or(TranslatorError::UnknownLanguage(from))?,
            None => "auto",
        };

        let trans_type = format!(
            "{}2{}",
            from,
            to.to_caiyun().ok_or(TranslatorError::UnknownLanguage(*to))?
        );

        let request = CaiyunRequest {
            trans_type,
            source: query,
            detect: if f.is_none() { Some(true) } else { None },
            request_id: &self.request_id,
        };

        let data: CaiyunResponse = self
            .client
            .post("https://api.interpreter.caiyunai.com/v1/translator")
            .header("content-type", "application/json")
            .header("x-authorization", format!("token {}", self.token))
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(TranslationListOutput {
            text: data.target.unwrap_or_default(),
            lang: None,
        })
    }
}

impl CaiyunTranslator {
    /// 创建新的彩云翻译器实例
    ///
    /// # 参数
    /// - `token`: 彩云API访问令牌
    /// - `request_id`: 请求ID
    ///
    /// # 返回值
    /// 新的翻译器实例
    pub fn new(token: &str, request_id: &str) -> Self {
        Self {
            client: Client::new(),
            token: token.to_string(),
            request_id: request_id.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fusion_translator::async_translator::{AsyncTranslator, Language};
    use crate::fusion_translator::caiyun_translator::CaiyunTranslator;

    /// 测试创建翻译器实例
    #[tokio::test]
    async fn test_create_translator() {
        let translator = CaiyunTranslator::new("test_token", "test_request_id");
        assert!(!translator.local());
    }

    /// 测试创建带默认请求ID的翻译器
    #[tokio::test]
    async fn test_new_with_default_request_id() {
        let translator = CaiyunTranslator::new("test_token", "demo");
        assert!(!translator.local());
        assert_eq!(translator.request_id, "demo");
    }

    /// 测试翻译器字段值
    #[tokio::test]
    async fn test_translator_fields() {
        let token = "my_token";
        let request_id = "my_request_id";
        let translator = CaiyunTranslator::new(token, request_id);

        assert_eq!(translator.token, token);
        assert_eq!(translator.request_id, request_id);
    }

    /// 测试中译英实际翻译
    ///
    /// 使用真实API测试中文翻译成英文
    /// 注意：此测试会计入API调用次数
    #[cfg(test)]
    #[tokio::test]
    async fn test_translate_chinese_to_english() {
        dotenv::dotenv().ok();
        let token = std::env::var("CAIYUN_TOKEN").expect("请设置 CAIYUN_TOKEN 环境变量");
        let request_id = std::env::var("CAIYUN_REQUEST_ID").unwrap_or_else(|_| "demo".to_string());
        let translator = CaiyunTranslator::new(&token, &request_id);
        let result = translator
            .translate("你好世界", Some(Language::Chinese), &Language::English)
            .await
            .expect("翻译失败");
        assert!(!result.text.is_empty());
        println!("中译英结果: {}", result.text);
    }

    /// 测试英译中实际翻译
    ///
    /// 使用真实API测试英文翻译成中文
    /// 注意：此测试会计入API调用次数
    #[cfg(test)]
    #[tokio::test]
    async fn test_translate_english_to_chinese() {
        dotenv::dotenv().ok();
        let token = std::env::var("CAIYUN_TOKEN").expect("请设置 CAIYUN_TOKEN 环境变量");
        let request_id = std::env::var("CAIYUN_REQUEST_ID").unwrap_or_else(|_| "demo".to_string());
        let translator = CaiyunTranslator::new(&token, &request_id);
        let result = translator
            .translate("Hello World", Some(Language::English), &Language::Chinese)
            .await
            .expect("翻译失败");
        assert!(!result.text.is_empty());
        println!("英译中结果: {}", result.text);
    }
}
