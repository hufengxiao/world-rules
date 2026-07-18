//! 规则复杂度评估
//!
//! 提供游戏规则复杂度分析工具，帮助评估规则的学习难度和理解难度。
//!
//! # 示例
//!
//! ```rust
//! use world_rules::rules::game_design_tools::complexity::*;
//!
//! let analyzer = ComplexityAnalyzer::new();
//! let report = analyzer.analyze("围棋", vec!["黑先白后", "落子无悔", "提子规则"]);
//! assert!(report.score >= 0.0);
//! ```

use crate::rules::core::Difficulty;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 复杂度分析器
///
/// 分析游戏规则的复杂度，包括规则数量、概念深度、执行步骤等多个维度。
///
/// # 示例
///
/// ```rust
/// use world_rules::rules::game_design_tools::complexity::ComplexityAnalyzer;
///
/// let analyzer = ComplexityAnalyzer::new();
/// assert!(analyzer.max_score > 0.0);
/// ```
#[derive(Debug, Clone)]
pub struct ComplexityAnalyzer {
    /// 最大复杂度分数
    pub max_score: f64,
    /// 维度权重
    pub weights: ComplexityWeights,
}

/// 复杂度权重配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityWeights {
    /// 规则数量权重
    pub rule_count: f64,
    /// 概念深度权重
    pub concept_depth: f64,
    /// 执行步骤权重
    pub execution_steps: f64,
    /// 特殊情况权重
    pub special_cases: f64,
    /// 交互复杂度权重
    pub interaction: f64,
}

impl Default for ComplexityWeights {
    fn default() -> Self {
        Self {
            rule_count: 0.25,
            concept_depth: 0.25,
            execution_steps: 0.20,
            special_cases: 0.15,
            interaction: 0.15,
        }
    }
}

/// 复杂度分析报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityReport {
    /// 游戏名称
    pub game_name: String,
    /// 总体复杂度分数（0.0-1.0）
    pub score: f64,
    /// 复杂度等级
    pub level: ComplexityLevel,
    /// 各维度分数
    pub dimension_scores: HashMap<String, f64>,
    /// 复杂度详情
    pub details: ComplexityDetails,
    /// 简化建议
    pub simplification_tips: Vec<String>,
}

/// 复杂度等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplexityLevel {
    /// 简单（0.0-0.2）
    Simple,
    /// 中等（0.2-0.4）
    Moderate,
    /// 复杂（0.4-0.6）
    Complex,
    /// 非常复杂（0.6-0.8）
    VeryComplex,
    /// 极度复杂（0.8-1.0）
    ExtremelyComplex,
}

/// 复杂度详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityDetails {
    /// 规则总数
    pub total_rules: usize,
    /// 核心规则数
    pub core_rules: usize,
    /// 特殊规则数
    pub special_rules: usize,
    /// 概念数量
    pub concepts: usize,
    /// 平均规则长度
    pub avg_rule_length: f64,
    /// 规则依赖数
    pub dependencies: usize,
}

impl Default for ComplexityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplexityAnalyzer {
    /// 创建新的复杂度分析器
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::complexity::ComplexityAnalyzer;
    ///
    /// let analyzer = ComplexityAnalyzer::new();
    /// assert_eq!(analyzer.max_score, 100.0);
    /// ```
    pub fn new() -> Self {
        Self {
            max_score: 100.0,
            weights: ComplexityWeights::default(),
        }
    }

    /// 设置最大分数
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::complexity::ComplexityAnalyzer;
    ///
    /// let analyzer = ComplexityAnalyzer::new().with_max_score(50.0);
    /// assert_eq!(analyzer.max_score, 50.0);
    /// ```
    pub fn with_max_score(mut self, max_score: f64) -> Self {
        self.max_score = max_score;
        self
    }

    /// 设置权重配置
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::complexity::{ComplexityAnalyzer, ComplexityWeights};
    ///
    /// let weights = ComplexityWeights::default();
    /// let analyzer = ComplexityAnalyzer::new().with_weights(weights);
    /// assert_eq!(analyzer.weights.rule_count, 0.25);
    /// ```
    pub fn with_weights(mut self, weights: ComplexityWeights) -> Self {
        self.weights = weights;
        self
    }

