//! 规则冲突检测模块
//!
//! 检测规则集合中的矛盾和冲突，包括语义冲突、逻辑冲突和条件冲突。

use crate::rules::core::Rule;
use std::collections::HashMap;

/// 冲突类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConflictType {
    /// 语义冲突 - 规则名称或描述存在矛盾
    Semantic,
    /// 逻辑冲突 - 规则之间存在逻辑矛盾
    Logical,
    /// 条件冲突 - 规则适用条件存在矛盾
    Conditional,
    /// 行为冲突 - 规则行为存在矛盾
    Behavioral,
    /// 版本冲突 - 规则版本不兼容
    Version,
}

impl std::fmt::Display for ConflictType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Semantic => write!(f, "语义冲突"),
            Self::Logical => write!(f, "逻辑冲突"),
            Self::Conditional => write!(f, "条件冲突"),
            Self::Behavioral => write!(f, "行为冲突"),
            Self::Version => write!(f, "版本冲突"),
        }
    }
}

/// 冲突严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConflictSeverity {
    /// 信息级别 - 不影响功能
    Info,
    /// 警告级别 - 可能影响功能
    Warning,
    /// 错误级别 - 影响功能
    Error,
    /// 严重级别 - 导致系统无法正常运行
    Critical,
}

impl std::fmt::Display for ConflictSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "信息"),
            Self::Warning => write!(f, "警告"),
            Self::Error => write!(f, "错误"),
            Self::Critical => write!(f, "严重"),
        }
    }
}

/// 冲突报告
#[derive(Debug, Clone)]
pub struct ConflictReport {
    /// 冲突类型
    pub conflict_type: ConflictType,
    /// 冲突严重程度
    pub severity: ConflictSeverity,
    /// 涉及的规则名称
    pub rules: Vec<String>,
    /// 冲突描述
    pub description: String,
    /// 修复建议
    pub suggestion: String,
}

impl ConflictReport {
    /// 创建新的冲突报告
    pub fn new(
        conflict_type: ConflictType,
        severity: ConflictSeverity,
        rules: Vec<String>,
        description: String,
        suggestion: String,
    ) -> Self {
        Self {
            conflict_type,
            severity,
            rules,
            description,
            suggestion,
        }
    }
}

impl std::fmt::Display for ConflictReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} ({}): {} - 建议: {}",
            self.severity,
            self.conflict_type,
            self.rules.join(", "),
            self.description,
            self.suggestion
        )
    }
}

/// 规则冲突检测器
///
/// 用于检测规则集合中存在的各种冲突。
///
/// # 示例
///
/// ```rust
/// use world_rules::rules::analysis::conflict::{ConflictDetector, ConflictType, ConflictSeverity};
///
/// let detector = ConflictDetector::new();
///
/// // 检测空规则集
/// let reports = detector.detect_conflicts(&[]);
/// assert!(reports.is_empty());
/// ```
#[derive(Debug, Default)]
pub struct ConflictDetector {
    /// 冲突检测阈值
    threshold: f64,
    /// 是否启用语义检测
    semantic_check: bool,
    /// 是否启用逻辑检测
    logical_check: bool,
}

impl ConflictDetector {
    /// 创建新的冲突检测器
    pub fn new() -> Self {
        Self {
            threshold: 0.7,
            semantic_check: true,
            logical_check: true,
        }
    }

    /// 设置检测阈值 (0.0 - 1.0)
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// 启用或禁用语义检测
    pub fn with_semantic_check(mut self, enabled: bool) -> Self {
        self.semantic_check = enabled;
        self
    }

    /// 启用或禁用逻辑检测
    pub fn with_logical_check(mut self, enabled: bool) -> Self {
        self.logical_check = enabled;
        self
    }

    /// 检测规则集合中的冲突
    pub fn detect_conflicts(&self, rules: &[Box<dyn Rule>]) -> Vec<ConflictReport> {
        let mut conflicts = Vec::new();

        if rules.is_empty() {
            return conflicts;
        }

        // 检测语义冲突
        if self.semantic_check {
            conflicts.extend(self.detect_semantic_conflicts(rules));
        }

        // 检测逻辑冲突
        if self.logical_check {
            conflicts.extend(self.detect_logical_conflicts(rules));
        }

        // 检测条件冲突
        conflicts.extend(self.detect_conditional_conflicts(rules));

        // 检测行为冲突
        conflicts.extend(self.detect_behavioral_conflicts(rules));

        conflicts
    }

    /// 检测语义冲突
    fn detect_semantic_conflicts(&self, rules: &[Box<dyn Rule>]) -> Vec<ConflictReport> {
        let mut conflicts = Vec::new();
        let mut name_count: HashMap<String, Vec<String>> = HashMap::new();

        for rule in rules {
            let name = rule.metadata().name.clone();
            let category = rule.category().to_string();
            name_count.entry(name).or_default().push(category);
        }

        // 检测同名称规则在不同分类中的冲突
        for (name, categories) in name_count {
            if categories.len() > 1 {
                conflicts.push(ConflictReport::new(
                    ConflictType::Semantic,
                    ConflictSeverity::Warning,
                    vec![name.clone()],
                    format!(
                        "规则名称 '{}' 在多个分类中出现: {}",
                        name,
                        categories.join(", ")
                    ),
                    "建议使用唯一的规则名称或添加命名空间前缀".to_string(),
                ));
            }
        }

        conflicts
    }

