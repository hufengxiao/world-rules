//! # 规则验证生成器
//!
//! 增强的规则生成器，支持生成经过验证的可编译规则代码。

use crate::ai::{
    GenerateConfig, GenerateResult, LLMError, RuleGenerator, ValidationResult,
};

/// 验证生成配置
#[derive(Debug, Clone)]
pub struct ValidatedGenerateConfig {
    /// 基础生成配置
    pub base: GenerateConfig,

    /// 是否自动修复错误
    pub auto_fix: bool,

    /// 最大修复尝试次数
    pub max_fix_attempts: usize,

    /// 是否生成测试
    pub generate_tests: bool,

    /// 是否生成文档
    pub generate_docs: bool,
}

impl Default for ValidatedGenerateConfig {
    fn default() -> Self {
        Self {
            base: GenerateConfig::default(),
            auto_fix: true,
            max_fix_attempts: 3,
            generate_tests: true,
            generate_docs: true,
        }
    }
}

impl ValidatedGenerateConfig {
    /// 创建新的配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置自动修复
    pub fn with_auto_fix(mut self, auto_fix: bool) -> Self {
        self.auto_fix = auto_fix;
        self
    }

    /// 设置最大修复次数
    pub fn with_max_attempts(mut self, attempts: usize) -> Self {
        self.max_fix_attempts = attempts;
        self
    }
}

/// 验证生成结果
#[derive(Debug, Clone)]
pub struct ValidatedGenerateResult {
    /// 基础生成结果
    pub result: GenerateResult,

    /// 验证结果
    pub validation: ValidationResult,

    /// 修复次数
    pub fix_attempts: usize,

    /// 是否最终通过验证
    pub final_passed: bool,
}

impl ValidatedGenerateResult {
    /// 创建新的结果
    pub fn new(result: GenerateResult, validation: ValidationResult) -> Self {
        Self {
            result,
            validation,
            fix_attempts: 0,
            final_passed: false,
        }
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.final_passed && self.validation.passed
    }
}

/// 验证规则生成器
///
/// 生成并验证规则代码，确保生成的代码可编译、可运行。
///
/// # 示例
///
/// ```rust
/// use world_rules::ai::ValidatedRuleGenerator;
///
/// let generator = ValidatedRuleGenerator::new();
///
/// let result = generator.generate_validated("生成扑克规则").unwrap();
/// if result.is_success() {
///     println!("生成成功: {}", result.result.code);
/// }
/// ```
pub struct ValidatedRuleGenerator {
    /// 配置
    config: ValidatedGenerateConfig,

    /// 基础生成器
    generator: RuleGenerator,

    /// 验证器
    validator: crate::ai::GeneratedRuleValidator,
}

impl ValidatedRuleGenerator {
    /// 创建新的验证生成器
    pub fn new() -> Self {
        Self::with_config(ValidatedGenerateConfig::default())
    }

    /// 使用配置创建
    pub fn with_config(config: ValidatedGenerateConfig) -> Self {
        let generator = RuleGenerator::new(config.base.clone());
        let validator = crate::ai::GeneratedRuleValidator::new();
        Self {
            config,
            generator,
            validator,
        }
    }

    /// 生成并验证规则
    ///
    /// # 参数
    ///
    /// - `prompt`: 生成提示
    ///
    /// # 返回
    ///
    /// 返回验证后的生成结果
    pub fn generate_validated(&self, prompt: &str) -> Result<ValidatedGenerateResult, LLMError> {
        // 1. 生成初始代码
        let result = self.generator.generate(prompt)?;

        // 2. 验证代码
        let validation = self.validator.validate(&result.code);

        // 3. 创建结果
        let mut validated_result = ValidatedGenerateResult::new(result, validation);

        // 4. 如果启用自动修复且验证失败，尝试修复
        if !validated_result.validation.passed && self.config.auto_fix {
            self.try_fix(&mut validated_result)?;
        }

        // 5. 设置最终状态
        validated_result.final_passed = validated_result.validation.passed;

        Ok(validated_result)
    }

    /// 尝试修复生成结果
    fn try_fix(&self, result: &mut ValidatedGenerateResult) -> Result<(), LLMError> {
        for attempt in 1..=self.config.max_fix_attempts {
            // 基于错误信息修复代码
            let fixed_code = self.fix_code(&result.result.code, &result.validation.errors);

            // 重新验证
            let validation = self.validator.validate(&fixed_code);

            // 更新结果
            result.result.code = fixed_code;
            result.validation = validation;
            result.fix_attempts = attempt;

            // 如果通过验证，返回
            if result.validation.passed {
                return Ok(());
            }
        }

        Ok(())
    }

