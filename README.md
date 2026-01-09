# fusion-translator

A multi-language translation fusion library written in Rust, supporting multiple translation service APIs with a modern asynchronous architecture design. This library provides efficient and reliable translation solutions for the Rust ecosystem by aggregating Baidu, Youdao, Caiyun, Alibaba, MyMemory, and other translation services.

## ✨ Features

- **Multi-Service Aggregation**: Aggregates Baidu, Youdao, Caiyun, Alibaba, and MyMemory translation services
- **Async-First Design**: Built on `async/await` pattern, fully utilizing Rust's async runtime
- **Factory Pattern**: Unified translator creation through `TranslatorFactory` for cleaner code
- **Type Safety**: Complete type definitions and error handling for compile-time safety
- **Easy Extension**: Follows the Open-Closed Principle; new translation services only need to implement the `AsyncTranslator` trait
- **Comprehensive Testing**: Includes unit tests and integration tests to ensure code quality

## 🏗️ Architecture

```
fusion-translator/
├── async_translator.rs      # Core trait and data structure definitions
├── translator_factory.rs    # Factory pattern implementation
├── translator_error.rs      # Unified error type definitions
├── baidu_translator.rs      # Baidu translation implementation
├── youdao_translator.rs     # Youdao translation implementation
├── caiyun_translator.rs     # Caiyun translation implementation
├── alibaba_translator.rs    # Alibaba translation implementation
├── mymemory_translator.rs   # MyMemory translation implementation
└── mod.rs                   # Module entry point
```

### Core Design

- **AsyncTranslator Trait**: Defines the common interface for translators; all translators must implement this trait
- **TranslationOutput**: Standardized translation result structure containing translated text and target language information
- **TranslationListOutput**: Batch translation result structure supporting multiple text translations
- **TranslatorError**: Unified error enum covering various error scenarios during translation
- **TranslatorFactory**: Factory class responsible for creating corresponding translator instances based on type

## 📦 Installation

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
fusion-translator = "x.x.x"
```
or

```toml
[dependencies]
fusion-translator = { path = "path/to/fusion-translator" }
```

Or install from Git:

```toml
[dependencies]
fusion-translator = { git = "https://github.com/tornado-product/FusionTranslator.git" }
```

## 🚀 Quick Start

### Basic Usage

```rust
use fusion_translator::{
    async_translator::{AsyncTranslator, TranslationOutput},
    translator_factory::TranslatorFactory,
    TranslatorType,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create translator factory
    let factory = TranslatorFactory;

    // Create Baidu translator (requires environment variables)
    // export BAIDU_APP_ID=your_app_id
    // export BAIDU_KEY=your_app_secret
    let baidu = factory.create_from_env(TranslatorType::Baidu)?;

    // Perform translation
    let result: TranslationOutput = baidu
        .translate("Hello World", None, &Language::Chinese)
        .await?;

    println!("Translated text: {}", result.text);
    println!("Target language: {:?}", result.lang);

    Ok(())
}
```

### Specified Language Translation

```rust
use fusion_translator::async_translator::Language;

// Chinese to English
let result = translator
    .translate("你好世界", Some(Language::Chinese), &Language::English)
    .await?;

// English to Chinese
let result = translator
    .translate("Hello World", Some(Language::English), &Language::Chinese)
    .await?;

// Auto-detect language
let result = translator
    .translate("Hello World", None, &Language::Japanese)
    .await?;
```

### Batch Translation

```rust
use translator::async_translator::AsyncTranslator;

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

### Using Different Translation Services

```rust
use fusion_translator::translator_factory::TranslatorType;

// Baidu translation
let baidu = TranslatorFactory.create_from_env(TranslatorType::Baidu)?;

// Youdao translation (requires YOUDAO_APP_KEY and YOUDAO_APP_SECRET)
let youdao = TranslatorFactory.create_from_env(TranslatorType::Youdao)?;

// Caiyun translation (requires CAIYUN_TOKEN)
let caiyun = TranslatorFactory.create_from_env(TranslatorType::Caiyun)?;

// Alibaba translation
let alibaba = TranslatorFactory.create_from_env(TranslatorType::Alibaba)?;

// MyMemory translation (free service, no API key required)
let mymemory = TranslatorFactory.create_from_env(TranslatorType::MyMemory)?;
```

