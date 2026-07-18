//! 规则平衡性分析
//!
//! 提供游戏规则平衡性分析工具，帮助识别规则中的不平衡因素。
//!
//! # 示例
//!
//! ```rust
//! use world_rules::rules::game_design_tools::balance::*;
//!
//! let analyzer = BalanceAnalyzer::new();
//! let report = analyzer.analyze_simple("围棋", vec!["先手优势", "资源平衡"]);
//! assert!(report.score >= 0.0);
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 平衡性分析器
///
/// 分析游戏规则的平衡性，包括策略多样性、资源分配、角色平衡等方面。
///
/// # 示例
///
/// ```rust
/// use world_rules::rules::game_design_tools::balance::BalanceAnalyzer;
///
/// let analyzer = BalanceAnalyzer::new();
/// assert!(analyzer.threshold >= 0.0);
/// ```
#[derive(Debug, Clone)]
pub struct BalanceAnalyzer {
    /// 平衡阈值（0.0-1.0）
    pub threshold: f64,
    /// 权重配置
    pub weights: BalanceWeights,
}

/// 平衡性权重配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceWeights {
    /// 策略多样性权重
    pub strategy_diversity: f64,
    /// 资源平衡权重
    pub resource_balance: f64,
    /// 角色平衡权重
    pub character_balance: f64,
    /// 时间平衡权重
    pub timing_balance: f64,
    /// 信息平衡权重
    pub information_balance: f64,
}

impl Default for BalanceWeights {
    fn default() -> Self {
        Self {
            strategy_diversity: 0.25,
            resource_balance: 0.25,
            character_balance: 0.20,
            timing_balance: 0.15,
            information_balance: 0.15,
        }
    }
}

/// 平衡性分析报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceReport {
    /// 游戏名称
    pub game_name: String,
    /// 总体平衡分数（0.0-1.0）
    pub score: f64,
    /// 平衡性等级
    pub grade: BalanceGrade,
    /// 各维度分数
    pub dimension_scores: HashMap<String, f64>,
    /// 发现的问题
    pub issues: Vec<BalanceIssue>,
    /// 改进建议
    pub recommendations: Vec<String>,
}

/// 平衡性等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceGrade {
    /// 优秀（0.9-1.0）
    Excellent,
    /// 良好（0.8-0.89）
    Good,
    /// 一般（0.7-0.79）
    Average,
    /// 需改进（0.6-0.69）
    NeedsImprovement,
    /// 不平衡（<0.6）
    Unbalanced,
}

/// 平衡性问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceIssue {
    /// 问题类型
    pub issue_type: IssueType,
    /// 问题描述
    pub description: String,
    /// 严重程度（0.0-1.0）
    pub severity: f64,
    /// 影响范围
    pub impact: String,
    /// 建议解决方案
    pub solution: Option<String>,
}

/// 问题类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueType {
    /// 策略单一
    StrategyMonotony,
    /// 资源不平衡
    ResourceImbalance,
    /// 角色不平衡
    CharacterImbalance,
    /// 时间不平衡
    TimingImbalance,
    /// 信息不对称
    InformationAsymmetry,
    /// 先手优势
    FirstMoveAdvantage,
    /// 后手劣势
    SecondMoveDisadvantage,
}

impl Default for BalanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl BalanceAnalyzer {
    /// 创建新的平衡性分析器
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::balance::BalanceAnalyzer;
    ///
    /// let analyzer = BalanceAnalyzer::new();
    /// assert_eq!(analyzer.threshold, 0.7);
    /// ```
    pub fn new() -> Self {
        Self {
            threshold: 0.7,
            weights: BalanceWeights::default(),
        }
    }

    /// 设置平衡阈值
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::balance::BalanceAnalyzer;
    ///
    /// let analyzer = BalanceAnalyzer::new().with_threshold(0.8);
    /// assert_eq!(analyzer.threshold, 0.8);
    /// ```
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// 设置权重配置
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::balance::{BalanceAnalyzer, BalanceWeights};
    ///
    /// let weights = BalanceWeights::default();
    /// let analyzer = BalanceAnalyzer::new().with_weights(weights);
    /// assert_eq!(analyzer.weights.strategy_diversity, 0.25);
    /// ```
    pub fn with_weights(mut self, weights: BalanceWeights) -> Self {
        self.weights = weights;
        self
    }

