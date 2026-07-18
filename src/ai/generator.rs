//! # 规则生成器
//!
//! 基于大语言模型的规则生成功能。

use crate::ai::{LLMError, LLMProvider};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 规则生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateConfig {
    /// 是否包含示例代码
    pub include_examples: bool,

    /// 是否包含文档注释
    pub include_docs: bool,

    /// 是否包含测试
    pub include_tests: bool,

    /// 目标规则类别（如 "law", "sports", "games"）
    pub target_category: Option<String>,

    /// 额外的生成提示
    pub extra_hints: Vec<String>,

    /// 是否严格模式（生成可编译的代码）
    pub strict_mode: bool,
}

impl Default for GenerateConfig {
    fn default() -> Self {
        Self {
            include_examples: true,
            include_docs: true,
            include_tests: true,
            target_category: None,
            extra_hints: Vec::new(),
            strict_mode: true,
        }
    }
}

impl GenerateConfig {
    /// 创建新的生成配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置目标规则类别
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.target_category = Some(category.into());
        self
    }

    /// 设置严格模式
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// 添加额外提示
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.extra_hints.push(hint.into());
        self
    }
}

/// 规则生成结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResult {
    /// 生成的代码
    pub code: String,

    /// 规则名称
    pub rule_name: String,

    /// 规则描述
    pub description: String,

    /// 生成的测试代码
    pub test_code: Option<String>,

    /// 使用示例
    pub examples: Vec<String>,

    /// 元数据
    pub metadata: HashMap<String, String>,

    /// 是否通过验证
    pub validated: bool,

    /// 验证错误（如果有）
    pub validation_errors: Vec<String>,
}

impl GenerateResult {
    /// 创建新的生成结果
    pub fn new(code: impl Into<String>, rule_name: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            rule_name: rule_name.into(),
            description: String::new(),
            test_code: None,
            examples: Vec::new(),
            metadata: HashMap::new(),
            validated: false,
            validation_errors: Vec::new(),
        }
    }

    /// 添加测试代码
    pub fn with_tests(mut self, test_code: impl Into<String>) -> Self {
        self.test_code = Some(test_code.into());
        self
    }

    /// 添加示例
    pub fn add_example(mut self, example: impl Into<String>) -> Self {
        self.examples.push(example.into());
        self
    }

    /// 设置描述
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// 设置验证状态
    pub fn set_validated(&mut self, validated: bool, errors: Vec<String>) {
        self.validated = validated;
        self.validation_errors = errors;
    }
}

/// 规则生成器
///
/// 使用 LLM 提供商生成新的规则代码。
///
/// # 示例
///
/// ```rust
/// use world_rules::ai::{RuleGenerator, GenerateConfig, MockLLMProvider};
///
/// let config = GenerateConfig::default();
/// let provider = MockLLMProvider::default_provider();
/// let generator = RuleGenerator::with_provider(config, Box::new(provider));
///
/// let result = generator.generate("生成扑克牌型验证规则").unwrap();
/// println!("生成规则: {}", result.rule_name);
/// ```
pub struct RuleGenerator {
    config: GenerateConfig,
    provider: Box<dyn LLMProvider>,
}

impl RuleGenerator {
    /// 创建新的规则生成器
    pub fn new(config: GenerateConfig) -> Self {
        // 使用模拟提供商作为默认
        use crate::ai::MockLLMProvider;
        use crate::ai::LLMConfig;
        
        let provider = MockLLMProvider::new(LLMConfig::default());
        Self {
            config,
            provider: Box::new(provider),
        }
    }

    /// 使用自定义提供商创建生成器
    pub fn with_provider(config: GenerateConfig, provider: Box<dyn LLMProvider>) -> Self {
        Self { config, provider }
    }

    /// 生成规则
    ///
    /// # 参数
    ///
    /// - `prompt`: 生成提示（描述需要生成的规则）
    ///
    /// # 返回
    ///
    /// 返回生成结果或错误
    ///
    /// # 示例
    ///
    /// ```rust
    /// use world_rules::ai::{RuleGenerator, GenerateConfig};
    ///
    /// let generator = RuleGenerator::new(GenerateConfig::default());
    /// let result = generator.generate("生成麻将胡牌规则").unwrap();
    /// ```
    pub fn generate(&self, prompt: &str) -> Result<GenerateResult, LLMError> {
        // 构建系统提示词
        let system_prompt = self.build_system_prompt();
        
        // 构建用户提示词
        let user_prompt = self.build_user_prompt(prompt);
        
        // 调用 LLM
        let response = self.provider.generate_with_system(&system_prompt, &user_prompt)?;
        
        // 解析响应
        let result = self.parse_response(&response)?;
        
        Ok(result)
    }

    /// 批量生成规则
    pub fn generate_batch(&self, prompts: &[&str]) -> Result<Vec<GenerateResult>, LLMError> {
        prompts.iter().map(|p| self.generate(p)).collect()
    }

