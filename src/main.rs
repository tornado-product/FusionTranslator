pub mod fusion_translator;

use crate::fusion_translator::translator_factory::TranslatorType;
use crate::fusion_translator::async_translator::Language;
use crate::fusion_translator::translator_factory::TranslatorFactory;
#[tokio::main]
async fn main() {
    //测试阿里翻译器
    let translator = TranslatorFactory::create_from_type(TranslatorType::Alibaba, "", "");
    let translate_result = translator.translate("test", Some(Language::English), &Language::Chinese).await;
    println!("{:?}", translate_result);
}