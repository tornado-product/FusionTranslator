use crate::fusion_translator::alibaba_translator::AlibabaTranslator;
use crate::fusion_translator::async_translator::AsyncTranslator;
use crate::fusion_translator::baidu_translator::BaiduTranslator;
use crate::fusion_translator::caiyun_translator::CaiyunTranslator;
use crate::fusion_translator::mymemory_translator::MyMemoryTranslator;
use crate::fusion_translator::youdao_translator::YoudaoTranslator;
use std::str::FromStr;
use std::sync::Arc;

/// 翻译器类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TranslatorType {
    Baidu,
    Youdao,
    Alibaba,
    Caiyun,
    MyMemory,
}

impl std::str::FromStr for TranslatorType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "baidu" => Ok(Self::Baidu),
            "youdao" => Ok(Self::Youdao),
            "alibaba" | "ali" => Ok(Self::Alibaba),
            "caiyun" | "彩云" => Ok(Self::Caiyun),
            "mymemory" | "my-memory" | "my memory" => Ok(Self::MyMemory),
            _ => Err(()),
        }
    }
}

impl TranslatorType {
    /// 从字符串解析翻译器类型
    pub fn parse(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }

    /// 转换为字符串
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Baidu => "baidu",
            Self::Youdao => "youdao",
            Self::Alibaba => "alibaba",
            Self::Caiyun => "caiyun",
            Self::MyMemory => "mymemory",
        }
    }
}

/// 翻译器配置
#[derive(Debug, Clone)]
pub enum TranslatorConfig {
    Baidu { app_id: String, key: String },
    Youdao { app_key: String, app_secret: String },
    Alibaba { token: String },
    Caiyun { token: String, request_id: String },
    MyMemory,
}

/// 翻译器工厂
pub struct TranslatorFactory;

impl TranslatorFactory {
    /// 根据类型和配置创建翻译器实例
    #[allow(dead_code)]
    pub fn create(config: TranslatorConfig) -> Arc<dyn AsyncTranslator> {
        match config {
            TranslatorConfig::Baidu { app_id, key } => {
                Arc::new(BaiduTranslator::new(&app_id, &key))
            }
            TranslatorConfig::Youdao { app_key, app_secret } => {
                Arc::new(YoudaoTranslator::new(&app_key, &app_secret))
            }
            TranslatorConfig::Alibaba { .. } => {
                Arc::new(AlibabaTranslator::new())
            }
            TranslatorConfig::Caiyun { token, request_id } => {
                Arc::new(CaiyunTranslator::new(&token, &request_id))
            }
            TranslatorConfig::MyMemory => {
                Arc::new(MyMemoryTranslator::new())
            }
        }
    }

    /// 根据类型字符串和配置创建翻译器实例
    #[allow(dead_code)]
    pub fn create_from_type(
        translator_type: TranslatorType,
        app_id: &str,
        secret: &str,
    ) -> Arc<dyn AsyncTranslator> {
        match translator_type {
            TranslatorType::Baidu => Arc::new(BaiduTranslator::new(app_id, secret)),
            TranslatorType::Youdao => Arc::new(YoudaoTranslator::new(app_id, secret)),
            TranslatorType::Alibaba => Arc::new(AlibabaTranslator::new()),
            TranslatorType::Caiyun => Arc::new(CaiyunTranslator::new(app_id, secret)),
            TranslatorType::MyMemory => Arc::new(MyMemoryTranslator::new()),
        }
    }

