//! # 规则验证器
//!
//! 验证生成的规则代码是否有效。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// 是否通过验证
    pub passed: bool,

    /// 验证分数（0.0 - 1.0）
    pub score: f32,

    /// 错误列表
    pub errors: Vec<String>,

    /// 警告列表
    pub warnings: Vec<String>,

    /// 建议
    pub suggestions: Vec<String>,

    /// 详细指标
    pub metrics: HashMap<String, f32>,
}

impl ValidationResult {
    /// 创建新的验证结果
    pub fn new() -> Self {
        Self {
            passed: false,
            score: 0.0,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
            metrics: HashMap::new(),
        }
    }

    /// 创建通过的结果
    pub fn passed(score: f32) -> Self {
        Self {
            passed: true,
            score,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
            metrics: HashMap::new(),
        }
    }

    /// 创建失败的结果
    pub fn failed(errors: Vec<String>) -> Self {
        Self {
            passed: false,
            score: 0.0,
            errors,
            warnings: Vec::new(),
            suggestions: Vec::new(),
            metrics: HashMap::new(),
        }
    }

    /// 添加错误
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
        self.passed = false;
    }

    /// 添加警告
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// 添加建议
    pub fn add_suggestion(&mut self, suggestion: impl Into<String>) {
        self.suggestions.push(suggestion.into());
    }

    /// 设置指标
    pub fn set_metric(&mut self, name: impl Into<String>, value: f32) {
        self.metrics.insert(name.into(), value);
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// 生成的规则验证器
///
/// 验证生成的规则代码是否符合要求。
///
/// # 示例
///
/// ```rust
/// use world_rules::ai::GeneratedRuleValidator;
///
/// let validator = GeneratedRuleValidator::new();
///
/// let code = r#"
/// struct MyRule;
/// impl Rule for MyRule {}
/// "#;
///
/// let result = validator.validate(code);
/// println!("验证通过: {}", result.passed);
/// ```
pub struct GeneratedRuleValidator {
    /// 是否检查语法
    check_syntax: bool,

    /// 是否检查结构
    check_structure: bool,

    /// 是否检查 trait 实现
    check_trait_impl: bool,
}

impl GeneratedRuleValidator {
    /// 创建新的验证器
    pub fn new() -> Self {
        Self {
            check_syntax: true,
            check_structure: true,
            check_trait_impl: true,
        }
    }

    /// 验证规则代码
    ///
    /// # 参数
    ///
    /// - `code`: 要验证的代码
    ///
    /// # 返回
    ///
    /// 返回验证结果
    pub fn validate(&self, code: &str) -> ValidationResult {
        let mut result = ValidationResult::new();
        let mut score = 1.0;

        // 1. 检查基本结构
        if self.check_structure {
            self.check_structure_features(code, &mut result, &mut score);
        }

        // 2. 检查语法特征
        if self.check_syntax {
            self.check_syntax_features(code, &mut result, &mut score);
        }

        // 3. 检查 trait 实现
        if self.check_trait_impl {
            self.check_trait_implementation(code, &mut result, &mut score);
        }

        // 设置最终分数
        result.score = score;
        result.passed = result.errors.is_empty() && score >= 0.6;

        // 添加建议
        self.add_suggestions(&mut result);

        result
    }

    /// 检查结构特征
    fn check_structure_features(&self, code: &str, result: &mut ValidationResult, score: &mut f32) {
        // 检查是否有 struct 定义
        if !code.contains("struct ") {
            result.add_error("缺少结构体定义");
            *score -= 0.2;
        } else {
            result.set_metric("has_struct", 1.0);
        }

        // 检查是否有合理的命名
        if code.contains("Test") || code.contains("Example") {
            result.add_warning("使用了示例名称，建议使用更具描述性的名称");
        }

        // 检查是否有文档注释
        if code.contains("///") || code.contains("//!") {
            result.set_metric("has_docs", 1.0);
        } else {
            result.add_suggestion("建议添加文档注释");
        }
    }

    /// 检查语法特征
    fn check_syntax_features(&self, code: &str, result: &mut ValidationResult, score: &mut f32) {
        // 检查括号匹配
        let open_braces = code.matches('{').count();
        let close_braces = code.matches('}').count();

        if open_braces != close_braces {
            result.add_error(format!(
                "大括号不匹配: {} 个 '{{' 但 {} 个 '}}'",
                open_braces, close_braces
            ));
            *score -= 0.3;
        }

        // 检查是否有明显的语法错误
        if code.contains("...") && !code.contains("..") {
            result.add_error("包含无效的省略号");
            *score -= 0.1;
        }

        // 检查是否有 TODO 标记
        if code.contains("TODO") {
            result.add_warning("代码包含 TODO 标记，需要完善实现");
        }
    }

    /// 检查 trait 实现
    fn check_trait_implementation(
        &self,
        code: &str,
        result: &mut ValidationResult,
        score: &mut f32,
    ) {
        // 检查是否实现 Rule trait
        if code.contains("impl Rule") {
            result.set_metric("implements_rule", 1.0);

            // 检查必需的方法
            if code.contains("fn metadata(") {
                result.set_metric("has_metadata", 1.0);
            } else {
                result.add_error("缺少 metadata() 方法");
                *score -= 0.1;
            }

            if code.contains("fn category(") {
                result.set_metric("has_category", 1.0);
            } else {
                result.add_error("缺少 category() 方法");
                *score -= 0.1;
            }

            if code.contains("fn validate(") {
                result.set_metric("has_validate", 1.0);
            } else {
                result.add_error("缺少 validate() 方法");
                *score -= 0.1;
            }

            if code.contains("fn explain(") {
                result.set_metric("has_explain", 1.0);
            } else {
                result.add_error("缺少 explain() 方法");
                *score -= 0.1;
            }
        } else {
            result.add_warning("建议实现 Rule trait");
        }
    }

    /// 添加建议
    fn add_suggestions(&self, result: &mut ValidationResult) {
        if !result.metrics.contains_key("has_docs") {
            result.add_suggestion("添加文档注释以提高代码可读性");
        }

        if !result.metrics.contains_key("has_tests") && !result.errors.is_empty() {
            result.add_suggestion("添加测试用例验证规则功能");
        }

        if result.score < 0.8 {
            result.add_suggestion("优化代码结构以提高质量分数");
        }
    }

    /// 快速验证（只检查关键特征）
    pub fn quick_validate(&self, code: &str) -> bool {
        code.contains("struct ") && code.contains("impl Rule")
    }
}

impl Default for GeneratedRuleValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_creation() {
        let result = ValidationResult::new();
        assert!(!result.passed);
        assert_eq!(result.score, 0.0);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validation_result_passed() {
        let result = ValidationResult::passed(0.9);
        assert!(result.passed);
        assert_eq!(result.score, 0.9);
    }

    #[test]
    fn test_validation_result_failed() {
        let errors = vec!["错误1".to_string(), "错误2".to_string()];
        let result = ValidationResult::failed(errors);
        assert!(!result.passed);
        assert_eq!(result.errors.len(), 2);
    }

    #[test]
    fn test_validator_basic() {
        let validator = GeneratedRuleValidator::new();

        let code = r#"
struct MyRule;

impl Rule for MyRule {
    fn metadata(&self) -> RuleMetadata { unimplemented!() }
    fn category(&self) -> RuleCategory { unimplemented!() }
    fn validate(&self, ctx: &ValidateContext) -> Result<bool, RuleError> { Ok(true) }
    fn explain(&self) -> String { String::new() }
}
"#;

        let result = validator.validate(code);
        assert!(result.passed);
        assert!(result.score > 0.6);
    }

    #[test]
    fn test_validator_missing_struct() {
        let validator = GeneratedRuleValidator::new();

        let code = "impl Rule for Test {}";
        let result = validator.validate(code);

        assert!(!result.passed);
        assert!(result.errors.iter().any(|e| e.contains("结构体")));
    }

    #[test]
    fn test_validator_brace_mismatch() {
        let validator = GeneratedRuleValidator::new();

        let code = "struct Test {";
        let result = validator.validate(code);

        assert!(!result.passed);
        assert!(result.errors.iter().any(|e| e.contains("大括号")));
    }

    #[test]
    fn test_validator_missing_methods() {
        let validator = GeneratedRuleValidator::new();

        let code = "struct Test;\nimpl Rule for Test {}";
        let result = validator.validate(code);

        assert!(result.errors.iter().any(|e| e.contains("metadata")));
    }

    #[test]
    fn test_quick_validate() {
        let validator = GeneratedRuleValidator::new();

        let valid_code = "struct Test;\nimpl Rule for Test {}";
        assert!(validator.quick_validate(valid_code));

        let invalid_code = "struct Test;";
        assert!(!validator.quick_validate(invalid_code));
    }

    #[test]
    fn test_validation_with_warnings() {
        let validator = GeneratedRuleValidator::new();

        let code = r#"
struct TestRule;

impl Rule for TestRule {
    fn metadata(&self) -> RuleMetadata { unimplemented!() }
    fn category(&self) -> RuleCategory { unimplemented!() }
    fn validate(&self, ctx: &ValidateContext) -> Result<bool, RuleError> { Ok(true) }
    fn explain(&self) -> String { String::new() }
}
"#;

        let result = validator.validate(code);
        assert!(result.passed);
    }
}
