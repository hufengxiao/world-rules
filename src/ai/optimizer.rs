//! # 规则优化建议系统
//!
//! 分析现有规则并提供优化建议。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 优化建议类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SuggestionType {
    /// 性能优化
    Performance,

    /// 代码质量
    CodeQuality,

    /// 可维护性
    Maintainability,

    /// 测试覆盖
    TestCoverage,

    /// 文档完善
    Documentation,

    /// 安全性
    Security,

    /// API 设计
    ApiDesign,
}

/// 优化建议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    /// 建议类型
    pub suggestion_type: SuggestionType,

    /// 建议标题
    pub title: String,

    /// 建议描述
    pub description: String,

    /// 优先级（1-5，1最高）
    pub priority: u8,

    /// 影响范围
    pub impact: String,

    /// 修复建议
    pub fix_suggestion: String,

    /// 相关代码位置
    pub location: Option<String>,

    /// 示例代码
    pub example_code: Option<String>,
}

impl OptimizationSuggestion {
    /// 创建新的优化建议
    pub fn new(
        suggestion_type: SuggestionType,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            suggestion_type,
            title: title.into(),
            description: description.into(),
            priority: 3,
            impact: String::new(),
            fix_suggestion: String::new(),
            location: None,
            example_code: None,
        }
    }

    /// 设置优先级
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority.clamp(1, 5);
        self
    }

    /// 设置影响范围
    pub fn with_impact(mut self, impact: impl Into<String>) -> Self {
        self.impact = impact.into();
        self
    }

    /// 设置修复建议
    pub fn with_fix(mut self, fix: impl Into<String>) -> Self {
        self.fix_suggestion = fix.into();
        self
    }

    /// 设置代码位置
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// 设置示例代码
    pub fn with_example(mut self, code: impl Into<String>) -> Self {
        self.example_code = Some(code.into());
        self
    }
}

/// 优化报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationReport {
    /// 分析的规则数量
    pub rules_analyzed: usize,

    /// 生成的建议数量
    pub suggestions_count: usize,

    /// 按类型分类的建议
    pub by_type: HashMap<String, usize>,

    /// 高优先级建议数量
    pub high_priority_count: usize,

    /// 建议列表
    pub suggestions: Vec<OptimizationSuggestion>,

    /// 整体评分（0-100）
    pub overall_score: f32,
}

impl OptimizationReport {
    /// 创建新的优化报告
    pub fn new() -> Self {
        Self {
            rules_analyzed: 0,
            suggestions_count: 0,
            by_type: HashMap::new(),
            high_priority_count: 0,
            suggestions: Vec::new(),
            overall_score: 100.0,
        }
    }

    /// 添加建议
    pub fn add_suggestion(&mut self, suggestion: OptimizationSuggestion) {
        // 更新统计
        let type_key = format!("{:?}", suggestion.suggestion_type);
        *self.by_type.entry(type_key).or_insert(0) += 1;

        if suggestion.priority <= 2 {
            self.high_priority_count += 1;
        }

        self.suggestions.push(suggestion);
        self.suggestions_count += 1;
    }

    /// 获取高优先级建议
    pub fn high_priority_suggestions(&self) -> Vec<&OptimizationSuggestion> {
        self.suggestions
            .iter()
            .filter(|s| s.priority <= 2)
            .collect()
    }

    /// 获取指定类型的建议
    pub fn suggestions_by_type(&self, suggestion_type: &SuggestionType) -> Vec<&OptimizationSuggestion> {
        self.suggestions
            .iter()
            .filter(|s| s.suggestion_type == *suggestion_type)
            .collect()
    }
}

impl Default for OptimizationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// 规则优化分析器
///
/// 分析规则代码并提供优化建议。
///
/// # 示例
///
/// ```rust
/// use world_rules::ai::RuleOptimizer;
///
/// let optimizer = RuleOptimizer::new();
///
/// let code = r#"
/// struct MyRule;
/// impl Rule for MyRule {
///     fn validate(&self, ctx: &ValidateContext) -> Result<bool, RuleError> {
///         Ok(true)
///     }
/// }
/// "#;
///
/// let report = optimizer.analyze(code);
/// println!("找到 {} 个优化建议", report.suggestions_count);
/// ```
pub struct RuleOptimizer {
    /// 是否检查性能
    check_performance: bool,

