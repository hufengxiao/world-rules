//! 规则完整性分析模块
//!
//! 分析规则是否定义完整，检查必要字段、描述质量、标签等。

use crate::rules::core::Rule;

/// 完整性级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum CompletenessLevel {
    /// 不完整 - 缺少必要字段
    #[default]
    Incomplete,
    /// 基本完整 - 包含必要字段
    Basic,
    /// 标准 - 包含所有推荐字段
    Standard,
    /// 完整 - 包含所有可选字段
    Complete,
    /// 优秀 - 超出完整性要求
    Excellent,
}

impl std::fmt::Display for CompletenessLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Incomplete => write!(f, "不完整"),
            Self::Basic => write!(f, "基本完整"),
            Self::Standard => write!(f, "标准"),
            Self::Complete => write!(f, "完整"),
            Self::Excellent => write!(f, "优秀"),
        }
    }
}

/// 完整性检查项
#[derive(Debug, Clone)]
pub struct CompletenessItem {
    /// 检查项名称
    pub name: String,
    /// 是否通过
    pub passed: bool,
    /// 权重
    pub weight: f64,
    /// 描述
    pub description: String,
    /// 改进建议
    pub suggestion: Option<String>,
}

impl CompletenessItem {
    /// 创建新的完整性检查项
    pub fn new(name: &str, passed: bool, weight: f64, description: &str) -> Self {
        Self {
            name: name.to_string(),
            passed,
            weight,
            description: description.to_string(),
            suggestion: None,
        }
    }

    /// 添加改进建议
    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }
}

/// 完整性报告
#[derive(Debug, Clone)]
pub struct CompletenessReport {
    /// 规则名称
    pub rule_name: String,
    /// 完整性级别
    pub level: CompletenessLevel,
    /// 完整性得分 (0-100)
    pub score: f64,
    /// 检查项列表
    pub items: Vec<CompletenessItem>,
    /// 整体建议
    pub overall_suggestion: String,
}

impl CompletenessReport {
    /// 创建新的完整性报告
    pub fn new(rule_name: String) -> Self {
        Self {
            rule_name,
            level: CompletenessLevel::Incomplete,
            score: 0.0,
            items: Vec::new(),
            overall_suggestion: String::new(),
        }
    }

    /// 计算完整性得分
    pub fn calculate_score(&mut self) {
        if self.items.is_empty() {
            self.score = 0.0;
            self.level = CompletenessLevel::Incomplete;
            return;
        }

        let total_weight: f64 = self.items.iter().map(|i| i.weight).sum();
        let passed_weight: f64 = self
            .items
            .iter()
            .filter(|i| i.passed)
            .map(|i| i.weight)
            .sum();

        self.score = (passed_weight / total_weight) * 100.0;

        // 根据得分确定级别
        self.level = if self.score >= 95.0 {
            CompletenessLevel::Excellent
        } else if self.score >= 85.0 {
            CompletenessLevel::Complete
        } else if self.score >= 70.0 {
            CompletenessLevel::Standard
        } else if self.score >= 50.0 {
            CompletenessLevel::Basic
        } else {
            CompletenessLevel::Incomplete
        };
    }

    /// 获取未通过的检查项
    pub fn failed_items(&self) -> Vec<&CompletenessItem> {
        self.items.iter().filter(|i| !i.passed).collect()
    }

    /// 获取通过的检查项
    pub fn passed_items(&self) -> Vec<&CompletenessItem> {
        self.items.iter().filter(|i| i.passed).collect()
    }
}

impl std::fmt::Display for CompletenessReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} 完整性得分: {:.1}% ({}项通过/{}项)",
            self.level,
            self.rule_name,
            self.score,
            self.passed_items().len(),
            self.items.len()
        )
    }
}

/// 规则完整性分析器
///
/// 用于分析规则定义的完整性。
///
/// # 示例
///
/// ```rust
/// use world_rules::rules::analysis::completeness::{CompletenessAnalyzer, CompletenessLevel};
///
/// let analyzer = CompletenessAnalyzer::new();
///
/// // 分析空规则集
/// let reports = analyzer.analyze_all(&[]);
/// assert!(reports.is_empty());
/// ```
#[derive(Debug, Default)]
pub struct CompletenessAnalyzer {
    /// 最低描述长度要求
    min_description_length: usize,
    /// 最少标签数量要求
    min_tags_count: usize,
    /// 是否检查来源
    check_origin: bool,
    /// 是否检查版本
    check_version: bool,
}

impl CompletenessAnalyzer {
    /// 创建新的完整性分析器
    pub fn new() -> Self {
        Self {
            min_description_length: 10,
            min_tags_count: 1,
            check_origin: true,
            check_version: true,
        }
    }

    /// 设置最小描述长度
    pub fn with_min_description_length(mut self, length: usize) -> Self {
        self.min_description_length = length;
        self
    }

    /// 设置最小标签数量
    pub fn with_min_tags_count(mut self, count: usize) -> Self {
        self.min_tags_count = count;
        self
    }

    /// 设置是否检查来源
    pub fn with_origin_check(mut self, check: bool) -> Self {
        self.check_origin = check;
        self
    }

    /// 设置是否检查版本
    pub fn with_version_check(mut self, check: bool) -> Self {
        self.check_version = check;
        self
    }

