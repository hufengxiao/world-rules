//! # AI 辅助规则生成模块
//!
//! 提供基于大语言模型（LLM）的规则生成、验证和优化功能。
//!
//! ## 功能特性
//!
//! - **LLM 规则生成接口**: 抽象的 LLM 提供商接口，支持多种后端
//! - **规则模板学习**: 从现有规则中学习模式和结构
//! - **规则验证生成**: 生成可编译、可运行的规则代码
//! - **规则优化建议**: 分析现有规则并提供优化建议
//!
//! ## 示例
//!
//! ```rust
//! use world_rules::ai::{LLMProvider, RuleGenerator, GenerateConfig};
//!
//! // 创建规则生成器
//! let config = GenerateConfig::default();
//! let generator = RuleGenerator::new(config);
//!
//! // 生成新规则
//! let prompt = "生成一个简单的扑克牌型验证规则";
//! let result = generator.generate(prompt).unwrap();
//! println!("生成的规则: {}", result.code);
//! ```
//!
//! ## LLM 提供商支持
//!
//! - OpenAI (GPT-4, GPT-3.5)
//! - Anthropic (Claude)
//! - 本地模型 (通过自定义实现)
//! - 其他兼容 OpenAI API 的服务

pub mod generator;
pub mod learner;
pub mod llm;
pub mod optimizer;
pub mod template;
pub mod validated_generator;
pub mod validator;

pub use generator::{GenerateConfig, GenerateResult, RuleGenerator};
pub use learner::{LearningStats, RulePattern, TemplateLearner};
pub use llm::{LLMConfig, LLMError, LLMProvider, MockLLMProvider};
pub use optimizer::{OptimizationReport, OptimizationSuggestion, RuleOptimizer, SuggestionType};
pub use template::{RuleTemplate, TemplateLibrary};
pub use validated_generator::{
    GenerateStats, ValidatedGenerateConfig, ValidatedGenerateResult, ValidatedRuleGenerator,
};
pub use validator::{GeneratedRuleValidator, ValidationResult};

/// AI 模块的公共接口
pub mod prelude {
    pub use crate::ai::{GenerateConfig, GenerateResult, RuleGenerator};
    pub use crate::ai::{GenerateStats, ValidatedGenerateConfig, ValidatedRuleGenerator};
    pub use crate::ai::{GeneratedRuleValidator, ValidationResult};
    pub use crate::ai::{LLMConfig, LLMProvider};
    pub use crate::ai::{LearningStats, RulePattern, TemplateLearner};
    pub use crate::ai::{
        OptimizationReport, OptimizationSuggestion, RuleOptimizer, SuggestionType,
    };
    pub use crate::ai::{RuleTemplate, TemplateLibrary};
}
