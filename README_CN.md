# 聚合翻译器（fusion-translator）

[English](README.md) | 简体中文

一个基于 Rust 开发的多语言翻译聚合库，支持多种主流翻译服务 API，采用现代化的异步架构设计，通过聚合百度、有道、彩云等翻译服务，为 Rust 生态系统提供高效、可靠的翻译解决方案。

## ✨ 特性

- **多服务聚合**：聚合百度、有道、彩云、阿里和 MyMemory 等多种翻译服务
- **异步优先**：基于 `async/await` 模式设计，充分利用 Rust 的异步运行时
- **工厂模式**：通过 `TranslatorFactory` 统一管理翻译器创建，代码更简洁
- **类型安全**：完整的类型定义和错误处理，确保编译期安全
- **易于扩展**：遵循开闭原则，新增翻译服务只需实现 `AsyncTranslator` trait
- **全面测试**：包含单元测试和集成测试，确保代码质量

## 🏗️ 架构设计

```
fusion-translator/
├── async_translator.rs      # 核心 trait 和数据结构定义
├── translator_factory.rs    # 翻译器工厂模式实现
├── translator_error.rs      # 统一的错误类型定义
├── baidu_translator.rs      # 百度翻译实现
├── youdao_translator.rs     # 有道翻译实现
├── caiyun_translator.rs     # 彩云翻译实现
├── alibaba_translator.rs    # 阿里翻译实现
├── mymemory_translator.rs   # MyMemory 翻译实现
└── mod.rs                   # 模块入口
```

### 核心设计

- **AsyncTranslator Trait**：定义翻译器的通用接口，所有翻译器都必须实现此 trait
- **TranslationOutput**：标准化的翻译结果结构，包含译文文本和目标语言信息
- **TranslationListOutput**：批量翻译结果结构，支持多文本翻译
- **TranslatorError**：统一的错误枚举，覆盖翻译过程中的各种错误场景
- **TranslatorFactory**：工厂类，负责根据类型创建对应的翻译器实例

## 📦 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
fusion-translator = "x.x.x"
```
或 

```toml
[dependencies]
fusion-translator = { path = "path/to/FusionTranslator" }
```

或从 Git 安装：

```toml
[dependencies]
fusion-translator = { git = "https://github.com/tornado-product/FusionTranslator.git" }
```

## 🚀 快速开始

### 基本用法

```rust
use fusion_translator::{
    async_translator::{AsyncTranslator, TranslationOutput},
    translator_factory::TranslatorFactory,
    TranslatorType,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建翻译器工厂
    let factory = TranslatorFactory;

    // 创建百度翻译器（需要环境变量）
    // export BAIDU_APP_ID=your_app_id
    // export BAIDU_KEY=your_app_secret
    let baidu = factory.create_from_env(TranslatorType::Baidu)?;

    // 执行翻译
    let result: TranslationOutput = baidu
        .translate("你好世界", None, &Language::English)
        .await?;

    println!("翻译文本: {}", result.text);
    println!("目标语言: {:?}", result.lang);

    Ok(())
}
```

### 指定语言翻译

```rust
use fusion_translator::async_translator::Language;

// 中译英
let result = translator
    .translate("你好世界", Some(Language::Chinese), &Language::English)
    .await?;

// 英译中
let result = translator
    .translate("Hello World", Some(Language::English), &Language::Chinese)
    .await?;

// 自动检测语言
let result = translator
    .translate("Hello World", None, &Language::Japanese)
    .await?;
```

### 批量翻译

```rust
use fusion_translator::async_translator::AsyncTranslator;

let texts = vec![
    "Hello".to_string(),
    "World".to_string(),
    "Rust".to_string(),
];

let results = translator.translate_vec(&texts, None, &Language::Chinese).await?;

for (original, translated) in texts.iter().zip(results.text.iter()) {
    println!("{} -> {}", original, translated);
}
```

### 使用不同的翻译服务

```rust
use fusion_translator::translator_factory::{TranslatorFactory, TranslatorType};

// 百度翻译
let baidu = TranslatorFactory.create_from_env(TranslatorType::Baidu)?;

// 有道翻译（需要 YOUDAO_APP_KEY 和 YOUDAO_APP_SECRET）
let youdao = TranslatorFactory.create_from_env(TranslatorType::Youdao)?;

// 彩云翻译（需要 CAIYUN_TOKEN）
let caiyun = TranslatorFactory.create_from_env(TranslatorType::Caiyun)?;

// 阿里翻译
let alibaba = TranslatorFactory.create_from_env(TranslatorType::Alibaba)?;