    /// 简单分析（基于规则名称和标签）
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::balance::BalanceAnalyzer;
    ///
    /// let analyzer = BalanceAnalyzer::new();
    /// let report = analyzer.analyze_simple("围棋", vec!["策略", "回合制"]);
    /// assert!(report.score >= 0.0);
    /// ```
    pub fn analyze_simple(&self, game_name: &str, tags: Vec<&str>) -> BalanceReport {
        let mut dimension_scores = HashMap::new();
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // 基于标签进行简单分析
        let strategy_score = if tags.contains(&"策略") { 0.85 } else { 0.7 };
        dimension_scores.insert("策略多样性".to_string(), strategy_score);

        let resource_score = if tags.contains(&"资源管理") {
            0.8
        } else {
            0.75
        };
        dimension_scores.insert("资源平衡".to_string(), resource_score);

        let character_score = if tags.contains(&"角色选择") {
            0.75
        } else {
            0.8
        };
        dimension_scores.insert("角色平衡".to_string(), character_score);

        let timing_score = if tags.contains(&"回合制") {
            0.8
        } else {
            0.7
        };
        dimension_scores.insert("时间平衡".to_string(), timing_score);

        let info_score = if tags.contains(&"信息对称") {
            0.85
        } else {
            0.75
        };
        dimension_scores.insert("信息平衡".to_string(), info_score);

        // 检测常见问题
        if tags.contains(&"先手优势") {
            issues.push(BalanceIssue {
                issue_type: IssueType::FirstMoveAdvantage,
                description: "存在先手优势问题".to_string(),
                severity: 0.3,
                impact: "可能影响游戏公平性".to_string(),
                solution: Some("考虑补偿机制或随机化先手".to_string()),
            });
        }

        if tags.contains(&"资源不平衡") {
            issues.push(BalanceIssue {
                issue_type: IssueType::ResourceImbalance,
                description: "某些资源过于重要或过于稀缺".to_string(),
                severity: 0.4,
                impact: "导致策略单一化".to_string(),
                solution: Some("调整资源生成率或价值".to_string()),
            });
        }

        // 计算总分
        let total_score = strategy_score * self.weights.strategy_diversity
            + resource_score * self.weights.resource_balance
            + character_score * self.weights.character_balance
            + timing_score * self.weights.timing_balance
            + info_score * self.weights.information_balance;

        // 生成建议
        if total_score < self.threshold {
            recommendations.push("建议增加策略多样性".to_string());
        }
        if strategy_score < 0.7 {
            recommendations.push("丰富策略选择，避免单一最优解".to_string());
        }
        if resource_score < 0.7 {
            recommendations.push("调整资源系统，确保多种资源都有价值".to_string());
        }

        BalanceReport {
            game_name: game_name.to_string(),
            score: total_score,
            grade: self.calculate_grade(total_score),
            dimension_scores,
            issues,
            recommendations,
        }
    }

    /// 分析角色平衡性
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::balance::BalanceAnalyzer;
    ///
    /// let analyzer = BalanceAnalyzer::new();
    /// let report = analyzer.analyze_characters("游戏", vec![
    ///     ("战士", 100.0, 50.0, 30.0),
    ///     ("法师", 60.0, 100.0, 40.0),
    /// ]);
    /// assert!(report.score >= 0.0);
    /// ```
    pub fn analyze_characters(
        &self,
        game_name: &str,
        characters: Vec<(&str, f64, f64, f64)>,
    ) -> BalanceReport {
        let mut dimension_scores = HashMap::new();
        let mut issues = Vec::new();
        let recommendations = Vec::new();

        if characters.is_empty() {
            return BalanceReport {
                game_name: game_name.to_string(),
                score: 1.0,
                grade: BalanceGrade::Excellent,
                dimension_scores,
                issues,
                recommendations,
            };
        }

        // 计算各角色总战力
        let powers: Vec<f64> = characters
            .iter()
            .map(|(_, hp, atk, def)| hp + atk + def)
            .collect();

        let avg_power = powers.iter().sum::<f64>() / powers.len() as f64;
        let variance: f64 =
            powers.iter().map(|p| (p - avg_power).powi(2)).sum::<f64>() / powers.len() as f64;
        let std_dev = variance.sqrt();

        // 角色平衡分数（方差越小越好）
        let balance_score = if avg_power > 0.0 {
            1.0 - (std_dev / avg_power).min(1.0)
        } else {
            1.0
        };

        dimension_scores.insert("角色平衡".to_string(), balance_score);

        // 检测不平衡角色
        for (name, hp, atk, def) in &characters {
            let total = hp + atk + def;
            if total > avg_power * 1.2 {
                issues.push(BalanceIssue {
                    issue_type: IssueType::CharacterImbalance,
                    description: format!("角色 {} 过强", name),
                    severity: (total - avg_power) / avg_power,
                    impact: "可能导致所有玩家都选择该角色".to_string(),
                    solution: Some(format!("降低 {} 的属性值", name)),
                });
            } else if total < avg_power * 0.8 {
                issues.push(BalanceIssue {
                    issue_type: IssueType::CharacterImbalance,
                    description: format!("角色 {} 过弱", name),
                    severity: (avg_power - total) / avg_power,
                    impact: "该角色可能无人选择".to_string(),
                    solution: Some(format!("提高 {} 的属性值", name)),
                });
            }
        }

        BalanceReport {
            game_name: game_name.to_string(),
            score: balance_score,
            grade: self.calculate_grade(balance_score),
            dimension_scores,
            issues,
            recommendations,
        }
    }