    /// 构建系统提示词
    fn build_system_prompt(&self) -> String {
        let mut prompt = String::from(
            "你是一个 Rust 规则生成助手。你的任务是根据用户的需求生成符合 world_rules 库规范的 Rust 规则代码。\n\n"
        );

        prompt.push_str("规则代码要求：\n");
        prompt.push_str("1. 实现 Rule trait\n");
        prompt.push_str("2. 包含完整的元数据（名称、版本、描述）\n");
        prompt.push_str("3. 提供验证功能\n");
        prompt.push_str("4. 包含详细说明\n\n");

        if self.config.include_docs {
            prompt.push_str("5. 添加 rustdoc 文档注释\n");
            prompt.push_str("6. 包含使用示例\n\n");
        }

        if self.config.include_tests {
            prompt.push_str("7. 提供测试用例\n\n");
        }

        if self.config.strict_mode {
            prompt.push_str("严格要求：生成的代码必须可编译、可运行，符合 Rust 语法规范。\n\n");
        }

        prompt.push_str("输出格式：\n");
        prompt.push_str("```rust\n// 生成的规则代码\n```\n\n");

        prompt
    }

    /// 构建用户提示词
    fn build_user_prompt(&self, base_prompt: &str) -> String {
        let mut prompt = base_prompt.to_string();

        if let Some(ref category) = self.config.target_category {
            prompt.push_str(&format!("\n目标规则类别: {}", category));
        }

        for hint in &self.config.extra_hints {
            prompt.push_str(&format!("\n额外要求: {}", hint));
        }

        prompt
    }

    /// 解析 LLM 响应
    fn parse_response(&self, response: &str) -> Result<GenerateResult, LLMError> {
        // 简单解析：提取代码块
        let code = self.extract_code_block(response)?;
        
        // 提取规则名称（简单实现）
        let rule_name = self.extract_rule_name(&code)?;
        
        // 创建结果
        let mut result = GenerateResult::new(code, rule_name);
        result.description = response.to_string();
        
        Ok(result)
    }

    /// 提取代码块
    fn extract_code_block(&self, text: &str) -> Result<String, LLMError> {
        // 查找第一个 ```rust 代码块
        if let Some(start) = text.find("```rust") {
            let start = start + 7; // 跳过 ```rust
            if let Some(end) = text[start..].find("```") {
                return Ok(text[start..start + end].trim().to_string());
            }
        }
        
        // 如果没有找到代码块，返回整个响应（作为简单实现）
        Ok(text.to_string())
    }

    /// 提取规则名称
    fn extract_rule_name(&self, code: &str) -> Result<String, LLMError> {
        // 简单实现：查找 struct 或 fn 定义
        for line in code.lines() {
            let line = line.trim();
            if line.starts_with("struct ") {
                let name = line
                    .strip_prefix("struct ")
                    .unwrap_or("")
                    .split_whitespace()
                    .next()
                    .unwrap_or("GeneratedRule");
                return Ok(name.to_string());
            }
        }
        
        Ok("GeneratedRule".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_config_builder() {
        let config = GenerateConfig::new()
            .with_category("games")
            .with_strict_mode(true)
            .with_hint("包含边界检查");

        assert_eq!(config.target_category, Some("games".to_string()));
        assert!(config.strict_mode);
        assert_eq!(config.extra_hints.len(), 1);
    }

    #[test]
    fn test_generate_result() {
        let result = GenerateResult::new("code", "TestRule")
            .with_description("测试规则")
            .with_tests("#[test]\nfn test() {}");

        assert_eq!(result.code, "code");
        assert_eq!(result.rule_name, "TestRule");
        assert_eq!(result.description, "测试规则");
        assert!(result.test_code.is_some());
    }

    #[test]
    fn test_rule_generator_creation() {
        let generator = RuleGenerator::new(GenerateConfig::default());
        assert!(generator.config.include_examples);
    }

    #[test]
    fn test_generate() {
        let generator = RuleGenerator::new(GenerateConfig::default());
        let result = generator.generate("测试生成").unwrap();
        
        assert!(!result.code.is_empty());
        assert!(!result.rule_name.is_empty());
    }

    #[test]
    fn test_generate_batch() {
        let generator = RuleGenerator::new(GenerateConfig::default());
        let prompts = vec!["生成规则1", "生成规则2"];
        
        let results = generator.generate_batch(&prompts).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_extract_code_block() {
        let generator = RuleGenerator::new(GenerateConfig::default());
        
        let text = "说明文字\n```rust\nfn test() {}\n```\n更多文字";
        let code = generator.extract_code_block(text).unwrap();
        assert_eq!(code, "fn test() {}");
    }

    #[test]
    fn test_extract_rule_name() {
        let generator = RuleGenerator::new(GenerateConfig::default());
        
        let code = "struct MyRule { field: i32 }";
        let name = generator.extract_rule_name(code).unwrap();
        assert_eq!(name, "MyRule");
    }
}