    /// 分析规则复杂度
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::complexity::ComplexityAnalyzer;
    ///
    /// let analyzer = ComplexityAnalyzer::new();
    /// let report = analyzer.analyze("围棋", vec!["规则1", "规则2"]);
    /// assert!(report.score >= 0.0);
    /// ```
    pub fn analyze(&self, game_name: &str, rules: Vec<&str>) -> ComplexityReport {
        let mut dimension_scores = HashMap::new();
        let mut simplification_tips = Vec::new();

        let total_rules = rules.len();
        let avg_length = if rules.is_empty() {
            0.0
        } else {
            rules.iter().map(|r| r.len() as f64).sum::<f64>() / rules.len() as f64
        };

        // 1. 规则数量复杂度
        let rule_count_score = self.calculate_rule_count_complexity(total_rules);
        dimension_scores.insert("规则数量".to_string(), rule_count_score);

        // 2. 概念深度复杂度
        let concepts = self.count_concepts(&rules);
        let concept_score = self.calculate_concept_complexity(concepts);
        dimension_scores.insert("概念深度".to_string(), concept_score);

        // 3. 执行步骤复杂度
        let execution_score = self.calculate_execution_complexity(&rules);
        dimension_scores.insert("执行步骤".to_string(), execution_score);

        // 4. 特殊情况复杂度
        let special_cases = self.count_special_cases(&rules);
        let special_score = self.calculate_special_complexity(special_cases);
        dimension_scores.insert("特殊情况".to_string(), special_score);

        // 5. 交互复杂度
        let interaction_score = self.calculate_interaction_complexity(total_rules);
        dimension_scores.insert("交互复杂度".to_string(), interaction_score);

        // 计算总分
        let total_score = rule_count_score * self.weights.rule_count
            + concept_score * self.weights.concept_depth
            + execution_score * self.weights.execution_steps
            + special_score * self.weights.special_cases
            + interaction_score * self.weights.interaction;

        // 生成简化建议
        if rule_count_score > 0.7 {
            simplification_tips.push("考虑合并相似规则，减少规则总数".to_string());
        }
        if concept_score > 0.7 {
            simplification_tips.push("添加术语解释和示例，帮助理解复杂概念".to_string());
        }
        if execution_score > 0.7 {
            simplification_tips.push("简化执行流程，减少必需步骤".to_string());
        }
        if special_score > 0.7 {
            simplification_tips.push("减少特殊规则，或将其移至进阶规则".to_string());
        }

        let details = ComplexityDetails {
            total_rules,
            core_rules: total_rules.saturating_sub(special_cases),
            special_rules: special_cases,
            concepts,
            avg_rule_length: avg_length,
            dependencies: self.count_dependencies(&rules),
        };

        ComplexityReport {
            game_name: game_name.to_string(),
            score: total_score,
            level: self.calculate_level(total_score),
            dimension_scores,
            details,
            simplification_tips,
        }
    }

    /// 分析带内容的规则
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::complexity::ComplexityAnalyzer;
    ///
    /// let analyzer = ComplexityAnalyzer::new();
    /// let report = analyzer.analyze_with_content("围棋", vec![
    ///     ("规则1", "这是规则1的内容"),
    ///     ("规则2", "这是规则2的内容"),
    /// ]);
    /// assert!(report.score >= 0.0);
    /// ```
    pub fn analyze_with_content(
        &self,
        game_name: &str,
        rules: Vec<(&str, &str)>,
    ) -> ComplexityReport {
        let rule_names: Vec<&str> = rules.iter().map(|(name, _)| *name).collect();
        let mut report = self.analyze(game_name, rule_names);

        // 额外分析规则内容
        let total_length: usize = rules.iter().map(|(_, content)| content.len()).sum();
        report.details.avg_rule_length = total_length as f64 / rules.len().max(1) as f64;

        report
    }

    /// 估算学习难度
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::complexity::ComplexityAnalyzer;
    ///
    /// let analyzer = ComplexityAnalyzer::new();
    /// let difficulty = analyzer.estimate_difficulty(0.8);
    /// assert!(difficulty >= world_rules::rules::core::Difficulty::Normal);
    /// ```
    pub fn estimate_difficulty(&self, complexity_score: f64) -> Difficulty {
        if complexity_score < 0.2 {
            Difficulty::Beginner
        } else if complexity_score < 0.4 {
            Difficulty::Easy
        } else if complexity_score < 0.6 {
            Difficulty::Normal
        } else if complexity_score < 0.8 {
            Difficulty::Hard
        } else if complexity_score < 0.9 {
            Difficulty::Expert
        } else {
            Difficulty::Master
        }
    }

    /// 计算规则数量复杂度
    fn calculate_rule_count_complexity(&self, count: usize) -> f64 {
        if count == 0 {
            0.0
        } else if count <= 5 {
            0.1
        } else if count <= 10 {
            0.3
        } else if count <= 20 {
            0.5
        } else if count <= 50 {
            0.7
        } else {
            0.9
        }
    }

    /// 计算概念复杂度
    fn calculate_concept_complexity(&self, concepts: usize) -> f64 {
        if concepts == 0 {
            0.0
        } else if concepts <= 3 {
            0.2
        } else if concepts <= 6 {
            0.4
        } else if concepts <= 10 {
            0.6
        } else {
            0.8
        }
    }

