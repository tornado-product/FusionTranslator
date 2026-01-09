/// 翻译模块
///
/// 提供多种翻译服务的集成支持，包括百度翻译、有道翻译、阿里翻译、彩云翻译和MyMemory翻译
/// 使用工厂模式统一管理不同翻译器的创建和使用
pub mod async_translator;
pub mod translator_factory;
pub mod translator_error;
mod baidu_translator;
mod youdao_translator;
mod caiyun_translator;
mod mymemory_translator;
mod alibaba_translator;
