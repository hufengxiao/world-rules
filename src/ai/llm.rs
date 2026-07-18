//! # LLM 提供商接口
//!
//! 定义了大语言模型（LLM）的抽象接口，支持多种提供商。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LLM 提供商的抽象接口
///
/// 所有 LLM 提供商都需要实现此接口，以便统一的规则生成器使用。
///
/// # 示例
///
/// ```rust
/// use world_rules::ai::{LLMProvider, LLMConfig};
///
/// // 使用模拟提供商进行测试
/// let config = LLMConfig::default();
/// let provider = world_rules::ai::MockLLMProvider::new(config);
///
/// let response = provider.generate("生成一个简单的规则").unwrap();
/// println!("生成结果: {}", response);
/// ```
pub trait LLMProvider: Send + Sync {
    /// 生成文本内容
    ///
    /// # 参数
    ///
    /// - `prompt`: 输入提示词
    ///
    /// # 返回
    ///
    /// 返回生成的文本内容或错误
    ///
    /// # 错误
    ///
    /// - `LLMError::ConnectionError`: 网络连接失败
    /// - `LLMError::AuthenticationError`: API 密钥无效
    /// - `LLMError::RateLimitError`: 超过请求限制
    /// - `LLMError::GenerationError`: 生成失败
    fn generate(&self, prompt: &str) -> Result<String, LLMError>;

    /// 使用系统提示词生成内容
    ///
    /// # 参数
    ///
    /// - `system_prompt`: 系统提示词（定义 AI 的角色和行为）
    /// - `user_prompt`: 用户提示词（具体任务）
    fn generate_with_system(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, LLMError> {
        // 默认实现：合并提示词
        let combined = format!("{}\n\n{}", system_prompt, user_prompt);
        self.generate(&combined)
    }

    /// 批量生成内容
    ///
    /// # 参数
    ///
    /// - `prompts`: 提示词列表
    ///
    /// # 返回
    ///
    /// 返回生成结果列表（顺序与输入一致）
    fn generate_batch(&self, prompts: &[&str]) -> Result<Vec<String>, LLMError> {
        prompts.iter().map(|p| self.generate(p)).collect()
    }

    /// 获取提供商名称
    fn provider_name(&self) -> &str;

    /// 获取模型名称
    fn model_name(&self) -> &str;
}

/// LLM 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// API 密钥（如果需要）
    pub api_key: Option<String>,

    /// API 端点（如果需要）
    pub endpoint: Option<String>,

    /// 模型名称
    pub model: String,

    /// 最大生成令牌数
    pub max_tokens: usize,

    /// 温度参数（0.0 - 2.0，越高越随机）
    pub temperature: f32,

    /// Top-p 采样参数
    pub top_p: f32,

    /// 额外参数
    pub extra: HashMap<String, String>,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            endpoint: None,
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 2048,
            temperature: 0.7,
            top_p: 1.0,
            extra: HashMap::new(),
        }
    }
}

impl LLMConfig {
    /// 创建新的配置
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            ..Default::default()
        }
    }

    /// 设置 API 密钥
    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// 设置 API 端点
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// 设置温度参数
    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = temp.clamp(0.0, 2.0);
        self
    }

    /// 设置最大令牌数
    pub fn with_max_tokens(mut self, tokens: usize) -> Self {
        self.max_tokens = tokens;
        self
    }
}

/// LLM 错误类型
#[derive(Debug, Clone, thiserror::Error)]
pub enum LLMError {
    /// 网络连接错误
    #[error("网络连接错误: {0}")]
    ConnectionError(String),

    /// 认证错误
    #[error("认证错误: API 密钥无效或缺失")]
    AuthenticationError,

    /// 请求限制错误
    #[error("请求限制: {0}")]
    RateLimitError(String),

    /// 生成错误
    #[error("生成失败: {0}")]
    GenerationError(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// 解析错误
    #[error("响应解析错误: {0}")]
    ParseError(String),
}

/// 模拟 LLM 提供商（用于测试）
///
/// 这是一个简单的模拟实现，用于测试和开发环境。
/// 它不需要真实的 API 调用，而是返回预定义的响应。
///
/// # 示例
///
/// ```rust
/// use world_rules::ai::{MockLLMProvider, LLMConfig, LLMProvider};
///
/// let config = LLMConfig::default();
/// let provider = MockLLMProvider::new(config);
///
/// // 生成内容
/// let result = provider.generate("测试提示词").unwrap();
/// assert!(result.contains("模拟生成"));
/// ```
pub struct MockLLMProvider {
    config: LLMConfig,
}

impl MockLLMProvider {
    /// 创建新的模拟提供商
    pub fn new(config: LLMConfig) -> Self {
        Self { config }
    }

    /// 创建默认配置的模拟提供商
    pub fn default_provider() -> Self {
        Self::new(LLMConfig::default())
    }
}

impl LLMProvider for MockLLMProvider {
    fn generate(&self, prompt: &str) -> Result<String, LLMError> {
        // 模拟生成一个简单的规则响应
        let response = format!(
            "模拟生成响应（提供商: {}, 模型: {}）\n\n提示词: {}\n\n生成的规则结构:\n```rust\nstruct GeneratedRule {{\n    name: String,\n    rules: Vec<String>,\n}}\n```",
            self.provider_name(),
            self.model_name(),
            prompt
        );
        Ok(response)
    }

    fn provider_name(&self) -> &str {
        "MockLLM"
    }

    fn model_name(&self) -> &str {
        &self.config.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_config_builder() {
        let config = LLMConfig::new("gpt-4")
            .with_api_key("test-key")
            .with_endpoint("https://api.example.com")
            .with_temperature(0.8)
            .with_max_tokens(4096);

        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.api_key, Some("test-key".to_string()));
        assert_eq!(config.endpoint, Some("https://api.example.com".to_string()));
        assert_eq!(config.temperature, 0.8);
        assert_eq!(config.max_tokens, 4096);
    }

    #[test]
    fn test_mock_provider() {
        let provider = MockLLMProvider::default_provider();

        let result = provider.generate("测试提示词").unwrap();
        assert!(result.contains("模拟生成"));
        assert!(result.contains("测试提示词"));
    }

    #[test]
    fn test_provider_info() {
        let provider = MockLLMProvider::default_provider();

        assert_eq!(provider.provider_name(), "MockLLM");
        assert_eq!(provider.model_name(), "gpt-3.5-turbo");
    }

    #[test]
    fn test_generate_with_system() {
        let provider = MockLLMProvider::default_provider();

        let result = provider
            .generate_with_system("你是一个规则生成助手", "生成扑克规则")
            .unwrap();

        assert!(result.contains("模拟生成"));
    }

    #[test]
    fn test_generate_batch() {
        let provider = MockLLMProvider::default_provider();

        let prompts = vec!["提示词1", "提示词2", "提示词3"];
        let results = provider.generate_batch(&prompts).unwrap();

        assert_eq!(results.len(), 3);
        for result in results {
            assert!(result.contains("模拟生成"));
        }
    }

    #[test]
    fn test_temperature_clamping() {
        let config = LLMConfig::default().with_temperature(5.0);
        assert_eq!(config.temperature, 2.0);

        let config = LLMConfig::default().with_temperature(-1.0);
        assert_eq!(config.temperature, 0.0);
    }
}
