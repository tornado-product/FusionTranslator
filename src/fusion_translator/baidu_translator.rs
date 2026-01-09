use crate::fusion_translator::async_translator::{
    AsyncTranslator, Language, TranslationListOutput, TranslationOutput,
};
use crate::fusion_translator::translator_error::{ApiError, TranslatorError};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// 百度翻译器实现
///
/// 通过调用百度翻译API实现文本翻译功能
pub struct BaiduTranslator {
    /// API请求地址
    url: String,
    /// 百度开放平台应用ID
    app_id: String,
    /// 百度开放平台应用密钥
    key: String,
    /// HTTP客户端
    client: Client,
}

#[async_trait::async_trait]
impl AsyncTranslator for BaiduTranslator {
    /// 判断是否为本地翻译器
    ///
    /// 百度翻译器需要调用远程API，返回false
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
        let to = to.to_baidu().ok_or(TranslatorError::UnknownLanguage(*to))?;
        let from = match from {
            Some(item) => item
                .to_baidu()
                .ok_or(TranslatorError::UnknownLanguage(item))?,
            None => "auto",
        };
        let form = Form::new(&self.app_id, query, "0", &self.key, from, to);
        let resp: Response = self
            .client
            .post(&self.url)
            .form(&form)
            .send()
            .await?
            .json()
            .await?;
        let resp = match resp {
            Response::Ok(v) => v,
            Response::Err(v) => {
                Err(TranslatorError::ApiError(ApiError::Baidu {
                    message: v.solution().to_owned(),
                    code: v.code,
                }))?;
                unreachable!()
            }
        };
        Ok(TranslationOutput {
            text: resp
                .trans_result
                .iter()
                .map(|v| v.dst.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
            lang: Some(
                Language::from_baidu(&resp.to)
                    .ok_or(TranslatorError::CouldNotMapLanguage(Some(resp.to)))?,
            ),
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
        let v = self.translate(&query.join("\n"), from, to).await?;
        Ok(TranslationListOutput {
            text: v.text.split('\n').map(|v| v.to_string()).collect(),
            lang: v.lang,
        })
    }
}

impl BaiduTranslator {
    /// 创建新的百度翻译器实例
    ///
    /// # 参数
    /// - `app_id`: 百度开放平台应用ID
    /// - `key`: 百度开放平台应用密钥
    ///
    /// # 返回值
    /// 新的翻译器实例
    #[allow(dead_code)]
    pub fn new(app_id: &str, key: &str) -> Self {
        Self {
            url: "https://fanyi-api.baidu.com/api/trans/vip/translate".to_string(),
            app_id: app_id.to_string(),
            key: key.to_string(),
            client: Client::new(),
        }
    }
}

/// 表单数据提交结构
///
/// 用于构造百度翻译API的请求参数
#[derive(Debug, Serialize)]
pub struct Form {
    /// 待翻译文本
    pub q: String,
    /// 源语言
    pub from: String,
    /// 目标语言
    pub to: String,
    /// 应用ID
    pub appid: String,
    /// 随机盐值
    pub salt: String,
    /// 签名
    pub sign: String,
}

impl Form {
    /// 创建新的表单实例
    ///
    /// # 参数
    /// - `appid`: 百度开放平台应用ID
    /// - `q`: 待翻译文本
    /// - `salt`: 随机盐值
    /// - `key`: 百度开放平台应用密钥
    /// - `from`: 源语言代码
    /// - `to`: 目标语言代码
    ///
    /// # 返回值
    /// 新的表单实例
    #[allow(dead_code)]
    fn new(appid: &str, q: &str, salt: &str, key: &str, from: &str, to: &str) -> Self {
        let data = format!("{}{}{}{}", &appid, q, salt, &key);
        let sign = format!("{:x}", md5::compute(data));
        Self {
            q: q.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            appid: appid.to_string(),
            salt: salt.to_string(),
            sign,
        }
    }
}

/// API响应枚举
///
/// 可能返回翻译成功结果或错误信息
#[derive(Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
enum Response {
    /// 翻译成功响应
    Ok(TranslationResponse),
    /// 错误响应
    Err(BaiduApiError),
}

/// 百度API错误信息
///
/// 包含错误代码和错误消息
#[derive(Debug, Clone, Deserialize)]
pub struct BaiduApiError {
    /// 错误代码
    #[serde(rename = "error_code")]
    pub code: String,
    /// 错误消息
    #[serde(rename = "error_msg")]
    pub msg: String,
}

impl std::error::Error for BaiduApiError {}
impl std::fmt::Display for BaiduApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error code: `{}`\nError message: `{}`\nError meaning: {}\nThe above content is returned by Baidu translation API",
            self.code,
            self.msg,
            self.solution()
        )
    }
}

