//! 规则质量评分模块
//!
//! 综合评估规则质量，涵盖正确性、可用性、可维护性、文档质量等维度。

use crate::rules::core::Rule;

/// 质量维度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualityDimension {
    /// 正确性 - 规则逻辑是否正确
    Correctness,
    /// 可用性 - 规则是否易于使用
    Usability,
    /// 可维护性 - 规则是否易于维护
    Maintainability,
    /// 文档质量 - 文档是否完善
    Documentation,
    /// 测试覆盖 - 测试是否充分
    TestCoverage,
    /// 性能 - 规则执行效率
    Performance,
    /// 安全性 - 规则是否安全
    Security,
    /// 兼容性 - 规则兼容性
    Compatibility,
}

impl std::fmt::Display for QualityDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Correctness => write!(f, "正确性"),
            Self::Usability => write!(f, "可用性"),
            Self::Maintainability => write!(f, "可维护性"),
            Self::Documentation => write!(f, "文档质量"),
            Self::TestCoverage => write!(f, "测试覆盖"),
            Self::Performance => write!(f, "性能"),
            Self::Security => write!(f, "安全性"),
            Self::Compatibility => write!(f, "兼容性"),
        }
    }
}

impl QualityDimension {
    /// 获取默认权重
    pub fn default_weight(&self) -> f64 {
        match self {
            Self::Correctness => 2.0,
            Self::Usability => 1.5,
            Self::Maintainability => 1.0,
            Self::Documentation => 1.0,
            Self::TestCoverage => 1.0,
            Self::Performance => 0.8,
            Self::Security => 1.2,
            Self::Compatibility => 0.8,
        }
    }
}

/// 质量评级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum QualityRating {
    /// F级 - 不合格
    #[default]
    F,
    /// D级 - 基本合格
    D,
    /// C级 - 中等
    C,
    /// B级 - 良好
    B,
    /// A级 - 优秀
    A,
    /// S级 - 卓越
    S,
}

impl std::fmt::Display for QualityRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F => write!(f, "F"),
            Self::D => write!(f, "D"),
            Self::C => write!(f, "C"),
            Self::B => write!(f, "B"),
            Self::A => write!(f, "A"),
            Self::S => write!(f, "S"),
        }
    }
}

impl QualityRating {
    /// 从分数获取评级
    pub fn from_score(score: f64) -> Self {
        if score >= 95.0 {
            Self::S
        } else if score >= 85.0 {
            Self::A
        } else if score >= 70.0 {
            Self::B
        } else if score >= 55.0 {
            Self::C
        } else if score >= 40.0 {
            Self::D
        } else {
            Self::F
        }
    }

    /// 获取评级的中文描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::S => "卓越",
            Self::A => "优秀",
            Self::B => "良好",
            Self::C => "中等",
            Self::D => "基本合格",
            Self::F => "不合格",
        }
    }
}

/// 质量维度评分
#[derive(Debug, Clone)]
pub struct DimensionScore {
    /// 维度
    pub dimension: QualityDimension,
    /// 得分 (0-100)
    pub score: f64,
    /// 权重
    pub weight: f64,
    /// 分析详情
    pub details: String,
    /// 改进建议
    pub suggestions: Vec<String>,
}

impl DimensionScore {
    /// 创建新的维度评分
    pub fn new(dimension: QualityDimension, score: f64, details: &str) -> Self {
        Self {
            dimension,
            score: score.clamp(0.0, 100.0),
            weight: dimension.default_weight(),
            details: details.to_string(),
            suggestions: Vec::new(),
        }
    }

    /// 添加改进建议
    pub fn add_suggestion(&mut self, suggestion: &str) {
        self.suggestions.push(suggestion.to_string());
    }

    /// 获取加权得分
    pub fn weighted_score(&self) -> f64 {
        self.score * self.weight
    }
}

