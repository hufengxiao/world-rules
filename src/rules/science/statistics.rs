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

    /// 概率分布
    pub fn probability_distributions(&self) -> Vec<&'static str> {
        vec![
            "正态分布: 68-95-99.7法则描述数据在均值附近的分布",
            "泊松分布: 描述单位时间内稀有事件发生次数的分布",
            "二项分布: n次独立重复试验中成功次数的概率分布",
            "指数分布: 描述独立事件间隔时间的连续分布",
            "卡方分布: 多个独立标准正态变量平方和的分布",
            "t分布: 小样本情况下均值检验的分布",
            "F分布: 两个独立卡方变量之比的分布",
        ]
    }

    /// 假设检验
    pub fn hypothesis_testing(&self) -> Vec<&'static str> {
        vec![
            "第一类错误: 拒绝了真实的原假设即假阳性",
            "第二类错误: 接受了虚假的原假设即假阴性",
            "p值: 在原假设为真时观察到当前或更极端结果的概率",
            "置信区间: 以特定概率包含总体参数的区间估计",
            "功效分析: 统计检验能正确拒绝假假设的概率",
            "多重比较问题: 多次检验导致第一类错误膨胀",
            "Bonferroni校正: 将显著性水平除以比较次数控制族错误率",
        ]
    }

    /// 回归与相关
    pub fn regression_methods(&self) -> Vec<&'static str> {
        vec![
            "皮尔逊相关系数: 衡量两个变量线性相关程度的指标",
            "斯皮尔曼秩相关: 基于秩次的非参数相关度量",
            "最小二乘法: 使残差平方和最小的参数估计方法",
            "多元回归: 一个因变量与多个自变量之间线性关系",
            "逻辑回归: 用于分类问题的广义线性模型",
            "R方: 模型解释的变异占总变异的比例",
            "共线性: 自变量高度相关导致回归系数不稳定",
        ]
    }


    /// 贝叶斯统计
    pub fn bayesian_statistics(&self) -> Vec<&'static str> {
        vec![
            "贝叶斯定理: P(A|B)=P(B|A)P(A)/P(B)",
            "先验分布: 在观测数据之前对参数的信念",
            "后验分布: 结合先验和数据后的参数分布",
            "贝叶斯推断: 基于后验分布进行参数估计",
            "MCMC方法: 马尔科夫链蒙特卡洛采样方法",
            "贝叶斯因子: 两个模型相对证据的比值",
            "可信区间: 贝叶斯框架下参数的区间估计",
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