    /// 计算执行复杂度
    fn calculate_execution_complexity(&self, rules: &[&str]) -> f64 {
        // 空规则没有执行复杂度
        if rules.is_empty() {
            return 0.0;
        }

        let step_keywords = ["步骤", "阶段", "流程", "步骤", "step", "phase"];
        let mut step_count = 0;

        for rule in rules {
            for keyword in &step_keywords {
                if rule.contains(keyword) {
                    step_count += 1;
                    break;
                }
            }
        }

        if step_count == 0 {
            0.2
        } else if step_count <= 2 {
            0.4
        } else if step_count <= 4 {
            0.6
        } else {
            0.8
        }
    }

    /// 计算特殊规则复杂度
    fn calculate_special_complexity(&self, special_count: usize) -> f64 {
        if special_count == 0 {
            0.0
        } else if special_count <= 2 {
            0.2
        } else if special_count <= 5 {
            0.4
        } else if special_count <= 10 {
            0.6
        } else {
            0.8
        }
    }

    /// 计算交互复杂度
    fn calculate_interaction_complexity(&self, rule_count: usize) -> f64 {
        // 空规则没有交互复杂度
        if rule_count == 0 {
            return 0.0;
        }

        // 规则越多，交互复杂度越高（组合爆炸）
        if rule_count <= 5 {
            0.1
        } else if rule_count <= 10 {
            0.3
        } else if rule_count <= 20 {
            0.5
        } else {
            0.7 + (rule_count - 20) as f64 * 0.01
        }
    }

    /// 计算概念数量
    fn count_concepts(&self, rules: &[&str]) -> usize {
        let concept_keywords = ["概念", "定义", "术语", "名词", "concept", "definition"];

        let mut count = 0;
        for rule in rules {
            for keyword in &concept_keywords {
                if rule.contains(keyword) {
                    count += 1;
                    break;
                }
            }
        }

        // 加上规则本身的数量作为基础概念
        count + rules.len().min(5)
    }

