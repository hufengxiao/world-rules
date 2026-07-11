//! 统计学规则
//!
//! 统计学是收集、分析、解释和呈现数据的科学。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: StatisticsRules,
    name: "统计学规则",
    desc: "统计方法、数据分析与推断规则",
    origin: "数学",
    tags: ["科学", "数学", "统计"]
}

impl StatisticsRules {
    /// 描述统计
    pub fn descriptive_statistics(&self) -> Vec<&'static str> {
        vec![
            "集中趋势: 均值、中位数、众数",
            "均值: μ = ∑x/n，数据的平均值",
            "中位数: 排序后中间位置的值",
            "众数: 出现次数最多的值",
            "离散程度: 方差、标准差、极差",
            "方差: σ² = ∑(x-μ)²/n",
            "标准差: σ = √σ²",
            "极差: 最大值 - 最小值",
        ]
    }

    /// 抽样方法
    pub fn sampling_methods(&self) -> Vec<&'static str> {
        vec![
            "简单随机抽样: 每个样本有相同概率",
            "分层抽样: 将总体分层后在各层抽样",
            "系统抽样: 按固定间隔抽样",
            "整群抽样: 抽取若干群体",
            "样本大小: n ≥ 30 为大样本",
            "抽样误差: 样本与总体参数的差异",
            "无偏抽样: 样本代表性好",
            "样本设计: 根据研究目的选择方法",
        ]
    }

    /// 分布类型
    pub fn distribution_types(&self) -> Vec<&'static str> {
        vec![
            "正态分布: 钟形曲线，μ 和 σ 参数",
            "偏态分布: 数据不对称分布",
            "左偏: 长尾在左侧",
            "右偏: 长尾在右侧",
            "偏态系数: 测量偏斜程度",
            "峰态: 分布峰的尖锐程度",
            "峰态系数: 正态分布为 0",
            "标准化: Z = (x-μ)/σ",
        ]
    }

    /// 参数估计
    pub fn parameter_estimation(&self) -> Vec<&'static str> {
        vec![
            "点估计: 用单个值估计参数",
            "矩估计: 用样本矩估计总体矩",
            "极大似然估计: 使似然函数最大",
            "置信区间: 参数的区间估计",
            "置信水平: 如 95%、99%",
            "标准误: 估计量的标准差",
            "无偏估计: 期望等于真实参数",
            "有效估计: 方差最小的无偏估计",
        ]
    }

    /// 假设检验
    pub fn hypothesis_testing(&self) -> Vec<&'static str> {
        vec![
            "零假设 H₀: 待检验的假设",
            "备择假设 H₁: 与 H₀ 对立的假设",
            "显著性水平 α: 拒绝 H₀ 的概率阈值",
            "检验统计量: 用于判断的统计量",
            "拒绝域: 统计量落入则拒绝 H₀",
            "P 值: 观察到的极端概率",
            "单侧检验: 检验大于或小于",
            "双侧检验: 检验不等于",
        ]
    }

    /// 检验类型
    pub fn test_types(&self) -> Vec<&'static str> {
        vec![
            "Z 检验: 大样本均值检验",
            "t 检验: 小样本均值检验",
            "卡方检验: 分类数据的检验",
            "F 检验: 方差比较检验",
            "ANOVA: 多组均值比较",
            "非参数检验: 不依赖分布的检验",
            "Mann-Whitney U 检验: 两独立样本",
            "Wilcoxon 检验: 配对样本检验",
        ]
    }

    /// 相关与回归
    pub fn correlation_and_regression(&self) -> Vec<&'static str> {
        vec![
            "相关系数 r: 测量线性关系强度",
            "正相关: r > 0，同向变化",
            "负相关: r < 0，反向变化",
            "散点图: 显示两变量关系",
            "线性回归: y = a + bx",
            "最小二乘法: 使残差平方和最小",
            "回归系数: b 表示斜率",
            "R²: 模型解释的变异比例",
        ]
    }

    /// 多元分析
    pub fn multivariate_analysis(&self) -> Vec<&'static str> {
        vec![
            "多元回归: 多个自变量的回归",
            "偏相关: 控制其他变量的相关性",
            "因子分析: 找出潜在因子",
            "主成分分析: 降维技术",
            "聚类分析: 数据分组",
            "判别分析: 分类预测",
            "路径分析: 因果关系分析",
            "结构方程模型: 检验复杂关系",
        ]
    }

    /// 时间序列
    pub fn time_series(&self) -> Vec<&'static str> {
        vec![
            "趋势分析: 长期变化趋势",
            "季节性: 周期性波动",
            "移动平均: 平滑时间序列",
            "自相关: 与滞后值的相关性",
            "ARIMA 模型: 自回归积分移动平均",
            "指数平滑: 加权移动平均",
            "预测: 未来值估计",
            "平稳性: 统计特性不随时间变化",
        ]
    }

    /// 应用领域
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "质量控制: 产品检验、过程控制",
            "市场研究: 消费者调查、需求预测",
            "医学统计: 临床试验、流行病学",
            "社会科学: 民意调查、社会研究",
            "金融统计: 风险分析、投资组合",
            "生物统计: 基因分析、生态统计",
            "工程统计: 可靠性、寿命分析",
            "体育统计: 运动分析、成绩统计",
        ]
    }
}

impl Rule for StatisticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("statistics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "统计学规则",
            &[
                ("描述统计", &self.descriptive_statistics()),
                ("抽样方法", &self.sampling_methods()),
                ("分布类型", &self.distribution_types()),
                ("参数估计", &self.parameter_estimation()),
                ("假设检验", &self.hypothesis_testing()),
                ("检验类型", &self.test_types()),
                ("相关与回归", &self.correlation_and_regression()),
                ("多元分析", &self.multivariate_analysis()),
                ("时间序列", &self.time_series()),
                ("应用领域", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_rules() {
        let rules = StatisticsRules::new();
        assert_eq!(rules.metadata().name, "统计学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.descriptive_statistics().is_empty());
        assert!(!rules.sampling_methods().is_empty());
        assert!(!rules.distribution_types().is_empty());
        assert!(!rules.parameter_estimation().is_empty());
        assert!(!rules.hypothesis_testing().is_empty());
        assert!(!rules.test_types().is_empty());
        assert!(!rules.correlation_and_regression().is_empty());
        assert!(!rules.multivariate_analysis().is_empty());
        assert!(!rules.time_series().is_empty());
        assert!(!rules.applications().is_empty());
    }
}