impl BaiduApiError {
    /// 获取错误说明
    ///
    /// 根据错误代码返回对应的错误说明和解决方案
    ///
    /// # 返回值
    /// 错误说明字符串
    ///
    /// 参考: [百度翻译API错误码列表](https://fanyi-api.baidu.com/doc/21)
    pub fn solution(&self) -> &str {
        match self.code.as_bytes() {
            b"52000" => "成功",
            b"52001" => "请求超时。\n解决方案：请重试。",
            b"52002" => "系统错误。\n解决方案：请重试。",
            b"52003" => {
                "未授权用户。\n解决方案：请检查appid是否正确或服务是否已开通。"
            }
            b"54000" => {
                "必填参数为空。\n解决方案：请检查是否传递了所有必要参数。"
            }
            b"54001" => {
                "签名错误。\n解决方案：请检查签名生成方式。"
            }
            b"54003" => {
                "访问频率受限。\n解决方案：请降低调用频率，或通过认证后切换到高级版本。"
            }
            b"54004" => {
                "账户余额不足。\n解决方案：请前往管理控制台充值。"
            }
            b"54005" => {
                "长查询请求过于频繁。\n解决方案：请降低长查询的发送频率，3秒后重试。"
            }
            b"58000" => {
                "客户端IP非法。\n解决方案：检查个人信息中填写的IP地址是否正确，可前往开发者信息-基本信息进行修改。"
            }
            b"58001" => {
                "目标语言方向不支持。\n解决方案：检查目标语言是否在语言列表中。"
            }
            b"58002" => {
                "服务目前已下线。\n解决方案：请前往管理控制台开启服务。"
            }
            b"58003" => {
                "如果同一IP在同一天使用多个APPID发送翻译请求，该IP将在当日剩余时间内被禁止请求，次日解封。请勿将APPID和密钥输入第三方软件。"
            }
            b"90107" => {
                "认证未通过或已失效。\n解决方案：请前往我的认证查看认证进度。"
            }
            b"20003" => {
                "请检查请求文本是否涉及颠覆、暴力或类似主题相关内容。"
            }
            _ => "未知错误",
        }
    }
}

/// 句子翻译结果
///
/// 包含翻译后的目标文本
#[derive(Deserialize)]
#[allow(dead_code)]
struct Sentence {
    /// 翻译后的目标文本
    pub dst: String,
}

/// 翻译成功响应
///
/// 包含翻译结果列表和目标语言
#[derive(Deserialize)]
struct TranslationResponse {
    /// 目标语言代码
    pub to: String,
    /// 翻译结果列表
    pub trans_result: Vec<Sentence>,
}

#[cfg(test)]
mod tests {

    use crate::fusion_translator::async_translator::{AsyncTranslator, Language};
    use crate::fusion_translator::baidu_translator::{BaiduTranslator, Form};
    use std::collections::HashSet;

    /// 测试翻译器实例创建
    ///
    /// 验证构造函数是否正确初始化所有字段
    #[tokio::test]
    async fn test_new_translator() {
        let translator = BaiduTranslator::new("test_app_id", "test_key");
        assert_eq!(translator.app_id, "test_app_id");
        assert_eq!(translator.key, "test_key");
        assert!(translator.url.contains("baidu.com"));
    }