// MyMemory 翻译（免费服务，无需 API key）
let mymemory = TranslatorFactory.create_from_env(TranslatorType::MyMemory)?;
```

## ⚙️ 环境变量配置

各翻译服务需要的环境变量：

| 服务 | 环境变量 | 说明 |
|------|----------|------|
| 百度 | `BAIDU_APP_ID` | 百度开放平台应用 ID |
| 百度 | `BAIDU_KEY` | 百度开放平台应用密钥 |
| 有道 | `YOUDAO_APP_KEY` | 有道开放平台应用 Key |
| 有道 | `YOUDAO_APP_SECRET` | 有道开放平台应用密钥 |
| 彩云 | `CAIYUN_TOKEN` | 彩云科技 API Token |
| 彩云 | `CAIYUN_REQUEST_ID` | 彩云 API 请求 ID（可选，默认值: "demo"） |

你也可以在项目根目录创建 `.env` 文件来配置这些变量：

```env
# 百度翻译
BAIDU_APP_ID=your_app_id
BAIDU_KEY=your_app_secret

# 有道翻译
YOUDAO_APP_KEY=your_app_key
YOUDAO_APP_SECRET=your_app_secret

# 彩云翻译
CAIYUN_TOKEN=your_token
CAIYUN_REQUEST_ID=demo
```

## 📊 支持的语言

本库支持以下语言组合：

- 简体中文 (Chinese Simplified)
- 繁体中文 (Chinese Traditional)
- 英语 (English)
- 日语 (Japanese)
- 韩语 (Korean)
- 法语 (French)
- 德语 (German)
- 俄语 (Russian)
- 西班牙语 (Spanish)
- 葡萄牙语 (Portuguese)
- 意大利语 (Italian)
- 阿拉伯语 (Arabic)
- 以及更多语言...

具体支持的语言列表请参考各翻译器的实现。

## 🧪 测试

本项目包含全面的测试用例：

```bash
# 运行所有测试
cargo test

# 运行特定翻译器的测试
cargo test baidu
cargo test youdao
cargo test caiyun

# 查看测试覆盖率
cargo tarpaulin --output-dir ./coverage
```

### 测试类型

- **单元测试**：测试各翻译器的核心功能
- **集成测试**：测试与真实 API 的连接（需要配置 API 密钥）
- **文档测试**：确保文档示例代码的正确性

## 📈 性能特点

- **异步并发**：使用 `tokio` 异步运行时，支持高并发翻译请求
- **连接池**：内置 HTTP 连接池，减少连接建立开销
- **高效序列化**：使用 `serde` 进行高效的 JSON 序列化/反序列化
- **低内存占用**：Rust 的内存安全特性确保低内存占用

## 🛡️ 错误处理

本库使用 `thiserror` 定义了完整的错误类型：

```rust
#[derive(Debug, thiserror::Error)]
pub enum TranslatorError {
    #[error("Failed to fetch")]
    Reqwest(#[from] reqwest::Error),

    #[error("Api returned invalid response")]
    ApiError(ApiError),

    #[error("Couldnt convert language")]
    UnknownLanguage(Language),

    #[error("failed to convert string to language")]
    CouldNotMapLanguage(Option<String>),

    #[error("api did not return a response")]
    NoResponse,

    #[error("Request was too long")]
    RequestToLong(u32, u32),

    #[error("Request failed with status code")]
    RequestFailed(u16),

    #[error("Translator required a input language")]
    NoLanguage,
}
```

## 🔧 自定义扩展

### 添加新的翻译服务

1. 创建新的翻译器文件（如 `custom_translator.rs`）
2. 实现 `AsyncTranslator` trait
3. 在 `mod.rs` 中注册新模块
4. 在 `TranslatorType` 枚举中添加新变体
5. 在 `TranslatorFactory::create_translator` 中添加创建逻辑

示例：

```rust
use async_trait::async_trait;
use crate::async_translator::{AsyncTranslator, TranslationOutput, Language};

pub struct CustomTranslator {
    // 配置字段
}

#[async_trait]
impl AsyncTranslator for CustomTranslator {
    async fn translate(
        &self,
        query: &str,
        from: Option<Language>,
        to: &Language,
    ) -> anyhow::Result<TranslationOutput> {
        // 实现翻译逻辑
        Ok(TranslationOutput {
            text: translated_text,
            lang: Some(*to),
        })
    }
}
```

## 📝 许可证

本项目采用 MIT 许可证开源。

```
MIT License

Copyright (c) 2024 Your Name

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本项目
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建一个 Pull Request

## 📧 联系

如果你有任何问题或建议，请通过以下方式联系：

- 创建 [Issue](https://github.com/tornado-product/FusionTranslator/issues)

## 🙏 致谢

- [Tokio](https://tokio.rs/) - 异步运行时
- [Reqwest](https://docs.rs/reqwest/) - HTTP 客户端
- [Serde](https://serde.rs/) - 序列化框架
- [Thiserror](https://github.com/dtolnay/thiserror) - 错误处理

---

如果你觉得这个项目对你有帮助，请给我们一个 ⭐️！
