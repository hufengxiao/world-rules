//! 规则复杂度分析模块
//!
//! 评估规则的复杂度级别，包括认知复杂度、结构复杂度等。

use crate::rules::core::{Rule, RuleMetadata};

/// 复杂度级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ComplexityLevel {
    /// 简单 - 一目了然
    Simple,
    /// 中等 - 需要一定理解
    #[default]
    Moderate,
    /// 复杂 - 需要深入理解
    Complex,
    /// 高度复杂 - 需要专业背景
    HighlyComplex,
    /// 极端复杂 - 需要专家级别
    ExtremelyComplex,
}

impl std::fmt::Display for ComplexityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple => write!(f, "简单"),
            Self::Moderate => write!(f, "中等"),
            Self::Complex => write!(f, "复杂"),
            Self::HighlyComplex => write!(f, "高度复杂"),
            Self::ExtremelyComplex => write!(f, "极端复杂"),
        }
    }
}

impl ComplexityLevel {
    /// 从分数获取级别
    pub fn from_score(score: f64) -> Self {
        if score <= 20.0 {
            Self::Simple
        } else if score <= 40.0 {
            Self::Moderate
        } else if score <= 60.0 {
            Self::Complex
        } else if score <= 80.0 {
            Self::HighlyComplex
        } else {
            Self::ExtremelyComplex
        }
    }
}

/// 复杂度因素
#[derive(Debug, Clone)]
pub struct ComplexityFactor {
    /// 因素名称
    pub name: String,
    /// 因素得分 (0-100)
    pub score: f64,
    /// 权重
    pub weight: f64,
    /// 描述
    pub description: String,
}

impl ComplexityFactor {
    /// 创建新的复杂度因素
    pub fn new(name: &str, score: f64, weight: f64, description: &str) -> Self {
        Self {
            name: name.to_string(),
            score: score.clamp(0.0, 100.0),
            weight,
            description: description.to_string(),
        }
    }

    /// 获取加权得分
    pub fn weighted_score(&self) -> f64 {
        self.score * self.weight
    }
}

/// 规则复杂度报告
#[derive(Debug, Clone)]
pub struct RuleComplexityReport {
    /// 规则名称
    pub rule_name: String,
    /// 整体复杂度级别
    pub level: ComplexityLevel,
    /// 整体复杂度得分 (0-100)
    pub overall_score: f64,
    /// 各因素分析
    pub factors: Vec<ComplexityFactor>,
    /// 改进建议
    pub suggestions: Vec<String>,
}

impl RuleComplexityReport {
    /// 创建新的复杂度报告
    pub fn new(rule_name: String) -> Self {
        Self {
            rule_name,
            level: ComplexityLevel::Simple,
            overall_score: 0.0,
            factors: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// 计算整体得分
    pub fn calculate_overall_score(&mut self) {
        if self.factors.is_empty() {
            self.overall_score = 0.0;
            self.level = ComplexityLevel::Simple;
            return;
        }

        let total_weight: f64 = self.factors.iter().map(|f| f.weight).sum();
        let weighted_sum: f64 = self.factors.iter().map(|f| f.weighted_score()).sum();

        self.overall_score = weighted_sum / total_weight;
        self.level = ComplexityLevel::from_score(self.overall_score);
    }

    /// 获取主要复杂度因素
    pub fn get_major_factors(&self, threshold: f64) -> Vec<&ComplexityFactor> {
        self.factors
            .iter()
            .filter(|f| f.score >= threshold)
            .collect()
    }
}

impl std::fmt::Display for RuleComplexityReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} 复杂度得分: {:.1}%",
            self.level, self.rule_name, self.overall_score
        )
    }
}

/// 规则复杂度分析器
///
/// 用于评估规则的复杂度。
///
/// # 示例
///
/// ```rust
/// use world_rules::rules::analysis::complexity::{RuleComplexityAnalyzer, ComplexityLevel};
///
/// let analyzer = RuleComplexityAnalyzer::new();
///
/// // 分析空规则集
/// let reports = analyzer.analyze_all(&[]);
/// assert!(reports.is_empty());
/// ```
#[derive(Debug, Default)]
pub struct RuleComplexityAnalyzer {
    /// 描述长度权重
    description_weight: f64,
    /// 标签数量权重
    tags_weight: f64,
    /// 分类复杂度权重
    category_weight: f64,
    /// 说明文本权重
    explanation_weight: f64,
}

impl RuleComplexityAnalyzer {
    /// 创建新的复杂度分析器
    pub fn new() -> Self {
        Self {
            description_weight: 0.25,
            tags_weight: 0.15,
            category_weight: 0.20,
            explanation_weight: 0.40,
        }
    }