    /// 测试表单创建功能
    ///
    /// 验证表单参数构造和签名生成是否正确
    #[test]
    fn test_form_creation() {
        let form = Form::new("appid", "hello", "salt", "key", "en", "zh");
        assert_eq!(form.q, "hello");
        assert_eq!(form.from, "en");
        assert_eq!(form.to, "zh");
        assert_eq!(form.appid, "appid");
        assert_eq!(form.salt, "salt");
        assert!(!form.sign.is_empty());
    }

    /// 测试空文本表单创建
    ///
    /// 验证空字符串也能正确生成签名
    #[test]
    fn test_form_empty_query() {
        let form = Form::new("appid", "", "salt", "key", "auto", "zh");
        assert!(form.sign.len() == 32);
    }

    /// 测试翻译器字段访问
    ///
    /// 验证可以正确访问翻译器的各个字段
    #[tokio::test]
    async fn test_translator_fields() {
        let translator = BaiduTranslator::new("app_id_123", "key_456");
        assert_eq!(translator.app_id, "app_id_123");
        assert_eq!(translator.key, "key_456");
        assert!(translator.url.starts_with("https://"));
    }

    /// 测试翻译器本地方法
    ///
    /// 百度翻译器需要调用远程API，应该返回false
    #[tokio::test]
    async fn test_translator_local() {
        let translator = BaiduTranslator::new("test_id", "test_key");
        assert!(!translator.local());
    }

    /// 测试语言代码映射功能
    ///
    /// 验证所有支持的语言代码都能正确映射
    #[test]
    fn test_language_mapping() {
        let langs = [
            "zh", "en", "yue", "wyw", "jp", "kor", "fra", "spa", "th", "ara", "ru", "pt", "de",
            "it", "el", "nl", "pl", "bul", "est", "dan", "fin", "cs", "rom", "slo", "swe", "hu",
            "cht", "vie", "ara", "gle", "oci", "alb", "arq", "aka", "arg", "amh", "asm", "aym",
            "aze", "ast", "oss", "est", "oji", "ori", "orm", "pl", "per", "bre", "bak", "baq",
            "pot", "bel", "ber", "pam", "bul", "sme", "ped", "bem", "bli", "bis", "bal", "ice",
            "bos", "bho", "chv", "tso", "dan", "de", "tat", "sha", "tet", "div", "log", "ru",
            "fra", "fil", "fin", "san", "fri", "ful", "fao", "gla", "kon", "ups", "hkm", "kal",
            "geo", "guj", "gra", "eno", "grn", "kor", "nl", "hup", "hak", "ht", "mot", "hau",
            "kir", "glg", "frn", "cat", "cs", "kab", "kan", "kau", "kah", "cor", "xho", "cos",
            "cre", "cri", "kli", "hrv", "que", "kas", "kok", "kur", "lat", "lao", "rom", "lag",
            "lav", "lim", "lin", "lug", "ltz", "ruy", "kin", "lit", "roh", "ro", "loj", "may",
            "bur", "mar", "mg", "mal", "mac", "mah", "mai", "glv", "mau", "mao", "ben", "mlt",
            "hmn", "nor", "nea", "nbl", "afr", "sot", "nep", "pt", "pan", "pap", "pus", "nya",
            "twi", "chr", "jp", "swe", "srd", "sm", "sec", "srp", "sol", "sin", "epo", "nob", "sk",
            "slo", "swa", "src", "som", "sco", "th", "tr", "tgk", "tam", "tgl", "tir", "tel",
            "tua", "tuk", "ukr", "wln", "wel", "ven", "wol", "urd", "spa", "heb", "el", "hu",
            "fry", "sil", "hil", "los", "haw", "nno", "nqo", "snd", "sna", "ceb", "syr", "sun",
            "en", "hi", "id", "it", "vie", "yid", "ina", "ach", "ing", "ibo", "ido", "yor", "arm",
            "iku", "zh", "cht", "wyw", "yue", "zaz", "frm", "zul", "jav",
        ];
        for lang_str in langs.into_iter().collect::<HashSet<_>>() {
            if lang_str == "slo" {
                continue;
            }
            Language::from_baidu(lang_str).expect(lang_str);
        }
    }