    /// 检测逻辑冲突
    fn detect_logical_conflicts(&self, rules: &[Box<dyn Rule>]) -> Vec<ConflictReport> {
        let mut conflicts = Vec::new();

        // 检测描述中的矛盾关键词
        for rule in rules {
            let desc = &rule.metadata().description;

            // 检测矛盾关键词对
            let contradictions = [
                ("必须", "禁止"),
                ("允许", "不允许"),
                ("可以", "不可以"),
                ("总是", "从不"),
                ("必须", "可选"),
            ];

            for (word1, word2) in contradictions {
                if desc.contains(word1) && desc.contains(word2) {
                    conflicts.push(ConflictReport::new(
                        ConflictType::Logical,
                        ConflictSeverity::Error,
                        vec![rule.metadata().name.clone()],
                        format!("规则描述中存在矛盾关键词: '{}' 和 '{}'", word1, word2),
                        "建议重新审视规则描述，消除矛盾表述".to_string(),
                    ));
                }
            }
        }

        conflicts
    }

    /// 检测条件冲突
    fn detect_conditional_conflicts(&self, rules: &[Box<dyn Rule>]) -> Vec<ConflictReport> {
        let mut conflicts = Vec::new();

        // 检测空标签冲突
        for rule in rules {
            if rule.metadata().tags.is_empty() {
                conflicts.push(ConflictReport::new(
                    ConflictType::Conditional,
                    ConflictSeverity::Info,
                    vec![rule.metadata().name.clone()],
                    "规则缺少标签，可能影响搜索和分类".to_string(),
                    "建议添加至少一个标签以便规则检索".to_string(),
                ));
            }
        }

        conflicts
    }

    /// 检测行为冲突
    fn detect_behavioral_conflicts(&self, rules: &[Box<dyn Rule>]) -> Vec<ConflictReport> {
        let mut conflicts = Vec::new();

        // 检测版本兼容性
        let mut version_map: HashMap<String, Vec<String>> = HashMap::new();
        for rule in rules {
            let category = rule.category().to_string();
            let version = rule.metadata().version.clone();
            version_map.entry(category).or_default().push(version);
        }

        for (category, versions) in version_map {
            if versions.len() > 1 {
                conflicts.push(ConflictReport::new(
                    ConflictType::Version,
                    ConflictSeverity::Warning,
                    vec![category.clone()],
                    format!(
                        "分类 '{}' 中存在多个版本: {}",
                        category,
                        versions.join(", ")
                    ),
                    "建议统一同一分类下的规则版本".to_string(),
                ));
            }
        }

        conflicts
    }

    /// 分析冲突严重程度分布
    pub fn analyze_severity_distribution(
        &self,
        conflicts: &[ConflictReport],
    ) -> HashMap<ConflictSeverity, usize> {
        let mut distribution = HashMap::new();
        for conflict in conflicts {
            *distribution.entry(conflict.severity).or_insert(0) += 1;
        }
        distribution
    }

    /// 获取冲突统计摘要
    pub fn get_conflict_summary(&self, conflicts: &[ConflictReport]) -> ConflictSummary {
        let mut summary = ConflictSummary::default();

        for conflict in conflicts {
            match conflict.severity {
                ConflictSeverity::Info => summary.info_count += 1,
                ConflictSeverity::Warning => summary.warning_count += 1,
                ConflictSeverity::Error => summary.error_count += 1,
                ConflictSeverity::Critical => summary.critical_count += 1,
            }
        }

        summary.total_count = conflicts.len();
        summary.has_critical = summary.critical_count > 0;
        summary.has_errors = summary.error_count > 0;

        summary
    }
}

/// 冲突统计摘要
#[derive(Debug, Clone, Default)]
pub struct ConflictSummary {
    /// 总冲突数
    pub total_count: usize,
    /// 信息级别数量
    pub info_count: usize,
    /// 警告级别数量
    pub warning_count: usize,
    /// 错误级别数量
    pub error_count: usize,
    /// 严重级别数量
    pub critical_count: usize,
    /// 是否存在严重冲突
    pub has_critical: bool,
    /// 是否存在错误
    pub has_errors: bool,
}

impl std::fmt::Display for ConflictSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "冲突统计: 总计 {} (严重: {}, 错误: {}, 警告: {}, 信息: {})",
            self.total_count,
            self.critical_count,
            self.error_count,
            self.warning_count,
            self.info_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_detector_empty() {
        let detector = ConflictDetector::new();
        let conflicts = detector.detect_conflicts(&[]);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_conflict_type_display() {
        assert_eq!(format!("{}", ConflictType::Semantic), "语义冲突");
        assert_eq!(format!("{}", ConflictType::Logical), "逻辑冲突");
    }

    #[test]
    fn test_conflict_severity_ordering() {
        assert!(ConflictSeverity::Info < ConflictSeverity::Warning);
        assert!(ConflictSeverity::Warning < ConflictSeverity::Error);
        assert!(ConflictSeverity::Error < ConflictSeverity::Critical);
    }

    #[test]
    fn test_conflict_summary() {
        let detector = ConflictDetector::new();
        let summary = detector.get_conflict_summary(&[]);
        assert_eq!(summary.total_count, 0);
        assert!(!summary.has_critical);
        assert!(!summary.has_errors);
    }

    #[test]
    fn test_conflict_report_creation() {
        let report = ConflictReport::new(
            ConflictType::Semantic,
            ConflictSeverity::Warning,
            vec!["rule1".to_string()],
            "测试冲突".to_string(),
            "测试建议".to_string(),
        );

        assert_eq!(report.conflict_type, ConflictType::Semantic);
        assert_eq!(report.severity, ConflictSeverity::Warning);
        assert_eq!(report.rules.len(), 1);
    }
}