    /// 是否检查文档
    check_documentation: bool,

    /// 是否检查测试
    check_tests: bool,

    /// 是否检查安全性
    check_security: bool,
}

impl RuleOptimizer {
    /// 创建新的优化分析器
    pub fn new() -> Self {
        Self {
            check_performance: true,
            check_documentation: true,
            check_tests: true,
            check_security: true,
        }
    }

    /// 分析规则代码
    ///
    /// # 参数
    ///
    /// - `code`: 规则源代码
    ///
    /// # 返回
    ///
    /// 返回优化报告
    pub fn analyze(&self, code: &str) -> OptimizationReport {
        let mut report = OptimizationReport::new();
        report.rules_analyzed = 1;

        // 1. 检查文档
        if self.check_documentation {
            self.check_documentation(code, &mut report);
        }

        // 2. 检查性能
        if self.check_performance {
            self.check_performance(code, &mut report);
        }

        // 3. 检查测试
        if self.check_tests {
            self.check_tests(code, &mut report);
        }

        // 4. 检查安全性
        if self.check_security {
            self.check_security(code, &mut report);
        }

        // 5. 检查代码质量
        self.check_code_quality(code, &mut report);

        // 6. 检查 API 设计
        self.check_api_design(code, &mut report);

        // 计算整体评分
        self.calculate_score(&mut report);

        report
    }

    /// 批量分析
    pub fn analyze_batch(&self, codes: &[&str]) -> OptimizationReport {
        let mut combined_report = OptimizationReport::new();
        combined_report.rules_analyzed = codes.len();

        for code in codes {
            let report = self.analyze(code);
            
            // 合并建议
            for suggestion in report.suggestions {
                combined_report.add_suggestion(suggestion);
            }
        }

        self.calculate_score(&mut combined_report);
        combined_report
    }

    // ===== 检查方法 =====

    /// 检查文档
    fn check_documentation(&self, code: &str, report: &mut OptimizationReport) {
        // 检查是否有文档注释
        if !code.contains("///") && !code.contains("//!") {
            report.add_suggestion(
                OptimizationSuggestion::new(
                    SuggestionType::Documentation,
                    "添加文档注释",
                    "规则缺少文档注释，建议添加 rustdoc 注释以提高可读性",
                )
                .with_priority(2)
                .with_impact("提高代码可读性和可维护性")
                .with_fix("添加 /// 注释说明规则用途和使用方法")
                .with_example("/// 检查扑克牌型是否有效\n/// \n/// # 示例\n/// ```\n/// let rule = PokerRule::new();\n/// ```"),
            );
        }

        // 检查是否有示例
        if !code.contains("# Example") && !code.contains("# 示例") {
            report.add_suggestion(
                OptimizationSuggestion::new(
                    SuggestionType::Documentation,
                    "添加使用示例",
                    "文档中缺少使用示例，建议添加代码示例",
                )
                .with_priority(3)
                .with_impact("帮助用户快速上手"),
            );
        }
    }

    /// 检查性能
    fn check_performance(&self, code: &str, report: &mut OptimizationReport) {
        // 检查是否有不必要的克隆
        if code.contains(".clone()") {
            let clone_count = code.matches(".clone()").count();
            if clone_count > 2 {
                report.add_suggestion(
                    OptimizationSuggestion::new(
                        SuggestionType::Performance,
                        "减少不必要的克隆操作",
                        format!("代码中包含 {} 个 clone() 调用，可能影响性能", clone_count),
                    )
                    .with_priority(2)
                    .with_impact("提高性能，减少内存分配")
                    .with_fix("考虑使用引用或 Arc 代替克隆"),
                );
            }
        }

        // 检查是否有大字符串拼接
        if code.contains("push_str") || code.contains("format!") {
            let format_count = code.matches("format!").count();
            if format_count > 3 {
                report.add_suggestion(
                    OptimizationSuggestion::new(
                        SuggestionType::Performance,
                        "优化字符串操作",
                        "大量字符串拼接可能影响性能",
                    )
                    .with_priority(3)
                    .with_fix("考虑使用 String::with_capacity 预分配"),
                );
            }
        }
    }