/// 质量报告
#[derive(Debug, Clone)]
pub struct QualityReport {
    /// 规则名称
    pub rule_name: String,
    /// 整体评级
    pub rating: QualityRating,
    /// 整体得分 (0-100)
    pub overall_score: f64,
    /// 各维度评分
    pub dimensions: Vec<DimensionScore>,
    /// 主要优点
    pub strengths: Vec<String>,
    /// 主要缺点
    pub weaknesses: Vec<String>,
    /// 改进建议
    pub improvement_plan: Vec<String>,
}

impl QualityReport {
    /// 创建新的质量报告
    pub fn new(rule_name: String) -> Self {
        Self {
            rule_name,
            rating: QualityRating::F,
            overall_score: 0.0,
            dimensions: Vec::new(),
            strengths: Vec::new(),
            weaknesses: Vec::new(),
            improvement_plan: Vec::new(),
        }
    }

    /// 计算整体得分
    pub fn calculate_overall_score(&mut self) {
        if self.dimensions.is_empty() {
            self.overall_score = 0.0;
            self.rating = QualityRating::F;
            return;
        }

        let total_weight: f64 = self.dimensions.iter().map(|d| d.weight).sum();
        let weighted_sum: f64 = self.dimensions.iter().map(|d| d.weighted_score()).sum();

        self.overall_score = weighted_sum / total_weight;
        self.rating = QualityRating::from_score(self.overall_score);
    }

    /// 获取特定维度的评分
    pub fn get_dimension_score(&self, dimension: QualityDimension) -> Option<f64> {
        self.dimensions
            .iter()
            .find(|d| d.dimension == dimension)
            .map(|d| d.score)
    }

    /// 获取最低分的维度
    pub fn get_lowest_dimension(&self) -> Option<&DimensionScore> {
        self.dimensions
            .iter()
            .min_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
    }

    /// 获取最高分的维度
    pub fn get_highest_dimension(&self) -> Option<&DimensionScore> {
        self.dimensions
            .iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
    }
}

impl std::fmt::Display for QualityReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} 质量得分: {:.1}% ({})",
            self.rating,
            self.rule_name,
            self.overall_score,
            self.rating.description()
        )
    }
}

/// 质量评分器
///
/// 用于综合评估规则质量。
///
/// # 示例
///
/// ```rust
/// use world_rules::rules::analysis::quality::{QualityScorer, QualityDimension};
///
/// let scorer = QualityScorer::new();
///
/// // 评分空规则集
/// let reports = scorer.score_all(&[]);
/// assert!(reports.is_empty());
/// ```
#[derive(Debug, Default)]
pub struct QualityScorer {
    /// 是否启用性能检查
    performance_check: bool,
    /// 是否启用安全检查
    security_check: bool,
    /// 是否启用兼容性检查
    compatibility_check: bool,
}

impl QualityScorer {
    /// 创建新的质量评分器
    pub fn new() -> Self {
        Self {
            performance_check: true,
            security_check: true,
            compatibility_check: true,
        }
    }

    /// 设置是否启用性能检查
    pub fn with_performance_check(mut self, enabled: bool) -> Self {
        self.performance_check = enabled;
        self
    }

    /// 设置是否启用安全检查
    pub fn with_security_check(mut self, enabled: bool) -> Self {
        self.security_check = enabled;
        self
    }

    /// 设置是否启用兼容性检查
    pub fn with_compatibility_check(mut self, enabled: bool) -> Self {
        self.compatibility_check = enabled;
        self
    }