    /// 计算特殊规则数量
    fn count_special_cases(&self, rules: &[&str]) -> usize {
        let special_keywords = [
            "特殊",
            "例外",
            "仅当",
            "除非",
            "但是",
            "如果",
            "special",
            "exception",
            "unless",
            "except",
        ];

        let mut count = 0;
        for rule in rules {
            for keyword in &special_keywords {
                if rule.contains(keyword) {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    /// 计算规则依赖数
    fn count_dependencies(&self, rules: &[&str]) -> usize {
        let dep_keywords = ["引用", "参见", "参见规则", "见规则", "参考"];

        let mut count = 0;
        for rule in rules {
            for keyword in &dep_keywords {
                if rule.contains(keyword) {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    /// 计算复杂度等级
    fn calculate_level(&self, score: f64) -> ComplexityLevel {
        if score < 0.2 {
            ComplexityLevel::Simple
        } else if score < 0.4 {
            ComplexityLevel::Moderate
        } else if score < 0.6 {
            ComplexityLevel::Complex
        } else if score < 0.8 {
            ComplexityLevel::VeryComplex
        } else {
            ComplexityLevel::ExtremelyComplex
        }
    }
}

impl ComplexityReport {
    /// 获取等级名称
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::complexity::{ComplexityAnalyzer, ComplexityLevel};
    ///
    /// let analyzer = ComplexityAnalyzer::new();
    /// let report = analyzer.analyze("围棋", vec!["规则"]);
    /// let level_name = report.level_name();
    /// assert!(!level_name.is_empty());
    /// ```
    pub fn level_name(&self) -> &'static str {
        match self.level {
            ComplexityLevel::Simple => "简单",
            ComplexityLevel::Moderate => "中等",
            ComplexityLevel::Complex => "复杂",
            ComplexityLevel::VeryComplex => "非常复杂",
            ComplexityLevel::ExtremelyComplex => "极度复杂",
        }
    }

    /// 是否过于复杂
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::complexity::ComplexityAnalyzer;
    ///
    /// let analyzer = ComplexityAnalyzer::new();
    /// let report = analyzer.analyze("围棋", vec!["规则"]);
    /// // report.is_too_complex() 返回布尔值
    /// ```
    pub fn is_too_complex(&self) -> bool {
        self.score > 0.7
    }

    /// 生成报告（Markdown格式）
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::complexity::ComplexityAnalyzer;
    ///
    /// let analyzer = ComplexityAnalyzer::new();
    /// let report = analyzer.analyze("围棋", vec!["规则"]);
    /// let md = report.generate_report();
    /// assert!(md.contains("围棋"));
    /// ```
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!("# {} 复杂度分析报告\n\n", self.game_name));
        report.push_str(&format!(
            "**复杂度分数**: {:.2}/1.0 ({})\n\n",
            self.score,
            self.level_name()
        ));

        report.push_str("## 各维度评分\n\n");
        for (dimension, score) in &self.dimension_scores {
            let bar = self.score_to_bar(*score);
            report.push_str(&format!("- **{}**: {:.2} {}\n", dimension, score, bar));
        }
        report.push('\n');

        report.push_str("## 详细信息\n\n");
        report.push_str(&format!("- **规则总数**: {}\n", self.details.total_rules));
        report.push_str(&format!("- **核心规则**: {}\n", self.details.core_rules));
        report.push_str(&format!("- **特殊规则**: {}\n", self.details.special_rules));
        report.push_str(&format!("- **概念数量**: {}\n", self.details.concepts));
        report.push_str(&format!(
            "- **平均规则长度**: {:.1} 字符\n",
            self.details.avg_rule_length
        ));
        report.push_str(&format!("- **规则依赖**: {}\n", self.details.dependencies));
        report.push('\n');

        if !self.simplification_tips.is_empty() {
            report.push_str("## 简化建议\n\n");
            for tip in &self.simplification_tips {
                report.push_str(&format!("- {}\n", tip));
            }
            report.push('\n');
        }

        report
    }

    /// 将分数转换为进度条
    fn score_to_bar(&self, score: f64) -> String {
        let filled = (score * 10.0) as usize;
        let empty = 10 - filled;
        format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = ComplexityAnalyzer::new();
        assert_eq!(analyzer.max_score, 100.0);
    }

    #[test]
    fn test_basic_analysis() {
        let analyzer = ComplexityAnalyzer::new();
        let report = analyzer.analyze("围棋", vec!["规则1", "规则2"]);

        assert!(report.score >= 0.0);
        assert_eq!(report.game_name, "围棋");
    }

    #[test]
    fn test_empty_analysis() {
        let analyzer = ComplexityAnalyzer::new();
        let report = analyzer.analyze("游戏", vec![]);

        // 空规则有基础复杂度（概念数量等）
        assert!(report.score >= 0.0);
        assert!(report.score < 0.2); // 应该是简单级别
        assert_eq!(report.details.total_rules, 0);
    }

    #[test]
    fn test_level_calculation() {
        let analyzer = ComplexityAnalyzer::new();

        assert_eq!(analyzer.calculate_level(0.1), ComplexityLevel::Simple);
        assert_eq!(analyzer.calculate_level(0.3), ComplexityLevel::Moderate);
        assert_eq!(analyzer.calculate_level(0.5), ComplexityLevel::Complex);
        assert_eq!(analyzer.calculate_level(0.7), ComplexityLevel::VeryComplex);
        assert_eq!(
            analyzer.calculate_level(0.9),
            ComplexityLevel::ExtremelyComplex
        );
    }

    #[test]
    fn test_difficulty_estimation() {
        let analyzer = ComplexityAnalyzer::new();

        assert_eq!(analyzer.estimate_difficulty(0.1), Difficulty::Beginner);
        assert_eq!(analyzer.estimate_difficulty(0.3), Difficulty::Easy);
        assert_eq!(analyzer.estimate_difficulty(0.5), Difficulty::Normal);
        assert_eq!(analyzer.estimate_difficulty(0.7), Difficulty::Hard);
        assert_eq!(analyzer.estimate_difficulty(0.85), Difficulty::Expert);
        assert_eq!(analyzer.estimate_difficulty(0.95), Difficulty::Master);
    }

    #[test]
    fn test_concept_counting() {
        let analyzer = ComplexityAnalyzer::new();

        let concepts = analyzer.count_concepts(&["概念定义", "规则"]);
        assert!(concepts > 0);
    }

    #[test]
    fn test_special_case_counting() {
        let analyzer = ComplexityAnalyzer::new();

        let special = analyzer.count_special_cases(&["特殊情况", "普通规则"]);
        assert!(special > 0);
    }

    #[test]
    fn test_report_generation() {
        let analyzer = ComplexityAnalyzer::new();
        let report = analyzer.analyze("围棋", vec!["规则"]);
        let md = report.generate_report();

        assert!(md.contains("围棋"));
        assert!(md.contains("复杂度分析报告"));
    }

    #[test]
    fn test_analyze_with_content() {
        let analyzer = ComplexityAnalyzer::new();
        let report = analyzer.analyze_with_content(
            "围棋",
            vec![
                ("规则1", "这是规则1的详细内容"),
                ("规则2", "这是规则2的详细内容"),
            ],
        );

        assert!(report.score >= 0.0);
        assert!(report.details.avg_rule_length > 0.0);
    }

    #[test]
    fn test_is_too_complex() {
        let analyzer = ComplexityAnalyzer::new();
        let report = analyzer.analyze("游戏", vec![]);

        // 空规则不复杂
        assert!(!report.is_too_complex());
    }
}
