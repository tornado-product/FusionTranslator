use std::time::{SystemTime, UNIX_EPOCH};
use crate::fusion_translator::async_translator::{AsyncTranslator, Language, TranslationListOutput, TranslationOutput};
use crate::fusion_translator::translator_error::TranslatorError;
use rand::Rng as _;
use reqwest::{Client, header::CONTENT_TYPE};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use uuid::{Context, Timestamp, Uuid};

/// 有道翻译器实现
///
/// 通过调用有道翻译API实现文本翻译功能
pub struct YoudaoTranslator {
    /// HTTP客户端
    client: reqwest::Client,
    /// 有道开放平台应用ID
    app_key: String,
    /// 有道开放平台应用密钥
    app_secret: String,
    /// UUID上下文，用于生成唯一请求ID
    context: Context,
    /// MAC地址，用于UUID生成
    mac: [u8; 6],
}

/// 生成随机MAC地址
///
/// 用于生成唯一的请求ID，设置的比特位确保生成合法的随机MAC地址
///
/// # 返回值
/// 长度为6的字节数组，表示MAC地址
fn generate_random_mac() -> [u8; 6] {
    let mut rng = rand::rng();
    let mut mac = [0u8; 6];
    rng.fill(&mut mac);

    mac[0] |= 0b00000010;
    mac[0] &= 0b11111110;

    mac
}

impl YoudaoTranslator {
    /// 创建新的有道翻译器实例
    ///
    /// # 参数
    /// - `app_key`: 有道开放平台应用ID
    /// - `app_secret`: 有道开放平台应用密钥
    ///
    /// # 返回值
    /// 新的翻译器实例
    pub fn new(app_key: &str, app_secret: &str) -> Self {
        let seed: u16 = rand::rng().random();
        Self {
            mac: generate_random_mac(),
            client: Client::new(),
            app_key: app_key.to_string(),
            app_secret: app_secret.to_string(),
            context: Context::new(seed),
        }
    }
}