    /// 对单个规则进行质量评分
    pub fn score(&self, rule: &dyn Rule) -> QualityReport {
        let mut report = QualityReport::new(rule.metadata().name.clone());

        // 1. 正确性评分
        report.dimensions.push(self.score_correctness(rule));

        // 2. 可用性评分
        report.dimensions.push(self.score_usability(rule));

        // 3. 可维护性评分
        report.dimensions.push(self.score_maintainability(rule));

        // 4. 文档质量评分
        report.dimensions.push(self.score_documentation(rule));

        // 5. 测试覆盖评分 (简化版本)
        report.dimensions.push(self.score_test_coverage(rule));

        // 6. 性能评分
        if self.performance_check {
            report.dimensions.push(self.score_performance(rule));
        }

        // 7. 安全性评分
        if self.security_check {
            report.dimensions.push(self.score_security(rule));
        }

        // 8. 兼容性评分
        if self.compatibility_check {
            report.dimensions.push(self.score_compatibility(rule));
        }

        // 计算整体得分
        report.calculate_overall_score();

        // 生成优缺点和建议
        self.analyze_strengths_and_weaknesses(&mut report);
        self.generate_improvement_plan(&mut report);

        report
    }

    /// 正确性评分
    fn score_correctness(&self, rule: &dyn Rule) -> DimensionScore {
        let meta = rule.metadata();
        let mut score = 100.0;
        let mut details = String::new();

        // 检查基本字段
        if meta.name.is_empty() {
            score -= 30.0;
            details.push_str("规则名称为空; ");
        }
        if meta.description.is_empty() {
            score -= 25.0;
            details.push_str("规则描述为空; ");
        }
        if meta.version.is_empty() {
            score -= 10.0;
            details.push_str("版本号为空; ");
        }

        // 检查分类
        let category = rule.category();
        details.push_str(&format!("分类: {}", category));

        let mut dim = DimensionScore::new(QualityDimension::Correctness, score, &details);

        if score < 80.0 {
            dim.add_suggestion("补充缺失的必要字段");
        }

        dim
    }

    /// 可用性评分
    fn score_usability(&self, rule: &dyn Rule) -> DimensionScore {
        let meta = rule.metadata();
        let mut score = 70.0; // 基础分
        let mut details = String::new();

        // 有标签加分
        if !meta.tags.is_empty() {
            score += 10.0;
            details.push_str(&format!("有{}个标签; ", meta.tags.len()));
        }

        // 有来源加分
        if meta.origin.is_some() {
            score += 10.0;
            details.push_str("有来源信息; ");
        }

        // 有详细说明加分
        let explanation = rule.explain();
        if explanation.len() > 50 {
            score += 10.0;
            details.push_str("有详细说明; ");
        }

        let mut dim = DimensionScore::new(QualityDimension::Usability, score, &details);

        if meta.tags.is_empty() {
            dim.add_suggestion("添加标签以便搜索和分类");
        }
        if meta.origin.is_none() {
            dim.add_suggestion("添加规则来源或地区信息");
        }

        dim
    }

    /// 可维护性评分
    fn score_maintainability(&self, rule: &dyn Rule) -> DimensionScore {
        let meta = rule.metadata();
        let mut score = 60.0;
        let mut details = String::new();

        // 版本管理
        if !meta.version.is_empty() && meta.version != "1.0.0" {
            score += 15.0;
            details.push_str(&format!("版本: {}; ", meta.version));
        }

        // 标签便于分类
        if meta.tags.len() >= 2 {
            score += 10.0;
        }

        // 描述质量
        if meta.description.len() > 20 {
            score += 15.0;
            details.push_str("有详细描述; ");
        }

        details.push_str(&format!("标签数: {}", meta.tags.len()));

        DimensionScore::new(QualityDimension::Maintainability, score, &details)
    }

    /// 文档质量评分
    fn score_documentation(&self, rule: &dyn Rule) -> DimensionScore {
        let meta = rule.metadata();
        let mut score = 50.0;
        let mut details = String::new();

        // 描述长度
        if meta.description.len() > 10 {
            score += 15.0;
        }
        if meta.description.len() > 50 {
            score += 10.0;
        }

        // 说明文本
        let explanation = rule.explain();
        if !explanation.is_empty() {
            score += 15.0;
            details.push_str(&format!("说明长度: {}字符; ", explanation.len()));
        }

        // 版本和来源
        if meta.origin.is_some() {
            score += 5.0;
            details.push_str("有来源; ");
        }

        details.push_str(&format!("描述长度: {}字符", meta.description.len()));

        let mut dim = DimensionScore::new(QualityDimension::Documentation, score, &details);

        if explanation.is_empty() {
            dim.add_suggestion("添加详细的规则说明文档");
        }

        dim
    }