    /// 分析单个规则的完整性
    pub fn analyze(&self, rule: &dyn Rule) -> CompletenessReport {
        let mut report = CompletenessReport::new(rule.metadata().name.clone());
        let meta = rule.metadata();

        // 1. 检查名称 (必需, 权重高)
        report.items.push(
            CompletenessItem::new("名称", !meta.name.is_empty(), 2.0, "规则名称不能为空")
                .with_suggestion("请为规则添加一个描述性的名称"),
        );

        // 2. 检查描述 (必需, 权重高)
        let desc_valid = meta.description.len() >= self.min_description_length;
        report.items.push(
            CompletenessItem::new(
                "描述",
                desc_valid,
                2.0,
                &format!("描述长度应至少{}个字符", self.min_description_length),
            )
            .with_suggestion("请添加更详细的规则描述"),
        );

        // 3. 检查版本 (推荐)
        if self.check_version {
            let version_valid = !meta.version.is_empty() && meta.version != "1.0.0";
            report.items.push(
                CompletenessItem::new("版本", version_valid, 1.0, "版本号应有意义")
                    .with_suggestion("建议使用语义化版本号"),
            );
        }

        // 4. 检查来源 (推荐)
        if self.check_origin {
            report.items.push(
                CompletenessItem::new(
                    "来源",
                    meta.origin.is_some(),
                    1.0,
                    "规则来源有助于理解规则背景",
                )
                .with_suggestion("建议添加规则来源或地区"),
            );
        }

        // 5. 检查标签 (推荐)
        let tags_valid = meta.tags.len() >= self.min_tags_count;
        report.items.push(
            CompletenessItem::new(
                "标签",
                tags_valid,
                1.0,
                &format!("应至少有{}个标签", self.min_tags_count),
            )
            .with_suggestion("添加标签有助于规则分类和搜索"),
        );

        // 6. 检查分类
        let category = rule.category();
        report.items.push(CompletenessItem::new(
            "分类",
            true,
            1.0,
            &format!("分类: {}", category),
        ));

        // 7. 检查说明文本 (可选)
        let explanation = rule.explain();
        let has_good_explanation =
            explanation.len() > 50 && explanation.contains("：") || explanation.contains(":");
        report.items.push(
            CompletenessItem::new("详细说明", has_good_explanation, 1.5, "应有详细的规则说明")
                .with_suggestion("建议添加包含示例的详细说明"),
        );

        // 计算最终得分
        report.calculate_score();

        // 生成整体建议
        let failed_count = report.failed_items().len();
        if failed_count == 0 {
            report.overall_suggestion = "规则定义完整，无需改进".to_string();
        } else {
            report.overall_suggestion = format!(
                "规则完整性可改进，建议关注: {}",
                report
                    .failed_items()
                    .iter()
                    .map(|i| i.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        report
    }

    /// 分析所有规则的完整性
    pub fn analyze_all(&self, rules: &[Box<dyn Rule>]) -> Vec<CompletenessReport> {
        rules.iter().map(|r| self.analyze(r.as_ref())).collect()
    }

    /// 获取完整性统计
    pub fn get_statistics(&self, reports: &[CompletenessReport]) -> CompletenessStatistics {
        let mut stats = CompletenessStatistics::default();

        for report in reports {
            match report.level {
                CompletenessLevel::Excellent => stats.excellent_count += 1,
                CompletenessLevel::Complete => stats.complete_count += 1,
                CompletenessLevel::Standard => stats.standard_count += 1,
                CompletenessLevel::Basic => stats.basic_count += 1,
                CompletenessLevel::Incomplete => stats.incomplete_count += 1,
            }
            stats.total_score += report.score;
        }

        stats.total_count = reports.len();
        if !reports.is_empty() {
            stats.average_score = stats.total_score / reports.len() as f64;
        }

        stats
    }
}

/// 完整性统计信息
#[derive(Debug, Clone, Default)]
pub struct CompletenessStatistics {
    /// 总规则数
    pub total_count: usize,
    /// 优秀数量
    pub excellent_count: usize,
    /// 完整数量
    pub complete_count: usize,
    /// 标准数量
    pub standard_count: usize,
    /// 基本数量
    pub basic_count: usize,
    /// 不完整数量
    pub incomplete_count: usize,
    /// 总得分
    pub total_score: f64,
    /// 平均得分
    pub average_score: f64,
}

impl std::fmt::Display for CompletenessStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "完整性统计: 总计 {} (优秀: {}, 完整: {}, 标准: {}, 基本: {}, 不完整: {}) 平均得分: {:.1}%",
            self.total_count,
            self.excellent_count,
            self.complete_count,
            self.standard_count,
            self.basic_count,
            self.incomplete_count,
            self.average_score
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completeness_level_ordering() {
        assert!(CompletenessLevel::Incomplete < CompletenessLevel::Basic);
        assert!(CompletenessLevel::Basic < CompletenessLevel::Standard);
        assert!(CompletenessLevel::Standard < CompletenessLevel::Complete);
        assert!(CompletenessLevel::Complete < CompletenessLevel::Excellent);
    }

    #[test]
    fn test_completeness_analyzer_empty() {
        let analyzer = CompletenessAnalyzer::new();
        let reports = analyzer.analyze_all(&[]);
        assert!(reports.is_empty());
    }

    #[test]
    fn test_completeness_report_score() {
        let mut report = CompletenessReport::new("test".to_string());
        report
            .items
            .push(CompletenessItem::new("item1", true, 1.0, "test"));
        report
            .items
            .push(CompletenessItem::new("item2", false, 1.0, "test"));
        report.calculate_score();

        assert_eq!(report.score, 50.0);
        assert_eq!(report.level, CompletenessLevel::Basic);
    }

    #[test]
    fn test_completeness_item() {
        let item =
            CompletenessItem::new("test", true, 1.0, "description").with_suggestion("suggestion");

        assert_eq!(item.name, "test");
        assert!(item.passed);
        assert!(item.suggestion.is_some());
    }

    #[test]
    fn test_completeness_statistics() {
        let analyzer = CompletenessAnalyzer::new();
        let stats = analyzer.get_statistics(&[]);

        assert_eq!(stats.total_count, 0);
        assert_eq!(stats.average_score, 0.0);
    }
}