    /// 分析单个规则的复杂度
    pub fn analyze(&self, rule: &dyn Rule) -> RuleComplexityReport {
        let mut report = RuleComplexityReport::new(rule.metadata().name.clone());
        let meta = rule.metadata();

        // 1. 分析描述长度复杂度
        let desc_score = self.analyze_description_complexity(meta);
        report.factors.push(ComplexityFactor::new(
            "描述长度",
            desc_score,
            self.description_weight,
            &format!("描述长度: {} 字符", meta.description.len()),
        ));

        // 2. 分析标签数量复杂度
        let tags_score = self.analyze_tags_complexity(meta);
        report.factors.push(ComplexityFactor::new(
            "标签数量",
            tags_score,
            self.tags_weight,
            &format!("标签数量: {}", meta.tags.len()),
        ));

        // 3. 分析分类复杂度
        let category = rule.category();
        let category_score = self.analyze_category_complexity(&category.to_string());
        report.factors.push(ComplexityFactor::new(
            "分类复杂度",
            category_score,
            self.category_weight,
            &format!("分类: {}", category),
        ));

        // 4. 分析说明文本复杂度
        let explanation = rule.explain();
        let exp_score = self.analyze_explanation_complexity(&explanation);
        report.factors.push(ComplexityFactor::new(
            "说明复杂度",
            exp_score,
            self.explanation_weight,
            &format!("说明长度: {} 字符", explanation.len()),
        ));

        // 计算整体得分
        report.calculate_overall_score();

        // 生成改进建议
        self.generate_suggestions(&mut report);

        report
    }

    /// 分析描述长度复杂度
    fn analyze_description_complexity(&self, meta: &RuleMetadata) -> f64 {
        let len = meta.description.len();

        // 根据长度计算复杂度得分
        if len <= 20 {
            10.0 // 简单描述
        } else if len <= 50 {
            25.0 // 中等描述
        } else if len <= 100 {
            40.0 // 较长描述
        } else if len <= 200 {
            55.0 // 详细描述
        } else if len <= 500 {
            70.0 // 非常详细
        } else {
            85.0 // 极其详细
        }
    }

    /// 分析标签数量复杂度
    fn analyze_tags_complexity(&self, meta: &RuleMetadata) -> f64 {
        let count = meta.tags.len();

        match count {
            0 => 5.0,       // 无标签
            1..=2 => 15.0,  // 少量标签
            3..=5 => 30.0,  // 适中标签
            6..=10 => 50.0, // 较多标签
            _ => 70.0,      // 大量标签
        }
    }

    /// 分析分类复杂度
    fn analyze_category_complexity(&self, category: &str) -> f64 {
        // 根据分类路径深度计算复杂度
        let depth = category.matches('/').count() + 1;

        match depth {
            1 => 20.0,
            2 => 35.0,
            3 => 50.0,
            4 => 65.0,
            _ => 80.0,
        }
    }

    /// 分析说明文本复杂度
    fn analyze_explanation_complexity(&self, explanation: &str) -> f64 {
        let len = explanation.len();

        // 计算各种复杂度指标
        let has_code = explanation.contains("```")
            || explanation.contains("fn ")
            || explanation.contains("let ");
        let has_formula =
            explanation.contains("=") || explanation.contains("+") || explanation.contains("-");
        let has_list = explanation.contains("- ") || explanation.contains("* ");
        let has_numbering = explanation.contains("1.") || explanation.contains("2.");

        let mut score: f64 = 10.0;

        // 长度贡献
        if len > 100 {
            score += 15.0;
        }
        if len > 300 {
            score += 15.0;
        }
        if len > 500 {
            score += 15.0;
        }

        // 结构复杂度
        if has_code {
            score += 20.0;
        }
        if has_formula {
            score += 10.0;
        }
        if has_list {
            score += 5.0;
        }
        if has_numbering {
            score += 5.0;
        }

        score.min(100.0)
    }

    /// 生成改进建议
    fn generate_suggestions(&self, report: &mut RuleComplexityReport) {
        // 根据复杂度级别给出建议
        match report.level {
            ComplexityLevel::Simple => {
                report
                    .suggestions
                    .push("规则简单易懂，可以考虑添加更多细节说明".to_string());
            }
            ComplexityLevel::Moderate => {
                report
                    .suggestions
                    .push("规则复杂度适中，保持当前水平即可".to_string());
            }
            ComplexityLevel::Complex => {
                report
                    .suggestions
                    .push("规则较为复杂，建议添加示例和分步说明".to_string());
            }
            ComplexityLevel::HighlyComplex => {
                report
                    .suggestions
                    .push("规则高度复杂，建议拆分为多个子规则或提供教程".to_string());
            }
            ComplexityLevel::ExtremelyComplex => {
                report
                    .suggestions
                    .push("规则极其复杂，强烈建议重构或提供详细文档".to_string());
            }
        }

        // 针对特定因素的建议
        for factor in &report.factors {
            if factor.score > 70.0 {
                report.suggestions.push(format!(
                    "建议简化 '{}' 方面的复杂度 ({:.0}%)",
                    factor.name, factor.score
                ));
            }
        }
    }