    /// 测试覆盖评分
    fn score_test_coverage(&self, rule: &dyn Rule) -> DimensionScore {
        // 简化版本，假设有基本测试
        let meta = rule.metadata();
        let score = if meta.tags.is_empty() {
            60.0
        } else if meta.tags.len() <= 3 {
            75.0
        } else {
            85.0
        };

        DimensionScore::new(
            QualityDimension::TestCoverage,
            score,
            &format!("基于标签数的推断测试覆盖: {}个标签", meta.tags.len()),
        )
    }

    /// 性能评分
    fn score_performance(&self, rule: &dyn Rule) -> DimensionScore {
        // 基于描述复杂度的简化性能评分
        let meta = rule.metadata();
        let score = if meta.description.len() > 500 {
            70.0 // 可能复杂
        } else if meta.description.len() > 200 {
            85.0 // 中等
        } else {
            95.0 // 简单
        };

        DimensionScore::new(
            QualityDimension::Performance,
            score,
            &format!("基于描述长度的推断性能: {}字符", meta.description.len()),
        )
    }

    /// 安全性评分
    fn score_security(&self, rule: &dyn Rule) -> DimensionScore {
        // 简化的安全性评分
        let meta = rule.metadata();
        let mut score = 80.0;

        // 检查是否有潜在不安全关键词
        let unsafe_keywords = ["unsafe", "panic", "unwrap"];
        let desc_lower = meta.description.to_lowercase();
        for keyword in unsafe_keywords {
            if desc_lower.contains(keyword) {
                score -= 10.0;
            }
        }

        DimensionScore::new(QualityDimension::Security, score, "基本安全检查通过")
    }

    /// 兼容性评分
    fn score_compatibility(&self, rule: &dyn Rule) -> DimensionScore {
        let meta = rule.metadata();
        let mut score = 75.0;

        // 有版本号表示关注兼容性
        if !meta.version.is_empty() && meta.version != "1.0.0" {
            score += 15.0;
        }

        DimensionScore::new(
            QualityDimension::Compatibility,
            score,
            &format!("版本: {}", meta.version),
        )
    }

    /// 分析优缺点
    fn analyze_strengths_and_weaknesses(&self, report: &mut QualityReport) {
        for dim in &report.dimensions {
            if dim.score >= 80.0 {
                report
                    .strengths
                    .push(format!("{}得分较高 ({:.0}%)", dim.dimension, dim.score));
            } else if dim.score < 60.0 {
                report
                    .weaknesses
                    .push(format!("{}得分较低 ({:.0}%)", dim.dimension, dim.score));
            }
        }

        if report.strengths.is_empty() {
            report.strengths.push("规则基本可用".to_string());
        }
        if report.weaknesses.is_empty() {
            report.weaknesses.push("暂无明显弱点".to_string());
        }
    }

    /// 生成改进计划
    fn generate_improvement_plan(&self, report: &mut QualityReport) {
        // 获取最低分的维度
        if let Some(lowest) = report.get_lowest_dimension() {
            if lowest.score < 70.0 {
                report.improvement_plan.push(format!(
                    "优先提升{}评分 (当前 {:.0}%)",
                    lowest.dimension, lowest.score
                ));
            }
        }

        // 收集所有建议
        for dim in &report.dimensions {
            for suggestion in &dim.suggestions {
                report.improvement_plan.push(suggestion.clone());
            }
        }

        // 确保至少有一条建议
        if report.improvement_plan.is_empty() {
            report
                .improvement_plan
                .push("继续保持当前质量水平".to_string());
        }
    }