    /// 测试重复语言代码去重
    ///
    /// 验证语言代码列表中存在重复时能正确处理
    #[test]
    fn test_duplicate_language_codes() {
        let langs = vec!["zh", "en", "zh", "en", "jp"];
        let unique_langs: HashSet<&str> = langs.into_iter().collect();
        assert_eq!(unique_langs.len(), 3);
    }

    /// 测试表单签名一致性
    ///
    /// 相同的输入应该产生相同的签名
    #[test]
    fn test_form_signature_consistency() {
        let form1 = Form::new("appid", "hello", "salt", "key", "en", "zh");
        let form2 = Form::new("appid", "hello", "salt", "key", "en", "zh");
        assert_eq!(form1.sign, form2.sign);
    }

    /// 测试不同输入产生不同签名
    ///
    /// 不同的翻译文本应该产生不同的签名
    #[test]
    fn test_different_queries_different_signatures() {
        let form1 = Form::new("appid", "hello", "salt", "key", "en", "zh");
        let form2 = Form::new("appid", "world", "salt", "key", "en", "zh");
        assert_ne!(form1.sign, form2.sign);
    }

    /// 测试中译英实际翻译
    ///
    /// 使用真实API测试中文翻译成英文
    /// 注意：此测试会计入API调用次数，可能受频率限制影响
    #[cfg(test)]
    #[tokio::test]
    async fn test_translate_chinese_to_english() {
        dotenv::dotenv().ok();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let app_id = std::env::var("BAIDU_APP_ID").expect("请设置 BAIDU_APP_ID 环境变量");
        let key = std::env::var("BAIDU_KEY").expect("请设置 BAIDU_KEY 环境变量");
        let translator = BaiduTranslator::new(&app_id, &key);
        let result = translator
            .translate("你好世界", Some(Language::Chinese), &Language::English)
            .await;

        match result {
            Ok(output) => {
                assert!(!output.text.is_empty());
                println!("中译英结果: {}", output.text);
            }
            Err(e) => {
                let err_msg = e.to_string();
                println!("中译英错误: {}", err_msg);

                if let Some(api_err) =
                    e.downcast_ref::<crate::fusion_translator::translator_error::TranslatorError>()
                {
                    println!("错误类型: {:?}", api_err);
                    if let crate::fusion_translator::translator_error::TranslatorError::ApiError(
                        api_details,
                    ) = api_err
                    {
                        println!("API错误详情: {:?}", api_details);
                    }
                }

                panic!("翻译失败: {}", err_msg);
            }
        }
    }

    /// 测试英译中实际翻译
    ///
    /// 使用真实API测试英文翻译成中文
    /// 注意：此测试会计入API调用次数，可能受频率限制影响
    #[cfg(test)]
    #[tokio::test]
    async fn test_translate_english_to_chinese() {
        dotenv::dotenv().ok();
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let app_id = std::env::var("BAIDU_APP_ID").expect("请设置 BAIDU_APP_ID 环境变量");
        let key = std::env::var("BAIDU_KEY").expect("请设置 BAIDU_KEY 环境变量");
        let translator = BaiduTranslator::new(&app_id, &key);
        let result = translator
            .translate("Hello World", Some(Language::English), &Language::Chinese)
            .await;

        match result {
            Ok(output) => {
                assert!(!output.text.is_empty());
                println!("英译中结果: {}", output.text);
            }
            Err(e) => {
                let err_msg = e.to_string();
                println!("英译中错误: {}", err_msg);

                if let Some(api_err) =
                    e.downcast_ref::<crate::fusion_translator::translator_error::TranslatorError>()
                {
                    println!("错误类型: {:?}", api_err);
                    if let crate::fusion_translator::translator_error::TranslatorError::ApiError(
                        api_details,
                    ) = api_err
                    {
                        println!("API错误详情: {:?}", api_details);
                    }
                }

                panic!("翻译失败: {}", err_msg);
            }
        }
    }
}
