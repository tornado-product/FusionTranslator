use crate::fusion_translator::async_translator::Language;

/// 翻译模块错误类型
///
/// 枚举定义了在翻译过程中可能遇到的各种错误情况
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum TranslatorError {
    /// 网络请求失败
    ///
    /// 发送HTTP请求时发生错误，可能是网络连接问题或服务器不可用
    #[error("Failed to fetch")]
    Reqwest(#[from] reqwest::Error),
    /// API返回错误响应
    ///
    /// 翻译API返回了错误响应，可能是认证失败、参数错误等
    #[error("Api returned invalid response")]
    ApiError(ApiError),
    /// 未知的目标语言
    ///
    /// 指定的目标语言无法被识别或不支持
    #[error("Couldnt convert language")]
    UnknownLanguage(Language),
    /// 语言代码映射失败
    ///
    /// 无法将字符串转换为有效的Language枚举值
    #[error("failed to convert string to language")]
    CouldNotMapLanguage(Option<String>),
    /// 未收到API响应
    ///
    /// 发送请求后未收到任何响应，可能是网络超时或服务器无响应
    #[error("api did not return a response")]
    NoResponse,
    /// 请求文本过长
    ///
    /// 发送的翻译请求超过了API允许的最大长度限制
    ///
    /// # 参数
    /// - 第一个u32: 实际请求长度
    /// - 第二个u32: 最大允许长度
    #[error("Request was too long")]
    RequestToLong(u32, u32),
    /// 请求失败
    ///
    /// HTTP请求返回了错误的响应状态码
    ///
    /// # 参数
    /// - u16: HTTP状态码
    #[error("Request failed with status code")]
    RequestFailed(u16),
    /// 缺少源语言参数
    ///
    /// 某些翻译API需要明确指定源语言，但调用时未提供
    #[error("Translator required a input language")]
    NoLanguage,
}

/// API错误详细信息
///
/// 包含特定翻译API返回的详细错误信息
#[derive(Debug)]
#[allow(dead_code)]
pub enum ApiError {
    /// 百度翻译API错误
    ///
    /// 包含错误代码和错误消息
    ///
    /// # 字段
    /// - `code`: 百度API返回的错误代码
    /// - `message`: 错误描述信息
    Baidu { code: String, message: String },
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Baidu { code, message } => {
                write!(f, "Baidu API Error [{}]: {}", code, message)
            }
        }
    }
}