/// SHA256哈希编码
///
/// 将输入字符串进行SHA256哈希运算并返回十六进制编码结果
///
/// # 参数
/// - `sign_str`: 待编码的字符串
///
/// # 返回值
/// SHA256哈希值的十六进制字符串表示
#[allow(dead_code)]
fn sha256_encode(sign_str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(sign_str.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[async_trait::async_trait]
impl AsyncTranslator for YoudaoTranslator {
    /// 判断是否为本地翻译器
    ///
    /// 有道翻译器需要调用远程API，返回false
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
        let mut t = self
            .translate_vec(&[query.to_owned()], from, to)
            .await?;
        Ok(TranslationOutput {
            text: t.text.remove(0),
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
    /// 翻译结果列表
    async fn translate_vec(
        &self,
        query: &[String],
        from: Option<Language>,
        to: &Language,
    ) -> anyhow::Result<TranslationListOutput> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let curtime = now.as_secs();
        let nanos = now.subsec_nanos();
        let ts = Timestamp::from_unix(&self.context, curtime, nanos);
        let salt = Uuid::new_v1(ts, &self.mac).to_string();
        let query = query.join("\n");
        let sign_str = format!(
            "{}{}{}{}{}",
            self.app_key,
            truncate(&query),
            salt,
            curtime,
            self.app_secret
        );
        let from = match from {
            Some(from) => from.to_youdao().ok_or(TranslatorError::UnknownLanguage(from))?,
            None => "auto",
        };
        let data: Resp = self
            .client
            .post("https://openapi.youdao.com/api")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&[
                ("from", from),
                ("to", to.to_youdao().ok_or(TranslatorError::UnknownLanguage(*to))?),
                ("signType", "v3"),
                ("curtime", &curtime.to_string()),
                ("appKey", self.app_key.as_str()),
                ("q", query.as_str()),
                ("salt", salt.as_str()),
                ("sign", &sha256_encode(&sign_str)),
            ])
            .send()
            .await?
            .json()
            .await?;
        Ok(TranslationListOutput {
            text: data
                .translation
                .into_iter()
                .flat_map(|v| v.split("/n").map(|v| v.to_owned()).collect::<Vec<String>>())
                .collect::<Vec<String>>(),
            lang: None,
        })
    }
}

/// API响应结构
///
/// 包含翻译结果列表
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Resp {
    /// 翻译结果列表
    translation: Vec<String>,
}

/// 文本截断处理
///
/// 根据有道翻译API的要求，对过长的文本进行截断处理
/// 规则：文本长度不超过20时保持原样，否则保留前10个字符、中间长度信息和后10个字符
///
/// # 参数
/// - `s`: 待处理的文本
///
/// # 返回值
/// 处理后的文本
#[allow(dead_code)]
fn truncate(s: &str) -> String {
    let size = s.len();
    if size <= 20 {
        s.to_string()
    } else {
        let start = &s[..10];
        let end = &s[size - 10..];
        format!("{}{}{}", start, size, end)
    }
}

#[cfg(test)]
mod tests {

    use crate::fusion_translator::async_translator::{AsyncTranslator as _, Language};
    use crate::fusion_translator::youdao_translator::{YoudaoTranslator, sha256_encode, truncate};

    /// 测试翻译器实例创建
    ///
    /// 验证构造函数是否正确初始化所有字段
    #[tokio::test]
    async fn test_new_translator() {
        let translator = YoudaoTranslator::new("test_app_key", "test_app_secret");
        assert_eq!(translator.app_key, "test_app_key");
        assert_eq!(translator.app_secret, "test_app_secret");
        assert_eq!(translator.mac.len(), 6);
    }

    /// 测试翻译器字段访问
    ///
    /// 验证可以正确访问翻译器的各个字段
    #[tokio::test]
    async fn test_translator_fields() {
        let translator = YoudaoTranslator::new("app_key_123", "app_secret_456");
        assert_eq!(translator.app_key, "app_key_123");
        assert_eq!(translator.app_secret, "app_secret_456");
    }

    /// 测试翻译器本地方法
    ///
    /// 有道翻译器需要调用远程API，应该返回false
    #[tokio::test]
    async fn test_translator_local() {
        let translator = YoudaoTranslator::new("test_key", "test_secret");
        assert!(!translator.local());
    }

    /// 测试翻译器MAC地址生成
    ///
    /// 验证MAC地址的长度是否符合预期
    #[tokio::test]
    async fn test_translator_mac() {
        let translator = YoudaoTranslator::new("key", "secret");
        assert_eq!(translator.mac.len(), 6);
    }

    /// 测试SHA256编码功能
    ///
    /// 验证字符串能否正确进行SHA256哈希编码
    #[test]
    fn test_sha256_encode() {
        let result = sha256_encode("test_string");
        assert_eq!(result.len(), 64);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// 测试SHA256编码一致性
    ///
    /// 相同的输入应该产生相同的哈希值
    #[test]
    fn test_sha256_consistency() {
        let result1 = sha256_encode("hello");
        let result2 = sha256_encode("hello");
        assert_eq!(result1, result2);
    }

    /// 测试SHA256编码差异性
    ///
    /// 不同的输入应该产生不同的哈希值
    #[test]
    fn test_sha256_different() {
        let result1 = sha256_encode("hello");
        let result2 = sha256_encode("world");
        assert_ne!(result1, result2);
    }

    /// 测试短文本截断
    ///
    /// 长度不超过20的文本应该保持原样
    #[test]
    fn test_truncate_short() {
        let result = truncate("hello world");
        assert_eq!(result, "hello world");
    }

    /// 测试长文本截断
    ///
    /// 长度超过20的文本应该被截断
    #[test]
    fn test_truncate_long() {
        let long_text = "this is a very long text that exceeds twenty characters";
        let result = truncate(long_text);
        assert!(result.len() <= 23);
        assert!(result.starts_with("this is a "));
        assert!(result.ends_with("characters"));
    }

    /// 测试空文本截断
    ///
    /// 空字符串应该返回空字符串
    #[test]
    fn test_truncate_empty() {
        let result = truncate("");
        assert_eq!(result, "");
    }

    /// 测试恰好20字符的文本截断
    ///
    /// 长度恰好为20的文本应该保持原样
    #[test]
    fn test_truncate_exactly_20() {
        let text = "12345678901234567890";
        let result = truncate(text);
        assert_eq!(result, text);
    }

    /// 测试截断文本包含长度信息
    ///
    /// 截断后的文本应该包含原始长度信息
    #[test]
    fn test_truncate_contains_length() {
        let long_text = "123456789012345678901";
        let result = truncate(long_text);
        assert!(result.contains("21"));
    }

    /// 测试语言代码映射功能
    ///
    /// 验证所有支持的语言代码都能正确映射
    #[tokio::test]
    async fn test_language_mapping() {
        let langs = [
            "ar", "de", "en", "es", "fr", "hi", "id", "it", "ja", "ko", "nl", "pt", "ru", "th",
            "vi", "zh-CHS", "zh-CHT", "af", "am", "az", "be", "bg", "bn", "bs", "ca", "ceb", "co",
            "cs", "cy", "da", "el", "eo", "et", "eu", "fa", "fi", "fj", "fy", "ga", "gd", "gl",
            "gu", "ha", "haw", "he", "hi", "hr", "ht", "hu", "hy", "ig", "is", "jw", "ka", "kk",
            "km", "kn", "ku", "ky", "la", "lb", "lo", "lt", "lv", "mg", "mi", "mk", "ml", "mn",
            "mr", "ms", "mt", "mww", "my", "ne", "nl", "no", "ny", "otq", "pa", "pl", "ps", "ro",
            "sd", "si", "sk", "sl", "sm", "sn", "so", "sq", "sr-Cyrl", "sr-Latn", "st", "su", "sv",
            "sw", "ta", "te", "tg", "tl", "tlh", "to", "tr", "ty", "uk", "ur", "uz", "xh", "yi",
            "yo", "yua", "yue", "zu",
        ];

        assert!(langs.len() > 0);
        for code in langs {
            Language::from_youdao(code).expect(code);
        }
    }

    /// 测试所有语言代码列表非空
    ///
    /// 验证有道翻译支持的语言代码列表是否包含元素
    #[tokio::test]
    async fn test_all_languages_available() {
        let langs = [
            "ar", "de", "en", "es", "fr", "hi", "id", "it", "ja", "ko", "nl", "pt", "ru", "th",
            "vi", "zh-CHS", "zh-CHT", "af", "am", "az", "be", "bg", "bn", "bs", "ca", "ceb", "co",
            "cs", "cy", "da", "el", "eo", "et", "eu", "fa", "fi", "fj", "fy", "ga", "gd", "gl",
            "gu", "ha", "haw", "he", "hi", "hr", "ht", "hu", "hy", "ig", "is", "jw", "ka", "kk",
            "km", "kn", "ku", "ky", "la", "lb", "lo", "lt", "lv", "mg", "mi", "mk", "ml", "mn",
            "mr", "ms", "mt", "mww", "my", "ne", "nl", "no", "ny", "otq", "pa", "pl", "ps", "ro",
            "sd", "si", "sk", "sl", "sm", "sn", "so", "sq", "sr-Cyrl", "sr-Latn", "st", "su", "sv",
            "sw", "ta", "te", "tg", "tl", "tlh", "to", "tr", "ty", "uk", "ur", "uz", "xh", "yi",
            "yo", "yua", "yue", "zu",
        ];

        assert!(langs.len() > 0);
        for code in langs {
            Language::from_youdao(code).expect(code);
        }
    }

    /// 测试中译英实际翻译
    ///
    /// 使用真实API测试中文翻译成英文
    /// 注意：此测试会计入API调用次数
    #[cfg(test)]
    #[tokio::test]
    async fn test_translate_chinese_to_english() {
        dotenv::dotenv().ok();
        let app_key = std::env::var("YOUDAO_APP_KEY").expect("请设置 YOUDAO_APP_KEY 环境变量");
        let app_secret = std::env::var("YOUDAO_APP_SECRET").expect("请设置 YOUDAO_APP_SECRET 环境变量");
        let translator = YoudaoTranslator::new(&app_key, &app_secret);
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
        let app_key = std::env::var("YOUDAO_APP_KEY").expect("请设置 YOUDAO_APP_KEY 环境变量");
        let app_secret = std::env::var("YOUDAO_APP_SECRET").expect("请设置 YOUDAO_APP_SECRET 环境变量");
        let translator = YoudaoTranslator::new(&app_key, &app_secret);
        let result = translator
            .translate("Hello World", Some(Language::English), &Language::Chinese)
            .await
            .expect("翻译失败");
        assert!(!result.text.is_empty());
        println!("英译中结果: {}", result.text);
    }
}
