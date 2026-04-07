//! 统计学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 统计学规则
pub struct StatisticsRules {
    metadata: RuleMetadata,
}

impl StatisticsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "统计学定律",
                "统计学基本定律和方法"
            )
            .with_origin("统计学")
            .with_tags(vec!["科学".into(), "统计学".into()]),
        }
    }

    /// 大数定律
    pub fn law_of_large_numbers(&self) -> Vec<&'static str> {
        vec![
            "弱大数定律: 样本均值趋近于期望值",
            "强大数定律: 样本均值几乎必然收敛于期望值",
            "样本量越大，样本均值越稳定",
            "赌博长期必输的原因",
        ]
    }

    /// 中心极限定理
    pub fn central_limit_theorem(&self) -> Vec<&'static str> {
        vec![
            "大量独立随机变量之和近似正态分布",
            "样本均值的分布近似正态分布",
            "无论总体分布如何，样本均值趋近正态",
            "统计推断的理论基础",
        ]
    }

    /// 常用分布
    pub fn common_distributions(&self) -> Vec<&'static str> {
        vec![
            "正态分布: 钟形曲线",
            "二项分布: n次独立试验成功次数",
            "泊松分布: 单位时间事件发生次数",
            "t分布: 小样本估计",
            "卡方分布: 方差检验",
        ]
    }

    /// 统计检验
    pub fn statistical_tests(&self) -> Vec<&'static str> {
        vec![
            "假设检验: 原假设和备择假设",
            "显著性水平: 通常取0.05",
            "p值: 拒绝原假设的概率",
            "置信区间: 参数估计的范围",
        ]
    }

    /// 相关与回归
    pub fn correlation_regression(&self) -> Vec<&'static str> {
        vec![
            "相关系数: -1到1，衡量线性关系",
            "线性回归: y = ax + b",
            "决定系数R²: 模型拟合程度",
            "相关不等于因果",
        ]
    }

    /// 常见谬误
    pub fn statistical_fallacies(&self) -> Vec<&'static str> {
        vec![
            "辛普森悖论: 分组与整体结论相反",
            "选择性偏差: 样本不具代表性",
            "幸存者偏差: 只考虑幸存者",
            "数据挖掘偏差: 过度拟合历史数据",
        ]
    }
}

impl Default for StatisticsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for StatisticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("statistics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【统计学定律】\n\n\
            大数定律:\n{}\n\n\
            中心极限定理:\n{}\n\n\
            常用分布:\n{}\n\n\
            常见谬误:\n{}\n",
            self.law_of_large_numbers().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.central_limit_theorem().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.common_distributions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.statistical_fallacies().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_rules() {
        let rules = StatisticsRules::new();
        assert!(!rules.common_distributions().is_empty());
    }
}