    /// 分析资源平衡性
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::balance::BalanceAnalyzer;
    ///
    /// let analyzer = BalanceAnalyzer::new();
    /// let report = analyzer.analyze_resources("游戏", vec![
    ///     ("金币", 100.0, 0.5),
    ///     ("木材", 50.0, 0.3),
    /// ]);
    /// assert!(report.score >= 0.0);
    /// ```
    pub fn analyze_resources(
        &self,
        game_name: &str,
        resources: Vec<(&str, f64, f64)>,
    ) -> BalanceReport {
        let mut dimension_scores = HashMap::new();
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        if resources.is_empty() {
            return BalanceReport {
                game_name: game_name.to_string(),
                score: 1.0,
                grade: BalanceGrade::Excellent,
                dimension_scores,
                issues,
                recommendations,
            };
        }

        // 计算资源价值（数量 * 权重）
        let values: Vec<f64> = resources
            .iter()
            .map(|(_, amount, weight)| amount * weight)
            .collect();
        let total_value: f64 = values.iter().sum();
        let _avg_value = total_value / values.len() as f64;

        // 检测资源使用率
        let usage_rates: Vec<f64> = values.iter().map(|v| v / total_value).collect();
        let max_usage = usage_rates.iter().cloned().fold(0.0f64, f64::max);
        let min_usage = usage_rates.iter().cloned().fold(1.0f64, f64::min);

        // 平衡分数（使用率差距越小越好）
        let balance_score = 1.0 - (max_usage - min_usage);

        dimension_scores.insert("资源平衡".to_string(), balance_score);

        // 检测问题资源
        for (name, amount, weight) in &resources {
            let value = amount * weight;
            let usage = value / total_value;
            if usage > 0.5 {
                issues.push(BalanceIssue {
                    issue_type: IssueType::ResourceImbalance,
                    description: format!("资源 {} 过于重要", name),
                    severity: usage - 0.33,
                    impact: "其他资源价值降低".to_string(),
                    solution: Some(format!("降低 {} 的价值或增加其他资源的价值", name)),
                });
            }
        }

        if balance_score < self.threshold {
            recommendations.push("调整资源权重，确保多种资源都有价值".to_string());
        }

        BalanceReport {
            game_name: game_name.to_string(),
            score: balance_score,
            grade: self.calculate_grade(balance_score),
            dimension_scores,
            issues,
            recommendations,
        }
    }

    /// 计算平衡性等级
    fn calculate_grade(&self, score: f64) -> BalanceGrade {
        if score >= 0.9 {
            BalanceGrade::Excellent
        } else if score >= 0.8 {
            BalanceGrade::Good
        } else if score >= 0.7 {
            BalanceGrade::Average
        } else if score >= 0.6 {
            BalanceGrade::NeedsImprovement
        } else {
            BalanceGrade::Unbalanced
        }
    }
}