    /// 检查测试
    fn check_tests(&self, code: &str, report: &mut OptimizationReport) {
        // 检查是否有测试
        if !code.contains("#[test]") {
            report.add_suggestion(
                OptimizationSuggestion::new(
                    SuggestionType::TestCoverage,
                    "添加单元测试",
                    "规则缺少单元测试",
                )
                .with_priority(1)
                .with_impact("提高代码可靠性")
                .with_fix("添加 #[test] 测试用例验证规则功能"),
            );
        }

        // 检查测试数量
        let test_count = code.matches("#[test]").count();
        if test_count > 0 && test_count < 3 {
            report.add_suggestion(
                OptimizationSuggestion::new(
                    SuggestionType::TestCoverage,
                    "增加测试覆盖",
                    format!("当前只有 {} 个测试，建议增加测试覆盖", test_count),
                )
                .with_priority(2),
            );
        }
    }

    /// 检查安全性
    fn check_security(&self, code: &str, report: &mut OptimizationReport) {
        // 检查是否有 unsafe 代码
        if code.contains("unsafe") {
            report.add_suggestion(
                OptimizationSuggestion::new(
                    SuggestionType::Security,
                    "审查 unsafe 代码",
                    "代码中包含 unsafe 块，需要仔细审查安全性",
                )
                .with_priority(1)
                .with_impact("确保内存安全")
                .with_fix("尽可能使用安全的 Rust 代码"),
            );
        }

        // 检查是否有 unwrap
        let unwrap_count = code.matches(".unwrap()").count();
        if unwrap_count > 0 {
            report.add_suggestion(
                OptimizationSuggestion::new(
                    SuggestionType::Security,
                    "处理潜在的 panic",
                    format!("代码中包含 {} 个 unwrap()，可能导致 panic", unwrap_count),
                )
                .with_priority(2)
                .with_fix("使用 ? 操作符或 match 处理错误"),
            );
        }
    }

    /// 检查代码质量
    fn check_code_quality(&self, code: &str, report: &mut OptimizationReport) {
        // 检查函数长度
        for line in code.lines() {
            if line.len() > 120 {
                report.add_suggestion(
                    OptimizationSuggestion::new(
                        SuggestionType::CodeQuality,
                        "减少行长度",
                        "代码行过长，建议拆分以提高可读性",
                    )
                    .with_priority(3)
                    .with_fix("将长行拆分为多行"),
                );
                break;
            }
        }

        // 检查是否有 TODO
        if code.contains("TODO") || code.contains("FIXME") {
            report.add_suggestion(
                OptimizationSuggestion::new(
                    SuggestionType::Maintainability,
                    "处理待办事项",
                    "代码中包含 TODO 或 FIXME 标记",
                )
                .with_priority(2)
                .with_fix("完成或移除 TODO 标记"),
            );
        }
    }

    /// 检查 API 设计
    fn check_api_design(&self, code: &str, report: &mut OptimizationReport) {
        // 检查是否实现了 Rule trait
        if code.contains("impl Rule") {
            // 检查是否有构造函数
            if !code.contains("fn new(") && !code.contains("pub fn new(") {
                report.add_suggestion(
                    OptimizationSuggestion::new(
                        SuggestionType::ApiDesign,
                        "添加构造函数",
                        "建议添加 new() 构造函数以方便使用",
                    )
                    .with_priority(3)
                    .with_example("pub fn new() -> Self { Self { } }"),
                );
            }

            // 检查是否有 Default 实现
            if !code.contains("impl Default") && !code.contains("#[derive(Default)]") {
                report.add_suggestion(
                    OptimizationSuggestion::new(
                        SuggestionType::ApiDesign,
                        "实现 Default trait",
                        "建议实现 Default trait 以支持默认值",
                    )
                    .with_priority(4),
                );
            }
        }
    }

    /// 计算整体评分
    fn calculate_score(&self, report: &mut OptimizationReport) {
        let mut score = 100.0;

        // 根据建议数量扣分
        let deduction = report.suggestions_count as f32 * 5.0;
        score -= deduction.min(50.0);

        // 高优先级建议额外扣分
        let high_priority_deduction = report.high_priority_count as f32 * 10.0;
        score -= high_priority_deduction.min(30.0);

        // 确保评分在 0-100 之间
        report.overall_score = score.clamp(0.0, 100.0);
    }