    /// 分析所有规则的复杂度
    pub fn analyze_all(&self, rules: &[Box<dyn Rule>]) -> Vec<RuleComplexityReport> {
        rules.iter().map(|r| self.analyze(r.as_ref())).collect()
    }

    /// 获取复杂度统计
    pub fn get_statistics(&self, reports: &[RuleComplexityReport]) -> ComplexityStatistics {
        let mut stats = ComplexityStatistics::default();

        for report in reports {
            match report.level {
                ComplexityLevel::Simple => stats.simple_count += 1,
                ComplexityLevel::Moderate => stats.moderate_count += 1,
                ComplexityLevel::Complex => stats.complex_count += 1,
                ComplexityLevel::HighlyComplex => stats.highly_complex_count += 1,
                ComplexityLevel::ExtremelyComplex => stats.extremely_complex_count += 1,
            }
            stats.total_score += report.overall_score;
        }

        stats.total_count = reports.len();
        if !reports.is_empty() {
            stats.average_score = stats.total_score / reports.len() as f64;
        }

        stats
    }
}

/// 复杂度统计信息
#[derive(Debug, Clone, Default)]
pub struct ComplexityStatistics {
    /// 总规则数
    pub total_count: usize,
    /// 简单数量
    pub simple_count: usize,
    /// 中等数量
    pub moderate_count: usize,
    /// 复杂数量
    pub complex_count: usize,
    /// 高度复杂数量
    pub highly_complex_count: usize,
    /// 极端复杂数量
    pub extremely_complex_count: usize,
    /// 总得分
    pub total_score: f64,
    /// 平均得分
    pub average_score: f64,
}

impl std::fmt::Display for ComplexityStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "复杂度统计: 总计 {} (简单: {}, 中等: {}, 复杂: {}, 高度: {}, 极端: {}) 平均: {:.1}%",
            self.total_count,
            self.simple_count,
            self.moderate_count,
            self.complex_count,
            self.highly_complex_count,
            self.extremely_complex_count,
            self.average_score
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complexity_level_ordering() {
        assert!(ComplexityLevel::Simple < ComplexityLevel::Moderate);
        assert!(ComplexityLevel::Moderate < ComplexityLevel::Complex);
        assert!(ComplexityLevel::Complex < ComplexityLevel::HighlyComplex);
        assert!(ComplexityLevel::HighlyComplex < ComplexityLevel::ExtremelyComplex);
    }

    #[test]
    fn test_complexity_level_from_score() {
        assert_eq!(ComplexityLevel::from_score(10.0), ComplexityLevel::Simple);
        assert_eq!(ComplexityLevel::from_score(30.0), ComplexityLevel::Moderate);
        assert_eq!(ComplexityLevel::from_score(50.0), ComplexityLevel::Complex);
        assert_eq!(
            ComplexityLevel::from_score(70.0),
            ComplexityLevel::HighlyComplex
        );
        assert_eq!(
            ComplexityLevel::from_score(90.0),
            ComplexityLevel::ExtremelyComplex
        );
    }

    #[test]
    fn test_complexity_analyzer_empty() {
        let analyzer = RuleComplexityAnalyzer::new();
        let reports = analyzer.analyze_all(&[]);
        assert!(reports.is_empty());
    }

    #[test]
    fn test_complexity_factor() {
        let factor = ComplexityFactor::new("test", 50.0, 2.0, "description");
        assert_eq!(factor.weighted_score(), 100.0);
    }

    #[test]
    fn test_complexity_report() {
        let mut report = RuleComplexityReport::new("test".to_string());
        report
            .factors
            .push(ComplexityFactor::new("f1", 40.0, 1.0, ""));
        report
            .factors
            .push(ComplexityFactor::new("f2", 60.0, 1.0, ""));
        report.calculate_overall_score();

        assert_eq!(report.overall_score, 50.0);
        assert_eq!(report.level, ComplexityLevel::Complex);
    }

    #[test]
    fn test_complexity_statistics() {
        let analyzer = RuleComplexityAnalyzer::new();
        let stats = analyzer.get_statistics(&[]);

        assert_eq!(stats.total_count, 0);
        assert_eq!(stats.average_score, 0.0);
    }
}