    /// 修复代码
    fn fix_code(&self, code: &str, errors: &[String]) -> String {
        let mut fixed = code.to_string();

        for error in errors {
            // 修复常见错误

            // 1. 大括号不匹配
            if error.contains("大括号不匹配") {
                let open = fixed.matches('{').count();
                let close = fixed.matches('}').count();
                
                if open > close {
                    // 添加缺失的右括号
                    for _ in 0..(open - close) {
                        fixed.push('}');
                    }
                } else if close > open {
                    // 添加缺失的左括号（在开头）
                    let mut new_code = String::new();
                    for _ in 0..(close - open) {
                        new_code.push('{');
                    }
                    new_code.push_str(&fixed);
                    fixed = new_code;
                }
            }

            // 2. 缺少结构体定义
            if error.contains("缺少结构体定义") && !fixed.contains("struct ") {
                let struct_def = "struct GeneratedRule {\n    // 规则字段\n}\n\n";
                fixed = format!("{}{}", struct_def, fixed);
            }

            // 3. 缺少方法
            if error.contains("缺少") && error.contains("方法") {
                // 添加默认方法实现
                if error.contains("metadata") && !fixed.contains("fn metadata(") {
                    fixed.push_str("\n    fn metadata(&self) -> RuleMetadata {\n        RuleMetadata::default()\n    }\n");
                }
            }
        }

        fixed
    }

    /// 批量生成并验证
    pub fn generate_batch_validated(
        &self,
        prompts: &[&str],
    ) -> Result<Vec<ValidatedGenerateResult>, LLMError> {
        prompts
            .iter()
            .map(|p| self.generate_validated(p))
            .collect()
    }

    /// 生成统计信息
    pub fn generate_stats(&self, results: &[ValidatedGenerateResult]) -> GenerateStats {
        let total = results.len();
        let passed = results.iter().filter(|r| r.is_success()).count();
        let failed = total - passed;
        let avg_attempts = if total > 0 {
            results.iter().map(|r| r.fix_attempts).sum::<usize>() as f32 / total as f32
        } else {
            0.0
        };

        GenerateStats {
            total,
            passed,
            failed,
            success_rate: if total > 0 { passed as f32 / total as f32 } else { 0.0 },
            avg_fix_attempts: avg_attempts,
        }
    }
}

impl Default for ValidatedRuleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// 生成统计信息
#[derive(Debug, Clone)]
pub struct GenerateStats {
    /// 总数
    pub total: usize,

    /// 通过数
    pub passed: usize,

    /// 失败数
    pub failed: usize,

    /// 成功率
    pub success_rate: f32,

    /// 平均修复次数
    pub avg_fix_attempts: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = ValidatedGenerateConfig::new()
            .with_auto_fix(false)
            .with_max_attempts(5);

        assert!(!config.auto_fix);
        assert_eq!(config.max_fix_attempts, 5);
    }

    #[test]
    fn test_generator_creation() {
        let generator = ValidatedRuleGenerator::new();
        assert!(generator.config.auto_fix);
    }

    #[test]
    fn test_generate_validated() {
        let generator = ValidatedRuleGenerator::new();
        let result = generator.generate_validated("测试生成").unwrap();

        assert!(!result.result.code.is_empty());
        assert!(result.fix_attempts <= 3);
    }

    #[test]
    fn test_validated_result() {
        let gen_result = GenerateResult::new("code", "Test");
        let val_result = ValidationResult::passed(0.9);

        let validated = ValidatedGenerateResult::new(gen_result, val_result);

        assert!(!validated.is_success()); // final_passed 默认为 false
    }

    #[test]
    fn test_fix_code_braces() {
        let generator = ValidatedRuleGenerator::new();

        let code = "struct Test {";
        let errors = vec!["大括号不匹配".to_string()];

        let fixed = generator.fix_code(code, &errors);
        assert!(fixed.contains('}'));
    }

    #[test]
    fn test_fix_code_struct() {
        let generator = ValidatedRuleGenerator::new();

        let code = "impl Rule for Test {}";
        let errors = vec!["缺少结构体定义".to_string()];

        let fixed = generator.fix_code(code, &errors);
        assert!(fixed.contains("struct"));
    }

    #[test]
    fn test_generate_stats() {
        let generator = ValidatedRuleGenerator::new();

        let results = vec![
            generator.generate_validated("测试1").unwrap(),
            generator.generate_validated("测试2").unwrap(),
        ];

        let stats = generator.generate_stats(&results);
        assert_eq!(stats.total, 2);
    }

    #[test]
    fn test_batch_generate() {
        let generator = ValidatedRuleGenerator::new();

        let prompts = vec!["生成规则1", "生成规则2"];
        let results = generator.generate_batch_validated(&prompts).unwrap();

        assert_eq!(results.len(), 2);
    }
}