## ⚙️ Environment Variable Configuration

Environment variables required by each translation service:

| Service | Environment Variable | Description |
|---------|---------------------|-------------|
| Baidu | `BAIDU_APP_ID` | Baidu Open Platform App ID |
| Baidu | `BAIDU_KEY` | Baidu Open Platform App Secret |
| Youdao | `YOUDAO_APP_KEY` | Youdao Open Platform App Key |
| Youdao | `YOUDAO_APP_SECRET` | Youdao Open Platform App Secret |
| Caiyun | `CAIYUN_TOKEN` | Caiyun Technology API Token |
| Caiyun | `CAIYUN_REQUEST_ID` | Caiyun API Request ID (optional, default: "demo") |

You can also configure these variables by creating a `.env` file in the project root:

```env
# Baidu Translation
BAIDU_APP_ID=your_app_id
BAIDU_KEY=your_app_secret

# Youdao Translation
YOUDAO_APP_KEY=your_app_key
YOUDAO_APP_SECRET=your_app_secret

# Caiyun Translation
CAIYUN_TOKEN=your_token
CAIYUN_REQUEST_ID=demo
```

## 📊 Supported Languages

This library supports the following language combinations:

- Chinese Simplified
- Chinese Traditional
- English
- Japanese
- Korean
- French
- German
- Russian
- Spanish
- Portuguese
- Italian
- Arabic
- And more...

Please refer to each translator's implementation for the specific supported language list.

## 🧪 Testing

This project includes comprehensive test cases:

```bash
# Run all tests
cargo test

# Run specific translator tests
cargo test baidu
cargo test youdao
cargo test caiyun

# View test coverage
cargo tarpaulin --output-dir ./coverage
```

### Test Types

- **Unit Tests**: Test core functionality of each translator
- **Integration Tests**: Test connection with real APIs (requires API keys)
- **Doc Tests**: Ensure correctness of documentation examples

## 📈 Performance Characteristics

- **Async Concurrency**: Uses `tokio` async runtime to support high-concurrency translation requests
- **Connection Pool**: Built-in HTTP connection pool to reduce connection establishment overhead
- **Efficient Serialization**: Uses `serde` for efficient JSON serialization/deserialization
- **Low Memory Footprint**: Rust's memory safety features ensure low memory usage

## 🛡️ Error Handling

This library uses `thiserror` to define complete error types:

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

## 🔧 Custom Extension

### Adding New Translation Services

1. Create a new translator file (e.g., `custom_translator.rs`)
2. Implement the `AsyncTranslator` trait
3. Register the new module in `mod.rs`
4. Add a new variant to the `TranslatorType` enum
5. 在 `TranslatorFactory::create_from_env` 中添加创建逻辑

Example:

```rust
use async_trait::async_trait;
use crate::async_translator::{AsyncTranslator, TranslationOutput, Language};

pub struct CustomTranslator {
    // Configuration fields
}

#[async_trait]
impl AsyncTranslator for CustomTranslator {
    async fn translate(
        &self,
        query: &str,
        from: Option<Language>,
        to: &Language,
    ) -> anyhow::Result<TranslationOutput> {
        // Implement translation logic
        Ok(TranslationOutput {
            text: translated_text,
            lang: Some(*to),
        })
    }
}
```

## 📝 License

This project is licensed under the MIT License.

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

## 🤝 Contributing

Issues and Pull Requests are welcome!

1. Fork this project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Create a Pull Request

## 📧 Contact

If you have any questions or suggestions, please contact us through:

- Create an [Issue](https://github.com/tornado-product/FusionTranslator/issues)
- Send an email to: 63542424@163.com

## 🙏 Acknowledgments

- [Tokio](https://tokio.rs/) - Async runtime
- [Reqwest](https://docs.rs/reqwest/) - HTTP client
- [Serde](https://serde.rs/) - Serialization framework
- [Thiserror](https://github.com/dtolnay/thiserror) - Error handling

---

If you find this project helpful, please give us a ⭐️!