    /// 从环境变量创建翻译器（便捷方法）
    pub fn create_from_env(translator_type: TranslatorType) -> Result<Arc<dyn AsyncTranslator>, String> {
        match translator_type {
            TranslatorType::Baidu => {
                let app_id = std::env::var("BAIDU_APP_ID")
                    .map_err(|_| "BAIDU_APP_ID environment variable not set")?;
                let key = std::env::var("BAIDU_KEY")
                    .map_err(|_| "BAIDU_KEY environment variable not set")?;
                Ok(Arc::new(BaiduTranslator::new(&app_id, &key)))
            }
            TranslatorType::Youdao => {
                let app_key = std::env::var("YOUDAO_APP_KEY")
                    .map_err(|_| "YOUDAO_APP_KEY environment variable not set")?;
                let app_secret = std::env::var("YOUDAO_APP_SECRET")
                    .map_err(|_| "YOUDAO_APP_SECRET environment variable not set")?;
                Ok(Arc::new(YoudaoTranslator::new(&app_key, &app_secret)))
            }
            TranslatorType::Alibaba => Ok(Arc::new(AlibabaTranslator::new())),
            TranslatorType::Caiyun => {
                let token = std::env::var("CAIYUN_TOKEN")
                    .map_err(|_| "CAIYUN_TOKEN environment variable not set")?;
                let request_id = std::env::var("CAIYUN_REQUEST_ID").unwrap_or_else(|_| "demo".to_string());
                Ok(Arc::new(CaiyunTranslator::new(&token, &request_id)))
            }
            TranslatorType::MyMemory => Ok(Arc::new(MyMemoryTranslator::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translator_type_parse() {
        assert_eq!(TranslatorType::parse("baidu"), Some(TranslatorType::Baidu));
        assert_eq!(TranslatorType::parse("Baidu"), Some(TranslatorType::Baidu));
        assert_eq!(TranslatorType::parse("BAIDU"), Some(TranslatorType::Baidu));
        assert_eq!(TranslatorType::parse("youdao"), Some(TranslatorType::Youdao));
        assert_eq!(TranslatorType::parse("alibaba"), Some(TranslatorType::Alibaba));
        assert_eq!(TranslatorType::parse("ali"), Some(TranslatorType::Alibaba));
        assert_eq!(TranslatorType::parse("caiyun"), Some(TranslatorType::Caiyun));
        assert_eq!(TranslatorType::parse("彩云"), Some(TranslatorType::Caiyun));
        assert_eq!(TranslatorType::parse("mymemory"), Some(TranslatorType::MyMemory));
        assert_eq!(TranslatorType::parse("unknown"), None);
    }

    #[test]
    fn test_translator_type_from_str() {
        assert_eq!(TranslatorType::from_str("baidu"), Ok(TranslatorType::Baidu));
        assert_eq!(TranslatorType::from_str("Baidu"), Ok(TranslatorType::Baidu));
        assert_eq!(TranslatorType::from_str("BAIDU"), Ok(TranslatorType::Baidu));
        assert_eq!(TranslatorType::from_str("youdao"), Ok(TranslatorType::Youdao));
        assert_eq!(TranslatorType::from_str("alibaba"), Ok(TranslatorType::Alibaba));
        assert_eq!(TranslatorType::from_str("ali"), Ok(TranslatorType::Alibaba));
        assert_eq!(TranslatorType::from_str("caiyun"), Ok(TranslatorType::Caiyun));
        assert_eq!(TranslatorType::from_str("彩云"), Ok(TranslatorType::Caiyun));
        assert_eq!(TranslatorType::from_str("mymemory"), Ok(TranslatorType::MyMemory));
        assert_eq!(TranslatorType::from_str("unknown"), Err(()));
    }

    #[test]
    fn test_translator_type_as_str() {
        assert_eq!(TranslatorType::Baidu.as_str(), "baidu");
        assert_eq!(TranslatorType::Youdao.as_str(), "youdao");
        assert_eq!(TranslatorType::Alibaba.as_str(), "alibaba");
        assert_eq!(TranslatorType::Caiyun.as_str(), "caiyun");
        assert_eq!(TranslatorType::MyMemory.as_str(), "mymemory");
    }

    #[tokio::test]
    async fn test_create_baidu_translator() {
        let app_id = std::env::var("BAIDU_APP_ID").expect("请设置 BAIDU_APP_ID 环境变量");
        let key = std::env::var("BAIDU_KEY").expect("请设置 BAIDU_KEY 环境变量");
        let config = TranslatorConfig::Baidu {
            app_id,
            key,
        };
        let translator = TranslatorFactory::create(config);
        assert!(!translator.local());
    }

    #[tokio::test]
    async fn test_create_youdao_translator() {
        let config = TranslatorConfig::Youdao {
            app_key: "test_app_key".to_string(),
            app_secret: "test_app_secret".to_string(),
        };
        let translator = TranslatorFactory::create(config);
        assert!(!translator.local());
    }

    #[tokio::test]
    async fn test_create_alibaba_translator() {
        let config = TranslatorConfig::Alibaba {
            token: "test_token".to_string(),
        };
        let translator = TranslatorFactory::create(config);
        assert!(!translator.local());
    }

    #[tokio::test]
    async fn test_create_caiyun_translator() {
        let config = TranslatorConfig::Caiyun {
            token: "test_token".to_string(),
            request_id: "test_request_id".to_string(),
        };
        let translator = TranslatorFactory::create(config);
        assert!(!translator.local());
    }

    #[tokio::test]
    async fn test_create_mymemory_translator() {
        let config = TranslatorConfig::MyMemory;
        let translator = TranslatorFactory::create(config);
        assert!(!translator.local());
    }

    #[tokio::test]
    async fn test_create_from_type() {
        let translator = TranslatorFactory::create_from_type(
            TranslatorType::Baidu,
            "test_app_id",
            "test_key",
        );
        assert!(!translator.local());

        let translator = TranslatorFactory::create_from_type(
            TranslatorType::Youdao,
            "test_app_key",
            "test_app_secret",
        );
        assert!(!translator.local());

        let translator = TranslatorFactory::create_from_type(
            TranslatorType::Alibaba,
            "",
            "",
        );
        assert!(!translator.local());

        let translator = TranslatorFactory::create_from_type(
            TranslatorType::Caiyun,
            "test_token",
            "test_request_id",
        );
        assert!(!translator.local());

        let translator = TranslatorFactory::create_from_type(
            TranslatorType::MyMemory,
            "",
            "",
        );
        assert!(!translator.local());
    }
}