impl BalanceReport {
    /// 获取等级名称
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::balance::{BalanceAnalyzer, BalanceGrade};
    ///
    /// let analyzer = BalanceAnalyzer::new();
    /// let report = analyzer.analyze_simple("围棋", vec!["策略"]);
    /// let grade_name = report.grade_name();
    /// assert!(!grade_name.is_empty());
    /// ```
    pub fn grade_name(&self) -> &'static str {
        match self.grade {
            BalanceGrade::Excellent => "优秀",
            BalanceGrade::Good => "良好",
            BalanceGrade::Average => "一般",
            BalanceGrade::NeedsImprovement => "需改进",
            BalanceGrade::Unbalanced => "不平衡",
        }
    }

    /// 是否平衡
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::balance::BalanceAnalyzer;
    ///
    /// let analyzer = BalanceAnalyzer::new();
    /// let report = analyzer.analyze_simple("围棋", vec!["策略"]);
    /// // report.is_balanced() 返回布尔值
    /// ```
    pub fn is_balanced(&self) -> bool {
        self.score >= 0.7
    }

    /// 生成报告（Markdown格式）
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::balance::BalanceAnalyzer;
    ///
    /// let analyzer = BalanceAnalyzer::new();
    /// let report = analyzer.analyze_simple("围棋", vec!["策略"]);
    /// let md = report.generate_report();
    /// assert!(md.contains("围棋"));
    /// ```
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!("# {} 平衡性分析报告\n\n", self.game_name));
        report.push_str(&format!(
            "**总体评分**: {:.2}/1.0 ({})\n\n",
            self.score,
            self.grade_name()
        ));

        report.push_str("## 各维度评分\n\n");
        for (dimension, score) in &self.dimension_scores {
            report.push_str(&format!("- **{}**: {:.2}\n", dimension, score));
        }
        report.push('\n');

        if !self.issues.is_empty() {
            report.push_str("## 发现的问题\n\n");
            for issue in &self.issues {
                report.push_str(&format!("### {:?}\n\n", issue.issue_type));
                report.push_str(&format!("{}\n\n", issue.description));
                report.push_str(&format!("- **严重程度**: {:.2}\n", issue.severity));
                report.push_str(&format!("- **影响范围**: {}\n", issue.impact));
                if let Some(ref solution) = issue.solution {
                    report.push_str(&format!("- **建议解决**: {}\n", solution));
                }
                report.push('\n');
            }
        }

        if !self.recommendations.is_empty() {
            report.push_str("## 改进建议\n\n");
            for rec in &self.recommendations {
                report.push_str(&format!("- {}\n", rec));
            }
            report.push('\n');
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = BalanceAnalyzer::new();
        assert_eq!(analyzer.threshold, 0.7);
    }

    #[test]
    fn test_simple_analysis() {
        let analyzer = BalanceAnalyzer::new();
        let report = analyzer.analyze_simple("围棋", vec!["策略", "回合制"]);

        assert!(report.score >= 0.0);
        assert!(report.score <= 1.0);
        assert_eq!(report.game_name, "围棋");
    }

    #[test]
    fn test_character_balance() {
        let analyzer = BalanceAnalyzer::new();
        let report = analyzer.analyze_characters(
            "游戏",
            vec![("战士", 100.0, 50.0, 30.0), ("法师", 60.0, 100.0, 40.0)],
        );

        assert!(report.score >= 0.0);
        assert!(report.dimension_scores.contains_key("角色平衡"));
    }

    #[test]
    fn test_resource_balance() {
        let analyzer = BalanceAnalyzer::new();
        let report =
            analyzer.analyze_resources("游戏", vec![("金币", 100.0, 0.5), ("木材", 50.0, 0.3)]);

        assert!(report.score >= 0.0);
        assert!(report.dimension_scores.contains_key("资源平衡"));
    }

    #[test]
    fn test_grade_calculation() {
        let analyzer = BalanceAnalyzer::new();

        assert_eq!(analyzer.calculate_grade(0.95), BalanceGrade::Excellent);
        assert_eq!(analyzer.calculate_grade(0.85), BalanceGrade::Good);
        assert_eq!(analyzer.calculate_grade(0.75), BalanceGrade::Average);
        assert_eq!(
            analyzer.calculate_grade(0.65),
            BalanceGrade::NeedsImprovement
        );
        assert_eq!(analyzer.calculate_grade(0.5), BalanceGrade::Unbalanced);
    }

    #[test]
    fn test_report_generation() {
        let analyzer = BalanceAnalyzer::new();
        let report = analyzer.analyze_simple("围棋", vec!["策略"]);
        let md = report.generate_report();

        assert!(md.contains("围棋"));
        assert!(md.contains("平衡性分析报告"));
    }

    #[test]
    fn test_threshold_setting() {
        let analyzer = BalanceAnalyzer::new().with_threshold(0.8);
        assert_eq!(analyzer.threshold, 0.8);
    }

    #[test]
    fn test_empty_analysis() {
        let analyzer = BalanceAnalyzer::new();
        let report = analyzer.analyze_characters("游戏", vec![]);
        assert_eq!(report.score, 1.0);
    }
}