    /// 对所有规则进行评分
    pub fn score_all(&self, rules: &[Box<dyn Rule>]) -> Vec<QualityReport> {
        rules.iter().map(|r| self.score(r.as_ref())).collect()
    }

    /// 获取质量统计
    pub fn get_statistics(&self, reports: &[QualityReport]) -> QualityStatistics {
        let mut stats = QualityStatistics::default();

        for report in reports {
            match report.rating {
                QualityRating::S => stats.s_count += 1,
                QualityRating::A => stats.a_count += 1,
                QualityRating::B => stats.b_count += 1,
                QualityRating::C => stats.c_count += 1,
                QualityRating::D => stats.d_count += 1,
                QualityRating::F => stats.f_count += 1,
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

/// 质量统计信息
#[derive(Debug, Clone, Default)]
pub struct QualityStatistics {
    /// 总规则数
    pub total_count: usize,
    /// S级数量
    pub s_count: usize,
    /// A级数量
    pub a_count: usize,
    /// B级数量
    pub b_count: usize,
    /// C级数量
    pub c_count: usize,
    /// D级数量
    pub d_count: usize,
    /// F级数量
    pub f_count: usize,
    /// 总得分
    pub total_score: f64,
    /// 平均得分
    pub average_score: f64,
}

impl std::fmt::Display for QualityStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "质量统计: 总计 {} (S: {}, A: {}, B: {}, C: {}, D: {}, F: {}) 平均: {:.1}%",
            self.total_count,
            self.s_count,
            self.a_count,
            self.b_count,
            self.c_count,
            self.d_count,
            self.f_count,
            self.average_score
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_rating_ordering() {
        assert!(QualityRating::F < QualityRating::D);
        assert!(QualityRating::D < QualityRating::C);
        assert!(QualityRating::C < QualityRating::B);
        assert!(QualityRating::B < QualityRating::A);
        assert!(QualityRating::A < QualityRating::S);
    }

    #[test]
    fn test_quality_rating_from_score() {
        assert_eq!(QualityRating::from_score(10.0), QualityRating::F);
        assert_eq!(QualityRating::from_score(50.0), QualityRating::D);
        assert_eq!(QualityRating::from_score(60.0), QualityRating::C);
        assert_eq!(QualityRating::from_score(75.0), QualityRating::B);
        assert_eq!(QualityRating::from_score(90.0), QualityRating::A);
        assert_eq!(QualityRating::from_score(98.0), QualityRating::S);
    }

    #[test]
    fn test_quality_scorer_empty() {
        let scorer = QualityScorer::new();
        let reports = scorer.score_all(&[]);
        assert!(reports.is_empty());
    }

    #[test]
    fn test_dimension_score() {
        let dim = DimensionScore::new(QualityDimension::Correctness, 80.0, "test");
        assert_eq!(
            dim.weighted_score(),
            80.0 * QualityDimension::Correctness.default_weight()
        );
    }

    #[test]
    fn test_quality_report() {
        let mut report = QualityReport::new("test".to_string());
        report
            .dimensions
            .push(DimensionScore::new(QualityDimension::Correctness, 80.0, ""));
        report
            .dimensions
            .push(DimensionScore::new(QualityDimension::Usability, 60.0, ""));
        report.calculate_overall_score();

        assert!(report.overall_score > 0.0);
    }

    #[test]
    fn test_quality_statistics() {
        let scorer = QualityScorer::new();
        let stats = scorer.get_statistics(&[]);

        assert_eq!(stats.total_count, 0);
        assert_eq!(stats.average_score, 0.0);
    }

    #[test]
    fn test_quality_dimension_default_weight() {
        assert!(
            QualityDimension::Correctness.default_weight()
                > QualityDimension::Performance.default_weight()
        );
    }
}