    /// 获取优化建议摘要
    pub fn get_summary(&self, report: &OptimizationReport) -> String {
        format!(
            "分析了 {} 个规则，生成 {} 个优化建议（其中 {} 个高优先级），整体评分: {:.1}/100",
            report.rules_analyzed,
            report.suggestions_count,
            report.high_priority_count,
            report.overall_score
        )
    }
}

impl Default for RuleOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestion_creation() {
        let suggestion = OptimizationSuggestion::new(
            SuggestionType::Performance,
            "测试建议",
            "测试描述",
        )
        .with_priority(2)
        .with_impact("测试影响");

        assert_eq!(suggestion.suggestion_type, SuggestionType::Performance);
        assert_eq!(suggestion.title, "测试建议");
        assert_eq!(suggestion.priority, 2);
    }

    #[test]
    fn test_optimization_report() {
        let mut report = OptimizationReport::new();
        assert_eq!(report.suggestions_count, 0);

        let suggestion = OptimizationSuggestion::new(
            SuggestionType::CodeQuality,
            "测试",
            "测试",
        ).with_priority(1);

        report.add_suggestion(suggestion);
        assert_eq!(report.suggestions_count, 1);
        assert_eq!(report.high_priority_count, 1);
    }

    #[test]
    fn test_optimizer_creation() {
        let optimizer = RuleOptimizer::new();
        assert!(optimizer.check_performance);
        assert!(optimizer.check_documentation);
    }

    #[test]
    fn test_analyze_basic() {
        let optimizer = RuleOptimizer::new();

        let code = "struct Test;\nimpl Rule for Test {}";
        let report = optimizer.analyze(code);

        assert!(report.suggestions_count > 0);
        assert!(report.overall_score < 100.0);
    }

    #[test]
    fn test_analyze_with_docs() {
        let optimizer = RuleOptimizer::new();

        let code = r#"
/// 测试规则
struct Test;

impl Rule for Test {
    fn validate(&self, ctx: &ValidateContext) -> Result<bool, RuleError> {
        Ok(true)
    }
}

#[test]
fn test_rule() {}
"#;

        let report = optimizer.analyze(code);
        assert!(report.overall_score > 50.0);
    }

    #[test]
    fn test_check_documentation() {
        let optimizer = RuleOptimizer::new();

        let code = "struct Test;";
        let report = optimizer.analyze(code);

        let doc_suggestions = report.suggestions_by_type(&SuggestionType::Documentation);
        assert!(!doc_suggestions.is_empty());
    }

    #[test]
    fn test_check_tests() {
        let optimizer = RuleOptimizer::new();

        let code = "struct Test;\nimpl Rule for Test {}";
        let report = optimizer.analyze(code);

        let test_suggestions = report.suggestions_by_type(&SuggestionType::TestCoverage);
        assert!(!test_suggestions.is_empty());
    }

    #[test]
    fn test_check_unsafe() {
        let optimizer = RuleOptimizer::new();

        let code = "unsafe { }";
        let report = optimizer.analyze(code);

        let security_suggestions = report.suggestions_by_type(&SuggestionType::Security);
        assert!(!security_suggestions.is_empty());
    }

    #[test]
    fn test_analyze_batch() {
        let optimizer = RuleOptimizer::new();

        let codes = vec!["struct A;", "struct B;"];
        let report = optimizer.analyze_batch(&codes);

        assert_eq!(report.rules_analyzed, 2);
    }

    #[test]
    fn test_high_priority_suggestions() {
        let mut report = OptimizationReport::new();

        report.add_suggestion(
            OptimizationSuggestion::new(SuggestionType::TestCoverage, "高", "")
                .with_priority(1),
        );
        report.add_suggestion(
            OptimizationSuggestion::new(SuggestionType::CodeQuality, "低", "")
                .with_priority(5),
        );

        let high = report.high_priority_suggestions();
        assert_eq!(high.len(), 1);
    }

    #[test]
    fn test_get_summary() {
        let optimizer = RuleOptimizer::new();

        let report = optimizer.analyze("struct Test;");
        let summary = optimizer.get_summary(&report);

        assert!(summary.contains("分析了 1 个规则"));
    }
}