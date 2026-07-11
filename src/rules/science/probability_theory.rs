//! 概率论规则
//!
//! 概率论研究随机现象的规律性，是统计学和数据分析的理论基础。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ProbabilityTheoryRules,
    name: "概率论规则",
    desc: "概率论基础定律与应用方法",
    origin: "数学",
    tags: ["科学", "数学", "概率"]
}

impl ProbabilityTheoryRules {
    /// 概率基础
    pub fn probability_basics(&self) -> Vec<&'static str> {
        vec![
            "概率定义: 事件发生的可能性，0 ≤ P(A) ≤ 1",
            "样本空间: 所有可能结果的集合 Ω",
            "事件: 样本空间的子集",
            "基本事件: 不能再分的最简单事件",
            "复合事件: 由多个基本事件组成",
            "必然事件: P(Ω) = 1",
            "不可能事件: P(∅) = 0",
            "互补事件: P(Ā) = 1 - P(A)",
        ]
    }

    /// 概率公式
    pub fn probability_formulas(&self) -> Vec<&'static str> {
        vec![
            "加法公式: P(A∪B) = P(A) + P(B) - P(A∩B)",
            "互斥事件: A∩B = ∅，P(A∪B) = P(A) + P(B)",
            "乘法公式: P(A∩B) = P(A)·P(B|A)",
            "独立事件: P(A∩B) = P(A)·P(B)",
            "条件概率: P(B|A) = P(A∩B)/P(A)",
            "全概率公式: P(B) = ∑P(B|A_i)·P(A_i)",
            "贝叶斯公式: P(A|B) = P(B|A)·P(A)/P(B)",
            "链式法则: P(A₁∩...∩A_n) = P(A₁)·P(A₂|A₁)·...·P(A_n|A₁...A_{n-1})",
        ]
    }

    /// 随机变量
    pub fn random_variables(&self) -> Vec<&'static str> {
        vec![
            "随机变量定义: 从样本空间到实数的函数",
            "离散随机变量: 取有限或可数无限个值",
            "连续随机变量: 取某区间内任意值",
            "分布函数: F(x) = P(X ≤ x)",
            "概率密度: f(x)，F(x) = ∫f(t)dt",
            "期望: E(X) = ∑x·p(x) 或 ∫x·f(x)dx",
            "方差: Var(X) = E(X²) - [E(X)]²",
            "标准差: σ = √Var(X)",
        ]
    }

    /// 重要分布
    pub fn important_distributions(&self) -> Vec<&'static str> {
        vec![
            "伯努利分布: 取值0或1，P(X=1) = p",
            "二项分布: B(n,p)，n次独立伯努利试验",
            "泊松分布: P(λ)，稀有事件计数模型",
            "几何分布: 首次成功所需的试验次数",
            "正态分布: N(μ,σ²)，钟形曲线",
            "均匀分布: 在区间[a,b]上均匀分布",
            "指数分布: 等待时间模型",
            "卡方分布: χ²分布，统计检验常用",
        ]
    }

    /// 大数定律
    pub fn laws_of_large_numbers(&self) -> Vec<&'static str> {
        vec![
            "伯努利大数定律: 频率收敛到概率",
            "辛钦大数定律: 样本均值收敛到期望",
            "切比雪夫大数定律: 需要方差有界",
            "强大数定律: 几乎处处收敛",
            "弱大数定律: 依概率收敛",
            "应用: 保险精算、质量检验",
            "频率稳定性: 大量试验的规律性",
            "蒙特卡洛方法: 基于大数定律的模拟",
        ]
    }

    /// 中心极限定理
    pub fn central_limit_theorem(&self) -> Vec<&'static str> {
        vec![
            "林德伯格-列维定理: 独立同分布随机变量和",
            "李雅普诺夫定理: 独立随机变量",
            "棣莫弗-拉普拉斯定理: 二项分布极限",
            "标准化: Z = (X-μ)/(σ/√n) → N(0,1)",
            "应用: 抽样分布、置信区间",
            "近似正态: 许多分布在大样本下近似正态",
            "重要性: 统计推断的理论基础",
            "收敛速度: Berry-Esseen 界",
        ]
    }

    /// 多元概率
    pub fn multivariate_probability(&self) -> Vec<&'static str> {
        vec![
            "联合分布: P(X,Y) 或 f(x,y)",
            "边缘分布: 从联合分布推导",
            "条件分布: P(Y|X=x)",
            "协方差: Cov(X,Y) = E[(X-μ_X)(Y-μ_Y)]",
            "相关系数: ρ = Cov(X,Y)/(σ_X·σ_Y)",
            "独立性: f(x,y) = f_X(x)·f_Y(y)",
            "多元正态分布: N(μ, Σ)",
            "协方差矩阵: Σ 描述多元相关性",
        ]
    }

    /// 随机过程
    pub fn stochastic_processes(&self) -> Vec<&'static str> {
        vec![
            "随机过程定义: 随时间演化的随机变量族",
            "马尔可夫过程: 无后效性",
            "泊松过程: 随机到达过程",
            "布朗运动: 连续时间随机过程",
            "鞅: 期望保持不变的过程",
            "平稳过程: 统计特性不随时间变化",
            "应用: 金融数学、信号处理",
            "扩散过程: 连续状态的马尔可夫过程",
        ]
    }

    /// 统计推断基础
    pub fn statistical_inference_basics(&self) -> Vec<&'static str> {
        vec![
            "参数估计: 点估计、区间估计",
            "矩估计法: 用样本矩估计参数",
            "极大似然估计: 使似然函数最大",
            "无偏性: E(θ̂) = θ",
            "有效性: 估计量的方差最小",
            "一致性: n→∞时θ̂→θ",
            "置信区间: 参数的区间估计",
            "假设检验: 检验参数假设",
        ]
    }

    /// 应用领域
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "风险评估: 金融、工程风险分析",
            "质量控制: 产品检验、过程控制",
            "保险精算: 保费计算、理赔预测",
            "医学统计: 临床试验、流行病学",
            "市场调研: 消费者行为分析",
            "机器学习: 贝叶斯方法、概率模型",
            "通信系统: 信道容量、噪声分析",
            "可靠性工程: 失效概率、寿命预测",
        ]
    }
}

impl Rule for ProbabilityTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("probability_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "概率论规则",
            &[
                ("概率基础", &self.probability_basics()),
                ("概率公式", &self.probability_formulas()),
                ("随机变量", &self.random_variables()),
                ("重要分布", &self.important_distributions()),
                ("大数定律", &self.laws_of_large_numbers()),
                ("中心极限定理", &self.central_limit_theorem()),
                ("多元概率", &self.multivariate_probability()),
                ("随机过程", &self.stochastic_processes()),
                ("统计推断基础", &self.statistical_inference_basics()),
                ("应用领域", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probability_rules() {
        let rules = ProbabilityTheoryRules::new();
        assert_eq!(rules.metadata().name, "概率论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.probability_basics().is_empty());
        assert!(!rules.probability_formulas().is_empty());
        assert!(!rules.random_variables().is_empty());
        assert!(!rules.important_distributions().is_empty());
        assert!(!rules.laws_of_large_numbers().is_empty());
        assert!(!rules.central_limit_theorem().is_empty());
        assert!(!rules.multivariate_probability().is_empty());
        assert!(!rules.stochastic_processes().is_empty());
        assert!(!rules.statistical_inference_basics().is_empty());
        assert!(!rules.applications().is_empty());
    }
}
